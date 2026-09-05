from __future__ import annotations

from dataclasses import dataclass
from typing import Mapping, Protocol, Sequence

from identity_authorization_enforcement import EnforcementDecision, enforce_authorized_operation
from policy_kernel import PolicyKernel, PolicyRequest


PROTECTED_DATA_ACCESS_VERSION = "1.0.0"


@dataclass(frozen=True)
class ProtectedDataRequest:
    authorization_context: Mapping[str, object]
    capability_id: str
    resource_type: str
    resource_id: str
    action: str
    target_tenant_id: str
    target_data_domain: str


class ProtectedDataAdapter(Protocol):
    def read(self, request: ProtectedDataRequest) -> object: ...
    def insert(self, request: ProtectedDataRequest, payload: Mapping[str, object]) -> object: ...
    def update(self, request: ProtectedDataRequest, payload: Mapping[str, object]) -> object: ...
    def delete(self, request: ProtectedDataRequest) -> object: ...


class ProtectedDataAccess:
    """Provider-neutral gate from canonical authorization to persistence."""

    def __init__(self, policy_kernel: PolicyKernel, adapter: ProtectedDataAdapter) -> None:
        self._policy_kernel = policy_kernel
        self._adapter = adapter

    def authorize(self, request: ProtectedDataRequest) -> EnforcementDecision:
        if not _valid_request(request):
            from identity_authorization_enforcement import EnforcementDecision
            return EnforcementDecision(False, "invalid_protected_data_request", None)
        policy_request = PolicyRequest(
            principal_id=str(request.authorization_context.get("principal_id", "")),
            principal_type=str(request.authorization_context.get("principal_type", "")),
            capability_id=request.capability_id,
            resource_type=request.resource_type,
            resource_id=request.resource_id,
            action=request.action,
            context={},
        )
        return enforce_authorized_operation(
            identity_context=request.authorization_context,
            target_tenant=request.target_tenant_id,
            target_data_domain=request.target_data_domain,
            request=policy_request,
            policy_kernel=self._policy_kernel,
        )

    def read(self, request: ProtectedDataRequest) -> object:
        decision = self.authorize(request)
        if not decision.allowed:
            raise PermissionError(decision.reason_code)
        return self._adapter.read(request)

    def insert(self, request: ProtectedDataRequest, payload: Mapping[str, object]) -> object:
        decision = self.authorize(request)
        if not decision.allowed:
            raise PermissionError(decision.reason_code)
        return self._adapter.insert(request, payload)

    def update(self, request: ProtectedDataRequest, payload: Mapping[str, object]) -> object:
        decision = self.authorize(request)
        if not decision.allowed:
            raise PermissionError(decision.reason_code)
        return self._adapter.update(request, payload)

    def delete(self, request: ProtectedDataRequest) -> object:
        decision = self.authorize(request)
        if not decision.allowed:
            raise PermissionError(decision.reason_code)
        return self._adapter.delete(request)


def _valid_request(request: ProtectedDataRequest) -> bool:
    if not isinstance(request.authorization_context, Mapping):
        return False
    return all(
        isinstance(value, str) and value.strip()
        for value in (
            request.capability_id,
            request.resource_type,
            request.resource_id,
            request.action,
            request.target_tenant_id,
            request.target_data_domain,
        )
    )
