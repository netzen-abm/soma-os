"""Bounded directness assessment boundary for SOMA evidence.

Directness records how closely a screened study addresses a protocol or
research question. It is independent of evidence strength, quality, bias,
causality, safety, and clinical recommendation.
"""
from __future__ import annotations

from dataclasses import dataclass

from evidence_research_core import EvidenceCandidate
from evidence_screening import ScreenedEvidenceCandidate

DIRECTNESS_VALUES = frozenset({
    "DIRECT_EXACT_PROTOCOL",
    "DIRECT_COMPONENT",
    "RELATED_INTERVENTION",
    "MECHANISTIC",
    "OBSERVATIONAL_ASSOCIATION",
    "PRECLINICAL",
    "INDIRECT",
    "NO_RELEVANT_EVIDENCE_FOUND",
})


@dataclass(frozen=True)
class EvidenceDirectnessDecision:
    """Explicit directness classification supplied by an assessor/rule."""

    directness: str
    rationale: str
    assessed_at: str
    assessment_method: str = "EXPLICIT_ASSESSMENT"

    def __post_init__(self) -> None:
        if self.directness not in DIRECTNESS_VALUES:
            raise ValueError("invalid directness")
        if not self.rationale.strip():
            raise ValueError("directness rationale must be non-empty")
        if not self.assessed_at.strip():
            raise ValueError("assessed_at must be non-empty")
        if not self.assessment_method.strip():
            raise ValueError("assessment_method must be non-empty")


@dataclass(frozen=True)
class DirectlyAssessedEvidence:
    """Structured extraction plus explicit directness, preserving provenance."""

    candidate: EvidenceCandidate
    decision: EvidenceDirectnessDecision


class EvidenceDirectnessBoundary:
    """Bind an explicit directness decision to an included screened candidate."""

    @staticmethod
    def apply(
        screened: ScreenedEvidenceCandidate,
        decision: EvidenceDirectnessDecision,
    ) -> DirectlyAssessedEvidence:
        if screened.screening_status != "INCLUDED":
            raise ValueError("directness assessment requires an INCLUDED candidate")
        if decision.directness == "NO_RELEVANT_EVIDENCE_FOUND":
            raise ValueError("NO_RELEVANT_EVIDENCE_FOUND is a search outcome, not a study directness classification")
        return DirectlyAssessedEvidence(candidate=screened.candidate, decision=decision)
