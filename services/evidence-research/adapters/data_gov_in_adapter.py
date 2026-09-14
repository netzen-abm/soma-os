"""Bounded Data.gov.in public-dataset retrieval adapter for SOMA.

Retrieval and source-contract binding only. This adapter does not assess
quality, infer meaning, or make clinical decisions.
"""

from __future__ import annotations

import json
import os
import urllib.parse
import urllib.request
from dataclasses import dataclass
from datetime import datetime, timezone

from external_knowledge_source import ExternalKnowledgeSource


@dataclass(frozen=True)
class NormalizedDataGovQuery:
    query_id: str
    resource_id: str
    filters: tuple[tuple[str, str], ...] = ()
    limit: int = 20


@dataclass(frozen=True)
class DataGovResult:
    provider_id: str
    provider_record_id: str
    title: str
    source_url: str
    verification_url: str
    data: dict
    source: ExternalKnowledgeSource


class DataGovProviderError(RuntimeError):
    """A Data.gov.in/provider failure; never interpret this as empty data."""


class DataGovInAdapter:
    provider_id = "data_gov_in"

    def __init__(self, api_base_url: str = "https://api.data.gov.in/resource/", timeout_seconds: int = 20) -> None:
        self.api_base_url = api_base_url.rstrip("/") + "/"
        self.timeout_seconds = timeout_seconds

    def source_contract(self) -> ExternalKnowledgeSource:
        source = ExternalKnowledgeSource(
            source_id="data_gov_in",
            provider="Data.gov.in",
            source_version=None,
            protocol="Data.gov.in Resource API",
            endpoint=self.api_base_url,
            authentication_mode="API_KEY_WHERE_REQUIRED",
            rate_policy_ref="DATA_GOV_IN_CURRENT_API_POLICY",
            license_status="SOURCE_SPECIFIC_TERMS_APPLY",
            attribution_required=True,
            permitted_use="PUBLIC_DATA_RETRIEVAL_SUBJECT_TO_SOURCE_TERMS",
            retention_policy_ref="SOURCE_SPECIFIC_POLICY",
            redistribution_status="SOURCE_SPECIFIC",
            source_record_id="SOURCE_LEVEL",
            source_url="https://data.gov.in/",
            verification_url="https://data.gov.in/",
            retrieved_at=datetime.now(timezone.utc),
            published_at=None,
            entity_types=("public_dataset", "dataset_record"),
            identifier_systems=("DATA_GOV_IN_RESOURCE_ID",),
            units=(),
            temporal_model="SOURCE_DEFINED",
            verification_status="SOURCE_VERIFIED",
            transformation_history=(),
            quality_status="NOT_ASSESSED",
            uncertainty="UNKNOWN_WHERE_NOT_PROVIDED",
        )
        source.validate()
        return source

    def _request(self, resource_id: str, params: dict[str, str]) -> dict:
        api_key = os.environ.get("DATA_GOV_IN_API_KEY")
        if not api_key:
            raise DataGovProviderError("DATA_GOV_IN_API_KEY is required for resource API access")
        url = self.api_base_url + urllib.parse.quote(resource_id, safe="") + "?" + urllib.parse.urlencode(params)
        request = urllib.request.Request(
            url,
            headers={"Accept": "application/json", "User-Agent": os.environ.get("SOMA_DATA_GOV_USER_AGENT", "SOMA-OS/1.0")},
        )
        try:
            with urllib.request.urlopen(request, timeout=self.timeout_seconds) as response:
                payload = json.loads(response.read().decode("utf-8"))
        except Exception as exc:
            raise DataGovProviderError(str(exc)) from exc
        if not isinstance(payload, dict):
            raise DataGovProviderError("Invalid Data.gov.in response")
        return payload

    def fetch(self, query: NormalizedDataGovQuery) -> DataGovResult:
        if not query.resource_id.strip():
            raise ValueError("resource_id is required")
        params = {"format": "json", "limit": str(query.limit)}
        params.update(dict(query.filters))
        payload = self._request(query.resource_id.strip(), params)
        records = payload.get("records", [])
        if not isinstance(records, list):
            raise DataGovProviderError("Invalid Data.gov.in records response")
        title = str(payload.get("title") or query.resource_id).strip()
        source_url = "https://data.gov.in/resource/" + urllib.parse.quote(query.resource_id.strip(), safe="")
        source = self.source_contract().with_record(
            source_record_id=query.resource_id.strip(),
            source_url=source_url,
            verification_url=source_url,
            retrieved_at=datetime.now(timezone.utc),
            published_at=None,
        )
        return DataGovResult(
            provider_id=self.provider_id,
            provider_record_id=query.resource_id.strip(),
            title=title,
            source_url=source_url,
            verification_url=source_url,
            data={"records": records, "count": len(records)},
            source=source,
        )


def from_environment() -> DataGovInAdapter:
    return DataGovInAdapter()
