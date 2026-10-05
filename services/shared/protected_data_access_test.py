from __future__ import annotations

from dataclasses import dataclass
import unittest

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




class ProtectedDataAccessTests(unittest.TestCase):
