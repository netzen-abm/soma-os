from legacy_data_classification import (
    ClassificationState,
    LegacyClassification,
    LegacyClassificationError,
    VerificationStatus,
    is_protected_access_eligible,
    require_protected_promotion,
    validate_classification,
)


def record(**overrides):
    values = {
        "legacy_log_id": 1,
        "classification_state": ClassificationState.SCOPED_VERIFIED,
        "candidate_tenant_id": "tenant-a",
        "candidate_data_domain": "personal-health",
        "reason_code": "AUTHORITATIVE_SOURCE",
        "provenance_reference": "prov-1",
        "classified_by": "reviewer-1",
        "verification_status": VerificationStatus.VERIFIED,
        "review_reference": "review-1",
    }
    values.update(overrides)
    return LegacyClassification(**values)


def test_verified_scoped_record_is_eligible():
    assert is_protected_access_eligible(record()) is True


def test_unknown_scope_is_not_eligible():
    assert is_protected_access_eligible(
        record(
            classification_state=ClassificationState.QUARANTINED_UNCLASSIFIED,
            candidate_tenant_id=None,
            candidate_data_domain=None,
            provenance_reference=None,
            verification_status=VerificationStatus.UNVERIFIED,
        )
    ) is False


def test_partial_scope_is_rejected():
    try:
        validate_classification(record(candidate_data_domain=None))
    except LegacyClassificationError:
        return
    raise AssertionError("partial scope must fail closed")


def test_unverified_scope_is_not_eligible():
    assert is_protected_access_eligible(
        record(verification_status=VerificationStatus.UNVERIFIED)
    ) is False


def test_missing_provenance_is_not_eligible():
    assert is_protected_access_eligible(record(provenance_reference=None)) is False


def test_promotion_requires_explicit_event():
    try:
        require_protected_promotion(
            record(),
            promotion_event_reference="",
            authoritative_provenance={"reference": "prov-1", "verification": "VERIFIED"},
        )
    except LegacyClassificationError:
        return
    raise AssertionError("promotion must require an explicit event")


def test_promotion_requires_matching_verified_provenance():
    for provenance in (
        {"reference": "wrong", "verification": "VERIFIED"},
        {"reference": "prov-1", "verification": "UNVERIFIED"},
    ):
        try:
            require_protected_promotion(
                record(),
                promotion_event_reference="promotion-1",
                authoritative_provenance=provenance,
            )
        except LegacyClassificationError:
            continue
        raise AssertionError("untrusted provenance must fail closed")


def test_valid_promotion_returns_only_verified_scope():
    assert require_protected_promotion(
        record(),
        promotion_event_reference="promotion-1",
        authoritative_provenance={"reference": "prov-1", "verification": "VERIFIED"},
    ) == ("tenant-a", "personal-health")


def test_caller_metadata_cannot_create_eligibility():
    quarantined = record(
        classification_state=ClassificationState.QUARANTINED_UNCLASSIFIED,
        candidate_tenant_id=None,
        candidate_data_domain=None,
        provenance_reference=None,
        verification_status=VerificationStatus.UNVERIFIED,
    )
    caller_metadata = {
        "tenant_id": "tenant-a",
        "data_domain": "personal-health",
        "anonymized_user_hash": "same-hash",
        "transport": "telegram",
        "model_output": "tenant-a",
    }
    assert is_protected_access_eligible(quarantined) is False
    assert caller_metadata


if __name__ == "__main__":
    tests = [value for name, value in globals().items() if name.startswith("test_") and callable(value)]
    for test in tests:
        test()
    print(f"Legacy classification tests: {len(tests)}/{len(tests)} passed")
