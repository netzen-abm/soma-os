"""Provider-neutral contract for explicit legacy-data promotion.

The promotion layer consumes classification evidence; it never infers tenant or
data-domain scope. Database adapters are responsible for persistence and must
apply the same authorization and trusted-context boundaries as normal protected
data access.
"""

from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime
from enum import Enum
from typing import Iterable

from legacy_data_classification import (
    ClassificationState,
    VerificationStatus,
)


class PromotionError(ValueError):
    """Raised when legacy promotion invariants are violated."""


@dataclass(frozen=True)
class ClassificationEvidence:
    classification_id: int
    legacy_log_id: int
    classification_state: ClassificationState
    candidate_tenant_id: str | None
    candidate_data_domain: str | None
    provenance_reference: str | None
    verification_status: VerificationStatus
    classified_at: datetime


@dataclass(frozen=True)
class AuthoritativeClassification:
    classification_id: int
    legacy_log_id: int
    tenant_id: str
    data_domain: str
    provenance_reference: str
    classified_at: datetime


def select_authoritative_classification(
    classifications: Iterable[ClassificationEvidence],
) -> AuthoritativeClassification | None:
    """Select one authoritative verified classification, or fail closed.

    A candidate is eligible only when it is explicitly SCOPED_VERIFIED, VERIFIED,
    fully scoped, and provenance-bound. The newest classified_at timestamp is used
    as the deterministic ordering key. Equal timestamps are ambiguous and are
    rejected rather than broken by an arbitrary tie-breaker.
    """
    candidates = [
        item
        for item in classifications
        if item.classification_state is ClassificationState.SCOPED_VERIFIED
        and item.verification_status is VerificationStatus.VERIFIED
        and item.candidate_tenant_id is not None
        and item.candidate_data_domain is not None
        and item.provenance_reference is not None
    ]

    if not candidates:
        return None

    newest_time = max(item.classified_at for item in candidates)
    newest = [item for item in candidates if item.classified_at == newest_time]
    if len(newest) != 1:
        raise PromotionError("multiple verified classifications share the authoritative timestamp")

    selected = newest[0]
    return AuthoritativeClassification(
        classification_id=selected.classification_id,
        legacy_log_id=selected.legacy_log_id,
        tenant_id=selected.candidate_tenant_id,
        data_domain=selected.candidate_data_domain,
        provenance_reference=selected.provenance_reference,
        classified_at=selected.classified_at,
    )


def require_authoritative_promotion(
    classification: AuthoritativeClassification,
    *,
    promotion_event_reference: str,
    authoritative_provenance_reference: str,
) -> tuple[str, str]:
    """Bind an explicit promotion event to the selected evidence provenance."""
    if not promotion_event_reference.strip():
        raise PromotionError("promotion event reference is required")
    if not authoritative_provenance_reference.strip():
        raise PromotionError("authoritative provenance reference is required")
    if classification.provenance_reference != authoritative_provenance_reference:
        raise PromotionError("promotion provenance does not match selected classification")
    return classification.tenant_id, classification.data_domain
