"""Focused contract tests for the Unpaywall access-location resolver."""

from pathlib import Path
import sys

ADAPTER_DIR = (
    Path(__file__).resolve().parents[1]
    / "services"
    / "evidence-research"
    / "adapters"
)
sys.path.insert(0, str(ADAPTER_DIR))

from unpaywall_adapter import UnpaywallAdapter
from research_source_contract import ResearchAccessLocation


def test_source_contract() -> None:
    source = UnpaywallAdapter(email="research@example.org").source_contract()
    source.validate()
    assert source.source_id == "unpaywall"
    assert source.provider == "Unpaywall"
    assert source.protocol == "Unpaywall REST API"
    assert source.authentication_mode == "PUBLIC_API_EMAIL_REQUIRED"
    assert source.quality_status == "NOT_ASSESSED"


def test_doi_normalization() -> None:
    adapter = UnpaywallAdapter(email="research@example.org")
    assert adapter._normalize_doi("https://doi.org/10.1000/example") == "10.1000/example"
    assert adapter._normalize_doi("10.1000/example/") == "10.1000/example"


def test_access_locations_are_normalized_without_evidence_judgment() -> None:
    adapter = UnpaywallAdapter(email="research@example.org")
    adapter._request = lambda doi: {
        "doi": doi,
        "oa_status": "green",
        "oa_locations": [
            {
                "url": "https://repository.example/article",
                "host_type": "repository",
                "version": "acceptedVersion",
                "license": None,
            }
        ],
    }
    locations = adapter.resolve("https://doi.org/10.1000/example")
    assert len(locations) == 1
    location = locations[0]
    assert isinstance(location, ResearchAccessLocation)
    assert location.source_id == "unpaywall"
    assert location.url == "https://repository.example/article"
    assert location.location_type == "repository"
    assert location.is_open_access is True
    assert location.license_if_reported is None
    assert location.version_label == "acceptedVersion"


def test_successful_empty_resolution_is_distinct_from_provider_failure() -> None:
    adapter = UnpaywallAdapter(email="research@example.org")
    adapter._request = lambda doi: {"doi": doi, "oa_locations": []}
    assert adapter.resolve("10.1000/example") == ()


def test_email_is_required_before_provider_access() -> None:
    adapter = UnpaywallAdapter(email="")
    try:
        adapter.resolve("10.1000/example")
    except ValueError as exc:
        assert "email" in str(exc)
    else:
        raise AssertionError("Unpaywall access must require configured contact email")
