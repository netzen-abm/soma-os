"""Focused contract tests for the OpenAlex source integration."""

from datetime import datetime, timezone

from openalex_adapter import OpenAlexAdapter, OpenAlexProviderError


def test_source_contract() -> None:
    source = OpenAlexAdapter().source_contract()
    source.validate()
    assert source.source_id == "openalex"
    assert source.provider == "OpenAlex"
    assert source.protocol == "OpenAlex REST API"
    assert source.authentication_mode == "PUBLIC_API_NO_KEY"
    assert source.quality_status == "NOT_ASSESSED"
    assert source.verification_status == "SOURCE_VERIFIED"
    assert source.source_version is None
    assert source.published_at is None


def test_record_mapping_preserves_provider_identity_and_unknown_date() -> None:
    adapter = OpenAlexAdapter()
    result = adapter._to_result(
        {
            "id": "https://openalex.org/W123456789",
            "title": "Example study",
            "publication_year": 2024,
            "doi": "https://doi.org/10.1000/example",
            "abstract_inverted_index": {"Example": [0], "study": [1]},
        }
    )
    assert result.provider_id == "openalex"
    assert result.provider_record_id == "https://openalex.org/W123456789"
    assert result.source.source_record_id == "https://openalex.org/W123456789"
    assert result.source.source_version is None
    assert result.source.published_at is None
    assert result.verification_url == "https://doi.org/10.1000/example"
    assert result.abstract_or_summary == "Example study"


def test_missing_provider_id_is_rejected() -> None:
    adapter = OpenAlexAdapter()
    try:
        adapter._to_result({"title": "Missing identifier"})
    except OpenAlexProviderError:
        return
    raise AssertionError("missing OpenAlex identifier must be rejected")


def test_unknown_publication_year_remains_unknown() -> None:
    adapter = OpenAlexAdapter()
    result = adapter._to_result(
        {
            "id": "https://openalex.org/W987654321",
            "title": "Undated study",
            "publication_year": None,
        }
    )
    assert result.publication_year is None
    assert result.source.published_at is None
