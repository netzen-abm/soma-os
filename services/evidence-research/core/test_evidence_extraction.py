from evidence_extraction import StructuredEvidenceExtractionBoundary
from evidence_research_core import EvidenceCandidate
from evidence_screening import EvidenceScreeningBoundary, EvidenceScreeningDecision


def candidate():
    return EvidenceCandidate(
        provider_id="pubmed",
        provider_record_id="123",
        title="Study",
        source_class="biomedical_literature",
        geography="global",
        source_url="https://example.org/source",
        verification_url="https://example.org/verify",
        publication_year=2025,
        abstract_or_summary="source text",
        identifiers=("PMID:123",),
    )


def included():
    return EvidenceScreeningBoundary.apply(
        candidate(),
        EvidenceScreeningDecision(
            screening_status="INCLUDED",
            relevance="RELEVANT",
            protocol_match="DIRECT_COMPONENT",
            population_match="MATCHED",
            outcome_match="MATCHED",
            safety_relevance="RELEVANT",
        ),
    )


def test_extraction_preserves_candidate_and_explicit_facts():
    screened = included()
    result = StructuredEvidenceExtractionBoundary.from_mapping(
        screened,
        {
            "authors": ["A. Researcher"],
            "year": 2025,
            "study_type": "randomized_trial",
            "sample_size": 120,
            "outcomes": ["Outcome A"],
            "effect_estimates": ["RR 0.80 (95% CI 0.70-0.92)"],
            "main_findings": "Reported finding",
        },
    )
    assert result.candidate is screened.candidate
    assert result.extraction.authors == ("A. Researcher",)
    assert result.extraction.sample_size == 120
    assert result.extraction.effect_estimates[0].startswith("RR")


def test_missing_fields_remain_missing():
    result = StructuredEvidenceExtractionBoundary.from_mapping(included(), {})
    assert result.extraction.title is None
    assert result.extraction.population is None
    assert result.extraction.outcomes == ()


def test_rejects_unknown_fields():
    try:
        StructuredEvidenceExtractionBoundary.from_mapping(included(), {"invented_fact": "x"})
    except ValueError as exc:
        assert "unknown extraction fields" in str(exc)
    else:
        raise AssertionError("unknown fields must be rejected")


def test_requires_included_candidate():
    screened = EvidenceScreeningBoundary.apply(
        candidate(),
        EvidenceScreeningDecision(screening_status="EXCLUDED", exclusion_reason="not relevant"),
    )
    try:
        StructuredEvidenceExtractionBoundary.from_mapping(screened, {})
    except ValueError as exc:
        assert "INCLUDED" in str(exc)
    else:
        raise AssertionError("excluded candidate must not enter extraction")
