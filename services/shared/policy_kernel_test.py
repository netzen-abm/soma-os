import unittest

from policy_kernel import Decision, PolicyKernel, PolicyRequest


class PolicyKernelTests(unittest.TestCase):
    def setUp(self):
        self.kernel = PolicyKernel()
        self.request = PolicyRequest(
            principal_id="agent-1",
            principal_type="agent",
            capability_id="evidence.research",
            resource_type="research-source",
            resource_id="source-1",
            action="read",
            context={},
        )

    def test_valid_request_allows(self):
        result = self.kernel.evaluate(self.request)
        self.assertEqual(result.decision, Decision.ALLOW)

    def test_missing_identity_fails_closed(self):
        request = PolicyRequest(
            principal_id="",
            principal_type="agent",
            capability_id="evidence.research",
            resource_type="research-source",
            resource_id="source-1",
            action="read",
            context={},
        )
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.DENY)

    def test_human_review_has_priority(self):
        request = self.request.__class__(
            **{**self.request.__dict__, "context": {"human_review": "required"}}
        )
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.REQUIRE_HUMAN_REVIEW)

    def test_consent_required(self):
        request = self.request.__class__(
            **{**self.request.__dict__, "context": {"consent": "required"}}
        )
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.REQUIRE_CONSENT)

    def test_explicit_deny(self):
        request = self.request.__class__(
            **{**self.request.__dict__, "context": {"deny": "true"}}
        )
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.DENY)

    def test_degrade(self):
        request = self.request.__class__(
            **{**self.request.__dict__, "context": {"degrade": "true"}}
        )
        result = self.kernel.evaluate(request)
        self.assertEqual(result.decision, Decision.DEGRADE)


if __name__ == "__main__":
    unittest.main()
