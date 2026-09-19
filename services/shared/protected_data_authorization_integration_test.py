from __future__ import annotations

from dataclasses import dataclass
from typing import Mapping

from policy_kernel import GrantKey, PolicyKernel
from protected_data_access import ProtectedDataAccess, ProtectedDataRequest


@dataclass
class Adapter:
    calls: int = 0

    def read(self, request: ProtectedDataRequest) -> object:
        self.calls += 1
        return {"resource_id": request.resource_id}

    def insert(self, request: ProtectedDataRequest, payload: Mapping[str, object]) -> object:
        self.calls += 1
        return payload

    def update(self, request: ProtectedDataRequest, payload: Mapping[str, object]) -> object:
        self.calls += 1
        return payload

    def delete(self, request: ProtectedDataRequest) -> object:
        self.calls += 1
        return True


def identity() -> dict[str, object]:
    return {
        "context_id": "ctx-1", "schema_version": "1.1.0", "principal_id": "person-1",
        "principal_type": "person", "identity_mode": "authenticated_account",
        "authentication_status": "VERIFIED",
        "authentication_provenance": {"issuer": "test", "method": "test", "verified_at": "2026-09-12T10:00:00Z", "verifier_id": "test"},
        "tenant_scope": ["tenant-a"], "data_domain_scope": ["health"], "assurance_level": "HIGH",
        "issued_at": "2026-09-12T10:00:00Z", "expires_at": "2026-09-12T11:00:00Z",
    }


def request(ctx: dict[str, object], resource_id: str = "r-1") -> ProtectedDataRequest:
    return ProtectedDataRequest(ctx, "health.read", "health_record", resource_id, "read", "tenant-a", "health", "person-1")


def kernel() -> PolicyKernel:
    grants: dict[GrantKey, bool] = {("person-1", "person", "health.read", "health_record", "r-1", "read", "person-1"): True}
    return PolicyKernel({"capabilities": [{"id": "health.read", "principal_types": ["person"]}]}, grants)


def test_allow_is_the_only_state_that_reaches_adapter() -> None:
    adapter = Adapter()
    access = ProtectedDataAccess(kernel(), adapter)
    result = access.read(request(identity()))
    assert result == {"resource_id": "r-1"}
    assert adapter.calls == 1


def test_denial_is_enforced_before_adapter_invocation() -> None:
    adapter = Adapter()
    access = ProtectedDataAccess(kernel(), adapter)
    try:
        access.read(request(identity(), resource_id="r-2"))
    except PermissionError as exc:
        assert str(exc) == "policy_authorization_required"
    else:
        raise AssertionError("expected authorization denial")
    assert adapter.calls == 0


def test_scope_denial_is_enforced_before_adapter_invocation() -> None:
    adapter = Adapter()
    access = ProtectedDataAccess(kernel(), adapter)
    ctx = identity()
    ctx["tenant_scope"] = ["tenant-b"]
    try:
        access.read(request(ctx))
    except PermissionError as exc:
        assert str(exc) == "target_scope_mismatch"
    else:
        raise AssertionError("expected scope denial")
    assert adapter.calls == 0
