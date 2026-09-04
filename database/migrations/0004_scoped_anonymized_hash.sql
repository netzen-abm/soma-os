-- Protected Data Access Enforcement v1 - scoped identity uniqueness
--
-- The original table used a globally unique anonymized_user_hash. Protected
-- persistence is tenant/data-domain scoped, so uniqueness must follow the same
-- security boundary. PostgreSQL UNIQUE permits multiple NULL values, preserving
-- coexistence of legacy/unclassified rows during the transition phase.

ALTER TABLE anonymized_user_vitals
    DROP CONSTRAINT IF EXISTS anonymized_user_vitals_anonymized_user_hash_key;

CREATE UNIQUE INDEX IF NOT EXISTS uq_anonymized_user_vitals_scope_hash
    ON anonymized_user_vitals(tenant_id, data_domain, anonymized_user_hash);

COMMENT ON INDEX uq_anonymized_user_vitals_scope_hash IS
    'Protected-data uniqueness is scoped to explicit tenant and data domain; legacy NULL-scope rows remain transition data.';
