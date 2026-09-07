"""Dependency-free contract tests for IdentityContext v1.1.

These tests establish the distinction between a verified local security context
and an authenticated account. Verification is a security fact, not proof of
real-world identity or an authorization grant.
"""

import json
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCHEMA_PATH = ROOT / "schemas" / "identity-context-v1.json"


def _load_schema():
    return json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))


def _valid_context(mode="authenticated_account"):
    return {
        "context_id": "ctx-1",
        "schema_version": "1.1.0",
        "principal_id": "person-1",
        "principal_type": "person",
        "identity_mode": mode,
        "authentication_status": "VERIFIED",
        "authentication_provenance": {
            "issuer": "auth.example" if mode == "authenticated_account" else "local-runtime",
            "method": "oidc" if mode == "authenticated_account" else "local-security-context",
            "verified_at": "2026-09-04T08:00:00Z",
            "verifier_id": "verifier-1",
        },
        "tenant_scope": ["tenant-a"],
        "data_domain_scope": ["health-records"],
        "assurance_level": "HIGH" if mode == "authenticated_account" else "LOW",
        "issued_at": "2026-09-04T08:00:00Z",
        "expires_at": "2026-09-04T09:00:00Z",
    }


def _assert_shape(context):
    schema = _load_schema()
    assert context.keys() <= schema["properties"].keys()
    assert set(schema["required"]) <= context.keys()
    assert context["schema_version"] == "1.1.0"
    assert context["identity_mode"] in schema["properties"]["identity_mode"]["enum"]
    assert context["principal_type"] in schema["properties"]["principal_type"]["enum"]
    assert context["authentication_status"] in schema["properties"]["authentication_status"]["enum"]
    assert context["assurance_level"] in schema["properties"]["assurance_level"]["enum"]
    assert context["tenant_scope"] and context["data_domain_scope"]


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
    assert schema["properties"]["schema_version"]["const"] == "1.1.0"


def test_authenticated_account_context_is_valid():
    context = _valid_context()
    _assert_shape(context)
    assert context["identity_mode"] == "authenticated_account"
    assert context["assurance_level"] == "HIGH"


def test_anonymous_local_context_is_valid_without_real_world_identity_claim():
    context = _valid_context("anonymous_local")
    _assert_shape(context)
    assert context["principal_type"] == "person"
    assert context["identity_mode"] == "anonymous_local"
    assert context["authentication_status"] == "VERIFIED"
    assert context["assurance_level"] == "LOW"
    assert context["authentication_provenance"]["method"] == "local-security-context"


def test_verified_does_not_mean_real_world_identity_verified():
    context = _valid_context("anonymous_local")
    assert context["authentication_status"] == "VERIFIED"
    assert context["identity_mode"] == "anonymous_local"
    assert context["authentication_provenance"]["issuer"] == "local-runtime"


def test_anonymous_local_cannot_claim_substantial_or_high_assurance():
    for assurance in ("SUBSTANTIAL", "HIGH"):
        context = _valid_context("anonymous_local")
        context["assurance_level"] = assurance
        assert not (context["identity_mode"] == "anonymous_local" and assurance != "LOW")


def test_anonymous_local_is_person_identity_only():
    for principal_type in ("agent", "service", "application", "device"):
        context = _valid_context("anonymous_local")
        context["principal_type"] = principal_type
        assert not (context["identity_mode"] == "anonymous_local" and context["principal_type"] != "person")


def test_unverified_context_fails_closed():
    context = _valid_context()
    context["authentication_status"] = "UNVERIFIED"
    assert not _is_verified_for_protected_use(context)


def test_missing_provenance_fails_closed():
    context = _valid_context()
    context["authentication_provenance"] = None
    assert not _is_verified_for_protected_use(context)


def test_missing_tenant_or_domain_scope_fails_closed():
    for field in ("tenant_scope", "data_domain_scope"):
        context = _valid_context()
        context[field] = []
        assert not _is_verified_for_protected_use(context)


def test_cross_tenant_and_domain_access_denied():
    context = _valid_context()
    assert _scope_allows(context, "tenant-a", "health-records")
    assert not _scope_allows(context, "tenant-b", "health-records")
    assert not _scope_allows(context, "tenant-a", "billing")


def test_scope_cannot_be_inferred_from_resource_type():
    context = _valid_context()
    assert not _scope_allows(context, "tenant-b", "health-records")
    assert not _scope_allows(context, "tenant-a", "billing")


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
        assert parsed.astimezone(timezone.utc)


if __name__ == "__main__":
    tests = [value for name, value in globals().items() if name.startswith("test_") and callable(value)]
    for test in tests:
        test()
    print(f"IdentityContext contract tests: {len(tests)}/{len(tests)} passed")
