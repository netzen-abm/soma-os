"""Bounded core consumption boundary for SOMA evidence research.

This module consumes the existing multi-source orchestration envelope and
creates a canonical, assessment-ready research package. It does not perform
clinical interpretation, evidence grading, causal inference, or synthesis.
"""
from __future__ import annotations

from dataclasses import dataclass
from typing import Sequence

from multi_source_orchestrator import OrchestrationResult, ProviderResult


@dataclass(frozen=True)
class EvidenceCandidate:
    """One deduplicated research record awaiting explicit assessment."""

    provider_id: str
    provider_record_id: str
    title: str
    source_class: str
    geography: str
    source_url: str
    verification_url: str
    publication_year: int | None
    abstract_or_summary: str | None
    identifiers: tuple[str, ...]
    screening_status: str = "UNSCREENED"
    relevance: str | None = None
    protocol_match: str | None = None
    population_match: str | None = None
    outcome_match: str | None = None
    safety_relevance: str | None = None
    duplicate_of: str | None = None
    exclusion_reason: str | None = None
    directness: str | None = None


@dataclass(frozen=True)
class EvidenceResearchPackage:
    """Assessment-ready package derived from one orchestration run."""

    query_ids: tuple[str, ...]
    candidates: tuple[EvidenceCandidate, ...]
    provider_status: tuple[tuple[str, str], ...]
    completed_routes: tuple[str, ...]
    failed_routes: tuple[str, ...]
    orchestration_status: str


class EvidenceResearchCore:
    """Create an assessment-ready package without making evidence judgments."""

    @staticmethod
    def _candidate(result: ProviderResult) -> EvidenceCandidate:
        if not result.provider_id.strip() or not result.provider_record_id.strip():
            raise ValueError("provider result must contain provider identity")
        if not result.source_url.strip() or not result.verification_url.strip():
            raise ValueError("provider result must preserve source verification links")
        return EvidenceCandidate(
            provider_id=result.provider_id,
            provider_record_id=result.provider_record_id,
            title=result.title,
            source_class=result.source_class,
            geography=result.geography,
            source_url=result.source_url,
            verification_url=result.verification_url,
            publication_year=result.publication_year,
            abstract_or_summary=result.abstract_or_summary,
            identifiers=result.identifiers,
        )

    def consume(
        self,
        orchestration: OrchestrationResult,
        query_ids: Sequence[str],
    ) -> EvidenceResearchPackage:
        """Consume a completed orchestration envelope without reassessing it."""
        normalized_query_ids = tuple(query_id.strip() for query_id in query_ids if query_id.strip())
        if not normalized_query_ids:
            raise ValueError("at least one query_id is required")

        candidates = tuple(self._candidate(result) for result in orchestration.results)
        return EvidenceResearchPackage(
            query_ids=normalized_query_ids,
            candidates=candidates,
            provider_status=tuple(sorted(orchestration.provider_status.items())),
            completed_routes=tuple(orchestration.completed_routes),
            failed_routes=tuple(orchestration.failed_routes),
            orchestration_status=orchestration.status,
        )
