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


POLICY_VERSION = "0.1.0"
GrantKey = tuple[str, str, str]


class PolicyKernel:
    """Deterministic policy boundary using registry and explicit grants."""

    def __init__(
        self,
        registry: Mapping[str, object],
        grants: Mapping[GrantKey, bool],
    ):
        self._capabilities = self._index_capabilities(registry)
        self._grants = dict(grants)

    def evaluate(self, request: PolicyRequest) -> PolicyDecision:
        if not self._request_is_valid(request):
            return self._deny("invalid_request")

        if request.capability_id not in self._capabilities:
            return self._deny("unknown_capability")

        if request.context.get("human_review") == "required":
            return PolicyDecision(
                Decision.REQUIRE_HUMAN_REVIEW,
                POLICY_VERSION,
                "human_review_required",
            )

        if request.context.get("consent") == "required":
            return PolicyDecision(
                Decision.REQUIRE_CONSENT,
                POLICY_VERSION,
                "consent_required",
            )

        if request.context.get("deny") == "true":
            return self._deny("policy_denied")

        grant_key = (
            request.principal_id,
            request.capability_id,
            request.action,
        )
        if self._grants.get(grant_key) is not True:
            return self._deny("authorization_required")

        if request.context.get("degrade") == "true":
            return PolicyDecision(
                Decision.DEGRADE,
                POLICY_VERSION,
                "degraded_execution_required",
            )

        return PolicyDecision(
            Decision.ALLOW,
            POLICY_VERSION,
            "policy_allowed",
        )

    @staticmethod
    def from_registry_file(
        path: Path,
        grants: Mapping[GrantKey, bool],
    ) -> "PolicyKernel":
        data = json.loads(path.read_text(encoding="utf-8"))
        return PolicyKernel(data, grants)

    @staticmethod
    def _index_capabilities(
        registry: Mapping[str, object],
    ) -> dict[str, Mapping[str, object]]:
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
    def _request_is_valid(request: PolicyRequest) -> bool:
        if not isinstance(request.context, Mapping):
            return False

        fields = (
            request.principal_id,
            request.principal_type,
            request.capability_id,
            request.resource_type,
            request.resource_id,
            request.action,
        )
        return all(
            isinstance(value, str) and bool(value.strip())
            for value in fields
        )

    @staticmethod
    def _deny(reason_code: str) -> PolicyDecision:
        return PolicyDecision(Decision.DENY, POLICY_VERSION, reason_code)
