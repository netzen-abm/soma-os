from __future__ import annotations

from dataclasses import dataclass
from typing import Mapping

from policy_kernel import Decision, GrantKey, PolicyKernel
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


def identity(tenant: str = "tenant-a", domain: str = "personal-health") -> dict[str, object]:
    return {
        "context_id": "ctx-1", "schema_version": "1.1.0", "principal_id": "person-1",
        "principal_type": "person", "identity_mode": "authenticated_account", "authentication_status": "VERIFIED",
        "authentication_provenance": {"issuer": "test", "method": "test", "verified_at": "2026-09-04T10:00:00Z", "verifier_id": "test"},
        "tenant_scope": [tenant], "data_domain_scope": [domain], "assurance_level": "HIGH",
        "issued_at": "2026-09-04T10:00:00Z",
    }


def request(ctx: dict[str, object], resource_id: str = "r-1", subject_ref: str = "person-1") -> ProtectedDataRequest:
    return ProtectedDataRequest(authorization_context=ctx, capability_id="health.read", capability_version="1.0.0", resource_type="health_record", resource_id=resource_id, action="read", target_tenant_id="tenant-a", target_data_domain="personal-health", subject_ref=subject_ref)


def kernel() -> PolicyKernel:
    grants: dict[GrantKey, bool] = {("person-1", "person", "health.read", "health_record", "r-1", "read", "person-1"): True}
    return PolicyKernel({"capabilities": [{"id": "health.read", "version": "1.0.0", "principal_types": ["person"]}]}, grants)


def test_authorized_read_reaches_adapter() -> None:
    adapter = Adapter(); access = ProtectedDataAccess(kernel(), adapter)
    assert access.read(request(identity())) == {"resource_id": "r-1"}
    assert adapter.calls == 1


def test_anonymous_local_identity_can_use_explicitly_granted_low_risk_capability() -> None:
    adapter = Adapter(); access = ProtectedDataAccess(kernel(), adapter)
    ctx = identity(); ctx.update({"identity_mode": "anonymous_local", "assurance_level": "LOW"})
    assert access.read(request(ctx)) == {"resource_id": "r-1"}
    assert adapter.calls == 1


def test_wrong_tenant_never_reaches_adapter() -> None:
    adapter = Adapter(); access = ProtectedDataAccess(kernel(), adapter)
    ctx = identity(tenant="tenant-b")
    try: access.read(request(ctx))
    except PermissionError as exc: assert str(exc) == "target_scope_mismatch"
    else: raise AssertionError("expected denial")
    assert adapter.calls == 0


def test_wrong_domain_never_reaches_adapter() -> None:
    adapter = Adapter(); access = ProtectedDataAccess(kernel(), adapter)
    ctx = identity(domain="another-domain")
    try: access.read(request(ctx))
    except PermissionError as exc: assert str(exc) == "target_scope_mismatch"
    else: raise AssertionError("expected denial")
    assert adapter.calls == 0


def test_wrong_resource_never_reaches_adapter() -> None:
    adapter = Adapter(); access = ProtectedDataAccess(kernel(), adapter)
    try: access.read(request(identity(), resource_id="r-2"))
    except PermissionError as exc: assert str(exc) == "policy_authorization_required"
    else: raise AssertionError("expected denial")
    assert adapter.calls == 0


def test_unverified_identity_never_reaches_adapter() -> None:
    adapter = Adapter(); access = ProtectedDataAccess(kernel(), adapter)
    ctx = identity(); ctx["authentication_status"] = "UNVERIFIED"
    try: access.read(request(ctx))
    except PermissionError as exc: assert str(exc) == "invalid_identity_context"
    else: raise AssertionError("expected denial")
    assert adapter.calls == 0


def test_capability_version_mismatch_never_reaches_adapter() -> None:
    adapter = Adapter(); access = ProtectedDataAccess(kernel(), adapter)
    bad = request(identity(), subject_ref="subject-001"); bad = ProtectedDataRequest(**{**bad.__dict__, "capability_version": "9.9.9"})
    try: access.read(bad)
    except PermissionError as exc: assert str(exc) == "policy_capability_version_mismatch"
    else: raise AssertionError("expected denial")
    assert adapter.calls == 0


def test_invalid_request_never_reaches_adapter() -> None:
    adapter = Adapter(); access = ProtectedDataAccess(kernel(), adapter)
    bad = ProtectedDataRequest(authorization_context=identity(), capability_id="health.read", capability_version="1.0.0", resource_type="health_record", resource_id="", action="read", target_tenant_id="tenant-a", target_data_domain="personal-health", subject_ref="person-1")
    try: access.read(bad)
    except PermissionError as exc: assert str(exc) == "invalid_protected_data_request"
    else: raise AssertionError("expected denial")
    assert adapter.calls == 0
