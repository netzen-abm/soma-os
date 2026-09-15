from evidence_research_core import EvidenceCandidate
from evidence_screening import EvidenceScreeningBoundary, EvidenceScreeningDecision


def candidate() -> EvidenceCandidate:
    return EvidenceCandidate(
        provider_id="pubmed",
        provider_record_id="123",
        title="Example study",
        source_class="biomedical_literature",
        geography="global",
        source_url="https://example.org/source/123",
        verification_url="https://pubmed.ncbi.nlm.nih.gov/123/",
        publication_year=2026,
        abstract_or_summary="Source-derived abstract",
        identifiers=("pmid:123",),
    )


def decision(**overrides: str | None) -> EvidenceScreeningDecision:
    values = {
        "screening_status": "INCLUDED",
        "relevance": "YES",
        "protocol_match": "UNCERTAIN",
        "population_match": "YES",
        "outcome_match": "YES",
        "safety_relevance": "NOT_ASSESSED",
    }
    values.update(overrides)
    return EvidenceScreeningDecision(**values)


def test_apply_preserves_candidate_and_records_explicit_decision() -> None:
    original = candidate()
    screened = EvidenceScreeningBoundary.apply(original, decision())

    assert screened.candidate is original
    assert screened.provider_id == "pubmed"
    assert screened.provider_record_id == "123"
    assert screened.screening_status == "INCLUDED"
    assert screened.decision.protocol_match == "UNCERTAIN"
    assert screened.decision.decision_method == "EXPLICIT_ASSESSMENT"


def test_excluded_requires_reason() -> None:
    try:
        decision(screening_status="EXCLUDED")
    except ValueError as exc:
        assert str(exc) == "excluded evidence requires exclusion_reason"
    else:
        raise AssertionError("expected exclusion reason validation")


def test_invalid_dimension_is_rejected() -> None:
    try:
        decision(relevance="MAYBE")
    except ValueError as exc:
        assert str(exc) == "invalid relevance"
    else:
        raise AssertionError("expected screening value validation")


def test_duplicate_cannot_point_to_itself() -> None:
    try:
        EvidenceScreeningBoundary.apply(candidate(), decision(duplicate_of="123"))
    except ValueError as exc:
        assert str(exc) == "candidate cannot be a duplicate of itself"
    else:
        raise AssertionError("expected self-duplicate validation")


def test_already_screened_candidate_cannot_be_reapplied() -> None:
    screened = EvidenceScreeningBoundary.apply(candidate(), decision())
    try:
        EvidenceScreeningBoundary.apply(
            EvidenceCandidate(
                provider_id=screened.candidate.provider_id,
                provider_record_id=screened.candidate.provider_record_id,
                title=screened.candidate.title,
                source_class=screened.candidate.source_class,
                geography=screened.candidate.geography,
                source_url=screened.candidate.source_url,
                verification_url=screened.candidate.verification_url,
                publication_year=screened.candidate.publication_year,
                abstract_or_summary=screened.candidate.abstract_or_summary,
                identifiers=screened.candidate.identifiers,
                screening_status="INCLUDED",
            ),
            decision(),
        )
    except ValueError as exc:
        assert str(exc) == "candidate must be UNSCREENED before screening"
    else:
        raise AssertionError("expected screening-state validation")
