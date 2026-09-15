"""Bounded cross-study consistency assessment boundary for SOMA."""
from __future__ import annotations

from dataclasses import dataclass
from typing import Sequence

from evidence_research_core import EvidenceCandidate
from evidence_quality_bias import QualityAssessedEvidence

CONSISTENCY_VALUES = frozenset({
    "CONSISTENT",
    "MIXED",
    "INCONSISTENT",
    "NOT_ASSESSED",
})


@dataclass(frozen=True)
class EvidenceConsistencyAssessment:
    """Explicit cross-study consistency assessment."""

    consistency: str
    evidence_ids: tuple[str, ...]
    rationale: str
    assessment_method: str
    assessed_at: str
    assessor: str | None = None

    def __post_init__(self) -> None:
        if self.consistency not in CONSISTENCY_VALUES:
            raise ValueError("invalid consistency")
        if len(self.evidence_ids) < 2:
            raise ValueError("at least two evidence records are required")
        if any(not evidence_id.strip() for evidence_id in self.evidence_ids):
            raise ValueError("evidence_ids must be non-empty")
        if len(set(self.evidence_ids)) != len(self.evidence_ids):
            raise ValueError("evidence_ids must be unique")
        for name in ("rationale", "assessment_method", "assessed_at"):
            if not getattr(self, name).strip():
                raise ValueError(f"{name} must be non-empty")
        if self.assessor is not None and not self.assessor.strip():
            raise ValueError("assessor must be non-empty when supplied")


@dataclass(frozen=True)
class ConsistencyAssessedEvidence:
    """Evidence records plus an explicit cross-study consistency assessment."""

    evidence: tuple[QualityAssessedEvidence, ...]
    assessment: EvidenceConsistencyAssessment


class EvidenceConsistencyBoundary:
    """Bind an explicit consistency assessment to quality-assessed evidence."""

    @staticmethod
    def apply(
        evidence: Sequence[QualityAssessedEvidence],
        assessment: EvidenceConsistencyAssessment,
    ) -> ConsistencyAssessedEvidence:
        records = tuple(evidence)
        if len(records) < 2:
            raise ValueError("consistency assessment requires at least two evidence records")

        for item in records:
            if not isinstance(item, QualityAssessedEvidence):
                raise ValueError("consistency assessment requires quality/bias assessed evidence")
            if item.candidate.screening_status != "INCLUDED":
                raise ValueError("consistency assessment requires INCLUDED evidence")
            if not isinstance(item.candidate, EvidenceCandidate):
                raise ValueError("invalid evidence candidate")
            if not item.candidate.provider_id.strip() or not item.candidate.provider_record_id.strip():
                raise ValueError("evidence records require non-empty provider and provider record identifiers")

        ids = tuple(
            f"{item.candidate.provider_id}:{item.candidate.provider_record_id}"
            for item in records
        )
        if len(set(ids)) != len(ids):
            raise ValueError("evidence records must have unique provider identifiers")
        if ids != assessment.evidence_ids:
            raise ValueError("assessment evidence_ids must match supplied evidence records")

        return ConsistencyAssessedEvidence(evidence=records, assessment=assessment)
