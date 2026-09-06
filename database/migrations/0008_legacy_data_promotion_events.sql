-- Legacy Data Promotion Events v1
--
-- Records explicit promotion intent/outcome without mutating classification history.
-- Scope is copied only from an authoritative verified classification selected by the
-- promotion service; this table is an audit ledger, not an authorization mechanism.

CREATE TABLE IF NOT EXISTS anonymized_user_vitals_legacy_promotion_event (
    promotion_event_id BIGSERIAL PRIMARY KEY,
    promotion_event_reference TEXT NOT NULL UNIQUE,
    legacy_log_id INTEGER NOT NULL REFERENCES anonymized_user_vitals(log_id),
    classification_id BIGINT NOT NULL REFERENCES anonymized_user_vitals_legacy_classification(classification_id),
    provenance_reference TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    data_domain TEXT NOT NULL,
    promotion_status TEXT NOT NULL CHECK (
        promotion_status IN ('REQUESTED', 'APPLIED', 'ROLLED_BACK', 'REJECTED')
    ),
    initiated_by TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    applied_at TIMESTAMPTZ,
    rollback_reference TEXT,
    CHECK (length(trim(promotion_event_reference)) > 0),
    CHECK (length(trim(provenance_reference)) > 0),
    CHECK (length(trim(tenant_id)) > 0),
    CHECK (length(trim(data_domain)) > 0),
    CHECK (length(trim(initiated_by)) > 0)
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_legacy_promotion_one_event_per_log
    ON anonymized_user_vitals_legacy_promotion_event(legacy_log_id)
    WHERE promotion_status IN ('REQUESTED', 'APPLIED');

CREATE INDEX IF NOT EXISTS idx_legacy_promotion_event_log_id
    ON anonymized_user_vitals_legacy_promotion_event(legacy_log_id);

COMMENT ON TABLE anonymized_user_vitals_legacy_promotion_event IS
    'Append-only audit ledger for explicit, provenance-bound legacy-data promotion events.';

-- No legacy row is mutated here. Promotion application must occur in a protected
-- transaction through the canonical authorization and trusted DB identity boundary.
-- Final NOT NULL remains a later migration after a verified zero-NULL preflight.
