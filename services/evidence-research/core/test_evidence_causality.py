from evidence_causality import EvidenceCausalityAssessment, EvidenceCausalityBoundary
from evidence_safety_gate import EvidenceSafetyAssessment, EvidenceSafetyGate
from evidence_synthesis import EvidenceSynthesisAssessment, EvidenceSynthesisBoundary
from evidence_quality_bias import EvidenceQualityBiasAssessment, QualityAssessedEvidence
from evidence_research_core import EvidenceCandidate


def assessed_item() -> QualityAssessedEvidence:
    candidate = EvidenceCandidate(
        provider_id="pubmed",
        provider_record_id="123",
        title="Study",
        source_class="SCHOLARLY",
        geography="global",
        source_url="https://example.org/source",
        verification_url="https://example.org/verify",
        publication_year=2026,
        abstract_or_summary=None,
        identifiers=("pubmed:123",),
        screening_status="INCLUDED",
    )
    quality = EvidenceQualityBiasAssessment(
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
        rationale="Explicit review",
        assessment_method="HUMAN_REVIEW",
        assessed_at="2026-09-15T00:00:00Z",
    )
    return QualityAssessedEvidence(candidate=candidate, assessment=quality)


def gated() -> object:
    item = assessed_item()
    synthesis = EvidenceSynthesisBoundary.apply(
        (item,),
        EvidenceSynthesisAssessment(
            status="MIXED",
            evidence_ids=("pubmed:123",),
            rationale="Findings remain mixed",
            assessment_method="EXPLICIT_REVIEW",
            assessed_at="2026-09-15T00:00:00Z",
        ),
    )
    return EvidenceSafetyGate.apply(
        synthesis,
        EvidenceSafetyAssessment(
            status="INSUFFICIENT_SAFETY_INFORMATION",
            evidence_ids=("pubmed:123",),
            rationale="Safety evidence remains limited",
            assessment_method="EXPLICIT_REVIEW",
            assessed_at="2026-09-15T00:00:00Z",
        ),
    )


def test_causality_preserves_safety_gated_evidence() -> None:
    safety_gated = gated()
    assessment = EvidenceCausalityAssessment(
        status="CAUSALITY_UNLIKELY",
        evidence_ids=("pubmed:123",),
        temporal_relationship="Outcome followed exposure with compatible timing",
        alternative_explanations=("Concurrent illness",),
        rationale="Alternative explanation remains stronger",
        assessment_method="EXPLICIT_REVIEW",
        assessed_at="2026-09-15T00:00:00Z",
    )
    result = EvidenceCausalityBoundary.apply(safety_gated, assessment)
    assert result.safety_gated_evidence is safety_gated
    assert result.assessment.status == "CAUSALITY_UNLIKELY"


def test_causality_rejects_mismatched_evidence_ids() -> None:
    safety_gated = gated()
    assessment = EvidenceCausalityAssessment(
        status="CAUSALITY_UNCERTAIN" if False else "INSUFFICIENT_CAUSALITY_INFORMATION",
        evidence_ids=("pubmed:999",),
        temporal_relationship="Temporal relationship recorded",
        rationale="Insufficient information",
        assessment_method="EXPLICIT_REVIEW",
        assessed_at="2026-09-15T00:00:00Z",
    )
    try:
        EvidenceCausalityBoundary.apply(safety_gated, assessment)
    except ValueError as exc:
        assert "match safety-gated evidence" in str(exc)
    else:
        raise AssertionError("expected mismatched evidence identifiers to be rejected")


def test_causality_requires_explicit_temporal_relationship() -> None:
    try:
        EvidenceCausalityAssessment(
            status="NOT_ASSESSED",
            evidence_ids=("pubmed:123",),
            temporal_relationship="",
            rationale="Not assessed",
            assessment_method="EXPLICIT_REVIEW",
            assessed_at="2026-09-15T00:00:00Z",
        )
    except ValueError as exc:
        assert "temporal_relationship" in str(exc)
    else:
        raise AssertionError("expected temporal relationship to be required")
