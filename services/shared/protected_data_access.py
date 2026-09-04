from __future__ import annotations

from dataclasses import dataclass
from typing import Mapping, Protocol

from identity_authorization_enforcement import EnforcementDecision, enforce_authorized_operation
from policy_kernel import PolicyKernel, PolicyRequest


PROTECTED_DATA_ACCESS_VERSION = "1.0.0"
_ALLOWED_ACTIONS = {"read", "insert", "update", "delete"}


@dataclass(frozen=True)
class ProtectedDataRequest:
    authorization_context: Mapping[str, object]
    capability_id: str
    resource_type: str
    resource_id: str
    action: str
    target_tenant_id: str
    target_data_domain: str
    policy_context: Mapping[str, str]


@dataclass(frozen=True)
class ProtectedDataDecision:
    allowed: bool
    reason_code: str
    authorization: EnforcementDecision | None
    version: str = PROTECTED_DATA_ACCESS_VERSION


class ProtectedDataProvider(Protocol):
    """Provider contract. Implementations receive only an authorized target."""

    def execute(self, request: ProtectedDataRequest) -> object: ...


def authorize_protected_data_access(
    *,
    request: ProtectedDataRequest,
    policy_request: PolicyRequest,
    policy_kernel: PolicyKernel,
    caller_metadata: Mapping[str, object] | None = None,
) -> ProtectedDataDecision:
    """Authorize an exact protected-data target before any provider operation.

    This function deliberately does not perform datastore I/O. Provider adapters
    must call it (or an equivalent trusted composition boundary) before execute().
    The Policy Kernel remains the sole grant evaluator.
    """
    if not _valid_request(request):
        return ProtectedDataDecision(False, "invalid_protected_data_request", None)

    if policy_request.resource_id != request.resource_id:
        return ProtectedDataDecision(False, "resource_id_mismatch", None)
    if policy_request.capability_id != request.capability_id:
        return ProtectedDataDecision(False, "capability_mismatch", None)
    if policy_request.resource_type != request.resource_type:
        return ProtectedDataDecision(False, "resource_type_mismatch", None)
    if policy_request.action != request.action:
        return ProtectedDataDecision(False, "action_mismatch", None)

    decision = enforce_authorized_operation(
        identity_context=request.authorization_context,
        target_tenant=request.target_tenant_id,
        target_data_domain=request.target_data_domain,
        request=policy_request,
        policy_kernel=policy_kernel,
        caller_metadata=caller_metadata,
    )
    if not decision.allowed:
        return ProtectedDataDecision(False, decision.reason_code, decision)

    return ProtectedDataDecision(True, "authorized", decision)


def _valid_request(request: ProtectedDataRequest) -> bool:
    if not isinstance(request.authorization_context, Mapping):
        return False
    if not isinstance(request.policy_context, Mapping):
        return False
    values = (
        request.capability_id,
        request.resource_type,
        request.resource_id,
        request.action,
        request.target_tenant_id,
        request.target_data_domain,
    )
    return all(isinstance(value, str) and value.strip() for value in values) and request.action in _ALLOWED_ACTIONS
