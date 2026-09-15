"""Explicit, auditable screening boundary for SOMA evidence candidates.

This module records supplied screening decisions; it does not infer relevance,
protocol fit, safety significance, or evidence quality.
"""
from __future__ import annotations

from dataclasses import dataclass

from evidence_research_core import EvidenceCandidate


SCREENING_STATUSES = frozenset({"UNSCREENED", "INCLUDED", "EXCLUDED", "UNCERTAIN"})
SCREENING_VALUES = frozenset({"YES", "NO", "UNCERTAIN", "NOT_ASSESSED"})


@dataclass(frozen=True)
class EvidenceScreeningDecision:
    """One explicit screening decision supplied by a reviewer or governed rule."""

    screening_status: str
    relevance: str
    protocol_match: str
    population_match: str
    outcome_match: str
    safety_relevance: str
    duplicate_of: str | None = None
    exclusion_reason: str | None = None
    decision_method: str = "EXPLICIT_ASSESSMENT"

    def __post_init__(self) -> None:
        if self.screening_status not in SCREENING_STATUSES:
            raise ValueError("invalid screening_status")
        for field_name in (
            "relevance",
            "protocol_match",
            "population_match",
            "outcome_match",
            "safety_relevance",
        ):
            if getattr(self, field_name) not in SCREENING_VALUES:
                raise ValueError(f"invalid {field_name}")
        if self.screening_status == "EXCLUDED" and not (self.exclusion_reason or "").strip():
            raise ValueError("excluded evidence requires exclusion_reason")
        if self.duplicate_of is not None and not self.duplicate_of.strip():
            raise ValueError("duplicate_of must be non-empty when supplied")
        if not self.decision_method.strip():
            raise ValueError("decision_method must be non-empty")


@dataclass(frozen=True)
class ScreenedEvidenceCandidate:
    """Immutable candidate plus its explicit screening decision."""

    candidate: EvidenceCandidate
    decision: EvidenceScreeningDecision

    @property
    def provider_id(self) -> str:
        return self.candidate.provider_id

    @property
    def provider_record_id(self) -> str:
        return self.candidate.provider_record_id

    @property
    def screening_status(self) -> str:
        return self.decision.screening_status


class EvidenceScreeningBoundary:
    """Apply an explicit screening decision without performing screening itself."""

    @staticmethod
    def apply(
        candidate: EvidenceCandidate,
        decision: EvidenceScreeningDecision,
    ) -> ScreenedEvidenceCandidate:
        """Bind a supplied decision to a candidate without mutating source data."""
        if candidate.screening_status != "UNSCREENED":
            raise ValueError("candidate must be UNSCREENED before screening")
        if decision.duplicate_of == candidate.provider_record_id:
            raise ValueError("candidate cannot be a duplicate of itself")
        return ScreenedEvidenceCandidate(candidate=candidate, decision=decision)
