import inspect
import unittest
from datetime import datetime, timezone

from authorization_policy_decision_boundary import AuthorizationPolicyDecisionBoundary
from governed_capability_executor import GovernedCapabilityExecutor
from policy_kernel import PolicyKernel, PolicyRequest


class GovernedCapabilityExecutorTests(unittest.TestCase):
    def setUp(self):
        self.executor = GovernedCapabilityExecutor()
        self.permission = {
            "permission_id": "perm-1", "operation_id": "op-1",
            "principal_ref": "person-1", "subject_ref": "subject-1",
            "capability_id": "device.camera.capture", "capability_version": "1.0.0",
            "purpose": "capture_health_image", "scope": ["camera:front"],
            "status": "ACTIVE", "granted_at": "2026-09-21T09:00:00Z",
            "expires_at": "2026-09-21T09:05:00Z", "auto_revoke": True,
            "regrant_requires_fresh_consent": True,
        }
        self.now = datetime(2026, 9, 21, 9, 2, tzinfo=timezone.utc)
        self.identity_context = {
            "context_id": "ctx-1", "schema_version": "1.1.0",
            "principal_id": "person-1", "principal_type": "person",
            "identity_mode": "authenticated_account", "authentication_status": "VERIFIED",
            "authentication_provenance": {
                "issuer": "test", "method": "test",
                "verified_at": "2026-09-21T09:00:00Z", "verifier_id": "test-verifier",
            },
            "tenant_scope": ["tenant-1"], "data_domain_scope": ["health"],
            "assurance_level": "HIGH", "issued_at": "2026-09-21T09:00:00Z",
        }
        self.policy_request = PolicyRequest(
            principal_id="person-1", principal_type="person",
            capability_id="device.camera.capture", capability_version="1.0.0",
            resource_type="device", resource_id="camera-1", action="capture",
            context={}, subject_ref="subject-1",
        )

    def boundary(self, allowed=True):
        grants = {
            ("person-1", "person", "device.camera.capture", "device",
             "camera-1", "capture", "subject-1"): True
        } if allowed else {}
        kernel = PolicyKernel({
            "capabilities": [{
                "id": "device.camera.capture", "version": "1.0.0",
                "principal_types": ["person"],
            }]
        }, grants)
        return AuthorizationPolicyDecisionBoundary(kernel)

    def execute(self, *, boundary, permission=None, operation):
        return self.executor.execute(
            operation_id="op-1", authorization_boundary=boundary,
            authorization_context=self.identity_context, target_tenant="tenant-1",
            target_data_domain="health", policy_request=self.policy_request,
            permission=self.permission if permission is None else permission,
            principal_ref="person-1", subject_ref="subject-1",
            capability_id="device.camera.capture", capability_version="1.0.0",
            purpose="capture_health_image", requested_scope=["camera:front"],
            operation=operation, now=self.now,
        )

    def test_executor_has_no_caller_supplied_authorization_boolean(self):
        self.assertNotIn("authorization_allowed", inspect.signature(self.executor.execute).parameters)

    def test_denied_canonical_authorization_never_executes(self):
        called = []
        result = self.execute(boundary=self.boundary(allowed=False), operation=lambda: called.append(True))
        self.assertFalse(result.succeeded)
        self.assertEqual(result.reason_code, "policy_authorization_required")
        self.assertEqual(called, [])

    def test_active_permission_executes_once_and_is_revoked(self):
        called = []
        result = self.execute(boundary=self.boundary(), operation=lambda: called.append(True))
        self.assertTrue(result.succeeded)
        self.assertEqual(called, [True])
        self.assertEqual(result.permission["status"], "REVOKED")
        self.assertEqual(result.permission["revocation_reason"], "operation_completed")

    def test_execution_failure_still_revokes_permission(self):
        def fail():
            raise RuntimeError("adapter failure")

        result = self.execute(boundary=self.boundary(), operation=fail)
        self.assertFalse(result.succeeded)
        self.assertEqual(result.reason_code, "execution_failed_permission_revoked")
        self.assertEqual(result.permission["status"], "REVOKED")

    def test_expired_permission_never_executes(self):
        called = []
        permission = dict(self.permission)
        permission["expires_at"] = "2000-01-01T00:00:00Z"
        result = self.execute(boundary=self.boundary(), permission=permission, operation=lambda: called.append(True))
        self.assertFalse(result.succeeded)
        self.assertEqual(result.reason_code, "permission_expired")
        self.assertEqual(called, [])


if __name__ == "__main__":
    unittest.main()
