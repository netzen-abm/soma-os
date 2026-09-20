import unittest
from pathlib import Path

from policy_kernel import Decision, PolicyKernel, PolicyRequest


REGISTRY = Path(__file__).with_name("capability_registry.json")
GRANTS = {
    ("person-1", "person", "evidence.research", "research-source", "source-1", "read"): True,
}


class PolicyKernelTests(unittest.TestCase):
    def setUp(self):
        self.kernel = PolicyKernel.from_registry_file(REGISTRY, GRANTS)
        self.request = PolicyRequest(
            principal_id="person-1", principal_type="person", capability_id="evidence.research",
            resource_type="research-source", resource_id="source-1", action="read", context={}, subject_ref=None,
        )

    def _evaluate(self, request):
        identity = {
            "principal_id": request.principal_id,
            "principal_type": request.principal_type,
            "authentication_status": "VERIFIED",
            "identity_mode": "anonymous_local",
            "assurance_level": "LOW",
        }
        return self.kernel.evaluate(request, identity_context=identity)

    def test_registered_capability_uses_resource_instance_grant(self):
        identity = {
            "principal_id": "person-1",
            "principal_type": "person",
            "authentication_status": "VERIFIED",
            "identity_mode": "anonymous_local",
            "assurance_level": "LOW",
        }
        result = self.kernel.evaluate(self.request, identity_context=identity)
        self.assertEqual(result.decision, Decision.ALLOW)

    def test_subject_bound_grant_requires_exact_subject(self):
        grants = {("person-1", "person", "health.read", "health_record", "r-1", "read", "subject-1"): True}
        request = PolicyRequest(principal_id="person-1", principal_type="person", capability_id="health.read", resource_type="health_record", resource_id="r-1", action="read", context={}, subject_ref="subject-1")
        kernel = PolicyKernel({"capabilities": [{"id": "health.read", "principal_types": ["person"]}]}, grants)
        self.assertEqual(kernel.evaluate(request).decision, Decision.ALLOW)
        self.assertEqual(kernel.evaluate(PolicyRequest(principal_id="person-1", principal_type="person", capability_id="health.read", resource_type="health_record", resource_id="r-1", action="read", context={}, subject_ref="subject-2")).reason_code, "authorization_required")

    def test_different_resource_id_fails_closed(self):
        result = self._evaluate(self._with_request(resource_id="source-2"))
        self.assertEqual(result.reason_code, "authorization_required")

    def test_different_identity_fails_closed(self):
        result = self.kernel.evaluate(self._with_request(principal_id="person-2"))
        self.assertEqual(result.reason_code, "authorization_required")

    def test_principal_type_mismatch_fails_closed(self):
        result = self.kernel.evaluate(self._with_request(principal_type="agent"))
        self.assertEqual(result.reason_code, "principal_type_mismatch")

    def test_resource_type_is_part_of_grant(self):
        result = self.kernel.evaluate(self._with_request(resource_type="user-profile"))
        self.assertEqual(result.reason_code, "authorization_required")

    def test_missing_grant_fails_closed(self):
        result = self.kernel.evaluate(self._with_request(action="write"))
        self.assertEqual(result.reason_code, "authorization_required")

    def test_unknown_capability_fails_closed(self):
        result = self.kernel.evaluate(self._with_request(capability_id="unknown.capability"))
        self.assertEqual(result.reason_code, "unknown_capability")

    def test_invalid_capability_principal_types_fail_closed(self):
        kernel = PolicyKernel({"capabilities": [{"id": "broken.capability"}]}, {})
        result = kernel.evaluate(self._with_request(capability_id="broken.capability"))
        self.assertEqual(result.reason_code, "invalid_capability_principal_types")

    def test_missing_identity_fails_closed(self):
        result = self.kernel.evaluate(self._with_request(principal_id=""))
        self.assertEqual(result.decision, Decision.DENY)

    def test_malformed_context_fails_closed(self):
        result = self.kernel.evaluate(self._with_request(context=None))
        self.assertEqual(result.reason_code, "invalid_request")

    def test_context_cannot_change_resource_scope(self):
        result = self._evaluate(self._with_context(resource_id="source-2"))
        self.assertEqual(result.decision, Decision.ALLOW)

    def test_human_review_precedes_grant(self):
        result = self.kernel.evaluate(self._with_context(human_review="required"))
        self.assertEqual(result.decision, Decision.REQUIRE_HUMAN_REVIEW)

    def test_consent_precedes_grant(self):
        result = self.kernel.evaluate(self._with_context(consent="required"))
        self.assertEqual(result.decision, Decision.REQUIRE_CONSENT)

    def test_explicit_deny_overrides_grant(self):
        result = self.kernel.evaluate(self._with_context(deny="true"))
        self.assertEqual(result.decision, Decision.DENY)

    def test_degrade_requires_grant(self):
        result = self.kernel.evaluate(self._with_context(degrade="true"))
        self.assertEqual(result.decision, Decision.DEGRADE)

    def test_person_capability_requires_authenticated_account_when_declared(self):
        capability = {
            "id": "remote.sync", "principal_types": ["person"],
            "identity_requirements": {
                "applies_to_principal_types": ["person"],
                "allowed_identity_modes": ["authenticated_account"],
                "minimum_assurance_level": "LOW",
                "requires_durable_identity": True,
            },
        }
        request = PolicyRequest(principal_id="person-1", principal_type="person", capability_id="remote.sync", resource_type="vault", resource_id="v1", action="read", context={}, subject_ref=None)
        grants = {("person-1", "person", "remote.sync", "vault", "v1", "read"): True}
        kernel = PolicyKernel({"capabilities": [capability]}, grants)
        anonymous = {"principal_id": "person-1", "principal_type": "person", "authentication_status": "VERIFIED", "identity_mode": "anonymous_local", "assurance_level": "LOW"}
        result = kernel.evaluate(request, identity_context=anonymous)
        self.assertEqual(result.reason_code, "identity_mode_not_allowed")

    def test_durable_identity_requirement_cannot_be_satisfied_by_anonymous(self):
        capability = {"id": "share.vault", "principal_types": ["person"], "identity_requirements": {"applies_to_principal_types": ["person"], "allowed_identity_modes": ["anonymous_local", "authenticated_account"], "minimum_assurance_level": "LOW", "requires_durable_identity": True}}
        request = PolicyRequest(principal_id="person-1", principal_type="person", capability_id="share.vault", resource_type="vault", resource_id="v1", action="share", context={}, subject_ref=None)
        kernel = PolicyKernel({"capabilities": [capability]}, {("person-1", "person", "share.vault", "vault", "v1", "share"): True})
        anonymous = {"principal_id": "person-1", "principal_type": "person", "authentication_status": "VERIFIED", "identity_mode": "anonymous_local", "assurance_level": "LOW"}
        result = kernel.evaluate(request, identity_context=anonymous)
        self.assertEqual(result.reason_code, "durable_identity_required")

    def test_insufficient_assurance_fails_closed(self):
        capability = {"id": "high.assurance", "principal_types": ["person"], "identity_requirements": {"applies_to_principal_types": ["person"], "allowed_identity_modes": ["authenticated_account"], "minimum_assurance_level": "HIGH", "requires_durable_identity": False}}
        request = PolicyRequest(principal_id="person-1", principal_type="person", capability_id="high.assurance", resource_type="resource", resource_id="r1", action="read", context={}, subject_ref=None)
        kernel = PolicyKernel({"capabilities": [capability]}, {("person-1", "person", "high.assurance", "resource", "r1", "read"): True})
        account = {"principal_id": "person-1", "principal_type": "person", "authentication_status": "VERIFIED", "identity_mode": "authenticated_account", "assurance_level": "SUBSTANTIAL"}
        result = kernel.evaluate(request, identity_context=account)
        self.assertEqual(result.reason_code, "insufficient_identity_assurance")

    def test_valid_authenticated_account_satisfies_declared_requirement(self):
        capability = {"id": "remote.sync", "principal_types": ["person"], "identity_requirements": {"applies_to_principal_types": ["person"], "allowed_identity_modes": ["authenticated_account"], "minimum_assurance_level": "LOW", "requires_durable_identity": True}}
        request = PolicyRequest(principal_id="person-1", principal_type="person", capability_id="remote.sync", resource_type="vault", resource_id="v1", action="read", context={}, subject_ref="person-1")
        kernel = PolicyKernel({"capabilities": [capability]}, {("person-1", "person", "remote.sync", "vault", "v1", "read", "person-1"): True})
        account = {"principal_id": "person-1", "principal_type": "person", "authentication_status": "VERIFIED", "identity_mode": "authenticated_account", "assurance_level": "LOW"}
        result = kernel.evaluate(request, identity_context=account)
        self.assertEqual(result.decision, Decision.ALLOW)

    def test_malformed_identity_requirements_fail_closed(self):
        capability = {"id": "broken", "principal_types": ["person"], "identity_requirements": {"allowed_identity_modes": ["authenticated_account"]}}
        request = PolicyRequest(principal_id="person-1", principal_type="person", capability_id="broken", resource_type="resource", resource_id="r1", action="read", context={}, subject_ref=None)
        kernel = PolicyKernel({"capabilities": [capability]}, {})
        result = kernel.evaluate(request, identity_context={})
        self.assertEqual(result.reason_code, "invalid_identity_requirements")

    def _with_context(self, **context):
        return self._with_request(context=context)

    def _with_request(self, **changes):
        values = {**self.request.__dict__, **changes}
        return self.request.__class__(**values)


if __name__ == "__main__":
    unittest.main()
