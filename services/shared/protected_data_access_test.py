from datetime import datetime, timezone

from identity_authorization_enforcement import enforce_authorized_operation
from policy_kernel import PolicyKernel, PolicyRequest
from protected_data_access import ProtectedDataRequest, authorize_protected_data_access


def identity():
    return {
        "context_id": "ctx-1",
        "schema_version": "1.0.0",
        "principal_id": "person-1",
        "principal_type": "person",
        "authentication_status": "VERIFIED",
        "authentication_provenance": {
            "issuer": "test-issuer",
            "method": "test",
            "verified_at": "2026-09-04T00:00:00+00:00",
            "verifier_id": "verifier-1",
        },
        "tenant_scope": ["tenant-a"],
        "data_domain_scope": ["health"],
        "assurance_level": "HIGH",
        "issued_at": "2026-09-04T00:00:00+00:00",
        "expires_at": "2026-09-05T00:00:00+00:00",
    }


def kernel():
    registry = {"capabilities": [{"id": "health.read", "principal_types": ["person"]}]}
    grants = {
        ("person-1", "person", "health.read", "health_state", "hs-1", "read"): True,
    }
    return PolicyKernel(registry, grants)


def policy_request(resource_id="hs-1", action="read"):
    return PolicyRequest(
        principal_id="person-1",
        principal_type="person",
        capability_id="health.read",
        resource_type="health_state",
        resource_id=resource_id,
        action=action,
        context={},
    )


def protected_request(resource_id="hs-1", tenant="tenant-a", domain="health", action="read"):
    return ProtectedDataRequest(
        authorization_context=identity(),
        capability_id="health.read",
        resource_type="health_state",
        resource_id=resource_id,
        action=action,
        target_tenant_id=tenant,
        target_data_domain=domain,
        policy_context={},
    )


def test_authorized_exact_target():
    result = authorize_protected_data_access(
        request=protected_request(),
        policy_request=policy_request(),
        policy_kernel=kernel(),
    )
    assert result.allowed


def test_wrong_tenant_denied():
    result = authorize_protected_data_access(
        request=protected_request(tenant="tenant-b"),
        policy_request=policy_request(),
        policy_kernel=kernel(),
    )
    assert not result.allowed
    assert result.reason_code == "target_scope_mismatch"


def test_wrong_domain_denied():
    result = authorize_protected_data_access(
        request=protected_request(domain="billing"),
        policy_request=policy_request(),
        policy_kernel=kernel(),
    )
    assert not result.allowed


def test_resource_id_cannot_be_rebound():
    result = authorize_protected_data_access(
        request=protected_request(resource_id="hs-2"),
        policy_request=policy_request(resource_id="hs-1"),
        policy_kernel=kernel(),
    )
    assert not result.allowed
    assert result.reason_code == "resource_id_mismatch"


def test_caller_metadata_cannot_broaden_scope():
    result = authorize_protected_data_access(
        request=protected_request(),
        policy_request=policy_request(),
        policy_kernel=kernel(),
        caller_metadata={"tenant_scope": ["tenant-a", "tenant-b"]},
    )
    assert not result.allowed
    assert result.reason_code == "caller_metadata_scope_override"


def test_missing_identity_fails_closed():
    request = protected_request()
    request = ProtectedDataRequest(
        authorization_context={},
        capability_id=request.capability_id,
        resource_type=request.resource_type,
        resource_id=request.resource_id,
        action=request.action,
        target_tenant_id=request.target_tenant_id,
        target_data_domain=request.target_data_domain,
        policy_context=request.policy_context,
    )
    result = authorize_protected_data_access(
        request=request,
        policy_request=policy_request(),
        policy_kernel=kernel(),
    )
    assert not result.allowed


def test_wrong_action_denied():
    result = authorize_protected_data_access(
        request=protected_request(action="update"),
        policy_request=policy_request(action="update"),
        policy_kernel=kernel(),
    )
    assert not result.allowed


def test_policy_deny_survives_access_boundary():
    result = authorize_protected_data_access(
        request=protected_request(resource_id="unknown"),
        policy_request=policy_request(resource_id="unknown"),
        policy_kernel=kernel(),
    )
    assert not result.allowed


def test_expired_identity_denied():
    context = identity()
    context["expires_at"] = "2026-09-03T00:00:00+00:00"
    decision = enforce_authorized_operation(
        identity_context=context,
        target_tenant="tenant-a",
        target_data_domain="health",
        request=policy_request(),
        policy_kernel=kernel(),
        now=datetime(2026, 9, 4, tzinfo=timezone.utc),
    )
    assert not decision.allowed
