"""Shared external knowledge source contract primitives.

This module models the governed source envelope without introducing a
persistence layer or provider-specific authority. Provider adapters remain
responsible for retrieval; these types carry source identity, access,
governance, provenance, semantics, and trust metadata.
"""

from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime
from typing import Optional, Tuple


@dataclass(frozen=True)
class ExternalKnowledgeSource:
    """Provider-neutral governance/provenance envelope for one source."""

    source_id: str
    provider: str
    source_version: Optional[str]
    protocol: str
    endpoint: Optional[str]
    authentication_mode: str
    rate_policy_ref: Optional[str]
    license_status: str
    attribution_required: bool
    permitted_use: str
    retention_policy_ref: Optional[str]
    redistribution_status: str
    source_record_id: str
    source_url: Optional[str]
    verification_url: Optional[str]
    retrieved_at: datetime
    published_at: Optional[datetime]
    entity_types: Tuple[str, ...]
    identifier_systems: Tuple[str, ...]
    units: Tuple[str, ...]
    temporal_model: Optional[str]
    verification_status: str
    transformation_history: Tuple[str, ...]
    quality_status: str
    uncertainty: str

    def validate(self) -> None:
        """Reject incomplete identity/provenance without inventing values."""
        required = {
            "source_id": self.source_id,
            "provider": self.provider,
            "protocol": self.protocol,
            "authentication_mode": self.authentication_mode,
            "license_status": self.license_status,
            "permitted_use": self.permitted_use,
            "redistribution_status": self.redistribution_status,
            "source_record_id": self.source_record_id,
            "verification_status": self.verification_status,
            "quality_status": self.quality_status,
            "uncertainty": self.uncertainty,
        }
        if any(not value.strip() for value in required.values()):
            raise ValueError("External knowledge source contract has empty required fields")
        if not self.entity_types:
            raise ValueError("External knowledge source must declare entity types")
        if not self.identifier_systems:
            raise ValueError("External knowledge source must declare identifier systems")

    def with_record(
        self,
        *,
        source_record_id: str,
        source_url: Optional[str],
        verification_url: Optional[str],
        retrieved_at: datetime,
        published_at: Optional[datetime],
    ) -> "ExternalKnowledgeSource":
        """Create a record-specific envelope while preserving source governance."""
        return ExternalKnowledgeSource(
            source_id=self.source_id,
            provider=self.provider,
            source_version=self.source_version,
            protocol=self.protocol,
            endpoint=self.endpoint,
            authentication_mode=self.authentication_mode,
            rate_policy_ref=self.rate_policy_ref,
            license_status=self.license_status,
            attribution_required=self.attribution_required,
            permitted_use=self.permitted_use,
            retention_policy_ref=self.retention_policy_ref,
            redistribution_status=self.redistribution_status,
            source_record_id=source_record_id,
            source_url=source_url,
            verification_url=verification_url,
            retrieved_at=retrieved_at,
            published_at=published_at,
            entity_types=self.entity_types,
            identifier_systems=self.identifier_systems,
            units=self.units,
            temporal_model=self.temporal_model,
            verification_status=self.verification_status,
            transformation_history=self.transformation_history,
            quality_status=self.quality_status,
            uncertainty=self.uncertainty,
        )
