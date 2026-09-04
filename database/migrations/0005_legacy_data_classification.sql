-- Legacy Data Classification & Quarantine v1
--
-- This migration records the disposition of pre-isolation rows without assigning
-- inferred tenant or data-domain scope. It intentionally does not enable RLS or
-- promote any legacy record into the protected population.

CREATE TABLE IF NOT EXISTS anonymized_user_vitals_legacy_classification (
    classification_id BIGSERIAL PRIMARY KEY,
    legacy_log_id INTEGER NOT NULL REFERENCES anonymized_user_vitals(log_id),
    classification_state TEXT NOT NULL CHECK (
        classification_state IN (
            'SCOPED_VERIFIED',
            'SCOPED_REVIEW_REQUIRED',
            'QUARANTINED_UNCLASSIFIED',
            'QUARANTINED_CONFLICT',
            'REJECTED_INVALID'
        )
    ),
    candidate_tenant_id TEXT,
    candidate_data_domain TEXT,
    reason_code TEXT NOT NULL,
    provenance_reference TEXT,
    classified_by TEXT NOT NULL,
    classified_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    verification_status TEXT NOT NULL CHECK (
        verification_status IN ('UNVERIFIED', 'VERIFIED')
    ),
    review_reference TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (
        (candidate_tenant_id IS NULL AND candidate_data_domain IS NULL)
        OR (candidate_tenant_id IS NOT NULL AND candidate_data_domain IS NOT NULL)
    )
);

CREATE INDEX IF NOT EXISTS idx_legacy_classification_log_id
    ON anonymized_user_vitals_legacy_classification(legacy_log_id);

CREATE INDEX IF NOT EXISTS idx_legacy_classification_state
    ON anonymized_user_vitals_legacy_classification(classification_state);

COMMENT ON TABLE anonymized_user_vitals_legacy_classification IS
    'Auditable disposition metadata for pre-isolation health-vital records; classification never infers protected scope.';

-- A legacy record must have at most one current classification disposition.
CREATE UNIQUE INDEX IF NOT EXISTS uq_legacy_classification_log_id
    ON anonymized_user_vitals_legacy_classification(legacy_log_id);

-- Do not add NOT NULL scope or RLS here. Those are later gates after classification
-- and quarantine have been verified against real migration data.
