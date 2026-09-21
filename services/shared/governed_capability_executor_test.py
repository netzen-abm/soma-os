import unittest
from datetime import datetime, timezone

from governed_capability_executor import GovernedCapabilityExecutor


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

    def test_denied_authorization_never_executes(self):
        called = []
        result = self.executor.execute(
            operation_id="op-1", authorization_allowed=False, permission=self.permission,
            principal_ref="person-1", subject_ref="subject-1",
            capability_id="device.camera.capture", capability_version="1.0.0",
            purpose="capture_health_image", requested_scope=["camera:front"],
            operation=lambda: called.append(True), now=self.now,
        )
        self.assertFalse(result.succeeded)
        self.assertEqual(result.reason_code, "authorization_required")
        self.assertEqual(called, [])

    def test_active_permission_executes_once_and_is_revoked(self):
        called = []
        result = self.executor.execute(
            operation_id="op-1", authorization_allowed=True, permission=self.permission,
            principal_ref="person-1", subject_ref="subject-1",
            capability_id="device.camera.capture", capability_version="1.0.0",
            purpose="capture_health_image", requested_scope=["camera:front"],
            operation=lambda: called.append(True), now=self.now,
        )
        self.assertTrue(result.succeeded)
        self.assertEqual(called, [True])
        self.assertEqual(result.permission["status"], "REVOKED")
        self.assertEqual(result.permission["revocation_reason"], "operation_completed")

    def test_execution_failure_still_revokes(self):
        def fail():
            raise RuntimeError("adapter failure")

        result = self.executor.execute(
            operation_id="op-1", authorization_allowed=True, permission=self.permission,
            principal_ref="person-1", subject_ref="subject-1",
            capability_id="device.camera.capture", capability_version="1.0.0",
            purpose="capture_health_image", requested_scope=["camera:front"],
            operation=fail, now=self.now,
        )
        self.assertFalse(result.succeeded)
        self.assertEqual(result.reason_code, "execution_failed_permission_revoked")
        self.assertEqual(result.permission["status"], "REVOKED")


if __name__ == "__main__":
    unittest.main()
