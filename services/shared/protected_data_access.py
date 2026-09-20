from __future__ import annotations

from dataclasses import dataclass
from typing import Mapping, Protocol

from authorization_policy_decision_boundary import AuthorizationDecision, AuthorizationPolicyDecisionBoundary
from policy_kernel import PolicyKernel, PolicyRequest

PROTECTED_DATA_ACCESS_VERSION = "1.0.0"

@dataclass(frozen=True)
class ProtectedDataRequest:
    authorization_context: Mapping[str, object]
    capability_id: str
    capability_version: str
    resource_type: str
    resource_id: str
    action: str
    target_tenant_id: str
    target_data_domain: str
    subject_ref: str

class ProtectedDataAdapter(Protocol):
    def read(self, request: ProtectedDataRequest) -> object: ...
    def insert(self, request: ProtectedDataRequest, payload: Mapping[str, object]) -> object: ...
    def update(self, request: ProtectedDataRequest, payload: Mapping[str, object]) -> object: ...
    def delete(self, request: ProtectedDataRequest) -> object: ...

class ProtectedDataAccess:
    """Provider-neutral protected-data gate using the canonical decision boundary."""
    def __init__(self, policy_kernel: PolicyKernel, adapter: ProtectedDataAdapter) -> None:
        self._decision_boundary = AuthorizationPolicyDecisionBoundary(policy_kernel)
        self._adapter = adapter

    def authorize(self, request: ProtectedDataRequest) -> AuthorizationDecision:
        if not _valid_request(request):
            raise PermissionError("invalid_protected_data_request")
        principal_id = request.authorization_context.get("principal_id")
        principal_type = request.authorization_context.get("principal_type")
        if not isinstance(principal_id, str) or not principal_id.strip() or not isinstance(principal_type, str) or not principal_type.strip():
            raise PermissionError("invalid_protected_data_request")
        policy_request = PolicyRequest(
            principal_id=principal_id, principal_type=principal_type,
            capability_id=request.capability_id, capability_version=request.capability_version,
            resource_type=request.resource_type,
            resource_id=request.resource_id, action=request.action, context={},
            subject_ref=request.subject_ref,
        )
        return self._decision_boundary.authorize(
            request_id=f"protected-data:{request.resource_type}:{request.resource_id}:{request.action}",
            identity_context=request.authorization_context,
            target_tenant=request.target_tenant_id,
            target_data_domain=request.target_data_domain,
            request=policy_request,
        )

    def read(self, request: ProtectedDataRequest) -> object:
        decision = self.authorize(request)
        if not decision_allowed(decision):
            raise PermissionError(decision.reason_code)
        return self._adapter.read(request)

    def insert(self, request: ProtectedDataRequest, payload: Mapping[str, object]) -> object:
        decision = self.authorize(request)
        if not decision_allowed(decision):
            raise PermissionError(decision.reason_code)
        return self._adapter.insert(request, payload)

    def update(self, request: ProtectedDataRequest, payload: Mapping[str, object]) -> object:
        decision = self.authorize(request)
        if not decision_allowed(decision):
            raise PermissionError(decision.reason_code)
        return self._adapter.update(request, payload)

    def delete(self, request: ProtectedDataRequest) -> object:
        decision = self.authorize(request)
        if not decision_allowed(decision):
            raise PermissionError(decision.reason_code)
        return self._adapter.delete(request)

def decision_allowed(decision: AuthorizationDecision) -> bool:
    return decision.decision.value == "ALLOW"

def _valid_request(request: ProtectedDataRequest) -> bool:
    if not isinstance(request.authorization_context, Mapping):
        return False
    return all(isinstance(value, str) and value.strip() for value in (
        request.capability_id, request.capability_version, request.resource_type, request.resource_id,
        request.action, request.target_tenant_id, request.target_data_domain, request.subject_ref,
    ))
