"""Provider-neutral OpenAlex scholarly retrieval adapter for SOMA.

Retrieval only: this adapter returns source-linked scholarly metadata and does
not assess evidence quality, safety, efficacy, causality, or clinical
significance. Those decisions belong to the core Evidence Research Engine.
"""

from __future__ import annotations

import json
import os
import urllib.parse
import urllib.request
from datetime import datetime, timezone
from typing import Optional

from external_knowledge_source import ExternalKnowledgeSource
from research_source_contract import NormalizedResearchQuery, ResearchRecord

BASE_URL = "https://api.openalex.org/"


class OpenAlexProviderError(RuntimeError):
    """A provider/network failure; never interpret this as no evidence."""


SearchResult = ResearchRecord


class OpenAlexAdapter:
    provider_id = "openalex"

    def __init__(self, timeout_seconds: int = 20) -> None:
        self.timeout_seconds = timeout_seconds

    def source_contract(self) -> ExternalKnowledgeSource:
        """Return OpenAlex's governed source-level contract metadata."""
        source = ExternalKnowledgeSource(
            source_id="openalex",
            provider="OpenAlex",
            source_version=None,
            protocol="OpenAlex REST API",
            endpoint=BASE_URL,
            authentication_mode="PUBLIC_API_NO_KEY",
            rate_policy_ref="OPENALEX_CURRENT_API_POLICY",
            license_status="PROVIDER_TERMS_APPLY",
            attribution_required=True,
            permitted_use="RESEARCH_RETRIEVAL_SUBJECT_TO_PROVIDER_TERMS",
            retention_policy_ref="SOURCE_SPECIFIC_POLICY",
            redistribution_status="RESTRICTED_BY_SOURCE_TERMS",
            source_record_id="SOURCE_LEVEL",
            source_url="https://openalex.org/",
            verification_url="https://openalex.org/",
            retrieved_at=datetime.now(timezone.utc),
            published_at=None,
            entity_types=("scholarly_work",),
            identifier_systems=("OPENALEX_ID", "DOI"),
            units=(),
            temporal_model="PUBLICATION_YEAR_OR_PROVIDER_DATE",
            verification_status="SOURCE_VERIFIED",
            transformation_history=(),
            quality_status="NOT_ASSESSED",
            uncertainty="UNKNOWN_WHERE_NOT_PROVIDED",
        )
        source.validate()
        return source

    def _request(self, endpoint: str, params: dict[str, str]) -> dict:
        url = BASE_URL + endpoint + "?" + urllib.parse.urlencode(params)
        request = urllib.request.Request(
            url,
            headers={
                "Accept": "application/json",
                "User-Agent": os.environ.get(
                    "SOMA_OPENALEX_USER_AGENT", "SOMA-OS/1.0"
                ),
            },
        )
        try:
            with urllib.request.urlopen(request, timeout=self.timeout_seconds) as response:
                return json.loads(response.read().decode("utf-8"))
        except Exception as exc:
            raise OpenAlexProviderError(str(exc)) from exc

    def search(self, query: NormalizedResearchQuery) -> list[SearchResult]:
        payload = self._request(
            "works",
            {"search": query.terms, "per-page": str(query.retmax)},
        )
        results = payload.get("results")
        if not isinstance(results, list):
            raise OpenAlexProviderError("Invalid OpenAlex works response")
        return [self._to_result(record, query) for record in results if isinstance(record, dict)]

    def fetch(self, record_id: str) -> SearchResult:
        if not record_id:
            raise ValueError("record_id is required")
        parsed = urllib.parse.urlparse(record_id)
        normalized_id = parsed.path.rsplit("/", 1)[-1] if parsed.scheme else record_id
        normalized_id = normalized_id.strip()
        if not normalized_id:
            raise ValueError("record_id is required")
        record = self._request(
            "works/" + urllib.parse.quote(normalized_id, safe=""), {}
        )
        return self._to_result(record)

    def _to_result(self, record: dict, query: Optional[NormalizedResearchQuery] = None) -> SearchResult:
        provider_record_id = str(record.get("id") or "").strip()
        if not provider_record_id:
            raise OpenAlexProviderError("OpenAlex record has no stable identifier")

        title = " ".join(str(record.get("title") or "").split())
        publication_year = record.get("publication_year")
        if not isinstance(publication_year, int):
            publication_year = None

        doi = str(record.get("doi") or "").strip()
        verification_url = (
            doi if doi.startswith("https://doi.org/") else provider_record_id
        )

        abstract = self._abstract_from_inverted_index(
            record.get("abstract_inverted_index")
        )
        source_contract = self.source_contract().with_record(
            source_record_id=provider_record_id,
            source_url=provider_record_id,
            verification_url=verification_url,
            retrieved_at=datetime.now(timezone.utc),
            published_at=None,
        )
        return SearchResult(
            provider_id=self.provider_id,
            provider_record_id=provider_record_id,
            title=title,
            source_class=query.source_class if query else "biomedical_literature",
            geography=query.geography if query else "global",
            publication_year=publication_year,
            source_url=provider_record_id,
            verification_url=verification_url,
            abstract_or_summary=abstract,
            identifiers=(
                (f"doi:{doi.removeprefix("https://doi.org/")}",) if doi else ()
            ),
            source=source_contract,
        )

    @staticmethod
    def _abstract_from_inverted_index(value: object) -> Optional[str]:
        if not isinstance(value, dict):
            return None
        tokens: list[tuple[int, str]] = []
        for word, positions in value.items():
            if not isinstance(word, str) or not isinstance(positions, list):
                continue
            for position in positions:
                if isinstance(position, int):
                    tokens.append((position, word))
        if not tokens:
            return None
        tokens.sort(key=lambda item: item[0])
        return " ".join(word for _, word in tokens)


def from_environment() -> OpenAlexAdapter:
    return OpenAlexAdapter()
