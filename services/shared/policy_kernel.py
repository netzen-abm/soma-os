import json
from dataclasses import dataclass
from enum import Enum
from pathlib import Path
from typing import Mapping


class Decision(str, Enum):
    ALLOW = "ALLOW"
    DENY = "DENY"
    REQUIRE_CONSENT = "REQUIRE_CONSENT"
    REQUIRE_HUMAN_REVIEW = "REQUIRE_HUMAN_REVIEW"
    DEGRADE = "DEGRADE"


@dataclass(frozen=True)
class PolicyRequest:
    principal_id: str
    principal_type: str
    capability_id: str
    resource_type: str
    resource_id: str
    action: str
    context: Mapping[str, str]


@dataclass(frozen=True)
class PolicyDecision:
    decision: Decision
    policy_version: str
    reason_code: str


POLICY_VERSION = "0.4.0"
GrantKey = tuple[str, str, str, str, str, str]
_ASSURANCE_RANK = {"LOW": 0, "SUBSTANTIAL": 1, "HIGH": 2}


class PolicyKernel:
    """Deterministic, fail-closed policy boundary for shared capabilities."""

    def __init__(self, registry: Mapping[str, object], grants: Mapping[GrantKey, bool]):
        self._capabilities = self._index_capabilities(registry)
        self._grants = dict(grants)

    def evaluate(
        self,
        request: PolicyRequest,
        *,
        identity_context: Mapping[str, object] | None = None,
    ) -> PolicyDecision:
        if not self._request_is_valid(request):
            return self._deny("invalid_request")

        capability = self._capabilities.get(request.capability_id)
        if capability is None:
            return self._deny("unknown_capability")

        principal_types = capability.get("principal_types")
        if not self._valid_principal_types(principal_types):
            return self._deny("invalid_capability_principal_types")
        if request.principal_type not in principal_types:
            return self._deny("principal_type_mismatch")

        identity_result = self._evaluate_identity_requirements(
            capability.get("identity_requirements"),
            request,
            identity_context,
        )
        if identity_result is not None:
            return identity_result

        if request.context.get("human_review") == "required":
            return PolicyDecision(Decision.REQUIRE_HUMAN_REVIEW, POLICY_VERSION, "human_review_required")
        if request.context.get("consent") == "required":
            return PolicyDecision(Decision.REQUIRE_CONSENT, POLICY_VERSION, "consent_required")
        if request.context.get("deny") == "true":
            return self._deny("policy_denied")

        grant_key = (
            request.principal_id,
            request.principal_type,
            request.capability_id,
            request.resource_type,
            request.resource_id,
            request.action,
        )
        if self._grants.get(grant_key) is not True:
            return self._deny("authorization_required")

        if request.context.get("degrade") == "true":
            return PolicyDecision(Decision.DEGRADE, POLICY_VERSION, "degraded_execution_required")

        return PolicyDecision(Decision.ALLOW, POLICY_VERSION, "policy_allowed")

    @staticmethod
    def from_registry_file(path: Path, grants: Mapping[GrantKey, bool]) -> "PolicyKernel":
        data = json.loads(path.read_text(encoding="utf-8"))
        return PolicyKernel(data, grants)

    @staticmethod
    def _index_capabilities(registry: Mapping[str, object]) -> dict[str, Mapping[str, object]]:
        capabilities = registry.get("capabilities")
        if not isinstance(capabilities, list):
            return {}
        indexed: dict[str, Mapping[str, object]] = {}
        for capability in capabilities:
            if not isinstance(capability, dict):
                continue
            capability_id = capability.get("id")
            if isinstance(capability_id, str) and capability_id.strip():
                indexed[capability_id] = capability
        return indexed

    @staticmethod
    def _valid_principal_types(value: object) -> bool:
        return isinstance(value, list) and bool(value) and all(isinstance(item, str) and bool(item.strip()) for item in value)

    @staticmethod
    def _evaluate_identity_requirements(
        requirements: object,
        request: PolicyRequest,
        identity_context: Mapping[str, object] | None,
    ) -> PolicyDecision | None:
        if requirements is None:
            return None
        if not isinstance(requirements, Mapping):
            return PolicyDecision(Decision.DENY, POLICY_VERSION, "invalid_identity_requirements")

        applies_to = requirements.get("applies_to_principal_types")
        allowed_modes = requirements.get("allowed_identity_modes")
        minimum_assurance = requirements.get("minimum_assurance_level")
        requires_durable = requirements.get("requires_durable_identity")
        if (
            not isinstance(applies_to, list) or not applies_to
            or not all(isinstance(item, str) and item.strip() for item in applies_to)
            or not isinstance(allowed_modes, list) or not allowed_modes
            or not all(item in {"anonymous_local", "authenticated_account"} for item in allowed_modes)
            or minimum_assurance not in _ASSURANCE_RANK
            or not isinstance(requires_durable, bool)
        ):
            return PolicyDecision(Decision.DENY, POLICY_VERSION, "invalid_identity_requirements")

        if request.principal_type not in applies_to:
            return None

        if not isinstance(identity_context, Mapping):
            return PolicyDecision(Decision.DENY, POLICY_VERSION, "identity_context_required")

        if identity_context.get("principal_id") != request.principal_id:
            return PolicyDecision(Decision.DENY, POLICY_VERSION, "identity_principal_mismatch")
        if identity_context.get("principal_type") != request.principal_type:
            return PolicyDecision(Decision.DENY, POLICY_VERSION, "identity_principal_type_mismatch")
        if identity_context.get("authentication_status") != "VERIFIED":
            return PolicyDecision(Decision.DENY, POLICY_VERSION, "identity_not_verified")

        identity_mode = identity_context.get("identity_mode")
        assurance = identity_context.get("assurance_level")
        if identity_mode not in allowed_modes:
            return PolicyDecision(Decision.DENY, POLICY_VERSION, "identity_mode_not_allowed")
        if assurance not in _ASSURANCE_RANK or _ASSURANCE_RANK[assurance] < _ASSURANCE_RANK[minimum_assurance]:
            return PolicyDecision(Decision.DENY, POLICY_VERSION, "insufficient_identity_assurance")
        if requires_durable and identity_mode != "authenticated_account":
            return PolicyDecision(Decision.DENY, POLICY_VERSION, "durable_identity_required")

        return None

    @staticmethod
    def _request_is_valid(request: PolicyRequest) -> bool:
        if not isinstance(request.context, Mapping):
            return False
        fields = (
            request.principal_id, request.principal_type, request.capability_id,
            request.resource_type, request.resource_id, request.action,
        )
        return all(isinstance(value, str) and bool(value.strip()) for value in fields)

    @staticmethod
    def _deny(reason_code: str) -> PolicyDecision:
        return PolicyDecision(Decision.DENY, POLICY_VERSION, reason_code)
