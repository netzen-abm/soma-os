"""Focused tests for the bounded evidence research core boundary."""
from __future__ import annotations

from pathlib import Path
import sys

ORCHESTRATOR_DIR = Path(__file__).resolve().parents[1] / "orchestrator"
CORE_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(ORCHESTRATOR_DIR))
sys.path.insert(0, str(CORE_DIR))

from evidence_research_core import EvidenceResearchCore  # noqa: E402
from multi_source_orchestrator import OrchestrationResult, ProviderResult  # noqa: E402


def make_result() -> OrchestrationResult:
    return OrchestrationResult(
        results=[
            ProviderResult(
                provider_id="pubmed",
                provider_record_id="123",
                title="Example study",
                source_class="literature",
                geography="global",
                source_url="https://pubmed.ncbi.nlm.nih.gov/123/",
                verification_url="https://pubmed.ncbi.nlm.nih.gov/123/",
                publication_year=2024,
                abstract_or_summary="Reported findings.",
                identifiers=("pmid:123",),
            )
        ],
        provider_status={"pubmed": "COMPLETED_WITH_RESULTS"},
        completed_routes=["literature"],
        failed_routes=[],
        status="COMPLETED_WITH_RESULTS",
    )


def test_consumption_preserves_identity_provenance_and_status() -> None:
    package = EvidenceResearchCore().consume(make_result(), ["q1"])
    candidate = package.candidates[0]
    assert candidate.provider_id == "pubmed"
    assert candidate.provider_record_id == "123"
    assert candidate.identifiers == ("pmid:123",)
    assert candidate.source_url.endswith("/123/")
    assert candidate.verification_url.endswith("/123/")
    assert candidate.screening_status == "UNSCREENED"
    assert candidate.directness is None
    assert package.orchestration_status == "COMPLETED_WITH_RESULTS"
    assert package.provider_status == (("pubmed", "COMPLETED_WITH_RESULTS"),)


def test_core_does_not_reclassify_orchestration_failure() -> None:
    failed = OrchestrationResult(
        results=[],
        provider_status={"pubmed": "PROVIDER_UNAVAILABLE"},
        completed_routes=[],
        failed_routes=["literature"],
        status="PROVIDER_UNAVAILABLE",
    )
    package = EvidenceResearchCore().consume(failed, ["q1"])
    assert package.candidates == ()
    assert package.orchestration_status == "PROVIDER_UNAVAILABLE"


def test_invalid_candidate_without_verification_is_rejected() -> None:
    bad = OrchestrationResult(
        results=[
            ProviderResult(
                provider_id="pubmed",
                provider_record_id="123",
                title="Example",
                source_class="literature",
                geography="global",
                source_url="",
                verification_url="",
            )
        ],
        provider_status={},
        completed_routes=[],
        failed_routes=[],
        status="COMPLETED_WITH_RESULTS",
    )
    try:
        EvidenceResearchCore().consume(bad, ["q1"])
    except ValueError:
        return
    raise AssertionError("missing verification links must be rejected")


def test_query_ids_are_required_and_normalized() -> None:
    package = EvidenceResearchCore().consume(make_result(), [" q1 ", "", "q2"])
    assert package.query_ids == ("q1", "q2")
    try:
        EvidenceResearchCore().consume(make_result(), ["", "  "])
    except ValueError:
        return
    raise AssertionError("missing query IDs must be rejected")
