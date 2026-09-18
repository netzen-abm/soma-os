"""Contract tests for the canonical research-source query/result boundary."""

from datetime import datetime, timezone
from pathlib import Path
import sys

ADAPTER_DIR = Path(__file__).resolve().parent.parent / "services" / "evidence-research" / "adapters"
sys.path.insert(0, str(ADAPTER_DIR))

from external_knowledge_source import ExternalKnowledgeSource
from research_source_contract import ResearchQuery, ResearchRecord


def test_query_is_provider_neutral() -> None:
    query = ResearchQuery("q1", "millet diabetes", "biomedical_literature")
    assert query.query_id == "q1"
    assert query.source_class == "biomedical_literature"
    assert query.geography == "global"


def test_record_preserves_source_provenance() -> None:
    source = ExternalKnowledgeSource(
        source_id="openalex",
        provider="OpenAlex",
        source_version=None,
        protocol="OpenAlex REST API",
        endpoint="https://api.openalex.org/",
        authentication_mode="PUBLIC_API_NO_KEY",
        rate_policy_ref="OPENALEX_CURRENT_API_POLICY",
        license_status="PROVIDER_TERMS_APPLY",
        attribution_required=True,
        permitted_use="RESEARCH_RETRIEVAL_SUBJECT_TO_PROVIDER_TERMS",
        retention_policy_ref="SOURCE_SPECIFIC_POLICY",
        redistribution_status="RESTRICTED_BY_SOURCE_TERMS",
        source_record_id="https://openalex.org/W1",
        source_url="https://openalex.org/W1",
        verification_url="https://openalex.org/W1",
        retrieved_at=datetime.now(timezone.utc),
        published_at=None,
        entity_types=("scholarly_work",),
        identifier_systems=("OPENALEX_ID",),
        units=(),
        temporal_model="PUBLICATION_YEAR_OR_PROVIDER_DATE",
        verification_status="SOURCE_VERIFIED",
        transformation_history=(),
        quality_status="NOT_ASSESSED",
        uncertainty="UNKNOWN_WHERE_NOT_PROVIDED",
    )
    source.validate()
    record = ResearchRecord(
        provider_id="openalex",
        provider_record_id="https://openalex.org/W1",
        title="Example",
        source_class="biomedical_literature",
        geography="global",
        source_url="https://openalex.org/W1",
        verification_url="https://openalex.org/W1",
        identifiers=("openalex:https://openalex.org/W1",),
        source=source,
    )
    assert record.source is source
    assert record.provider_id == "openalex"


def test_invalid_record_cannot_hide_missing_verification() -> None:
    try:
        ResearchRecord(
            provider_id="openalex",
            provider_record_id="W1",
            title="Example",
            source_class="biomedical_literature",
            geography="global",
            source_url="https://openalex.org/W1",
            verification_url="",
        )
    except ValueError as exc:
        assert "verification_url" in str(exc)
    else:
        raise AssertionError("missing verification URL must be rejected")
