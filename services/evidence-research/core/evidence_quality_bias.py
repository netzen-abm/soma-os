"""Bounded evidence quality and bias assessment boundary for SOMA."""
from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from evidence_research_core import EvidenceCandidate
from evidence_screening import ScreenedEvidenceCandidate
from evidence_directness import DirectlyAssessedEvidence


JUDGMENTS = frozenset(
    {"FAVORABLE", "SOME_CONCERNS", "SERIOUS_CONCERNS", "NOT_ASSESSED"}
)


@dataclass(frozen=True)
class EvidenceQualityBiasAssessment:
    """Explicit multidimensional methodological assessment."""

    study_design: str
    risk_of_bias: str
    sample_size_precision: str
    comparator_quality: str
    outcome_validity: str
    follow_up_duration: str
    attrition: str
    selective_reporting: str
    confounding: str
    external_validity: str
    rationale: str
    assessment_method: str
    assessed_at: str
    assessor: str | None = None

    def __post_init__(self) -> None:
        if not self.study_design.strip():
            raise ValueError("study_design must be non-empty")
        for name in (
            "risk_of_bias",
            "sample_size_precision",
            "comparator_quality",
            "outcome_validity",
            "follow_up_duration",
            "attrition",
            "selective_reporting",
            "confounding",
            "external_validity",
        ):
            value = getattr(self, name)
            if value not in JUDGMENTS:
                raise ValueError(f"invalid {name}: {value}")
        for name in ("rationale", "assessment_method", "assessed_at"):
            if not getattr(self, name).strip():
                raise ValueError(f"{name} must be non-empty")
        if self.assessor is not None and not self.assessor.strip():
            raise ValueError("assessor must be non-empty when supplied")


@dataclass(frozen=True)
class QualityAssessedEvidence:
    """Original candidate plus its explicit quality/bias assessment."""

    candidate: EvidenceCandidate
    assessment: EvidenceQualityBiasAssessment


class EvidenceQualityBiasBoundary:
    """Apply an explicit quality/bias assessment without inference."""

    @staticmethod
    def apply(
        directly_assessed: DirectlyAssessedEvidence,
        assessment: EvidenceQualityBiasAssessment,
    ) -> QualityAssessedEvidence:
        candidate = directly_assessed.candidate
        if candidate.screening_status != "INCLUDED":
            raise ValueError("quality/bias assessment requires an INCLUDED candidate")
        return QualityAssessedEvidence(candidate=candidate, assessment=assessment)
