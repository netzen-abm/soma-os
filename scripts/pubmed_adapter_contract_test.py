from __future__ import annotations

import sys
from pathlib import Path

import pytest

ADAPTER_DIR = Path(__file__).resolve().parents[1] / "services" / "evidence-research" / "adapters"
sys.path.insert(0, str(ADAPTER_DIR))

from pubmed_adapter import PubMedAdapter  # noqa: E402
from external_knowledge_source import ExternalKnowledgeSource  # noqa: E402


def test_pubmed_source_contract_is_valid_and_provider_scoped() -> None:
    adapter = PubMedAdapter(
        tool_name="soma-test",
        contact_email="test@example.invalid",
    )

    source = adapter.source_contract()

    source.validate()
    assert source.source_id == "pubmed"
    assert source.provider == "NCBI PubMed"
    assert source.identifier_systems == ("PMID", "DOI")
    assert source.quality_status == "NOT_ASSESSED"
    assert source.verification_status == "SOURCE_VERIFIED"
    assert source.source_version is None
    assert source.published_at is None


def test_external_source_contract_rejects_empty_required_identity() -> None:
    adapter = PubMedAdapter(
        tool_name="soma-test",
        contact_email="test@example.invalid",
    )
    source = adapter.source_contract()

    invalid = ExternalKnowledgeSource(
        **{
            field: "" if field == "provider" else value
            for field, value in source.__dict__.items()
        }
    )

    with pytest.raises(ValueError, match="empty required fields"):
        invalid.validate()


def test_pubmed_result_preserves_record_provenance_without_fabricating_date() -> None:
    adapter = PubMedAdapter(
        tool_name="soma-test",
        contact_email="test@example.invalid",
    )
    xml = """
    <PubmedArticleSet>
      <PubmedArticle>
        <MedlineCitation>
          <PMID>12345678</PMID>
          <Article>
            <ArticleTitle>Example study</ArticleTitle>
            <Abstract><AbstractText>Example abstract.</AbstractText></Abstract>
          </Article>
        </MedlineCitation>
      </PubmedArticle>
    </PubmedArticleSet>
    """
    adapter._request = lambda endpoint, params: xml  # type: ignore[method-assign]

    results = adapter._fetch(["12345678"])

    assert len(results) == 1
    result = results[0]
    assert result.provider_record_id == "12345678"
    assert result.source.source_id == "pubmed"
    assert result.source.source_record_id == "12345678"
    assert result.source.verification_url == "https://pubmed.ncbi.nlm.nih.gov/12345678/"
    assert result.source.published_at is None
    assert result.source.source_version is None
