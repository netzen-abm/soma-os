from __future__ import annotations

import sys
from pathlib import Path

CORE_DIR = Path(__file__).resolve().parent
if str(CORE_DIR) not in sys.path:
    sys.path.insert(0, str(CORE_DIR))

import pytest

from evidence_quality_bias import (
    EvidenceQualityBiasAssessment,
    EvidenceQualityBiasBoundary,
)
from evidence_research_core import EvidenceCandidate
from evidence_screening import (
    EvidenceScreeningDecision,
    EvidenceScreeningBoundary,
)
from evidence_directness import (
    EvidenceDirectnessDecision,
    EvidenceDirectnessBoundary,
)


def candidate(status: str = "UNSCREENED") -> EvidenceCandidate:
    return EvidenceCandidate(
        provider_id="pubmed",
        provider_record_id="PM1",
        title="Example study",
        source_class="scholarly",
        geography="global",
        source_url="https://example.org/source",
        verification_url="https://example.org/verify",
        screening_status=status,
    )


def assessed_candidate() -> object:
    screened = EvidenceScreeningBoundary.apply(
        candidate(),
        EvidenceScreeningDecision(
            screening_status="INCLUDED",
            relevance="YES",
            protocol_match="YES",
            population_match="YES",
            outcome_match="YES",
            safety_relevance="YES",
            uncertainty="NO",
            assessed_at="2026-09-15T00:00:00Z",
            rationale="Meets the predefined inclusion criteria.",
        ),
    )
    return EvidenceDirectnessBoundary.apply(
        screened,
        EvidenceDirectnessDecision(
            directness="DIRECT_COMPONENT",
            rationale="Tests a defined component rather than the complete protocol.",
            assessed_at="2026-09-15T00:01:00Z",
            assessment_method="Human review of extracted study characteristics",
        ),
    )


def assessment(**overrides: str) -> EvidenceQualityBiasAssessment:
    values = {
        "study_design": "Randomized controlled trial",
        "risk_of_bias": "SOME_CONCERNS",
        "sample_size_precision": "SOME_CONCERNS",
        "comparator_quality": "FAVORABLE",
        "outcome_validity": "FAVORABLE",
        "follow_up_duration": "SOME_CONCERNS",
        "attrition": "FAVORABLE",
        "selective_reporting": "NOT_ASSESSED",
        "confounding": "FAVORABLE",
        "external_validity": "SOME_CONCERNS",
        "rationale": "Assessment records the available methodological information without collapsing it into a score.",
        "assessment_method": "Structured methodological review",
        "assessed_at": "2026-09-15T00:02:00Z",
    }
    values.update(overrides)
    return EvidenceQualityBiasAssessment(**values)


def test_assessment_preserves_candidate_and_dimensions() -> None:
    direct = assessed_candidate()
    result = EvidenceQualityBiasBoundary.apply(direct, assessment())

    assert result.candidate is direct.candidate
    assert result.assessment.risk_of_bias == "SOME_CONCERNS"
    assert result.assessment.selective_reporting == "NOT_ASSESSED"


def test_missing_information_is_not_treated_as_favorable() -> None:
    assert assessment(selective_reporting="NOT_ASSESSED").selective_reporting == "NOT_ASSESSED"


def test_invalid_judgment_is_rejected() -> None:
    with pytest.raises(ValueError, match="invalid risk_of_bias"):
        assessment(risk_of_bias="HIGH")


def test_empty_rationale_is_rejected() -> None:
    with pytest.raises(ValueError, match="rationale"):
        assessment(rationale="")


def test_non_included_candidate_cannot_be_assessed() -> None:
    screened = EvidenceScreeningBoundary.apply(
        candidate(),
        EvidenceScreeningDecision(
            screening_status="EXCLUDED",
            relevance="NO",
            protocol_match="NO",
            population_match="NOT_ASSESSED",
            outcome_match="NOT_ASSESSED",
            safety_relevance="NOT_ASSESSED",
            uncertainty="NO",
            exclusion_reason="Not relevant",
            assessed_at="2026-09-15T00:00:00Z",
            rationale="Fails relevance criterion.",
        ),
    )
    with pytest.raises(ValueError, match="INCLUDED"):
        EvidenceQualityBiasBoundary.apply(
            type("Direct", (), {"candidate": screened.candidate})(),
            assessment(),
        )
