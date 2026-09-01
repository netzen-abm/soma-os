from multi_source_orchestrator import (
    MultiSourceOrchestrator,
    ProviderResult,
    ResearchQuery,
    SearchRoute,
)


class FakeAdapter:
    def __init__(self, provider_id, results=None, error=None):
        self.provider_id = provider_id
        self.results = results or []
        self.error = error

    def search(self, query):
        if self.error:
            raise self.error
        return self.results


def study(provider, identifier, title="Millet and glycemic outcomes"):
    return ProviderResult(
        provider_id=provider,
        provider_record_id=identifier,
        title=title,
        source_class="biomedical_literature",
        geography="global",
        source_url=f"https://example.test/{identifier}",
        verification_url=f"https://example.test/{identifier}",
        publication_year=2024,
        identifiers=(identifier,),
    )


def test_same_doi_across_providers_is_one_underlying_study():
    adapters = {
        "pubmed": FakeAdapter("pubmed", [study("pubmed", "doi:10.1000/example")]),
        "europe_pmc": FakeAdapter("europe_pmc", [study("europe_pmc", "doi:10.1000/example")]),
    }
    orchestrator = MultiSourceOrchestrator(adapters)
    result = orchestrator.run(
        [ResearchQuery("q1", "millet diabetes", "biomedical_literature")],
        [SearchRoute("biomedical_literature", ("pubmed", "europe_pmc"))],
    )
    assert len(result.results) == 1
    assert result.status == "COMPLETED_WITH_RESULTS"


def test_india_route_can_combine_indian_and_global_sources():
    adapters = {
        "ctri": FakeAdapter("ctri", [study("ctri", "ctri:2024/01/001")]),
        "pubmed": FakeAdapter("pubmed", [study("pubmed", "pmid:12345678")]),
        "ayush": FakeAdapter("ayush", [study("ayush", "ayush:record-1")]),
    }
    orchestrator = MultiSourceOrchestrator(adapters)
    result = orchestrator.run(
        [
            ResearchQuery("q-india", "millet diabetes", "clinical_trials", "india"),
            ResearchQuery("q-lit", "millet diabetes", "biomedical_literature", "global"),
            ResearchQuery("q-trad", "millet diabetes", "traditional_knowledge", "india"),
        ],
        [
            SearchRoute("clinical_trials", ("ctri",)),
            SearchRoute("biomedical_literature", ("pubmed",)),
            SearchRoute("traditional_knowledge", ("ayush",)),
        ],
    )
    assert {r.provider_id for r in result.results} == {"ctri", "pubmed", "ayush"}
    assert result.status == "COMPLETED_WITH_RESULTS"


def test_provider_failure_is_not_no_evidence():
    orchestrator = MultiSourceOrchestrator({
        "pubmed": FakeAdapter("pubmed", error=RuntimeError("timeout")),
    })
    result = orchestrator.run(
        [ResearchQuery("q1", "millet diabetes", "biomedical_literature")],
        [SearchRoute("biomedical_literature", ("pubmed",))],
    )
    assert result.status == "PARTIAL_PROVIDER_FAILURE"
    assert result.provider_status["pubmed"] == "PROVIDER_UNAVAILABLE"
    assert result.results == []


def test_completed_empty_search_is_distinct_from_provider_failure():
    orchestrator = MultiSourceOrchestrator({
        "pubmed": FakeAdapter("pubmed", results=[]),
    })
    result = orchestrator.run(
        [ResearchQuery("q1", "very-specific-query", "biomedical_literature")],
        [SearchRoute("biomedical_literature", ("pubmed",))],
    )
    assert result.status == "COMPLETED_NO_RELEVANT_RESULTS"
    assert result.provider_status["pubmed"] == "COMPLETED_NO_RESULTS"


def test_contradictory_records_are_not_dropped_by_source_deduplication():
    supporting = study("pubmed", "doi:10.1000/support", "Millet improves glycemic outcomes")
    contradictory = study("europe_pmc", "doi:10.1000/contradict", "Millet shows no significant glycemic benefit")
    orchestrator = MultiSourceOrchestrator({
        "pubmed": FakeAdapter("pubmed", [supporting]),
        "europe_pmc": FakeAdapter("europe_pmc", [contradictory]),
    })
    result = orchestrator.run(
        [ResearchQuery("q1", "millet diabetes", "biomedical_literature")],
        [SearchRoute("biomedical_literature", ("pubmed", "europe_pmc"))],
    )
    assert len(result.results) == 2


def test_verification_links_and_provider_provenance_survive_normalization():
    record = study("pubmed", "pmid:999", "A study")
    orchestrator = MultiSourceOrchestrator({"pubmed": FakeAdapter("pubmed", [record])})
    result = orchestrator.run(
        [ResearchQuery("q1", "study", "biomedical_literature")],
        [SearchRoute("biomedical_literature", ("pubmed",))],
    )
    assert result.results[0].provider_id == "pubmed"
    assert result.results[0].verification_url.endswith("pmid:999")
