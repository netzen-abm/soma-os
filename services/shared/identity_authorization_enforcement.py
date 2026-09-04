from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime, timezone
from typing import Mapping, Sequence

from policy_kernel import Decision, PolicyDecision, PolicyKernel, PolicyRequest


ENFORCEMENT_VERSION = "1.0.0"


@dataclass(frozen=True)
class EnforcementDecision:
    allowed: bool
    reason_code: str
    policy_decision: PolicyDecision | None
    enforcement_version: str = ENFORCEMENT_VERSION


def enforce_authorized_operation(
    *,
    identity_context: Mapping[str, object],
    target_tenant: str,
    target_data_domain: str,
    request: PolicyRequest,
    policy_kernel: PolicyKernel,
    caller_metadata: Mapping[str, object] | None = None,
    now: datetime | None = None,
) -> EnforcementDecision:
    """Compose verified identity/scope checks with Policy Kernel v0.3.

    This function is an enforcement boundary, not a second authorization engine.
    Security-sensitive identity and target fields are authoritative only from the
    supplied trusted IdentityContext/target binding. Caller metadata is checked
    for attempted overrides but is never merged into the trusted context.
    """
    if not _valid_identity_context(identity_context):
        return _deny("invalid_identity_context")

    if caller_metadata and _attempts_to_broaden(caller_metadata, identity_context):
        return _deny("caller_metadata_scope_override")

    if not _target_is_in_scope(identity_context, target_tenant, target_data_domain):
        return _deny("target_scope_mismatch")

    if (
        request.principal_id != identity_context["principal_id"]
        or request.principal_type != identity_context["principal_type"]
    ):
        return _deny("principal_identity_mismatch")

    if not _valid_policy_dimensions(request):
        return _deny("invalid_policy_request")

    decision = policy_kernel.evaluate(request)
    if decision.decision != Decision.ALLOW:
        return EnforcementDecision(False, f"policy_{decision.reason_code}", decision)

    return EnforcementDecision(True, "authorized", decision)


def _valid_identity_context(context: Mapping[str, object], now: datetime | None = None) -> bool:
    required = (
        "context_id",
        "schema_version",
        "principal_id",
        "principal_type",
        "authentication_status",
        "authentication_provenance",
        "tenant_scope",
        "data_domain_scope",
        "assurance_level",
        "issued_at",
    )
    if not all(isinstance(context.get(key), str) and str(context[key]).strip() for key in required if key not in {"authentication_provenance", "tenant_scope", "data_domain_scope"}):
        return False
    if context.get("schema_version") != "1.0.0":
        return False
    if context.get("authentication_status") != "VERIFIED":
        return False
    if context.get("principal_type") not in {"person", "agent", "service", "application", "device"}:
        return False

    provenance = context.get("authentication_provenance")
    if not isinstance(provenance, Mapping):
        return False
    if not all(isinstance(provenance.get(k), str) and provenance[k].strip() for k in ("issuer", "method", "verified_at", "verifier_id")):
        return False

    for field in ("tenant_scope", "data_domain_scope"):
        value = context.get(field)
        if not isinstance(value, Sequence) or isinstance(value, (str, bytes)) or not value:
            return False
        if any(not isinstance(item, str) or not item.strip() for item in value):
            return False

    if not _valid_timestamp(context["issued_at"]):
        return False
    if "expires_at" in context and not _valid_timestamp(context["expires_at"]):
        return False
    if now is not None:
        if now.tzinfo is None:
            return False
        if "expires_at" in context and _parse_timestamp(str(context["expires_at"])) <= now.astimezone(timezone.utc):
            return False

    # Credentials, tokens and secrets must never be part of this boundary.
    forbidden = {"token", "access_token", "refresh_token", "credential", "secret", "password", "session_material"}
    if forbidden.intersection(context.keys()):
        return False
    return True


def _target_is_in_scope(context: Mapping[str, object], tenant: str, data_domain: str) -> bool:
    if not isinstance(tenant, str) or not tenant.strip() or not isinstance(data_domain, str) or not data_domain.strip():
        return False
    tenant_scope = context.get("tenant_scope")
    domain_scope = context.get("data_domain_scope")
    return tenant in tenant_scope and data_domain in domain_scope


def _valid_policy_dimensions(request: PolicyRequest) -> bool:
    return all(
        isinstance(value, str) and value.strip()
        for value in (
            request.principal_id,
            request.principal_type,
            request.capability_id,
            request.resource_type,
            request.resource_id,
            request.action,
        )
    )


def _attempts_to_broaden(metadata: Mapping[str, object], context: Mapping[str, object]) -> bool:
    for field in ("principal_id", "principal_type", "tenant_scope", "data_domain_scope", "authentication_status", "assurance_level"):
        if field not in metadata:
            continue
        if field in {"tenant_scope", "data_domain_scope"}:
            supplied = metadata[field]
            trusted = context[field]
            if supplied != trusted:
                return True
        elif metadata[field] != context.get(field):
            return True
    return False


def _valid_timestamp(value: object) -> bool:
    if not isinstance(value, str):
        return False
    try:
        _parse_timestamp(value)
    except ValueError:
        return False
    return True


def _parse_timestamp(value: str) -> datetime:
    parsed = datetime.fromisoformat(value.replace("Z", "+00:00"))
    if parsed.tzinfo is None:
        raise ValueError("timestamp must include timezone")
    return parsed.astimezone(timezone.utc)


def _deny(reason_code: str) -> EnforcementDecision:
    return EnforcementDecision(False, reason_code, None)
