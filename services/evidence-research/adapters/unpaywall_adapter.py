"""Unpaywall open-access location resolver for SOMA.

Resolution only: this adapter does not assess evidence quality, licensing
sufficiency, clinical relevance, or source authority.
"""

from __future__ import annotations

import json
import os
import urllib.parse
import urllib.request
from dataclasses import dataclass
from datetime import datetime, timezone
from typing import Optional

from external_knowledge_source import ExternalKnowledgeSource
from research_source_contract import ResearchAccessLocation

BASE_URL = "https://api.unpaywall.org/v2/"


class UnpaywallProviderError(RuntimeError):
    """Provider/network failure; never interpret this as no open access."""


class UnpaywallAdapter:
    provider_id = "unpaywall"

    def __init__(self, email: str, timeout_seconds: int = 20) -> None:
        if not email.strip():
            raise ValueError("email is required")
        self.email = email
        self.timeout_seconds = timeout_seconds

    def source_contract(self) -> ExternalKnowledgeSource:
        source = ExternalKnowledgeSource(
            source_id="unpaywall",
            provider="Unpaywall",
            source_version=None,
            protocol="Unpaywall REST API",
            endpoint=BASE_URL,
            authentication_mode="PUBLIC_API_EMAIL_REQUIRED",
            rate_policy_ref="UNPAYWALL_CURRENT_API_POLICY",
            license_status="PROVIDER_TERMS_APPLY",
            attribution_required=True,
            permitted_use="RESEARCH_ACCESS_LOCATION_RESOLUTION_SUBJECT_TO_PROVIDER_TERMS",
            retention_policy_ref="SOURCE_SPECIFIC_POLICY",
            redistribution_status="RESTRICTED_BY_SOURCE_TERMS",
            source_record_id="SOURCE_LEVEL",
            source_url="https://unpaywall.org/",
            verification_url="https://unpaywall.org/",
            retrieved_at=datetime.now(timezone.utc),
            published_at=None,
            entity_types=("open_access_location",),
            identifier_systems=("DOI",),
            units=(),
            temporal_model="PROVIDER_RETRIEVAL_TIME",
            verification_status="SOURCE_VERIFIED",
            transformation_history=(),
            quality_status="NOT_ASSESSED",
            uncertainty="UNKNOWN_WHERE_NOT_PROVIDED",
        )
        source.validate()
        return source

    def resolve(self, doi: str) -> list[ResearchAccessLocation]:
        normalized_doi = doi.strip()
        if normalized_doi.lower().startswith("https://doi.org/"):
            normalized_doi = normalized_doi[len("https://doi.org/"):]
        if not normalized_doi:
            raise ValueError("doi is required")

        encoded = urllib.parse.quote(normalized_doi, safe="")
        url = BASE_URL + encoded + "?" + urllib.parse.urlencode({"email": self.email})
        request = urllib.request.Request(
            url,
            headers={
                "Accept": "application/json",
                "User-Agent": os.environ.get("SOMA_UNPAYWALL_USER_AGENT", "SOMA-OS/1.0"),
            },
        )
        try:
            with urllib.request.urlopen(request, timeout=self.timeout_seconds) as response:
                payload = json.loads(response.read().decode("utf-8"))
        except Exception as exc:
            raise UnpaywallProviderError(str(exc)) from exc

        source = self.source_contract()
        locations: list[ResearchAccessLocation] = []
        for raw in payload.get("oa_locations") or []:
            if not isinstance(raw, dict):
                continue
            url_for_pdf = raw.get("url_for_pdf")
            url = raw.get("url")
            resolved_url = str(url_for_pdf or url or "").strip()
            if not resolved_url:
                continue
            location_type = str(
                raw.get("host_type") or raw.get("oa_version") or "unknown"
            ).strip()
            version = str(raw.get("version") or "").strip() or None
            license_value = str(raw.get("license") or "").strip() or None
            record_id = str(payload.get("doi") or normalized_doi).strip()
            locations.append(
                ResearchAccessLocation(
                    provider_id=self.provider_id,
                    work_identifier=record_id,
                    url=resolved_url,
                    location_type=location_type,
                    is_open_access=True,
                    license_if_reported=license_value,
                    version=version,
                    source=source.with_record(
                        source_record_id=record_id,
                        source_url=resolved_url,
                        verification_url=resolved_url,
                        retrieved_at=datetime.now(timezone.utc),
                        published_at=None,
                    ),
                )
            )
        return locations


def from_environment() -> UnpaywallAdapter:
    return UnpaywallAdapter(
        email=os.environ.get("UNPAYWALL_EMAIL", ""),
    )
