"""Provider-neutral multi-source research orchestration for SOMA.

This module coordinates adapters; it does not make clinical or efficacy
judgments. Assessment belongs to the core evidence layer.
"""
from __future__ import annotations

from dataclasses import dataclass
from typing import Protocol, Sequence


@dataclass(frozen=True)
class ResearchQuery:
    query_id: str
    terms: str
    source_class: str
    geography: str = "global"


@dataclass(frozen=True)
class ProviderResult:
    provider_id: str
    provider_record_id: str
    title: str
    source_class: str
    geography: str
    source_url: str
    verification_url: str
    publication_year: int | None = None
    abstract_or_summary: str | None = None
    identifiers: tuple[str, ...] = ()


class ResearchAdapter(Protocol):
    provider_id: str

    def search(self, query: ResearchQuery) -> Sequence[ProviderResult]:
        ...


@dataclass(frozen=True)
class SearchRoute:
    source_class: str
    provider_ids: tuple[str, ...]
    required: bool = True


@dataclass
class OrchestrationResult:
    results: list[ProviderResult]
    provider_status: dict[str, str]
    completed_routes: list[str]
    failed_routes: list[str]
    status: str


class MultiSourceOrchestrator:
    """Run routed research searches and deduplicate underlying studies."""

    def __init__(self, adapters: dict[str, ResearchAdapter]) -> None:
        self.adapters = adapters

    @staticmethod
    def _dedupe_key(result: ProviderResult) -> tuple[str, str]:
        """Prefer stable identifiers; fall back to normalized title/year."""
        for identifier in result.identifiers:
            if identifier:
                prefix = identifier.split(":", 1)[0].lower()
                if prefix in {"doi", "pmid", "nct", "ctri", "isrctn"}:
                    return (prefix, identifier.lower())
        title = " ".join(result.title.lower().split())
        return ("title-year", f"{title}|{result.publication_year or ''}")

    def run(self, queries: Sequence[ResearchQuery], routes: Sequence[SearchRoute]) -> OrchestrationResult:
        unique: dict[tuple[str, str], ProviderResult] = {}
        provider_status: dict[str, str] = {}
        completed_routes: list[str] = []
        failed_routes: list[str] = []

        for route in routes:
            route_completed = True
            for query in (q for q in queries if q.source_class == route.source_class):
                for provider_id in route.provider_ids:
                    adapter = self.adapters.get(provider_id)
                    if adapter is None:
                        provider_status[provider_id] = "PROVIDER_UNAVAILABLE"
                        route_completed = False
                        continue
                    try:
                        found = adapter.search(query)
                        provider_status[provider_id] = "COMPLETED_WITH_RESULTS" if found else "COMPLETED_NO_RESULTS"
                        for result in found:
                            unique.setdefault(self._dedupe_key(result), result)
                    except Exception:
                        provider_status[provider_id] = "PROVIDER_UNAVAILABLE"
                        route_completed = False
            if route_completed:
                completed_routes.append(route.source_class)
            else:
                failed_routes.append(route.source_class)

        required_classes = {r.source_class for r in routes if r.required}
        completed_classes = set(completed_routes)
        if required_classes - completed_classes:
            status = "PARTIAL_PROVIDER_FAILURE"
        elif unique:
            status = "COMPLETED_WITH_RESULTS"
        else:
            status = "COMPLETED_NO_RELEVANT_RESULTS"

        return OrchestrationResult(
            results=list(unique.values()),
            provider_status=provider_status,
            completed_routes=completed_routes,
            failed_routes=failed_routes,
            status=status,
        )
