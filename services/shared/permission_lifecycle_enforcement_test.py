import unittest
from datetime import datetime, timezone

from permission_lifecycle_enforcement import PermissionLifecycleEnforcer, complete_permission


class PermissionLifecycleEnforcementTests(unittest.TestCase):
    def setUp(self):
        self.enforcer = PermissionLifecycleEnforcer()
        self.base = {
            "permission_id": "perm-1",
            "operation_id": "op-1",
            "principal_ref": "person-1",
            "subject_ref": "subject-1",
            "capability_id": "device.camera.capture",
            "capability_version": "1.0.0",
            "purpose": "capture_health_image",
            "scope": ["camera:front"],
            "status": "ACTIVE",
            "granted_at": "2026-09-21T09:00:00Z",
            "expires_at": "2026-09-21T09:05:00Z",
            "auto_revoke": True,
            "regrant_requires_fresh_consent": True,
        }

    def validate(self, permission=None, **changes):
        value = dict(permission or self.base)
        value.update(changes)
        return self.enforcer.validate(
            permission=value, operation_id="op-1", principal_ref="person-1",
            subject_ref="subject-1", capability_id="device.camera.capture",
            capability_version="1.0.0", purpose="capture_health_image",
            requested_scope=["camera:front"],
            now=datetime(2026, 9, 21, 9, 2, tzinfo=timezone.utc),
        )

    def test_active_bound_permission_allows(self):
        result = self.validate()
        self.assertTrue(result.allowed)
        self.assertEqual(result.reason_code, "permission_active")

    def test_expired_permission_denies(self):
        result = self.validate(expires_at="2026-09-21T09:01:00Z")
        self.assertFalse(result.allowed)
        self.assertEqual(result.reason_code, "permission_expired")

    def test_revoked_permission_denies(self):
        result = self.validate(status="REVOKED")
        self.assertFalse(result.allowed)
        self.assertEqual(result.reason_code, "permission_revoked")

    def test_wrong_subject_denies(self):
        result = self.enforcer.validate(
            permission=self.base, operation_id="op-1", principal_ref="person-1",
            subject_ref="other-subject", capability_id="device.camera.capture",
            capability_version="1.0.0", purpose="capture_health_image",
            requested_scope=["camera:front"],
            now=datetime(2026, 9, 21, 9, 2, tzinfo=timezone.utc),
        )
        self.assertFalse(result.allowed)
        self.assertEqual(result.reason_code, "permission_binding_mismatch")

    def test_scope_cannot_expand(self):
        result = self.enforcer.validate(
            permission=self.base, operation_id="op-1", principal_ref="person-1",
            subject_ref="subject-1", capability_id="device.camera.capture",
            capability_version="1.0.0", purpose="capture_health_image",
            requested_scope=["camera:front", "camera:rear"],
            now=datetime(2026, 9, 21, 9, 2, tzinfo=timezone.utc),
        )
        self.assertFalse(result.allowed)
        self.assertEqual(result.reason_code, "permission_scope_exceeded")

    def test_completion_revokes(self):
        revoked = complete_permission(self.base, completed_at=datetime(2026, 9, 21, 9, 3, tzinfo=timezone.utc))
        self.assertEqual(revoked["status"], "REVOKED")
        self.assertEqual(revoked["revocation_reason"], "operation_completed")
        self.assertEqual(revoked["revoked_at"], "2026-09-21T09:03:00Z")

    def test_regrant_requires_new_operation_binding(self):
        revoked = complete_permission(self.base, completed_at=datetime(2026, 9, 21, 9, 3, tzinfo=timezone.utc))
        result = self.validate(permission=revoked)
        self.assertFalse(result.allowed)


if __name__ == "__main__":
    unittest.main()
