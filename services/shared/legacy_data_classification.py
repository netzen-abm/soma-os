"""Provider-neutral legacy health-data classification and promotion gate.

This module does not mutate database rows or infer security scope. It only defines
what evidence is sufficient for a legacy classification to become eligible for a
protected-data promotion workflow.
"""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
from typing import Mapping


class ClassificationState(str, Enum):
    SCOPED_VERIFIED = "SCOPED_VERIFIED"
    SCOPED_REVIEW_REQUIRED = "SCOPED_REVIEW_REQUIRED"
    QUARANTINED_UNCLASSIFIED = "QUARANTINED_UNCLASSIFIED"
    QUARANTINED_CONFLICT = "QUARANTINED_CONFLICT"
    REJECTED_INVALID = "REJECTED_INVALID"


class VerificationStatus(str, Enum):
    UNVERIFIED = "UNVERIFIED"
    VERIFIED = "VERIFIED"


@dataclass(frozen=True)
class LegacyClassification:
    legacy_log_id: int
    classification_state: ClassificationState
    candidate_tenant_id: str | None
    candidate_data_domain: str | None
    reason_code: str
    provenance_reference: str | None
    classified_by: str
    verification_status: VerificationStatus
    review_reference: str | None = None


class LegacyClassificationError(ValueError):
    """Raised when a classification violates promotion invariants."""


def validate_classification(record: LegacyClassification) -> None:
    """Validate structural classification invariants without inferring anything."""
    if record.legacy_log_id <= 0:
        raise LegacyClassificationError("legacy_log_id must be positive")
    if not record.reason_code.strip() or not record.classified_by.strip():
        raise LegacyClassificationError("reason_code and classified_by are required")

    tenant_present = record.candidate_tenant_id is not None
    domain_present = record.candidate_data_domain is not None
    if tenant_present != domain_present:
        raise LegacyClassificationError("tenant and data-domain candidates must be both present or both absent")

    for value, field in (
        (record.candidate_tenant_id, "candidate_tenant_id"),
        (record.candidate_data_domain, "candidate_data_domain"),
        (record.provenance_reference, "provenance_reference"),
        (record.review_reference, "review_reference"),
    ):
        if value is not None and (not value.strip() or any(char.isspace() and char in "\r\n\t" for char in value)):
            raise LegacyClassificationError(f"{field} contains invalid control whitespace")


def is_protected_access_eligible(record: LegacyClassification) -> bool:
    """Return true only for an explicitly verified, fully scoped classification."""
    validate_classification(record)
    return (
        record.classification_state is ClassificationState.SCOPED_VERIFIED
        and record.verification_status is VerificationStatus.VERIFIED
        and record.candidate_tenant_id is not None
        and record.candidate_data_domain is not None
        and record.provenance_reference is not None
    )


def require_protected_promotion(
    record: LegacyClassification,
    *,
    promotion_event_reference: str,
    authoritative_provenance: Mapping[str, str],
) -> tuple[str, str]:
    """Require an explicit verified promotion event and return trusted scope.

    Caller-supplied transport/model/hash metadata is deliberately ignored. The
    promotion event must reference the same provenance recorded by classification.
    """
    if not promotion_event_reference.strip():
        raise LegacyClassificationError("explicit promotion event reference is required")
    if not is_protected_access_eligible(record):
        raise LegacyClassificationError("legacy classification is not eligible for protected promotion")

    if authoritative_provenance.get("reference") != record.provenance_reference:
        raise LegacyClassificationError("authoritative provenance does not match classification evidence")
    if authoritative_provenance.get("verification") != VerificationStatus.VERIFIED.value:
        raise LegacyClassificationError("authoritative provenance is not verified")

    return record.candidate_tenant_id, record.candidate_data_domain
