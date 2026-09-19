"""Provider-neutral multi-source research orchestration for SOMA.

This module coordinates existing provider adapters; it does not make clinical
or efficacy judgments. Assessment belongs to the core evidence layer.
"""
from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Protocol, Sequence

from research_source_contract import ResearchQuery, ResearchRecord

ProviderResult = ResearchRecord



class ResearchAccessResolver(Protocol):
    provider_id: str

    def resolve(self, doi: str) -> Sequence[Any]:
        """Resolve provider-reported access locations for a stable work identifier."""
        ...


def _doi_from_record(record: ResearchRecord) -> str | None:
    """Return a normalized DOI identifier when the canonical record contains one."""
    for identifier in record.identifiers:
        value = identifier.strip()
        lowered = value.lower()
        if lowered.startswith("doi:"):
            return value[4:].strip()
        if lowered.startswith("https://doi.org/"):
            return value[len("https://doi.org/"):].strip().strip("/")
        if lowered.startswith("http://doi.org/"):
            return value[len("http://doi.org/"):].strip().strip("/")
    return None


def attach_access_locations(
    record: ResearchRecord,
    resolver: ResearchAccessResolver,
) -> ResearchRecord:
    """Attach provider-derived access locations without changing evidence state.

    This composition boundary is intentionally separate from retrieval and
    evidence assessment. A resolver failure must be handled by the governing
    orchestration layer; it must never be represented as evidence absence.
    """
    doi = _doi_from_record(record)
    if not doi:
        return record
    locations = tuple(resolver.resolve(doi))
    return ResearchRecord(
        provider_id=record.provider_id,
        provider_record_id=record.provider_record_id,
        title=record.title,
        source_class=record.source_class,
        geography=record.geography,
        source_url=record.source_url,
        verification_url=record.verification_url,
        publication_year=record.publication_year,
        abstract_or_summary=record.abstract_or_summary,
        identifiers=record.identifiers,
        source=record.source,
        access_locations=locations,
    )


class ResearchAdapter(Protocol):
    provider_id: str

    def search(self, query: Any) -> Sequence[Any]:
        """Search using the adapter's normalized query shape."""
        ...


@dataclass(frozen=True)
class SearchRoute:
    source_class: str
    provider_ids: tuple[str, ...]
    required: bool = True

    def __post_init__(self) -> None:
        if not self.source_class:
            raise ValueError("source_class is required")
        if not self.provider_ids:
            raise ValueError("provider_ids must not be empty")


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
    def _normalize_result(
        result: Any, query: ResearchQuery
    ) -> ProviderResult:
        """Convert a governed adapter result into the shared orchestration envelope."""
        provider_id = str(getattr(result, "provider_id", "")).strip()
        provider_record_id = str(getattr(result, "provider_record_id", "")).strip()
        if not provider_id or not provider_record_id:
            raise ValueError("adapter result must contain provider identity")

        identifiers = tuple(
            identifier
            for identifier in (provider_record_id, *getattr(result, "identifiers", ()))
            if isinstance(identifier, str) and identifier.strip()
        )
        return ProviderResult(
            provider_id=provider_id,
            provider_record_id=provider_record_id,
            title=" ".join(str(getattr(result, "title", "")).split()),
            source_class=query.source_class,
            geography=query.geography,
            source_url=str(getattr(result, "source_url", "")),
            verification_url=str(getattr(result, "verification_url", "")),
            publication_year=getattr(result, "publication_year", None),
            abstract_or_summary=getattr(result, "abstract_or_summary", None),
            identifiers=identifiers,
            source=getattr(result, "source", None),
        )

    @staticmethod
    def _dedupe_key(result: ProviderResult) -> tuple[str, str]:
        """Prefer stable cross-provider identifiers; fall back to title/year."""
        for identifier in result.identifiers:
            if not identifier:
                continue
            normalized = identifier.strip().lower()
            prefix = normalized.split(":", 1)[0]
            if prefix in {"doi", "pmid", "nct", "ctri", "isrctn"}:
                return (prefix, normalized)
        title = " ".join(result.title.lower().split())
        return ("title-year", f"{title}|{result.publication_year or ''}")

    def run(
        self,
        queries: Sequence[ResearchQuery],
        routes: Sequence[SearchRoute],
    ) -> OrchestrationResult:
        unique: dict[tuple[str, str], ProviderResult] = {}
        provider_status: dict[str, str] = {}
        completed_routes: list[str] = []
        failed_routes: list[str] = []

        for route in routes:
            matching_queries = [q for q in queries if q.source_class == route.source_class]
            if not matching_queries:
                provider_status[f"route:{route.source_class}"] = "NO_QUERY"
                if route.required:
                    failed_routes.append(route.source_class)
                continue

            route_completed = True
            for query in matching_queries:
                for provider_id in route.provider_ids:
                    adapter = self.adapters.get(provider_id)
                    if adapter is None:
                        provider_status[provider_id] = "PROVIDER_UNAVAILABLE"
                        route_completed = False
                        continue
                    try:
                        found = adapter.search(query)
                        if not found:
                            provider_status[provider_id] = "COMPLETED_NO_RESULTS"
                            continue
                        provider_status[provider_id] = "COMPLETED_WITH_RESULTS"
                        for raw_result in found:
                            normalized = self._normalize_result(raw_result, query)
                            unique.setdefault(self._dedupe_key(normalized), normalized)
                    except Exception:
                        provider_status[provider_id] = "PROVIDER_UNAVAILABLE"
                        route_completed = False

            if route_completed:
                completed_routes.append(route.source_class)
            else:
                failed_routes.append(route.source_class)

        required_failed = {
            route.source_class for route in routes if route.required
        } & set(failed_routes)
        attempted_required = {
            route.source_class for route in routes if route.required
        }
        all_required_failed = bool(attempted_required) and all(
            provider_status.get(provider_id) == "PROVIDER_UNAVAILABLE"
            for route in routes
            if route.required
            for provider_id in route.provider_ids
        )

        if all_required_failed:
            status = "PROVIDER_UNAVAILABLE"
        elif required_failed:
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
