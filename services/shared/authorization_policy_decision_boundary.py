from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime, timezone
from typing import Mapping

from identity_authorization_enforcement import enforce_authorized_operation
from policy_kernel import Decision, PolicyDecision, PolicyKernel, PolicyRequest


DECISION_BOUNDARY_VERSION = "1.0.0"


@dataclass(frozen=True)
class AuthorizationDecision:
    request_id: str
    principal_id: str
    principal_type: str
    capability_id: str
    resource_type: str
    resource_id: str
    action: str
    tenant_scope_binding: str
    data_domain_scope_binding: str
    decision: Decision
    policy_version: str
    reason_code: str
    enforcement_version: str
    decision_time: str


class AuthorizationPolicyDecisionBoundary:
    """Canonical shared authorization composition boundary.

    This class composes Identity Authorization Enforcement with the existing
    Policy Kernel. It does not authenticate, execute protected operations, or
    implement a second policy evaluator.
    """

    def __init__(self, policy_kernel: PolicyKernel):
        self._policy_kernel = policy_kernel

    def authorize(
        self,
        *,
        request_id: str,
        identity_context: Mapping[str, object],
        target_tenant: str,
        target_data_domain: str,
        request: PolicyRequest,
        caller_metadata: Mapping[str, object] | None = None,
        now: datetime | None = None,
    ) -> AuthorizationDecision:
        decision_time = _decision_time(now)
        enforcement = enforce_authorized_operation(
            identity_context=identity_context,
            target_tenant=target_tenant,
            target_data_domain=target_data_domain,
            request=request,
            policy_kernel=self._policy_kernel,
            caller_metadata=caller_metadata,
            now=now,
        )

        if enforcement.policy_decision is None:
            decision = Decision.DENY
            policy_version = _policy_version(self._policy_kernel)
            reason_code = enforcement.reason_code
        else:
            decision = enforcement.policy_decision.decision
            policy_version = enforcement.policy_decision.policy_version
            reason_code = enforcement.reason_code

        return AuthorizationDecision(
            request_id=_required_string(request_id, "request_id"),
            principal_id=request.principal_id,
            principal_type=request.principal_type,
            capability_id=request.capability_id,
            resource_type=request.resource_type,
            resource_id=request.resource_id,
            action=request.action,
            tenant_scope_binding=target_tenant,
            data_domain_scope_binding=target_data_domain,
            decision=decision,
            policy_version=policy_version,
            reason_code=reason_code,
            enforcement_version=enforcement.enforcement_version,
            decision_time=decision_time,
        )


def _required_string(value: object, field: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise ValueError(f"{field} must be a non-empty string")
    return value


def _policy_version(policy_kernel: PolicyKernel) -> str:
    # The Policy Kernel's canonical version is exposed through its decision
    # contract. No independent policy version is introduced here.
    probe = getattr(policy_kernel, "POLICY_VERSION", None)
    if isinstance(probe, str) and probe.strip():
        return probe
    # Current Policy Kernel implementation exports POLICY_VERSION at module
    # scope; importing it here would create an unnecessary second dependency.
    return "unknown"


def _decision_time(now: datetime | None) -> str:
    value = now or datetime.now(timezone.utc)
    if value.tzinfo is None:
        raise ValueError("now must be timezone-aware")
    return value.astimezone(timezone.utc).isoformat().replace("+00:00", "Z")
