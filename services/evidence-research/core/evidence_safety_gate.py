"""Bounded, explicit safety assessment boundary for SOMA."""
from __future__ import annotations

from dataclasses import dataclass
from typing import Sequence

from evidence_synthesis import SynthesizedEvidence

SAFETY_STATUSES = frozenset(
    {
        "NO_IDENTIFIED_SAFETY_CONCERN",
        "SAFETY_CONCERN_IDENTIFIED",
        "INSUFFICIENT_SAFETY_INFORMATION",
        "REQUIRES_HUMAN_REVIEW",
        "NOT_ASSESSED",
    }
)


@dataclass(frozen=True)
class EvidenceSafetyAssessment:
    """Explicit safety assessment; never an efficacy or causality judgment."""

    status: str
    evidence_ids: tuple[str, ...]
    safety_findings: tuple[str, ...] = ()
    contraindications: tuple[str, ...] = ()
    interaction_concerns: tuple[str, ...] = ()
    exposure_risk_concerns: tuple[str, ...] = ()
    unresolved_questions: tuple[str, ...] = ()
    rationale: str = ""
    assessment_method: str = ""
    assessed_at: str = ""
    assessor: str | None = None

    def __post_init__(self) -> None:
        if self.status not in SAFETY_STATUSES:
            raise ValueError("invalid safety status")
        if not self.evidence_ids:
            raise ValueError("at least one evidence record is required")
        if any(not value.strip() for value in self.evidence_ids):
            raise ValueError("evidence_ids must be non-empty")
        if len(set(self.evidence_ids)) != len(self.evidence_ids):
            raise ValueError("evidence_ids must be unique")
        for name in (
            "safety_findings",
            "contraindications",
            "interaction_concerns",
            "exposure_risk_concerns",
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
class SafetyGatedEvidence:
    """Synthesized evidence preserved with an explicit safety assessment."""

    synthesized_evidence: SynthesizedEvidence
    assessment: EvidenceSafetyAssessment


class EvidenceSafetyGate:
    """Bind an explicit safety assessment to already synthesized evidence."""

    @staticmethod
    def apply(
        synthesized: SynthesizedEvidence,
        assessment: EvidenceSafetyAssessment,
    ) -> SafetyGatedEvidence:
        if not isinstance(synthesized, SynthesizedEvidence):
            raise ValueError("safety gate requires synthesized evidence")

        canonical_ids = tuple(
            f"{item.candidate.provider_id.strip()}:{item.candidate.provider_record_id.strip()}"
            for item in synthesized.evidence
        )
        if any(not value.split(":", 1)[0] or not value.split(":", 1)[1] for value in canonical_ids):
            raise ValueError("evidence records require provider and provider record identifiers")
        if assessment.evidence_ids != canonical_ids:
            raise ValueError("assessment evidence_ids must match synthesized evidence")

        return SafetyGatedEvidence(
            synthesized_evidence=synthesized,
            assessment=assessment,
        )
