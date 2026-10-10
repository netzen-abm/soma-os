from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime
from typing import Callable, Mapping

from authorization_policy_decision_boundary import AuthorizationPolicyDecisionBoundary
from permission_lifecycle_enforcement import PermissionLifecycleEnforcer, complete_permission
from policy_kernel import PolicyRequest


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
        authorization_boundary: AuthorizationPolicyDecisionBoundary,
        authorization_context: Mapping[str, object],
        target_tenant: str,
        target_data_domain: str,
        policy_request: PolicyRequest,
        permission: Mapping[str, object] | None,
        principal_ref: str,
        subject_ref: str,
        capability_id: str,
        capability_version: str,
        purpose: str,
        requested_scope: list[str],
        operation: Callable[[], object],
        now: datetime | None = None,
    ) -> GovernedExecutionResult:
        decision = authorization_boundary.authorize(
            request_id=operation_id,
            identity_context=authorization_context,
            target_tenant=target_tenant,
            target_data_domain=target_data_domain,
            request=policy_request,
            now=now,
        )
        if not decision.allowed:
            return GovernedExecutionResult(operation_id, False, decision.reason_code, dict(permission or {}))

        if permission is not None:
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

        execution_count = 0

        def invoke_once() -> None:
            nonlocal execution_count
            if execution_count != 0:
                raise RuntimeError("governed_operation_invoked_more_than_once")
            execution_count += 1
            operation()

        try:
            invoke_once()
        except Exception:
            revoked = complete_permission(permission, completed_at=now) if permission is not None else {}
            reason = "execution_failed_permission_revoked" if permission is not None else "execution_failed"
            return GovernedExecutionResult(operation_id, False, reason, revoked)

        if execution_count != 1:
            revoked = complete_permission(permission, completed_at=now) if permission is not None else {}
            reason = "execution_count_invalid_permission_revoked" if permission is not None else "execution_count_invalid"
            return GovernedExecutionResult(operation_id, False, reason, revoked)

        revoked = complete_permission(permission, completed_at=now) if permission is not None else {}
        reason = "executed_permission_revoked" if permission is not None else "executed"
        return GovernedExecutionResult(operation_id, True, reason, revoked)
