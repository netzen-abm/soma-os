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
from dataclasses import dataclass
from typing import Optional

BASE_URL = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/"


@dataclass(frozen=True)
class NormalizedResearchQuery:
    query_id: str
    terms: str
    retmax: int = 20


@dataclass(frozen=True)
class SearchResult:
    provider_id: str
    provider_record_id: str
    title: str
    publication_year: Optional[int]
    source_url: str
    verification_url: str
    abstract_or_summary: Optional[str]


class PubMedProviderError(RuntimeError):
    """A provider/network failure; never interpret this as no evidence."""


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
        return self._fetch(ids)

    def _fetch(self, ids: list[str]) -> list[SearchResult]:
        raw = self._request(
            "efetch.fcgi",
            {"db": "pubmed", "id": ",".join(ids), "retmode": "xml"},
        )
        try:
            root = ET.fromstring(raw)
        except ET.ParseError as exc:
            raise PubMedProviderError("Invalid PubMed EFetch response") from exc

        results: list[SearchResult] = []
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
            results.append(SearchResult(
                provider_id=self.provider_id,
                provider_record_id=pmid,
                title=title,
                publication_year=year,
                source_url=verification_url,
                verification_url=verification_url,
                abstract_or_summary=" ".join(abstract_parts) or None,
            ))
        return results


def from_environment() -> PubMedAdapter:
    return PubMedAdapter(
        tool_name=os.environ.get("PUBMED_TOOL_NAME", "soma-os-evidence-research"),
        contact_email=os.environ.get("PUBMED_CONTACT_EMAIL", ""),
        api_key=os.environ.get("PUBMED_API_KEY") or None,
    )
