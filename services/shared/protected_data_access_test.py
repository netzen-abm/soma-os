from __future__ import annotations

import unittest
from dataclasses import dataclass
from typing import Mapping

from protected_data_access import ProtectedDataAccess, ProtectedDataRequest
from policy_kernel import GrantKey, PolicyKernel


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
        "context_id": "ctx-1",
        "schema_version": "1.1.0",
        "principal_id": "person-1",
        "principal_type": "person",
        "identity_mode": "authenticated_account",
        "authentication_status": "VERIFIED",
        "authentication_provenance": {
            "issuer": "test",
            "method": "test",
            "verified_at": "2026-09-04T10:00:00Z",
            "verifier_id": "test",
        },
        "tenant_scope": [tenant],
        "data_domain_scope": [domain],
        "assurance_level": "HIGH",
        "issued_at": "2026-09-04T10:00:00Z",
        "expires_at": "2026-09-04T12:00:00Z",
    }


def request(ctx: dict[str, object], resource_id: str = "r-1") -> ProtectedDataRequest:
    return ProtectedDataRequest(
        authorization_context=ctx,
        capability_id="health.read",
        capability_version="1.0.0",
        resource_type="health_record",
        resource_id=resource_id,
        action="read",
        target_tenant_id="tenant-a",
        target_data_domain="personal-health",
        subject_ref="person-1",
    )


def kernel() -> PolicyKernel:
    grants: dict[GrantKey, bool] = {
        ("person-1", "person", "health.read", "health_record", "r-1", "read", "person-1"): True
    }
    return PolicyKernel(
        {"capabilities": [{"id": "health.read", "version": "1.0.0", "principal_types": ["person"]}]},
        grants,
    )


class ProtectedDataAccessTests(unittest.TestCase):
    def test_authorized_read_reaches_adapter(self) -> None:
        adapter = Adapter()
        access = ProtectedDataAccess(kernel(), adapter)
        self.assertEqual(access.read(request(identity())), {"resource_id": "r-1"})
        self.assertEqual(adapter.calls, 1)

    def test_wrong_tenant_never_reaches_adapter(self) -> None:
        adapter = Adapter()
        access = ProtectedDataAccess(kernel(), adapter)
        with self.assertRaisesRegex(PermissionError, "target_scope_mismatch"):
            access.read(request(identity(tenant="tenant-b")))
        self.assertEqual(adapter.calls, 0)

    def test_wrong_domain_never_reaches_adapter(self) -> None:
        adapter = Adapter()
        access = ProtectedDataAccess(kernel(), adapter)
        with self.assertRaisesRegex(PermissionError, "target_scope_mismatch"):
            access.read(request(identity(domain="another-domain")))
        self.assertEqual(adapter.calls, 0)

    def test_wrong_resource_never_reaches_adapter(self) -> None:
        adapter = Adapter()
        access = ProtectedDataAccess(kernel(), adapter)
        with self.assertRaisesRegex(PermissionError, "policy_authorization_required"):
            access.read(request(identity(), resource_id="r-2"))
        self.assertEqual(adapter.calls, 0)

    def test_unverified_identity_never_reaches_adapter(self) -> None:
        adapter = Adapter()
        access = ProtectedDataAccess(kernel(), adapter)
        ctx = identity()
        ctx["authentication_status"] = "UNVERIFIED"
        with self.assertRaisesRegex(PermissionError, "invalid_identity_context"):
            access.read(request(ctx))
        self.assertEqual(adapter.calls, 0)

    def test_capability_version_mismatch_never_reaches_adapter(self) -> None:
        adapter = Adapter()
        access = ProtectedDataAccess(kernel(), adapter)
        bad = ProtectedDataRequest(
            authorization_context=identity(),
            capability_id="health.read",
            capability_version="9.9.9",
            resource_type="health_record",
            resource_id="r-1",
            action="read",
            target_tenant_id="tenant-a",
            target_data_domain="personal-health",
            subject_ref="person-1",
        )
        with self.assertRaisesRegex(PermissionError, "policy_capability_version_mismatch"):
            access.read(bad)
        self.assertEqual(adapter.calls, 0)

    def test_invalid_request_never_reaches_adapter(self) -> None:
        adapter = Adapter()
        access = ProtectedDataAccess(kernel(), adapter)
        bad = request(identity())
        bad = ProtectedDataRequest(
            authorization_context=bad.authorization_context,
            capability_id=bad.capability_id,
            capability_version=bad.capability_version,
            resource_type=bad.resource_type,
            resource_id="",
            action=bad.action,
            target_tenant_id=bad.target_tenant_id,
            target_data_domain=bad.target_data_domain,
            subject_ref=bad.subject_ref,
        )
        with self.assertRaisesRegex(PermissionError, "invalid_protected_data_request"):
            access.read(bad)
        self.assertEqual(adapter.calls, 0)


if __name__ == "__main__":
    unittest.main()
