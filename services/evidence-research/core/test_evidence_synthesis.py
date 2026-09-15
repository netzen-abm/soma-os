from evidence_quality_bias import EvidenceQualityBiasAssessment, QualityAssessedEvidence
from evidence_research_core import EvidenceCandidate
from evidence_synthesis import EvidenceSynthesisAssessment, EvidenceSynthesisBoundary


def quality_evidence(provider: str = "pubmed", record: str = "123") -> QualityAssessedEvidence:
    candidate = EvidenceCandidate(
        provider_id=provider,
        provider_record_id=record,
        title="Study",
        source_class="SCHOLARLY",
        geography="global",
        source_url="https://example.org/source",
        verification_url="https://example.org/verify",
        publication_year=2026,
        abstract_or_summary=None,
        identifiers=(f"{provider}:{record}",),
        screening_status="INCLUDED",
    )
    assessment = EvidenceQualityBiasAssessment(
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
    return QualityAssessedEvidence(candidate=candidate, assessment=assessment)


def synthesis(evidence_ids: tuple[str, ...]) -> EvidenceSynthesisAssessment:
    return EvidenceSynthesisAssessment(
        status="MIXED",
        evidence_ids=evidence_ids,
        supporting_evidence_ids=evidence_ids[:1],
        contradictory_evidence_ids=evidence_ids[1:],
        rationale="Findings remain mixed",
        assessment_method="EXPLICIT_REVIEW",
        assessed_at="2026-09-15T00:00:00Z",
    )


def test_synthesis_preserves_assessed_evidence_and_identity() -> None:
    item = quality_evidence()
    result = EvidenceSynthesisBoundary.apply((item,), synthesis(("pubmed:123",)))
    assert result.evidence[0] is item
    assert result.assessment.status == "MIXED"


def test_synthesis_rejects_non_included_evidence() -> None:
    item = quality_evidence()
    candidate = EvidenceCandidate(
        provider_id=item.candidate.provider_id,
        provider_record_id=item.candidate.provider_record_id,
        title=item.candidate.title,
        source_class=item.candidate.source_class,
        geography=item.candidate.geography,
        source_url=item.candidate.source_url,
        verification_url=item.candidate.verification_url,
        publication_year=item.candidate.publication_year,
        abstract_or_summary=item.candidate.abstract_or_summary,
        identifiers=item.candidate.identifiers,
        screening_status="EXCLUDED",
    )
    rejected = QualityAssessedEvidence(candidate=candidate, assessment=item.assessment)
    try:
        EvidenceSynthesisBoundary.apply((rejected,), synthesis(("pubmed:123",)))
    except ValueError as exc:
        assert "INCLUDED" in str(exc)
    else:
        raise AssertionError("expected excluded evidence to be rejected")


def test_synthesis_rejects_unknown_group_identifier() -> None:
    item = quality_evidence()
    assessment = EvidenceSynthesisAssessment(
        status="SUPPORTIVE",
        evidence_ids=("pubmed:123",),
        supporting_evidence_ids=("pubmed:999",),
        rationale="Explicit review",
        assessment_method="EXPLICIT_REVIEW",
        assessed_at="2026-09-15T00:00:00Z",
    )
    try:
        EvidenceSynthesisBoundary.apply((item,), assessment)
    except ValueError as exc:
        assert "outside the supplied set" in str(exc)
    else:
        raise AssertionError("expected unknown evidence identifier to be rejected")
