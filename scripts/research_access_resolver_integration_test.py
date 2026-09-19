"""Integration tests for research access resolver orchestration."""

from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "services" / "evidence-research" / "adapters"))
sys.path.insert(0, str(ROOT / "services" / "evidence-research" / "orchestrator"))

from multi_source_orchestrator import MultiSourceOrchestrator, SearchRoute
from research_source_contract import ResearchQuery, ResearchRecord, ResearchAccessLocation


class FakeAdapter:
    provider_id = "openalex"

    def search(self, query):
        return [ResearchRecord(
            provider_id="openalex",
            provider_record_id="W1",
            title="Example",
            source_class=query.source_class,
            geography=query.geography,
            source_url="https://openalex.org/W1",
            verification_url="https://doi.org/10.1000/example",
            identifiers=("doi:10.1000/example",),
        )]


class FakeResolver:
    provider_id = "unpaywall"

    def resolve(self, doi):
        return (ResearchAccessLocation(
            location_id="u1",
            source_id="unpaywall",
            url="https://repository.example/article",
            location_type="repository",
            is_open_access=True,
            license_if_reported=None,
        ),)


def test_orchestrator_composes_access_locations():
    result = MultiSourceOrchestrator(
        {"openalex": FakeAdapter()},
        {"unpaywall": FakeResolver()},
    ).run(
        [ResearchQuery("q1", "example", "biomedical_literature")],
        [SearchRoute("biomedical_literature", ("openalex",))],
    )
    assert len(result.results) == 1
    assert len(result.results[0].access_locations) == 1
    assert result.provider_status["access:unpaywall"] == "COMPLETED_WITH_RESULTS"


class FailingResolver(FakeResolver):
    def resolve(self, doi):
        raise RuntimeError("provider unavailable")


def test_resolver_failure_is_not_evidence_absence():
    result = MultiSourceOrchestrator(
        {"openalex": FakeAdapter()},
        {"unpaywall": FailingResolver()},
    ).run(
        [ResearchQuery("q1", "example", "biomedical_literature")],
        [SearchRoute("biomedical_literature", ("openalex",))],
    )
    assert len(result.results) == 1
    assert result.results[0].access_locations == ()
    assert result.status == "COMPLETED_WITH_RESULTS"
    assert result.provider_status["access:unpaywall"] == "PROVIDER_UNAVAILABLE"
