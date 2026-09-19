"""Bounded Unpaywall access-location resolver for SOMA.

Unpaywall is used only to resolve publicly reported access locations for a
scholarly work identified by DOI. It is not an evidence-quality authority,
clinical authority, or authorization authority.
"""

from __future__ import annotations

import json
import os
import urllib.parse
import urllib.request
from datetime import datetime, timezone
from typing import Any

from external_knowledge_source import ExternalKnowledgeSource
from research_source_contract import ResearchAccessLocation


BASE_URL = "https://api.unpaywall.org/v2/"


class UnpaywallProviderError(RuntimeError):
    """A provider/network/response failure; never interpret as no evidence."""


class UnpaywallAdapter:
    provider_id = "unpaywall"

    def __init__(self, email: str | None = None, timeout_seconds: int = 20) -> None:
        self.email = (email or os.environ.get("SOMA_UNPAYWALL_EMAIL", "")).strip()
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
            entity_types=("scholarly_work", "access_location"),
            identifier_systems=("DOI",),
            units=(),
            temporal_model="PROVIDER_REPORTED",
            verification_status="SOURCE_VERIFIED",
            transformation_history=(),
            quality_status="NOT_ASSESSED",
            uncertainty="UNKNOWN_WHERE_NOT_PROVIDED",
        )
        source.validate()
        return source

    def resolve(self, doi: str) -> tuple[ResearchAccessLocation, ...]:
        """Resolve provider-reported OA locations for one DOI.

        A missing DOI or missing email is an invalid request, not an empty
        evidence result. A successful response with no locations returns an
        empty tuple.
        """
        normalized_doi = self._normalize_doi(doi)
        if not normalized_doi:
            raise ValueError("doi is required")
        if not self.email:
            raise ValueError("email is required")

        payload = self._request(normalized_doi)
        if not isinstance(payload, dict):
            raise UnpaywallProviderError("Invalid Unpaywall response")

        source_contract = self.source_contract()
        retrieved_at = datetime.now(timezone.utc).isoformat()
        locations: list[ResearchAccessLocation] = []

        for index, raw in enumerate(payload.get("oa_locations") or ()):
            if not isinstance(raw, dict):
                continue
            url = str(raw.get("url_for_pdf") or raw.get("url") or "").strip()
            if not url:
                continue
            host_type = str(raw.get("host_type") or "unknown").strip()
            version = str(raw.get("version") or "").strip() or None
            license_value = str(raw.get("license") or "").strip() or None
            locations.append(
                ResearchAccessLocation(
                    location_id=f"unpaywall:{normalized_doi}:{index}",
                    source_id=source_contract.source_id,
                    url=url,
                    location_type=host_type,
                    is_open_access=True,
                    license_if_reported=license_value,
                    version_label=version,
                    retrieved_at=retrieved_at,
                )
            )
        return tuple(locations)

    @staticmethod
    def _normalize_doi(value: str) -> str:
        doi = value.strip()
        for prefix in ("https://doi.org/", "http://doi.org/"):
            if doi.lower().startswith(prefix):
                doi = doi[len(prefix) :]
                break
        return doi.strip().strip("/")

    def _request(self, doi: str) -> Any:
        url = BASE_URL + urllib.parse.quote(doi, safe="") + "?" + urllib.parse.urlencode(
            {"email": self.email}
        )
        request = urllib.request.Request(
            url,
            headers={
                "Accept": "application/json",
                "User-Agent": os.environ.get(
                    "SOMA_UNPAYWALL_USER_AGENT", "SOMA-OS/1.0"
                ),
            },
        )
        try:
            with urllib.request.urlopen(request, timeout=self.timeout_seconds) as response:
                return json.loads(response.read().decode("utf-8"))
        except Exception as exc:
            raise UnpaywallProviderError(str(exc)) from exc
