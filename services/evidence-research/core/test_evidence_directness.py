from evidence_directness import EvidenceDirectnessBoundary, EvidenceDirectnessDecision
from evidence_research_core import EvidenceCandidate
from evidence_screening import EvidenceScreeningBoundary, EvidenceScreeningDecision


def included_candidate() -> object:
    candidate = EvidenceCandidate(
        provider_id="pubmed",
        provider_record_id="123",
        title="Study",
        source_class="SCHOLARLY",
        geography="global",
        source_url="https://example.org/source",
        verification_url="https://example.org/verify",
        publication_year=2025,
        abstract_or_summary="summary",
        identifiers=("PMID:123",),
    )
    screening = EvidenceScreeningDecision(
        screening_status="INCLUDED",
        relevance="YES",
        protocol_match="YES",
        population_match="YES",
        outcome_match="YES",
        safety_relevance="NOT_ASSESSED",
    )
    return EvidenceScreeningBoundary.apply(candidate, screening)


def test_binds_explicit_directness_without_mutating_candidate() -> None:
    screened = included_candidate()
    decision = EvidenceDirectnessDecision(
        directness="DIRECT_EXACT_PROTOCOL",
        rationale="The study tested the complete protocol under the stated conditions.",
        assessed_at="2026-09-15T10:00:00+05:30",
    )
    assessed = EvidenceDirectnessBoundary.apply(screened, decision)
    assert assessed.candidate is screened.candidate
    assert assessed.decision.directness == "DIRECT_EXACT_PROTOCOL"


def test_strong_indirectness_remains_indirect() -> None:
    screened = included_candidate()
    decision = EvidenceDirectnessDecision(
        directness="INDIRECT",
        rationale="The intervention and population differ materially from the protocol.",
        assessed_at="2026-09-15T10:00:00+05:30",
    )
    assessed = EvidenceDirectnessBoundary.apply(screened, decision)
    assert assessed.decision.directness == "INDIRECT"


def test_non_included_candidate_cannot_receive_directness() -> None:
    candidate = EvidenceCandidate(
        provider_id="pubmed",
        provider_record_id="456",
        title="Study",
        source_class="SCHOLARLY",
        geography="global",
        source_url="https://example.org/source",
        verification_url="https://example.org/verify",
        publication_year=None,
        abstract_or_summary=None,
        identifiers=(),
    )
    screening = EvidenceScreeningBoundary.apply(
        candidate,
        EvidenceScreeningDecision(
            screening_status="UNCERTAIN",
            relevance="UNCERTAIN",
            protocol_match="UNCERTAIN",
            population_match="UNCERTAIN",
            outcome_match="UNCERTAIN",
            safety_relevance="NOT_ASSESSED",
        ),
    )
    decision = EvidenceDirectnessDecision(
        directness="RELATED_INTERVENTION",
        rationale="Related intervention only.",
        assessed_at="2026-09-15T10:00:00+05:30",
    )
    try:
        EvidenceDirectnessBoundary.apply(screening, decision)
    except ValueError as exc:
        assert "INCLUDED" in str(exc)
    else:
        raise AssertionError("non-included candidate was accepted")


def test_no_evidence_search_outcome_is_not_a_study_directness_value() -> None:
    try:
        EvidenceDirectnessDecision(
            directness="NO_RELEVANT_EVIDENCE_FOUND",
            rationale="No study was found.",
            assessed_at="2026-09-15T10:00:00+05:30",
        )
        raise AssertionError("search outcome was accepted as a study decision")
    except ValueError as exc:
        assert "search outcome" in str(exc)


def test_invalid_directness_and_missing_rationale_rejected() -> None:
    for value in ("", "STRONG", "CAUSES"):
        try:
            EvidenceDirectnessDecision(
                directness=value,
                rationale="Reason",
                assessed_at="2026-09-15T10:00:00+05:30",
            )
        except ValueError:
            pass
        else:
            raise AssertionError("invalid directness accepted")

    try:
        EvidenceDirectnessDecision(
            directness="MECHANISTIC",
            rationale=" ",
            assessed_at="2026-09-15T10:00:00+05:30",
        )
    except ValueError:
        pass
    else:
        raise AssertionError("empty rationale accepted")
