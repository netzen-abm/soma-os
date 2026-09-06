from datetime import datetime, timedelta, timezone

from legacy_data_classification import ClassificationState, VerificationStatus
from legacy_data_promotion import (
    ClassificationEvidence,
    PromotionError,
    require_authoritative_promotion,
    select_authoritative_classification,
)


BASE = datetime(2026, 1, 1, tzinfo=timezone.utc)


def evidence(**overrides):
    values = {
        "classification_id": 1,
        "legacy_log_id": 10,
        "classification_state": ClassificationState.SCOPED_VERIFIED,
        "candidate_tenant_id": "tenant-a",
        "candidate_data_domain": "personal-health",
        "provenance_reference": "prov-1",
        "verification_status": VerificationStatus.VERIFIED,
        "classified_at": BASE,
    }
    values.update(overrides)
    return ClassificationEvidence(**values)


def test_no_verified_candidate_is_ineligible():
    assert select_authoritative_classification([
        evidence(
            classification_state=ClassificationState.QUARANTINED_UNCLASSIFIED,
            candidate_tenant_id=None,
            candidate_data_domain=None,
            provenance_reference=None,
            verification_status=VerificationStatus.UNVERIFIED,
        )
    ]) is None


def test_latest_verified_classification_is_authoritative():
    selected = select_authoritative_classification([
        evidence(),
        evidence(
            classification_id=2,
            candidate_tenant_id="tenant-b",
            candidate_data_domain="research",
            provenance_reference="prov-2",
            classified_at=BASE + timedelta(minutes=1),
        ),
    ])
    assert selected is not None
    assert selected.classification_id == 2
    assert selected.tenant_id == "tenant-b"
    assert selected.data_domain == "research"
    assert selected.provenance_reference == "prov-2"


def test_same_timestamp_is_ambiguous_and_fails_closed():
    try:
        select_authoritative_classification([
            evidence(classification_id=1),
            evidence(classification_id=2, candidate_tenant_id="tenant-b"),
        ])
    except PromotionError:
        return
    raise AssertionError("equal authoritative timestamps must fail closed")


def test_quarantined_or_unverified_history_cannot_displace_verified_candidate():
    selected = select_authoritative_classification([
        evidence(),
        evidence(
            classification_id=3,
            classification_state=ClassificationState.QUARANTINED_CONFLICT,
            candidate_tenant_id=None,
            candidate_data_domain=None,
            provenance_reference=None,
            verification_status=VerificationStatus.UNVERIFIED,
            classified_at=BASE + timedelta(minutes=2),
        ),
    ])
    assert selected is not None
    assert selected.classification_id == 1


def test_promotion_requires_matching_provenance():
    selected = select_authoritative_classification([evidence()])
    assert selected is not None
    try:
        require_authoritative_promotion(
            selected,
            promotion_event_reference="promotion-1",
            authoritative_provenance_reference="wrong",
        )
    except PromotionError:
        return
    raise AssertionError("provenance mismatch must fail closed")


def test_promotion_returns_only_selected_verified_scope():
    selected = select_authoritative_classification([evidence()])
    assert selected is not None
    assert require_authoritative_promotion(
        selected,
        promotion_event_reference="promotion-1",
        authoritative_provenance_reference="prov-1",
    ) == ("tenant-a", "personal-health")


if __name__ == "__main__":
    tests = [value for name, value in globals().items() if name.startswith("test_") and callable(value)]
    for test in tests:
        test()
    print(f"Legacy promotion tests: {len(tests)}/{len(tests)} passed")
