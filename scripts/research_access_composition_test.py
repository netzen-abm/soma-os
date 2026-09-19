"""Focused tests for canonical research access-location composition."""

from pathlib import Path
import sys

ADAPTER_DIR = (
    Path(__file__).resolve().parents[1]
    / "services"
    / "evidence-research"
    / "adapters"
)
ORCH_DIR = Path(__file__).resolve().parents[1] / "services" / "evidence-research" / "orchestrator"
sys.path.insert(0, str(ADAPTER_DIR))
sys.path.insert(0, str(ORCH_DIR))

from research_source_contract import ResearchRecord, ResearchAccessLocation
from multi_source_orchestrator import attach_access_locations


class FakeResolver:
    provider_id = "unpaywall"

    def resolve(self, doi: str):
        assert doi == "10.1000/example"
        return (
            ResearchAccessLocation(
                location_id="unpaywall:10.1000/example:0",
                source_id="unpaywall",
                url="https://repository.example/article",
                location_type="repository",
                is_open_access=True,
                license_if_reported=None,
                version_label="acceptedVersion",
            ),
        )


def test_access_locations_attach_without_changing_record_identity() -> None:
    record = ResearchRecord(
        provider_id="openalex",
        provider_record_id="https://openalex.org/W1",
        title="Example",
        source_class="biomedical_literature",
        geography="global",
        source_url="https://openalex.org/W1",
        verification_url="https://doi.org/10.1000/example",
        identifiers=("doi:10.1000/example",),
    )

    enriched = attach_access_locations(record, FakeResolver())

    assert enriched.provider_id == record.provider_id
    assert enriched.provider_record_id == record.provider_record_id
    assert enriched.title == record.title
    assert enriched.identifiers == record.identifiers
    assert len(enriched.access_locations) == 1
    assert enriched.access_locations[0].source_id == "unpaywall"


def test_records_without_doi_are_left_unchanged() -> None:
    record = ResearchRecord(
        provider_id="pubmed",
        provider_record_id="123",
        title="Example",
        source_class="biomedical_literature",
        geography="global",
        source_url="https://pubmed.ncbi.nlm.nih.gov/123/",
        verification_url="https://pubmed.ncbi.nlm.nih.gov/123/",
    )
    enriched = attach_access_locations(record, FakeResolver())
    assert enriched == record


def test_existing_access_locations_are_replaced_by_fresh_resolution() -> None:
    existing = ResearchAccessLocation(
        location_id="old",
        source_id="unpaywall",
        url="https://old.example",
        location_type="repository",
        is_open_access=True,
        license_if_reported=None,
    )
    record = ResearchRecord(
        provider_id="openalex",
        provider_record_id="W1",
        title="Example",
        source_class="biomedical_literature",
        geography="global",
        source_url="https://openalex.org/W1",
        verification_url="https://doi.org/10.1000/example",
        identifiers=("https://doi.org/10.1000/example",),
        access_locations=(existing,),
    )
    enriched = attach_access_locations(record, FakeResolver())
    assert enriched.access_locations[0].location_id != "old"
