import unittest
from pathlib import Path

from policy_kernel import Decision, PolicyKernel, PolicyRequest


REGISTRY = Path(__file__).with_name("capability_registry.json")
GRANTS = {
    (
        "agent-1",
        "agent",
        "evidence.research",
        "research-source",
        "source-1",
        "read",
    ): True,
}


class PolicyKernelTests(unittest.TestCase):
    def setUp(self):
        self.kernel = PolicyKernel.from_registry_file(REGISTRY, GRANTS)
        self.request = PolicyRequest(
            principal_id="agent-1",
            principal_type="agent",
            capability_id="evidence.research",
            resource_type="research-source",
            resource_id="source-1",
            action="read",
            context={},
        )

    def test_registered_capability_uses_resource_instance_grant(self):
        result = self.kernel.evaluate(self.request)
        self.assertEqual(result.decision, Decision.ALLOW)

    def test_different_resource_id_fails_closed(self):
        request = self._with_request(resource_id="source-2")
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.DENY)
        self.assertEqual(result.reason_code, "authorization_required")

    def test_different_identity_fails_closed(self):
        request = self._with_request(principal_id="agent-2")
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.DENY)
        self.assertEqual(result.reason_code, "authorization_required")

    def test_principal_type_mismatch_fails_closed(self):
        request = self._with_request(principal_type="user")
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.DENY)
        self.assertEqual(result.reason_code, "principal_type_mismatch")

    def test_resource_type_is_part_of_grant(self):
        request = self._with_request(resource_type="user-profile")
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.DENY)
        self.assertEqual(result.reason_code, "authorization_required")

    def test_missing_grant_fails_closed(self):
        request = self._with_request(action="write")
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.DENY)
        self.assertEqual(result.reason_code, "authorization_required")

    def test_unknown_capability_fails_closed(self):
        request = self._with_request(capability_id="unknown.capability")
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.DENY)
        self.assertEqual(result.reason_code, "unknown_capability")

    def test_invalid_capability_principal_types_fail_closed(self):
        registry = {"capabilities": [{"id": "broken.capability"}]}
        kernel = PolicyKernel(registry, {})
        request = self._with_request(capability_id="broken.capability")
        result = kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.DENY)
        self.assertEqual(result.reason_code, "invalid_capability_principal_types")

    def test_missing_identity_fails_closed(self):
        request = self._with_request(principal_id="")
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.DENY)

    def test_malformed_context_fails_closed(self):
        request = self._with_request(context=None)
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.DENY)
        self.assertEqual(result.reason_code, "invalid_request")

    def test_context_cannot_change_resource_scope(self):
        request = self._with_context(resource_id="source-2")
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.DENY)
        self.assertEqual(result.reason_code, "authorization_required")

    def test_human_review_precedes_grant(self):
        request = self._with_context(human_review="required")
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.REQUIRE_HUMAN_REVIEW)

    def test_consent_precedes_grant(self):
        request = self._with_context(consent="required")
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.REQUIRE_CONSENT)

    def test_explicit_deny_overrides_grant(self):
        request = self._with_context(deny="true")
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.DENY)

    def test_degrade_requires_grant(self):
        request = self._with_context(degrade="true")
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.DEGRADE)

    def _with_context(self, **context):
        return self._with_request(context=context)

    def _with_request(self, **changes):
        values = {
            **self.request.__dict__,
            **changes,
        }
        return self.request.__class__(**values)


if __name__ == "__main__":
    unittest.main()
