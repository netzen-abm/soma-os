"""Canonical normalized research-source query, result, and access contracts for SOMA.

Provider adapters and orchestration consume these shared types. Provider-specific
modules must not define competing normalized query/result/access models.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Optional

from external_knowledge_source import ExternalKnowledgeSource


@dataclass(frozen=True)
class ResearchQuery:
    """Provider-neutral, governed research retrieval query."""

    query_id: str
    terms: str
    source_class: str
    geography: str = "global"
    retmax: int = 20

    def __post_init__(self) -> None:
        if not self.query_id.strip():
            raise ValueError("query_id is required")
        if not self.terms.strip():
            raise ValueError("terms are required")
        if not self.source_class.strip():
            raise ValueError("source_class is required")
        if not self.geography.strip():
            raise ValueError("geography is required")
        if self.retmax <= 0:
            raise ValueError("retmax must be positive")


@dataclass(frozen=True)
class ResearchAccessLocation:
    """Provider-derived access location; not an evidence-quality judgment."""

    location_id: str
    source_id: str
    url: str
    location_type: str
    is_open_access: Optional[bool]
    license_if_reported: Optional[str]
    version_label: Optional[str] = None
    retrieved_at: Optional[str] = None

    def __post_init__(self) -> None:
        if not self.location_id.strip():
            raise ValueError("location_id is required")
        if not self.source_id.strip():
            raise ValueError("source_id is required")
        if not self.url.strip():
            raise ValueError("url is required")
        if not self.location_type.strip():
            raise ValueError("location_type is required")


@dataclass(frozen=True)
class ResearchRecord:
    """Canonical source-derived scholarly record awaiting assessment."""

    provider_id: str
    provider_record_id: str
    title: str
    source_class: str
    geography: str
    source_url: str
    verification_url: str
    publication_year: Optional[int] = None
    abstract_or_summary: Optional[str] = None
    identifiers: tuple[str, ...] = ()
    source: Optional[ExternalKnowledgeSource] = None
    access_locations: tuple[ResearchAccessLocation, ...] = ()

    def __post_init__(self) -> None:
        if not self.provider_id.strip():
            raise ValueError("provider_id is required")
        if not self.provider_record_id.strip():
            raise ValueError("provider_record_id is required")
        if not self.source_class.strip():
            raise ValueError("source_class is required")
        if not self.geography.strip():
            raise ValueError("geography is required")
        if not self.source_url.strip():
            raise ValueError("source_url is required")
        if not self.verification_url.strip():
            raise ValueError("verification_url is required")
        if self.publication_year is not None and not 0 < self.publication_year <= 3000:
            raise ValueError("publication_year is invalid")


# Compatibility aliases: existing adapters may expose these names without
# defining competing normalized contracts. New integrations should use the
# canonical types above.
SearchResult = ResearchRecord
NormalizedResearchQuery = ResearchQuery
