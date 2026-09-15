from __future__ import annotations

import sys
from pathlib import Path

CORE_DIR = Path(__file__).resolve().parent
if str(CORE_DIR) not in sys.path:
    sys.path.insert(0, str(CORE_DIR))

import pytest

from evidence_consistency import (
    EvidenceConsistencyAssessment,
    EvidenceConsistencyBoundary,
)
from evidence_directness import EvidenceDirectnessDecision, EvidenceDirectnessBoundary
from evidence_quality_bias import EvidenceQualityBiasAssessment, EvidenceQualityBiasBoundary
from evidence_research_core import EvidenceCandidate
from evidence_screening import EvidenceScreeningBoundary, EvidenceScreeningDecision


def assessed_evidence(provider: str, record: str):
    candidate = EvidenceCandidate(
        provider_id=provider,
        provider_record_id=record,
        title=f"Study {record}",
        source_class="scholarly",
        geography="global",
        source_url="https://example.org/source/" + record,
        verification_url="https://example.org/verify/" + record,
    )
    screened = EvidenceScreeningBoundary.apply(
        candidate,
        EvidenceScreeningDecision(
            screening_status="INCLUDED",
            relevance="YES",
            protocol_match="YES",
            population_match="YES",
            outcome_match="YES",
            safety_relevance="YES",
            uncertainty="NO",
            assessed_at="2026-09-15T00:00:00Z",
            rationale="Meets predefined inclusion criteria.",
        ),
    )
    direct = EvidenceDirectnessBoundary.apply(
        screened,
        EvidenceDirectnessDecision(
            directness="DIRECT_COMPONENT",
            rationale="Addresses a defined component.",
            assessed_at="2026-09-15T00:01:00Z",
        ),
    )
    quality = EvidenceQualityBiasBoundary.apply(
        direct,
        EvidenceQualityBiasAssessment(
            study_design="Randomized controlled trial",
            risk_of_bias="SOME_CONCERNS",
            sample_size_precision="SOME_CONCERNS",
            comparator_quality="FAVORABLE",
            outcome_validity="FAVORABLE",
            follow_up_duration="SOME_CONCERNS",
            attrition="FAVORABLE",
            selective_reporting="NOT_ASSESSED",
            confounding="FAVORABLE",
            external_validity="SOME_CONCERNS",
            rationale="Structured review of available methodological information.",
            assessment_method="Structured methodological review",
            assessed_at="2026-09-15T00:02:00Z",
        ),
    )
    return quality


def assessment(ids: tuple[str, ...], consistency: str = "CONSISTENT"):
    return EvidenceConsistencyAssessment(
        consistency=consistency,
        evidence_ids=ids,
        rationale="The assessed evidence points in a materially compatible direction for the research question.",
        assessment_method="Explicit cross-study reviewer assessment",
        assessed_at="2026-09-15T00:03:00Z",
    )


def test_consistency_preserves_evidence_and_mixed_state() -> None:
    first = assessed_evidence("pubmed", "PM1")
    second = assessed_evidence("openalex", "W2")
    result = EvidenceConsistencyBoundary.apply(
        [first, second],
        assessment(("pubmed:PM1", "openalex:W2"), "MIXED"),
    )

    assert result.evidence[0] is first
    assert result.evidence[1] is second
    assert result.assessment.consistency == "MIXED"


def test_at_least_two_records_required() -> None:
    first = assessed_evidence("pubmed", "PM1")
    with pytest.raises(ValueError, match="at least two"):
        EvidenceConsistencyBoundary.apply(
            [first],
            assessment(("pubmed:PM1", "openalex:W2")),
        )


def test_assessment_ids_must_match_records() -> None:
    first = assessed_evidence("pubmed", "PM1")
    second = assessed_evidence("openalex", "W2")
    with pytest.raises(ValueError, match="match"):
        EvidenceConsistencyBoundary.apply(
            [first, second],
            assessment(("pubmed:PM1", "openalex:WRONG")),
        )


def test_duplicate_record_ids_are_rejected() -> None:
    first = assessed_evidence("pubmed", "PM1")
    second = assessed_evidence("pubmed", "PM1")
    with pytest.raises(ValueError, match="unique"):
        EvidenceConsistencyBoundary.apply(
            [first, second],
            assessment(("pubmed:PM1", "pubmed:PM1")),
        )


def test_invalid_consistency_judgment_is_rejected() -> None:
    with pytest.raises(ValueError, match="invalid consistency"):
        assessment(("pubmed:PM1", "openalex:W2"), "POSITIVE")


def test_not_assessed_remains_explicit() -> None:
    result = EvidenceConsistencyAssessment(
        consistency="NOT_ASSESSED",
        evidence_ids=("pubmed:PM1", "openalex:W2"),
        rationale="Available evidence is insufficient for a consistency judgment.",
        assessment_method="Explicit reviewer assessment",
        assessed_at="2026-09-15T00:03:00Z",
    )
    assert result.consistency == "NOT_ASSESSED"
