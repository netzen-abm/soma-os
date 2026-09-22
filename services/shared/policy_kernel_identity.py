from typing import Mapping

from policy_kernel_contract import (
    Decision,
    POLICY_VERSION,
    PolicyDecision,
    PolicyRequest,
    _ASSURANCE_RANK,
)


def evaluate_identity_requirements(
    requirements: object,
    request: PolicyRequest,
    identity_context: Mapping[str, object] | None,
) -> PolicyDecision | None:
    if requirements is None:
        return None
    if not isinstance(requirements, Mapping):
        return _deny("invalid_identity_requirements")
    applies_to = requirements.get("applies_to_principal_types")
    allowed_modes = requirements.get("allowed_identity_modes")
    minimum_assurance = requirements.get("minimum_assurance_level")
    requires_durable = requirements.get("requires_durable_identity")
    if not _valid_requirements(applies_to, allowed_modes, minimum_assurance, requires_durable):
        return _deny("invalid_identity_requirements")
    if request.principal_type not in applies_to:
        return None
    if not isinstance(identity_context, Mapping):
        return _deny("identity_context_required")
    if identity_context.get("principal_id") != request.principal_id:
        return _deny("identity_principal_mismatch")
    if identity_context.get("principal_type") != request.principal_type:
        return _deny("identity_principal_type_mismatch")
    if identity_context.get("authentication_status") != "VERIFIED":
        return _deny("identity_not_verified")
    identity_mode = identity_context.get("identity_mode")
    assurance = identity_context.get("assurance_level")
    if identity_mode not in allowed_modes:
        return _deny("identity_mode_not_allowed")
    if assurance not in _ASSURANCE_RANK or _ASSURANCE_RANK[assurance] < _ASSURANCE_RANK[minimum_assurance]:
        return _deny("insufficient_identity_assurance")
    if requires_durable and identity_mode != "authenticated_account":
        return _deny("durable_identity_required")
    return None


def _valid_requirements(
    applies_to: object,
    allowed_modes: object,
    minimum_assurance: object,
    requires_durable: object,
) -> bool:
    return (
        isinstance(applies_to, list) and bool(applies_to)
        and all(isinstance(item, str) and item.strip() for item in applies_to)
        and isinstance(allowed_modes, list) and bool(allowed_modes)
        and all(item in {"anonymous_local", "authenticated_account"} for item in allowed_modes)
        and minimum_assurance in _ASSURANCE_RANK
        and isinstance(requires_durable, bool)
    )


def _deny(reason_code: str) -> PolicyDecision:
    return PolicyDecision(Decision.DENY, POLICY_VERSION, reason_code)
