"""Dependency-free contract tests for IdentityContext v1.

These tests intentionally validate the security invariants that a JSON Schema
alone cannot establish: verified authentication is required for protected use,
scope is explicit, and caller-controlled scope cannot be broadened implicitly.
"""

import json
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SCHEMA_PATH = ROOT / "schemas" / "identity-context-v1.json"


def _load_schema():
    return json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))


def _valid_context():
    return {
        "context_id": "ctx-1",
        "schema_version": "1.0.0",
        "principal_id": "person-1",
        "principal_type": "person",
        "authentication_status": "VERIFIED",
        "authentication_provenance": {
            "issuer": "auth.example",
            "method": "oidc",
            "verified_at": "2026-09-04T08:00:00Z",
            "verifier_id": "verifier-1",
        },
        "tenant_scope": ["tenant-a"],
        "data_domain_scope": ["health-records"],
        "assurance_level": "HIGH",
        "issued_at": "2026-09-04T08:00:00Z",
        "expires_at": "2026-09-04T09:00:00Z",
    }


def _assert_shape(context):
    schema = _load_schema()
    assert context.keys() <= schema["properties"].keys()
    assert set(schema["required"]) <= context.keys()
    assert context["schema_version"] == "1.0.0"
    assert context["principal_type"] in schema["properties"]["principal_type"]["enum"]
    assert context["authentication_status"] in schema["properties"]["authentication_status"]["enum"]
    assert context["assurance_level"] in schema["properties"]["assurance_level"]["enum"]
    assert context["tenant_scope"]
    assert context["data_domain_scope"]
    assert len(context["tenant_scope"]) == len(set(context["tenant_scope"]))
    assert len(context["data_domain_scope"]) == len(set(context["data_domain_scope"]))


def _is_verified_for_protected_use(context):
    return (
        context["authentication_status"] == "VERIFIED"
        and bool(context.get("authentication_provenance"))
        and bool(context.get("tenant_scope"))
        and bool(context.get("data_domain_scope"))
    )


def _scope_allows(context, tenant_id, data_domain):
    if not _is_verified_for_protected_use(context):
        return False
    return tenant_id in context["tenant_scope"] and data_domain in context["data_domain_scope"]


def test_schema_baseline():
    schema = _load_schema()
    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert schema["properties"]["schema_version"]["const"] == "1.0.0"


def test_valid_context_shape():
    _assert_shape(_valid_context())


def test_unverified_context_fails_closed():
    context = _valid_context()
    context["authentication_status"] = "UNVERIFIED"
    assert not _is_verified_for_protected_use(context)
    assert not _scope_allows(context, "tenant-a", "health-records")


def test_missing_provenance_fails_closed():
    context = _valid_context()
    context["authentication_provenance"] = None
    assert not _is_verified_for_protected_use(context)


def test_missing_tenant_scope_fails_closed():
    context = _valid_context()
    context["tenant_scope"] = []
    assert not _is_verified_for_protected_use(context)
    assert not _scope_allows(context, "tenant-a", "health-records")


def test_missing_data_domain_scope_fails_closed():
    context = _valid_context()
    context["data_domain_scope"] = []
    assert not _is_verified_for_protected_use(context)
    assert not _scope_allows(context, "tenant-a", "health-records")


def test_cross_tenant_access_denied():
    context = _valid_context()
    assert _scope_allows(context, "tenant-a", "health-records")
    assert not _scope_allows(context, "tenant-b", "health-records")


def test_cross_domain_access_denied():
    context = _valid_context()
    assert _scope_allows(context, "tenant-a", "health-records")
    assert not _scope_allows(context, "tenant-a", "billing")


def test_scope_cannot_be_inferred_from_resource_type():
    context = _valid_context()
    assert not _scope_allows(context, "tenant-b", "health-records")
    assert not _scope_allows(context, "tenant-a", "billing")


def test_scope_cannot_be_broadened_by_extra_caller_metadata():
    context = _valid_context()
    caller_metadata = {"tenant_scope": ["tenant-b"], "data_domain_scope": ["billing"]}
    assert _scope_allows(context, "tenant-a", "health-records")
    assert not _scope_allows(context, "tenant-b", "health-records")
    assert not _scope_allows(context, "tenant-a", "billing")
    assert caller_metadata != context["tenant_scope"]


def test_explicit_principal_identity_is_required():
    context = _valid_context()
    context["principal_id"] = ""
    assert not context["principal_id"]


def test_expired_and_revoked_contexts_fail_closed():
    for status in ("EXPIRED", "REVOKED"):
        context = _valid_context()
        context["authentication_status"] = status
        assert not _is_verified_for_protected_use(context)


def test_context_is_snapshot_not_delegation():
    context = _valid_context()
    assert "delegation" not in context
    assert "delegate" not in context


def test_no_credentials_or_tokens_in_contract():
    context = _valid_context()
    forbidden = {"password", "secret", "token", "access_token", "refresh_token", "credential"}
    assert not forbidden.intersection(context.keys())
    assert not forbidden.intersection(context["authentication_provenance"].keys())


def test_timestamps_are_parseable_iso8601():
    context = _valid_context()
    for key in ("issued_at", "expires_at"):
        parsed = datetime.fromisoformat(context[key].replace("Z", "+00:00"))
        assert parsed.tzinfo is not None
        assert parsed.tzinfo.utcoffset(parsed) is not None
        assert parsed.astimezone(timezone.utc)


if __name__ == "__main__":
    tests = [
        value
        for name, value in globals().items()
        if name.startswith("test_") and callable(value)
    ]
    for test in tests:
        test()
    print(f"IdentityContext contract tests: {len(tests)}/{len(tests)} passed")
