"""Bounded, explicit causality assessment boundary for SOMA."""
from __future__ import annotations

from dataclasses import dataclass

from evidence_safety_gate import SafetyGatedEvidence

CAUSALITY_STATUSES = frozenset(
    {
        "CAUSALITY_SUPPORTED",
        "CAUSALITY_PLAUSIBLE",
        "CAUSALITY_UNLIKELY",
        "CAUSALITY_NOT_SUPPORTED",
        "INSUFFICIENT_CAUSALITY_INFORMATION",
        "REQUIRES_HUMAN_REVIEW",
        "NOT_ASSESSED",
    }
)


@dataclass(frozen=True)
class EvidenceCausalityAssessment:
    """Explicit causality assessment; never an automatic causal inference."""

    status: str
    evidence_ids: tuple[str, ...]
    temporal_relationship: str
    alternative_explanations: tuple[str, ...] = ()
    co_exposures: tuple[str, ...] = ()
    relevant_prior_history: tuple[str, ...] = ()
    causal_factors_considered: tuple[str, ...] = ()
    unresolved_questions: tuple[str, ...] = ()
    rationale: str = ""
    assessment_method: str = ""
    assessed_at: str = ""
    assessor: str | None = None

    def __post_init__(self) -> None:
        if self.status not in CAUSALITY_STATUSES:
            raise ValueError("invalid causality status")
        if not self.evidence_ids:
            raise ValueError("at least one evidence record is required")
        if any(not value.strip() for value in self.evidence_ids):
            raise ValueError("evidence_ids must be non-empty")
        if len(set(self.evidence_ids)) != len(self.evidence_ids):
            raise ValueError("evidence_ids must be unique")
        if not self.temporal_relationship.strip():
            raise ValueError("temporal_relationship must be non-empty")
        for name in (
            "alternative_explanations",
            "co_exposures",
            "relevant_prior_history",
            "causal_factors_considered",
            "unresolved_questions",
        ):
            values = getattr(self, name)
            if any(not value.strip() for value in values):
                raise ValueError(f"{name} must contain non-empty entries")
        for name in ("rationale", "assessment_method", "assessed_at"):
            if not getattr(self, name).strip():
                raise ValueError(f"{name} must be non-empty")
        if self.assessor is not None and not self.assessor.strip():
            raise ValueError("assessor must be non-empty when supplied")


@dataclass(frozen=True)
class CausalityAssessedEvidence:
    """Safety-gated evidence preserved with an explicit causality assessment."""

    safety_gated_evidence: SafetyGatedEvidence
    assessment: EvidenceCausalityAssessment


class EvidenceCausalityBoundary:
    """Bind an explicit causality assessment to safety-gated evidence."""

    @staticmethod
    def apply(
        safety_gated: SafetyGatedEvidence,
        assessment: EvidenceCausalityAssessment,
    ) -> CausalityAssessedEvidence:
        if not isinstance(safety_gated, SafetyGatedEvidence):
            raise ValueError("causality boundary requires safety-gated evidence")

        canonical_ids = tuple(
            f"{item.candidate.provider_id.strip()}:{item.candidate.provider_record_id.strip()}"
            for item in safety_gated.synthesized_evidence.evidence
        )
        if any(not value.split(":", 1)[0] or not value.split(":", 1)[1] for value in canonical_ids):
            raise ValueError("evidence records require provider and provider record identifiers")
        if assessment.evidence_ids != canonical_ids:
            raise ValueError("assessment evidence_ids must match safety-gated evidence")

        return CausalityAssessedEvidence(
            safety_gated_evidence=safety_gated,
            assessment=assessment,
        )
