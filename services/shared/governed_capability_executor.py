from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime
from typing import Callable, Mapping

from permission_lifecycle_enforcement import PermissionLifecycleDecision, PermissionLifecycleEnforcer, complete_permission


@dataclass(frozen=True)
class GovernedExecutionResult:
    operation_id: str
    succeeded: bool
    reason_code: str
    permission: Mapping[str, object]


class GovernedCapabilityExecutor:
    """Shared execution boundary joining canonical authorization and permission lifecycle.

    Authorization is supplied by the existing canonical decision boundary.
    This class never evaluates policy itself.
    """

    def __init__(self, permission_enforcer: PermissionLifecycleEnforcer | None = None) -> None:
        self._permissions = permission_enforcer or PermissionLifecycleEnforcer()

    def execute(
        self,
        *,
        operation_id: str,
        authorization_allowed: bool,
        permission: Mapping[str, object],
        principal_ref: str,
        subject_ref: str,
        capability_id: str,
        capability_version: str,
        purpose: str,
        requested_scope: list[str],
        operation: Callable[[], object],
        now: datetime | None = None,
    ) -> GovernedExecutionResult:
        if not authorization_allowed:
            return GovernedExecutionResult(operation_id, False, "authorization_required", dict(permission))

        lifecycle = self._permissions.validate(
            permission=permission,
            operation_id=operation_id,
            principal_ref=principal_ref,
            subject_ref=subject_ref,
            capability_id=capability_id,
            capability_version=capability_version,
            purpose=purpose,
            requested_scope=requested_scope,
            now=now,
        )
        if not lifecycle.allowed:
            return GovernedExecutionResult(operation_id, False, lifecycle.reason_code, dict(permission))

        try:
            operation()
        except Exception:
            revoked = complete_permission(permission, completed_at=now)
            return GovernedExecutionResult(operation_id, False, "execution_failed_permission_revoked", revoked)

        revoked = complete_permission(permission, completed_at=now)
        return GovernedExecutionResult(operation_id, True, "executed_permission_revoked", revoked)
