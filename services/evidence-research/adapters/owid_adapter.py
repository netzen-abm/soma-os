"""Bounded Our World in Data Grapher retrieval adapter for SOMA.

Retrieval only: returns source-linked public data and metadata. It does not
assess evidence quality, causality, clinical significance, or recommendations.
"""

from __future__ import annotations

import csv
import io
import json
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass
from datetime import datetime, timezone
from typing import Optional

from external_knowledge_source import ExternalKnowledgeSource

BASE_URL = "https://ourworldindata.org/grapher/"


@dataclass(frozen=True)
class NormalizedOwidQuery:
    query_id: str
    slug: str
    limit: int = 100


@dataclass(frozen=True)
class OwidResult:
    provider_id: str
    provider_record_id: str
    title: str
    source_url: str
    verification_url: str
    metadata: dict
    records: tuple[dict[str, str], ...]
    source: ExternalKnowledgeSource


class OwidProviderError(RuntimeError):
    """A provider/network/response failure; never interpret as no data."""


class OwidAdapter:
    provider_id = "owid"

    def __init__(self, base_url: str = BASE_URL, timeout_seconds: int = 20) -> None:
        self.base_url = base_url.rstrip("/") + "/"
        self.timeout_seconds = timeout_seconds

    def source_contract(self) -> ExternalKnowledgeSource:
        source = ExternalKnowledgeSource(
            source_id="owid",
            provider="Our World in Data",
            source_version=None,
            protocol="OWID Grapher CSV and metadata JSON",
            endpoint=self.base_url,
            authentication_mode="PUBLIC_NO_KEY",
            rate_policy_ref="OWID_CURRENT_ACCESS_POLICY",
            license_status="SOURCE_SPECIFIC_TERMS_APPLY",
            attribution_required=True,
            permitted_use="PUBLIC_DATA_RETRIEVAL_SUBJECT_TO_SOURCE_TERMS",
            retention_policy_ref="SOURCE_SPECIFIC_POLICY",
            redistribution_status="SOURCE_SPECIFIC",
            source_record_id="SOURCE_LEVEL",
            source_url="https://ourworldindata.org/",
            verification_url="https://ourworldindata.org/",
            retrieved_at=datetime.now(timezone.utc),
            published_at=None,
            entity_types=("public_dataset", "dataset_record"),
            identifier_systems=("OWID_GRAPher_SLUG",),
            units=(),
            temporal_model="SOURCE_DEFINED",
            verification_status="SOURCE_VERIFIED",
            transformation_history=(),
            quality_status="NOT_ASSESSED",
            uncertainty="UNKNOWN_WHERE_NOT_PROVIDED",
        )
        source.validate()
        return source

    def _request(self, path: str) -> bytes:
        url = self.base_url + path
        request = urllib.request.Request(
            url,
            headers={"Accept": "text/csv, application/json", "User-Agent": "SOMA-OS/1.0"},
        )
        try:
            with urllib.request.urlopen(request, timeout=self.timeout_seconds) as response:
                return response.read()
        except Exception as exc:
            raise OwidProviderError(str(exc)) from exc

    def fetch(self, query: NormalizedOwidQuery) -> OwidResult:
        slug = query.slug.strip().strip("/")
        if not slug:
            raise ValueError("slug is required")
        if query.limit <= 0:
            raise ValueError("limit must be positive")

        encoded_slug = urllib.parse.quote(slug, safe="-")
        csv_bytes = self._request(encoded_slug + ".csv")
        metadata_bytes = self._request(encoded_slug + ".metadata.json")
        try:
            reader = csv.DictReader(io.StringIO(csv_bytes.decode("utf-8")))
            fieldnames = reader.fieldnames
            if not fieldnames:
                raise ValueError("missing CSV header")
            rows = []
            for row in reader:
                rows.append({str(k): str(v) for k, v in row.items() if k is not None})
                if len(rows) >= query.limit:
                    break
            metadata = json.loads(metadata_bytes.decode("utf-8"))
            if not isinstance(metadata, dict):
                raise ValueError("metadata must be an object")
        except (UnicodeDecodeError, ValueError, json.JSONDecodeError) as exc:
            raise OwidProviderError(f"Invalid OWID response: {exc}") from exc

        source_url = "https://ourworldindata.org/grapher/" + encoded_slug
        source = self.source_contract().with_record(
            source_record_id=slug,
            source_url=source_url,
            verification_url=source_url,
            retrieved_at=datetime.now(timezone.utc),
            published_at=None,
        )
        title = str(metadata.get("title") or slug).strip()
        return OwidResult(
            provider_id=self.provider_id,
            provider_record_id=slug,
            title=title,
            source_url=source_url,
            verification_url=source_url,
            metadata=metadata,
            records=tuple(rows),
            source=source,
        )


def from_environment() -> OwidAdapter:
    return OwidAdapter()
