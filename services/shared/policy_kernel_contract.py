from dataclasses import dataclass
from enum import Enum
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
    capability_version: str
    resource_type: str
    resource_id: str
    action: str
    context: Mapping[str, str]
    subject_ref: str


@dataclass(frozen=True)
class PolicyDecision:
    decision: Decision
    policy_version: str
    reason_code: str


POLICY_VERSION = "0.4.0"
GrantKey = tuple[str, str, str, str, str, str, str | None]
_ASSURANCE_RANK = {"LOW": 0, "SUBSTANTIAL": 1, "HIGH": 2}
