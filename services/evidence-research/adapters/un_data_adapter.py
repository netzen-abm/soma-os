"""Bounded United Nations Data SDMX retrieval adapter for SOMA.

Retrieval only: returns source-linked public statistical data. It does not
assess evidence quality, causality, clinical significance, or recommendations.
"""

from __future__ import annotations

import csv
import io
import urllib.parse
import urllib.request
from dataclasses import dataclass
from datetime import datetime, timezone

from external_knowledge_source import ExternalKnowledgeSource

BASE_URL = "https://data.un.org/WS/rest/"


@dataclass(frozen=True)
class NormalizedUNDataQuery:
    query_id: str
    agency_id: str
    dataflow_id: str
    series_key: str = ""
    start_period: str | None = None
    end_period: str | None = None
    limit: int = 100


@dataclass(frozen=True)
class UNDataResult:
    provider_id: str
    provider_record_id: str
    source_url: str
    verification_url: str
    records: tuple[dict[str, str], ...]
    source: ExternalKnowledgeSource


class UNDataProviderError(RuntimeError):
    """A provider/network/response failure; never interpret as no data."""


class UNDataAdapter:
    provider_id = "un_data"

    def __init__(self, base_url: str = BASE_URL, timeout_seconds: int = 20) -> None:
        self.base_url = base_url.rstrip("/") + "/"
        self.timeout_seconds = timeout_seconds

    def source_contract(self) -> ExternalKnowledgeSource:
        source = ExternalKnowledgeSource(
            source_id="un_data",
            provider="United Nations Data",
            source_version=None,
            protocol="UNdata SDMX REST API",
            endpoint=self.base_url,
            authentication_mode="PUBLIC_NO_KEY",
            rate_policy_ref="UNDATA_TERMS_OF_USE",
            license_status="SOURCE_SPECIFIC_TERMS_APPLY",
            attribution_required=True,
            permitted_use="PUBLIC_DATA_RETRIEVAL_SUBJECT_TO_SOURCE_TERMS",
            retention_policy_ref="UNDATA_TERMS_OF_USE",
            redistribution_status="SOURCE_SPECIFIC",
            source_record_id="SOURCE_LEVEL",
            source_url="https://data.un.org/",
            verification_url="https://data.un.org/",
            retrieved_at=datetime.now(timezone.utc),
            published_at=None,
            entity_types=("public_dataset", "dataset_record"),
            identifier_systems=("SDMX_DATAFLOW", "SDMX_SERIES_KEY"),
            units=(),
            temporal_model="SOURCE_DEFINED",
            verification_status="SOURCE_VERIFIED",
            transformation_history=(),
            quality_status="NOT_ASSESSED",
            uncertainty="UNKNOWN_WHERE_NOT_PROVIDED",
        )
        source.validate()
        return source

    def _request(self, path: str, query: dict[str, str]) -> bytes:
        url = self.base_url + path
        if query:
            url += "?" + urllib.parse.urlencode(query)
        request = urllib.request.Request(
            url,
            headers={
                "Accept": "text/csv",
                "User-Agent": "SOMA-OS/1.0",
            },
        )
        try:
            with urllib.request.urlopen(request, timeout=self.timeout_seconds) as response:
                return response.read()
        except Exception as exc:
            raise UNDataProviderError(str(exc)) from exc

    def fetch(self, query: NormalizedUNDataQuery) -> UNDataResult:
        agency_id = query.agency_id.strip()
        dataflow_id = query.dataflow_id.strip()
        if not agency_id or not dataflow_id:
            raise ValueError("agency_id and dataflow_id are required")
        if query.limit <= 0:
            raise ValueError("limit must be positive")

        encoded_agency = urllib.parse.quote(agency_id, safe="._-~")
        encoded_flow = urllib.parse.quote(dataflow_id, safe="._-~")
        encoded_key = urllib.parse.quote(query.series_key.strip(), safe=".+_-")
        path = f"data/{encoded_agency},{encoded_flow}/"
        if encoded_key:
            path += encoded_key

        params: dict[str, str] = {"format": "csv"}
        if query.start_period:
            params["startPeriod"] = query.start_period
        if query.end_period:
            params["endPeriod"] = query.end_period

        csv_bytes = self._request(path, params)
        try:
            reader = csv.DictReader(io.StringIO(csv_bytes.decode("utf-8-sig")))
            if not reader.fieldnames:
                raise ValueError("missing CSV header")
            rows: list[dict[str, str]] = []
            for row in reader:
                rows.append({str(k): str(v) for k, v in row.items() if k is not None})
                if len(rows) >= query.limit:
                    break
        except (UnicodeDecodeError, ValueError) as exc:
            raise UNDataProviderError(f"Invalid UNdata response: {exc}") from exc

        source_record_id = f"{agency_id}:{dataflow_id}:{query.series_key.strip()}"
        source_url = "https://data.un.org/"
        source = self.source_contract().with_record(
            source_record_id=source_record_id,
            source_url=source_url,
            verification_url=source_url,
            retrieved_at=datetime.now(timezone.utc),
            published_at=None,
        )
        return UNDataResult(
            provider_id=self.provider_id,
            provider_record_id=source_record_id,
            source_url=source_url,
            verification_url=source_url,
            records=tuple(rows),
            source=source,
        )


def from_environment() -> UNDataAdapter:
    return UNDataAdapter()
