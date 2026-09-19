"""Provider-neutral PubMed retrieval adapter for SOMA.

Retrieval only: this module does not assess efficacy, safety, directness,
or clinical significance. It returns source-linked records for the core
Evidence Research Engine to screen and assess.
"""

from __future__ import annotations

import json
import os
import time
import urllib.parse
import urllib.request
import xml.etree.ElementTree as ET
from datetime import datetime, timezone
from typing import Optional

from external_knowledge_source import ExternalKnowledgeSource
from research_source_contract import NormalizedResearchQuery, ResearchRecord

BASE_URL = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/"


class PubMedProviderError(RuntimeError):
    """A provider/network failure; never interpret this as no evidence."""


SearchResult = ResearchRecord


class PubMedAdapter:
    provider_id = "pubmed"

    def __init__(self, tool_name: str, contact_email: str, api_key: Optional[str] = None,
                 min_interval_seconds: float = 0.34, timeout_seconds: int = 20) -> None:
        if not tool_name or not contact_email:
            raise ValueError("tool_name and contact_email are required")
        self.tool_name = tool_name
        self.contact_email = contact_email
        self.api_key = api_key
        self.min_interval_seconds = min_interval_seconds
        self.timeout_seconds = timeout_seconds
        self._last_request = 0.0

    def source_contract(self) -> ExternalKnowledgeSource:
        """Return PubMed's governed source-level contract metadata."""
        source = ExternalKnowledgeSource(
            source_id="pubmed",
            provider="NCBI PubMed",
            source_version=None,
            protocol="Entrez E-utilities",
            endpoint=BASE_URL,
            authentication_mode="PUBLIC_API_KEY_OPTIONAL",
            rate_policy_ref="NCBI_EUTILITIES_CURRENT_POLICY",
            license_status="PROVIDER_TERMS_APPLY",
            attribution_required=True,
            permitted_use="RESEARCH_RETRIEVAL_SUBJECT_TO_PROVIDER_TERMS",
            retention_policy_ref="SOURCE_SPECIFIC_POLICY",
            redistribution_status="RESTRICTED_BY_SOURCE_TERMS",
            source_record_id="SOURCE_LEVEL",
            source_url="https://pubmed.ncbi.nlm.nih.gov/",
            verification_url="https://pubmed.ncbi.nlm.nih.gov/",
            retrieved_at=datetime.now(timezone.utc),
            published_at=None,
            entity_types=("biomedical_literature_record",),
            identifier_systems=("PMID", "DOI"),
            units=(),
            temporal_model="PUBLICATION_YEAR_OR_PROVIDER_DATE",
            verification_status="SOURCE_VERIFIED",
            transformation_history=(),
            quality_status="NOT_ASSESSED",
            uncertainty="UNKNOWN_WHERE_NOT_PROVIDED",
        )
        source.validate()
        return source

    def _request(self, endpoint: str, params: dict[str, str]) -> str:
        now = time.monotonic()
        wait = self.min_interval_seconds - (now - self._last_request)
        if wait > 0:
            time.sleep(wait)

        params = dict(params)
        params["tool"] = self.tool_name
        params["email"] = self.contact_email
        if self.api_key:
            params["api_key"] = self.api_key
        url = BASE_URL + endpoint + "?" + urllib.parse.urlencode(params)
        try:
            with urllib.request.urlopen(url, timeout=self.timeout_seconds) as response:
                body = response.read().decode("utf-8")
                self._last_request = time.monotonic()
                return body
        except Exception as exc:
            raise PubMedProviderError(str(exc)) from exc

    def search(self, query: NormalizedResearchQuery) -> list[SearchResult]:
        raw = self._request(
            "esearch.fcgi",
            {"db": "pubmed", "term": query.terms, "retmode": "json", "retmax": str(query.retmax)},
        )
        try:
            ids = json.loads(raw)["esearchresult"]["idlist"]
        except (KeyError, json.JSONDecodeError) as exc:
            raise PubMedProviderError("Invalid PubMed ESearch response") from exc
        if not ids:
            return []
        return self._fetch(ids, query)

    def _fetch(self, ids: list[str], query: Optional[NormalizedResearchQuery] = None) -> list[SearchResult]:
        raw = self._request(
            "efetch.fcgi",
            {"db": "pubmed", "id": ",".join(ids), "retmode": "xml"},
        )
        try:
            root = ET.fromstring(raw)
        except ET.ParseError as exc:
            raise PubMedProviderError("Invalid PubMed EFetch response") from exc

        results: list[SearchResult] = []
        retrieved_at = datetime.now(timezone.utc)
        source_contract = self.source_contract()
        for article in root.findall(".//PubmedArticle"):
            pmid = article.findtext(".//PMID")
            title = " ".join(article.findtext(".//ArticleTitle", default="").split())
            abstract_parts = [" ".join(x.text.split()) for x in article.findall(".//AbstractText") if x.text]
            year_text = (
                article.findtext(".//PubDate/Year")
                or article.findtext(".//PubDate/MedlineDate", default="")[:4]
            )
            try:
                year = int(year_text) if year_text.isdigit() else None
            except ValueError:
                year = None
            if not pmid:
                continue
            verification_url = f"https://pubmed.ncbi.nlm.nih.gov/{pmid}/"
            doi_values = [
                value.text.strip()
                for value in article.findall(".//ArticleIdList/ArticleId[@IdType=\"doi\"]")
                if value.text and value.text.strip()
            ]
            results.append(SearchResult(
                provider_id=self.provider_id,
                provider_record_id=pmid,
                title=title,
                source_class=query.source_class if query else "biomedical_literature",
                geography=query.geography if query else "global",
                publication_year=year,
                source_url=verification_url,
                verification_url=verification_url,
                abstract_or_summary=" ".join(abstract_parts) or None,
                identifiers=tuple(f"doi:{value}" for value in doi_values),
                source=source_contract.with_record(
                    source_record_id=pmid,
                    source_url=verification_url,
                    verification_url=verification_url,
                    retrieved_at=retrieved_at,
                    published_at=None,
                ),
            ))
        return results


def from_environment() -> PubMedAdapter:
    return PubMedAdapter(
        tool_name=os.environ.get("PUBMED_TOOL_NAME", "soma-os-evidence-research"),
        contact_email=os.environ.get("PUBMED_CONTACT_EMAIL", ""),
        api_key=os.environ.get("PUBMED_API_KEY") or None,
    )
