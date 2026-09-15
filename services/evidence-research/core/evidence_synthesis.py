"""Bounded, explicit evidence synthesis boundary for SOMA."""
from __future__ import annotations

from dataclasses import dataclass
from typing import Sequence

from evidence_quality_bias import QualityAssessedEvidence

SYNTHESIS_STATUSES = frozenset(
    {"SUPPORTIVE", "NULL", "MIXED", "CONTRADICTORY", "INSUFFICIENT", "NOT_ASSESSED"}
)


@dataclass(frozen=True)
class EvidenceSynthesisAssessment:
    """Explicit synthesis judgment; no automatic evidence weighting."""

    status: str
    evidence_ids: tuple[str, ...]
    supporting_evidence_ids: tuple[str, ...] = ()
    null_evidence_ids: tuple[str, ...] = ()
    contradictory_evidence_ids: tuple[str, ...] = ()
    indirect_evidence_ids: tuple[str, ...] = ()
    unresolved_questions: tuple[str, ...] = ()
    rationale: str = ""
    assessment_method: str = ""
    assessed_at: str = ""
    assessor: str | None = None

    def __post_init__(self) -> None:
        if self.status not in SYNTHESIS_STATUSES:
            raise ValueError("invalid synthesis status")
        if not self.evidence_ids:
            raise ValueError("at least one evidence record is required")
        if any(not value.strip() for value in self.evidence_ids):
            raise ValueError("evidence_ids must be non-empty")
        if len(set(self.evidence_ids)) != len(self.evidence_ids):
            raise ValueError("evidence_ids must be unique")
        for group_name in (
            "supporting_evidence_ids",
            "null_evidence_ids",
            "contradictory_evidence_ids",
            "indirect_evidence_ids",
        ):
            values = getattr(self, group_name)
            if any(not value.strip() for value in values):
                raise ValueError(f"{group_name} must contain non-empty identifiers")
            if len(set(values)) != len(values):
                raise ValueError(f"{group_name} must be unique")
        for name in ("rationale", "assessment_method", "assessed_at"):
            if not getattr(self, name).strip():
                raise ValueError(f"{name} must be non-empty")
        if self.assessor is not None and not self.assessor.strip():
            raise ValueError("assessor must be non-empty when supplied")


@dataclass(frozen=True)
class SynthesizedEvidence:
    """Assessed evidence preserved with its explicit synthesis judgment."""

    evidence: tuple[QualityAssessedEvidence, ...]
    assessment: EvidenceSynthesisAssessment


class EvidenceSynthesisBoundary:
    """Bind an explicit synthesis assessment to quality-assessed evidence."""

    @staticmethod
    def apply(
        evidence: Sequence[QualityAssessedEvidence],
        assessment: EvidenceSynthesisAssessment,
    ) -> SynthesizedEvidence:
        records = tuple(evidence)
        if not records:
            raise ValueError("synthesis requires evidence records")

        ids = []
        for item in records:
            if not isinstance(item, QualityAssessedEvidence):
                raise ValueError("synthesis requires quality/bias assessed evidence")
            if item.candidate.screening_status != "INCLUDED":
                raise ValueError("synthesis requires INCLUDED evidence")
            provider = item.candidate.provider_id.strip()
            record = item.candidate.provider_record_id.strip()
            if not provider or not record:
                raise ValueError("evidence records require provider and provider record identifiers")
            ids.append(f"{provider}:{record}")

        canonical_ids = tuple(ids)
        if len(set(canonical_ids)) != len(canonical_ids):
            raise ValueError("evidence records must have unique provider identifiers")
        if assessment.evidence_ids != canonical_ids:
            raise ValueError("assessment evidence_ids must match supplied evidence records")

        all_ids = set(canonical_ids)
        for group_name in (
            "supporting_evidence_ids",
            "null_evidence_ids",
            "contradictory_evidence_ids",
            "indirect_evidence_ids",
        ):
            if not set(getattr(assessment, group_name)).issubset(all_ids):
                raise ValueError(f"{group_name} contains an evidence identifier outside the supplied set")

        return SynthesizedEvidence(evidence=records, assessment=assessment)
