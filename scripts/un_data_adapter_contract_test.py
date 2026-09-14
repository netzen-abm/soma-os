"""Focused contract tests for the UN Data source integration."""

from pathlib import Path
import sys

ADAPTER_DIR = (
    Path(__file__).resolve().parents[1]
    / "services"
    / "evidence-research"
    / "adapters"
)
sys.path.insert(0, str(ADAPTER_DIR))

from un_data_adapter import (  # noqa: E402
    NormalizedUNDataQuery,
    UNDataAdapter,
    UNDataProviderError,
)


def test_source_contract() -> None:
    source = UNDataAdapter().source_contract()
    source.validate()
    assert source.source_id == "un_data"
    assert source.provider == "United Nations Data"
    assert source.protocol == "UNdata SDMX REST API"
    assert source.authentication_mode == "PUBLIC_NO_KEY"
    assert source.quality_status == "NOT_ASSESSED"
    assert source.verification_status == "SOURCE_VERIFIED"
    assert source.source_version is None
    assert source.published_at is None


def test_response_mapping_preserves_identity_and_unknown_date(monkeypatch) -> None:
    adapter = UNDataAdapter()

    def fake_request(path: str, query: dict[str, str]) -> bytes:
        assert path == "data/UNSD,SNA/"
        assert query == {"format": "csv", "startPeriod": "2020", "endPeriod": "2024"}
        return b"COUNTRY,Year,Value\nIndia,2024,12.5\n"

    monkeypatch.setattr(adapter, "_request", fake_request)
    result = adapter.fetch(
        NormalizedUNDataQuery(
            "q1", "UNSD", "SNA", start_period="2020", end_period="2024", limit=10
        )
    )
    assert result.provider_id == "un_data"
    assert result.provider_record_id == "UNSD:SNA:"
    assert result.source.source_record_id == "UNSD:SNA:"
    assert result.source.source_version is None
    assert result.source.published_at is None
    assert result.records[0]["COUNTRY"] == "India"
    assert result.records[0]["Value"] == "12.5"


def test_series_key_is_encoded_and_provider_failure_is_explicit(monkeypatch) -> None:
    adapter = UNDataAdapter()

    def fail_request(path: str, query: dict[str, str]) -> bytes:
        assert path == "data/UNSD,SNA/INDIA.+"
        raise UNDataProviderError("provider unavailable")

    monkeypatch.setattr(adapter, "_request", fail_request)
    try:
        adapter.fetch(NormalizedUNDataQuery("q1", "UNSD", "SNA", series_key="INDIA.+"))
    except UNDataProviderError:
        return
    raise AssertionError("provider failure must remain explicit")


def test_invalid_query_is_rejected() -> None:
    adapter = UNDataAdapter()
    for query in (
        NormalizedUNDataQuery("q1", "", "SNA"),
        NormalizedUNDataQuery("q1", "UNSD", "", limit=10),
        NormalizedUNDataQuery("q1", "UNSD", "SNA", limit=0),
    ):
        try:
            adapter.fetch(query)
        except ValueError:
            continue
        raise AssertionError("invalid UN Data query must be rejected")
