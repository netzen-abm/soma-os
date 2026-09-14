"""Focused contract tests for the Data.gov.in source integration."""

from pathlib import Path
import sys

ADAPTER_DIR = Path(__file__).resolve().parents[1] / "services" / "evidence-research" / "adapters"
sys.path.insert(0, str(ADAPTER_DIR))

from data_gov_in_adapter import DataGovInAdapter, DataGovProviderError, NormalizedDataGovQuery


def test_source_contract() -> None:
    source = DataGovInAdapter().source_contract()
    source.validate()
    assert source.source_id == "data_gov_in"
    assert source.provider == "Data.gov.in"
    assert source.protocol == "Data.gov.in Resource API"
    assert source.authentication_mode == "API_KEY_WHERE_REQUIRED"
    assert source.quality_status == "NOT_ASSESSED"
    assert source.source_version is None
    assert source.published_at is None


def test_missing_api_key_is_explicit_provider_failure(monkeypatch) -> None:
    monkeypatch.delenv("DATA_GOV_IN_API_KEY", raising=False)
    adapter = DataGovInAdapter()
    try:
        adapter._request("example-resource", {})
    except DataGovProviderError:
        return
    raise AssertionError("missing API key must remain an explicit provider failure")


def test_configured_api_key_is_transmitted_without_logging(monkeypatch) -> None:
    monkeypatch.setenv("DATA_GOV_IN_API_KEY", "test-secret")
    captured = {}

    class Response:
        def __enter__(self):
            return self

        def __exit__(self, exc_type, exc, tb):
            return False

        def read(self):
            return b'{"records": []}'

    def fake_urlopen(request, timeout):
        captured["url"] = request.full_url
        captured["timeout"] = timeout
        return Response()

    monkeypatch.setattr("data_gov_in_adapter.urllib.request.urlopen", fake_urlopen)
    result = DataGovInAdapter()._request("resource-1", {"format": "json"})
    assert result == {"records": []}
    assert "api-key=test-secret" in captured["url"]
    assert captured["timeout"] == 20


def test_empty_successful_records_remain_empty_not_failure(monkeypatch) -> None:
    adapter = DataGovInAdapter()
    monkeypatch.setattr(adapter, "_request", lambda resource_id, params: {"records": []})
    result = adapter.fetch(NormalizedDataGovQuery("q1", "resource-1"))
    assert result.data == {"records": [], "count": 0}
    assert result.provider_record_id == "resource-1"
    assert result.source.source_record_id == "resource-1"
    assert result.source.source_version is None
    assert result.source.published_at is None


def test_malformed_records_are_rejected(monkeypatch) -> None:
    adapter = DataGovInAdapter()
    monkeypatch.setattr(adapter, "_request", lambda resource_id, params: {"records": {}})
    try:
        adapter.fetch(NormalizedDataGovQuery("q1", "resource-1"))
    except DataGovProviderError:
        return
    raise AssertionError("malformed records must be rejected")
