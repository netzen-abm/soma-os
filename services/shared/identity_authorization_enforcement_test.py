import unittest
from datetime import datetime, timezone

from identity_authorization_enforcement import enforce_authorized_operation
from policy_kernel import Decision, PolicyKernel, PolicyRequest


class IdentityAuthorizationEnforcementTests(unittest.TestCase):
    def setUp(self):
        self.registry = {
            "capabilities": [
                {"id": "health.read", "principal_types": ["person", "agent", "service"]}
            ]
        }
        self.principal = ("p-1", "person", "health.read", "record", "r-1", "read")
        self.kernel = PolicyKernel(self.registry, {self.principal: True})
        self.context = self._context("authenticated_account")

    def _context(self, mode):
        return {
            "context_id": "ctx-1",
            "schema_version": "1.1.0",
            "principal_id": "p-1",
            "principal_type": "person",
            "identity_mode": mode,
            "authentication_status": "VERIFIED",
            "authentication_provenance": {
                "issuer": "auth.example" if mode == "authenticated_account" else "local-runtime",
                "method": "oidc" if mode == "authenticated_account" else "local-security-context",
                "verified_at": "2026-09-04T10:00:00Z",
                "verifier_id": "verifier-1",
            },
            "tenant_scope": ["tenant-a"],
            "data_domain_scope": ["health"],
            "assurance_level": "HIGH" if mode == "authenticated_account" else "LOW",
            "issued_at": "2026-09-04T10:00:00Z",
            "expires_at": "2026-09-04T20:00:00Z",
        }

    def request(self, **overrides):
        values = dict(
            principal_id="p-1", principal_type="person", capability_id="health.read",
            resource_type="record", resource_id="r-1", action="read", context={}, subject_ref=None
        )
        values.update(overrides)
        return PolicyRequest(**values)

    def evaluate(self, context=None, tenant="tenant-a", domain="health", request=None, metadata=None, now=None):
        return enforce_authorized_operation(
            identity_context=context or self.context,
            target_tenant=tenant,
            target_data_domain=domain,
            request=request or self.request(),
            policy_kernel=self.kernel,
            caller_metadata=metadata,
            now=now,
        )

    def test_verified_matching_account_scope_and_exact_grant_allows(self):
        result = self.evaluate()
        self.assertTrue(result.allowed)
        self.assertEqual(result.policy_decision.decision, Decision.ALLOW)

    def test_anonymous_local_identity_is_accepted_as_verified_security_context(self):
        result = self.evaluate(context=self._context("anonymous_local"))
        self.assertTrue(result.allowed)
        self.assertEqual(result.policy_decision.decision, Decision.ALLOW)

    def test_anonymous_local_does_not_gain_authority_from_verified_status(self):
        context = self._context("anonymous_local")
        result = self.evaluate(context=context, request=self.request(capability_id="admin.write"))
        self.assertFalse(result.allowed)

    def test_authenticated_account_does_not_gain_authority_merely_from_login(self):
        context = self._context("authenticated_account")
        result = self.evaluate(context=context, request=self.request(capability_id="admin.write"))
        self.assertFalse(result.allowed)

    def test_login_mode_does_not_change_exact_capability_grant_semantics(self):
        account = self.evaluate(context=self._context("authenticated_account"))
        local = self.evaluate(context=self._context("anonymous_local"))
        self.assertEqual(account.allowed, local.allowed)
        self.assertEqual(account.reason_code, local.reason_code)

    def test_wrong_tenant_denied(self):
        result = self.evaluate(tenant="tenant-b")
        self.assertFalse(result.allowed)
        self.assertEqual(result.reason_code, "target_scope_mismatch")

    def test_wrong_data_domain_denied(self):
        result = self.evaluate(domain="billing")
        self.assertFalse(result.allowed)
        self.assertEqual(result.reason_code, "target_scope_mismatch")

    def test_unverified_denied(self):
        context = dict(self.context, authentication_status="UNVERIFIED")
        result = self.evaluate(context=context)
        self.assertFalse(result.allowed)
        self.assertEqual(result.reason_code, "invalid_identity_context")

    def test_expired_context_denied(self):
        now = datetime(2026, 9, 4, 21, 0, tzinfo=timezone.utc)
        result = self.evaluate(now=now)
        self.assertFalse(result.allowed)
        self.assertEqual(result.reason_code, "invalid_identity_context")

    def test_revoked_context_denied(self):
        context = dict(self.context, authentication_status="REVOKED")
        self.assertFalse(self.evaluate(context=context).allowed)

    def test_missing_provenance_denied(self):
        context = dict(self.context)
        context.pop("authentication_provenance")
        self.assertFalse(self.evaluate(context=context).allowed)

    def test_caller_cannot_broaden_tenant_scope(self):
        result = self.evaluate(metadata={"tenant_scope": ["tenant-a", "tenant-b"]})
        self.assertFalse(result.allowed)
        self.assertEqual(result.reason_code, "caller_metadata_scope_override")

    def test_caller_cannot_broaden_data_domain_scope(self):
        result = self.evaluate(metadata={"data_domain_scope": ["health", "billing"]})
        self.assertFalse(result.allowed)

    def test_model_or_mcp_metadata_cannot_override_identity(self):
        result = self.evaluate(metadata={"tenant_scope": ["tenant-b"], "source": "mcp"})
        self.assertFalse(result.allowed)

    def test_principal_mismatch_denied(self):
        result = self.evaluate(request=self.request(principal_id="p-2"))
        self.assertFalse(result.allowed)
        self.assertEqual(result.reason_code, "principal_identity_mismatch")

    def test_missing_resource_id_denied_and_never_wildcarded(self):
        result = self.evaluate(request=self.request(resource_id=""))
        self.assertFalse(result.allowed)
        self.assertEqual(result.reason_code, "invalid_policy_request")

    def test_exact_resource_instance_stays_enforced(self):
        result = self.evaluate(request=self.request(resource_id="r-2"))
        self.assertFalse(result.allowed)
        self.assertEqual(result.policy_decision.reason_code, "authorization_required")

    def test_adapter_cannot_override_verified_principal_type(self):
        result = self.evaluate(metadata={"principal_type": "agent"})
        self.assertFalse(result.allowed)

    def test_adapter_cannot_upgrade_identity_mode_or_assurance(self):
        result = self.evaluate(context=self._context("anonymous_local"), metadata={"identity_mode": "authenticated_account", "assurance_level": "HIGH"})
        self.assertFalse(result.allowed)

    def test_forbidden_token_material_denied(self):
        context = dict(self.context, access_token="secret")
        self.assertFalse(self.evaluate(context=context).allowed)

    def test_long_running_execution_requires_current_valid_context(self):
        now = datetime(2026, 9, 4, 19, 0, tzinfo=timezone.utc)
        self.assertTrue(self.evaluate(now=now).allowed)
        now = datetime(2026, 9, 4, 21, 0, tzinfo=timezone.utc)
        self.assertFalse(self.evaluate(now=now).allowed)

    def test_unknown_or_ambiguous_target_fails_closed(self):
        self.assertFalse(self.evaluate(tenant="").allowed)
        self.assertFalse(self.evaluate(domain="").allowed)


if __name__ == "__main__":
    unittest.main(verbosity=2)
