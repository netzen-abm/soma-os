"""Focused contract tests for the OWID source integration."""

from pathlib import Path
import sys

ADAPTER_DIR = (
    Path(__file__).resolve().parents[1]
    / "services"
    / "evidence-research"
    / "adapters"
)
sys.path.insert(0, str(ADAPTER_DIR))

from owid_adapter import OwidAdapter, OwidProviderError


def test_source_contract() -> None:
    source = OwidAdapter().source_contract()
    source.validate()
    assert source.source_id == "owid"
    assert source.provider == "Our World in Data"
    assert source.protocol == "OWID Grapher CSV and metadata JSON"
    assert source.authentication_mode == "PUBLIC_NO_KEY"
    assert source.quality_status == "NOT_ASSESSED"
    assert source.verification_status == "SOURCE_VERIFIED"
    assert source.source_version is None
    assert source.published_at is None


def test_response_mapping_preserves_identity_and_unknown_date(monkeypatch) -> None:
    adapter = OwidAdapter()

    def fake_request(path: str) -> bytes:
        if path.endswith(".csv"):
            return b"Entity,Code,Year,value\nIndia,IND,2024,12.5\n"
        return b'{"title":"Example indicator","description":"Source metadata"}'

    monkeypatch.setattr(adapter, "_request", fake_request)
    result = adapter.fetch(type("Q", (), {"slug": "example-indicator", "limit": 10})())
    assert result.provider_id == "owid"
    assert result.provider_record_id == "example-indicator"
    assert result.source.source_record_id == "example-indicator"
    assert result.source.source_version is None
    assert result.source.published_at is None
    assert result.title == "Example indicator"
    assert result.records[0]["Entity"] == "India"
    assert result.records[0]["value"] == "12.5"


def test_provider_failure_is_distinct_from_empty_result(monkeypatch) -> None:
    adapter = OwidAdapter()

    def fail_request(path: str) -> bytes:
        raise OwidProviderError("provider unavailable")

    monkeypatch.setattr(adapter, "_request", fail_request)
    try:
        adapter.fetch(type("Q", (), {"slug": "example-indicator", "limit": 10})())
    except OwidProviderError:
        return
    raise AssertionError("provider failure must remain explicit")


def test_invalid_slug_is_rejected() -> None:
    adapter = OwidAdapter()
    try:
        adapter.fetch(type("Q", (), {"slug": "", "limit": 10})())
    except ValueError:
        return
    raise AssertionError("empty OWID slug must be rejected")
