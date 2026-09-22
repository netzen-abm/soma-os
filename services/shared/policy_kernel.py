import json
from pathlib import Path
from typing import Mapping

from policy_kernel_contract import (
    GrantKey,
    Decision,
    POLICY_VERSION,
    PolicyDecision,
    PolicyRequest,
)
from policy_kernel_identity import evaluate_identity_requirements

__all__ = [
    "Decision",
    "GrantKey",
    "POLICY_VERSION",
    "PolicyDecision",
    "PolicyRequest",
    "PolicyKernel",
]


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
        if request.capability_version != capability.get("version"):
            return self._deny("capability_version_mismatch")
        principal_types = capability.get("principal_types")
        if not self._valid_principal_types(principal_types):
            return self._deny("invalid_capability_principal_types")
        if request.principal_type not in principal_types:
            return self._deny("principal_type_mismatch")
        identity_result = evaluate_identity_requirements(
            capability.get("identity_requirements"), request, identity_context
        )
        if identity_result is not None:
            return identity_result
        if request.context.get("human_review") == "required":
            return PolicyDecision(
                Decision.REQUIRE_HUMAN_REVIEW, POLICY_VERSION, "human_review_required"
            )
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
            request.subject_ref,
        )
        if self._grants.get(grant_key) is not True:
            return self._deny("authorization_required")
        if request.context.get("degrade") == "true":
            return PolicyDecision(
                Decision.DEGRADE, POLICY_VERSION, "degraded_execution_required"
            )
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
        return {
            item.get("id"): item
            for item in capabilities
            if isinstance(item, dict)
            and isinstance(item.get("id"), str)
            and item.get("id").strip()
        }

    @staticmethod
    def _valid_principal_types(value: object) -> bool:
        return isinstance(value, list) and bool(value) and all(
            isinstance(item, str) and bool(item.strip()) for item in value
        )

    @staticmethod
    def _request_is_valid(request: PolicyRequest) -> bool:
        if not isinstance(request.context, Mapping):
            return False
        fields = (
            request.principal_id,
            request.principal_type,
            request.capability_id,
            request.capability_version,
            request.resource_type,
            request.resource_id,
            request.action,
        )
        return all(isinstance(value, str) and bool(value.strip()) for value in fields) and (
            isinstance(request.subject_ref, str) and bool(request.subject_ref.strip())
        )

    @staticmethod
    def _deny(reason_code: str) -> PolicyDecision:
        return PolicyDecision(Decision.DENY, POLICY_VERSION, reason_code)
