from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime, timezone
from typing import Iterable, Mapping


@dataclass(frozen=True)
class PermissionLifecycleDecision:
    allowed: bool
    reason_code: str
    status: str


class PermissionLifecycleEnforcer:
    """Canonical, provider-neutral validation of purpose-bound capability grants.

    This component validates lifecycle state; it does not perform policy
    evaluation or become an authorization authority.
    """

    def validate(
        self,
        *,
        permission: Mapping[str, object],
        operation_id: str,
        principal_ref: str,
        subject_ref: str,
        capability_id: str,
        capability_version: str,
        purpose: str,
        requested_scope: Iterable[str],
        now: datetime | None = None,
    ) -> PermissionLifecycleDecision:
        if not self._identity_matches(permission, operation_id, principal_ref, subject_ref):
            return PermissionLifecycleDecision(False, "permission_binding_mismatch", self._status(permission))
        if permission.get("capability_id") != capability_id or permission.get("capability_version") != capability_version:
            return PermissionLifecycleDecision(False, "permission_capability_mismatch", self._status(permission))
        if permission.get("purpose") != purpose:
            return PermissionLifecycleDecision(False, "permission_purpose_mismatch", self._status(permission))

        granted_scope = permission.get("scope")
        if not isinstance(granted_scope, list) or not all(isinstance(item, str) and item.strip() for item in granted_scope):
            return PermissionLifecycleDecision(False, "invalid_permission_scope", self._status(permission))
        if not set(requested_scope).issubset(set(granted_scope)):
            return PermissionLifecycleDecision(False, "permission_scope_exceeded", self._status(permission))

        status = self._status(permission)
        if status != "ACTIVE":
            return PermissionLifecycleDecision(False, f"permission_{status.lower()}", status)

        if permission.get("auto_revoke") is not True or permission.get("regrant_requires_fresh_consent") is not True:
            return PermissionLifecycleDecision(False, "unsafe_permission_lifecycle", status)

        expires_at = self._parse_time(permission.get("expires_at"))
        current = now or datetime.now(timezone.utc)
        if current.tzinfo is None:
            raise ValueError("now must be timezone-aware")
        if expires_at is None:
            return PermissionLifecycleDecision(False, "invalid_permission_expiry", status)
        if current.astimezone(timezone.utc) >= expires_at:
            return PermissionLifecycleDecision(False, "permission_expired", "EXPIRED")

        return PermissionLifecycleDecision(True, "permission_active", status)

    @staticmethod
    def _identity_matches(permission: Mapping[str, object], operation_id: str, principal_ref: str, subject_ref: str) -> bool:
        return (
            permission.get("operation_id") == operation_id
            and permission.get("principal_ref") == principal_ref
            and permission.get("subject_ref") == subject_ref
        )

    @staticmethod
    def _status(permission: Mapping[str, object]) -> str:
        status = permission.get("status")
        return status if isinstance(status, str) else "INVALID"

    @staticmethod
    def _parse_time(value: object) -> datetime | None:
        if not isinstance(value, str) or not value.strip():
            return None
        try:
            parsed = datetime.fromisoformat(value.replace("Z", "+00:00"))
        except ValueError:
            return None
        if parsed.tzinfo is None:
            return None
        return parsed.astimezone(timezone.utc)


def complete_permission(permission: Mapping[str, object], *, completed_at: datetime | None = None) -> dict[str, object]:
    """Return a new revoked lifecycle record after governed operation completion."""
    if permission.get("auto_revoke") is not True:
        raise ValueError("permission must require automatic revocation")
    result = dict(permission)
    result["status"] = "REVOKED"
    result["revoked_at"] = (completed_at or datetime.now(timezone.utc)).astimezone(timezone.utc).isoformat().replace("+00:00", "Z")
    result["revocation_reason"] = "operation_completed"
    return result
