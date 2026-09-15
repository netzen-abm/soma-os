"""Shared immutable assessment-chain envelope for SOMA evidence.

This module preserves prior study-level assessments without creating a new
clinical or evidence store. It prevents downstream boundaries from silently
losing directness or quality/bias provenance.
"""
from __future__ import annotations

from dataclasses import dataclass

from evidence_directness import DirectlyAssessedEvidence
from evidence_quality_bias import EvidenceQualityBiasAssessment


@dataclass(frozen=True)
class QualityAssessedEvidenceChain:
    """Directness and quality/bias assessments over one evidence candidate."""

    directness: DirectlyAssessedEvidence
    quality_bias: EvidenceQualityBiasAssessment

    @property
    def candidate(self):
        return self.directness.candidate

    def __post_init__(self) -> None:
        if self.directness.candidate.screening_status != "INCLUDED":
            raise ValueError("assessment chain requires an INCLUDED candidate")
