from __future__ import annotations

import json
import unittest
from datetime import datetime, timezone
from pathlib import Path

from authorization_policy_decision_boundary import AuthorizationPolicyDecisionBoundary
from policy_kernel import Decision, PolicyKernel, PolicyRequest


ROOT = Path(__file__).resolve().parents[2]
REGISTRY_PATH = ROOT / "schemas/capability-registry-v1.json"


class AuthorizationPolicyDecisionBoundaryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.registry = json.loads(REGISTRY_PATH.read_text(encoding="utf-8"))
        cls.grant = ("principal-001", "person", "health.timeline.read", "health_observation", "observation-001", "read")
        cls.kernel = PolicyKernel(cls.registry, {cls.grant: True})
        cls.boundary = AuthorizationPolicyDecisionBoundary(cls.kernel)
        cls.now = datetime(2026, 9, 12, 8, 0, tzinfo=timezone.utc)

    def identity(self, **overrides):
        value = {
            "context_id": "ctx-001",
            "schema_version": "1.1.0",
            "principal_id": "principal-001",
            "principal_type": "person",
            "identity_mode": "authenticated_account",
            "authentication_status": "VERIFIED",
            "authentication_provenance": {
                "issuer": "test",
                "method": "test",
                "verified_at": "2026-09-12T07:00:00Z",
                "verifier_id": "verifier-001",
            },
            "tenant_scope": ["tenant-a"],
            "data_domain_scope": ["health"],
            "assurance_level": "HIGH",
            "issued_at": "2026-09-12T07:00:00Z",
            "expires_at": "2026-09-12T09:00:00Z",
        }
        value.update(overrides)
        return value

    def request(self, **overrides):
        value = {
            "principal_id": "principal-001",
            "principal_type": "person",
            "subject_ref": "subject-001",
            "capability_id": "health.timeline.read",
            "resource_type": "health_observation",
            "resource_id": "observation-001",
            "action": "read",
            "context": {},
        }
        value.update(overrides)
        return PolicyRequest(**value)

    def authorize(self, *, request=None, **kwargs):
        return self.boundary.authorize(
            request_id="req-001",
            identity_context=self.identity(),
            target_tenant="tenant-a",
            target_data_domain="health",
            request=request or self.request(),
            now=self.now,
            **kwargs,
        )

    def test_valid_request_produces_canonical_allow(self):
        result = self.authorize()
        self.assertIs(result.decision, Decision.ALLOW)
        self.assertEqual(result.policy_version, "0.4.0")
        self.assertEqual(result.reason_code, "authorized")
        self.assertEqual(result.enforcement_version, "1.1.0")
        self.assertEqual(result.decision_time, "2026-09-12T08:00:00Z")

    def test_wrong_principal_is_denied(self):
        result = self.authorize(request=self.request(principal_id="principal-evil"))
        self.assertIs(result.decision, Decision.DENY)
        self.assertEqual(result.reason_code, "principal_identity_mismatch")

    def test_wrong_tenant_is_denied(self):
        result = self.boundary.authorize(
            request_id="req-001", identity_context=self.identity(),
            target_tenant="tenant-b", target_data_domain="health",
            request=self.request(), now=self.now,
        )
        self.assertIs(result.decision, Decision.DENY)
        self.assertEqual(result.reason_code, "target_scope_mismatch")

    def test_wrong_data_domain_is_denied(self):
        result = self.boundary.authorize(
            request_id="req-001", identity_context=self.identity(),
            target_tenant="tenant-a", target_data_domain="billing",
            request=self.request(), now=self.now,
        )
        self.assertIs(result.decision, Decision.DENY)
        self.assertEqual(result.reason_code, "target_scope_mismatch")

    def test_caller_scope_override_is_denied(self):
        result = self.authorize(caller_metadata={"tenant_scope": ["tenant-a", "tenant-b"]})
        self.assertIs(result.decision, Decision.DENY)
        self.assertEqual(result.reason_code, "caller_metadata_scope_override")

    def test_expired_identity_is_denied(self):
        result = self.boundary.authorize(
            request_id="req-001", identity_context=self.identity(expires_at="2026-09-12T07:59:59Z"),
            target_tenant="tenant-a", target_data_domain="health",
            request=self.request(), now=self.now,
        )
        self.assertIs(result.decision, Decision.DENY)
        self.assertEqual(result.reason_code, "invalid_identity_context")

    def test_unknown_capability_is_denied(self):
        result = self.authorize(request=self.request(capability_id="health.unknown"))
        self.assertIs(result.decision, Decision.DENY)
        self.assertEqual(result.reason_code, "policy_unknown_capability")

    def test_all_non_allow_decisions_are_preserved(self):
        cases = (
            ({"deny": "true"}, Decision.DENY, "policy_policy_denied"),
            ({"consent": "required"}, Decision.REQUIRE_CONSENT, "policy_consent_required"),
            ({"human_review": "required"}, Decision.REQUIRE_HUMAN_REVIEW, "policy_human_review_required"),
            ({"degrade": "true"}, Decision.DEGRADE, "policy_degraded_execution_required"),
        )
        for context, expected, reason in cases:
            result = self.authorize(request=self.request(context=context))
            self.assertIs(result.decision, expected)
            self.assertEqual(result.reason_code, reason)

    def test_sensitive_payload_cannot_be_added_to_canonical_result(self):
        result = self.authorize()
        self.assertFalse(hasattr(result, "raw_payload"))
        self.assertFalse(hasattr(result, "credential"))
        self.assertFalse(hasattr(result, "token"))

    def test_naive_now_is_rejected(self):
        with self.assertRaises(ValueError):
            self.boundary.authorize(
                request_id="req-001", identity_context=self.identity(),
                target_tenant="tenant-a", target_data_domain="health",
                request=self.request(), now=datetime(2026, 9, 12, 8, 0),
            )


if __name__ == "__main__":
    unittest.main()
