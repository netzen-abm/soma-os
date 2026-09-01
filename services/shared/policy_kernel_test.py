import unittest
from pathlib import Path

from policy_kernel import Decision, PolicyKernel, PolicyRequest


REGISTRY = Path(__file__).with_name("capability_registry.json")


class PolicyKernelTests(unittest.TestCase):
    def setUp(self):
        self.kernel = PolicyKernel.from_registry_file(REGISTRY)
        self.request = PolicyRequest(
            principal_id="agent-1",
            principal_type="agent",
            capability_id="evidence.research",
            resource_type="research-source",
            resource_id="source-1",
            action="read",
            context={},
        )

    def test_registered_capability_requires_authorization(self):
        result = self.kernel.evaluate(self.request)
        self.assertEqual(result.decision, Decision.DENY)
        self.assertEqual(result.reason_code, "authorization_required")

    def test_authorized_registered_capability_allows(self):
        request = self._with_context(grant="true")
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.ALLOW)

    def test_unknown_capability_fails_closed(self):
        request = self.request.__class__(
            **{
                **self.request.__dict__,
                "capability_id": "unknown.capability",
                "context": {"grant": "true"},
            }
        )
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.DENY)
        self.assertEqual(result.reason_code, "unknown_capability")

    def test_missing_identity_fails_closed(self):
        request = self.request.__class__(
            **{**self.request.__dict__, "principal_id": ""}
        )
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.DENY)

    def test_human_review_precedes_grant(self):
        request = self._with_context(
            human_review="required",
            grant="true",
        )
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.REQUIRE_HUMAN_REVIEW)

    def test_consent_precedes_grant(self):
        request = self._with_context(
            consent="required",
            grant="true",
        )
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.REQUIRE_CONSENT)

    def test_explicit_deny_overrides_grant(self):
        request = self._with_context(
            deny="true",
            grant="true",
        )
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.DENY)

    def test_degrade_requires_grant(self):
        request = self._with_context(
            degrade="true",
            grant="true",
        )
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.DEGRADE)

    def _with_context(self, **context):
        return self.request.__class__(
            **{**self.request.__dict__, "context": context}
        )


if __name__ == "__main__":
    unittest.main()
