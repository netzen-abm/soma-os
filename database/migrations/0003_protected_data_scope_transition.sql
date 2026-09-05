-- Protected Data Access Enforcement v1 - transition migration
--
-- This migration intentionally does NOT enable RLS yet. Existing records do not
-- have trustworthy tenant/data-domain provenance, so silently inventing scope
-- would be a security defect. The application access boundary must be deployed
-- and legacy rows classified/quarantined before database enforcement is enabled.

ALTER TABLE anonymized_user_vitals
    ADD COLUMN IF NOT EXISTS tenant_id TEXT,
    ADD COLUMN IF NOT EXISTS data_domain TEXT;

COMMENT ON COLUMN anonymized_user_vitals.tenant_id IS
    'Explicit security isolation scope. NULL means legacy/unclassified and is not protected-access eligible.';
COMMENT ON COLUMN anonymized_user_vitals.data_domain IS
    'Explicit governed data domain. NULL means legacy/unclassified and is not protected-access eligible.';

CREATE INDEX IF NOT EXISTS idx_anonymized_user_vitals_tenant_domain
    ON anonymized_user_vitals(tenant_id, data_domain);

-- Deliberately no default tenant/domain and no NOT NULL constraint at this stage.
-- Phase 2 must classify or quarantine legacy rows before enforcing non-null scope.
