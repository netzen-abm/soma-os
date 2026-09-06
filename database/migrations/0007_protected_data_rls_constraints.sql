-- Protected Data PostgreSQL RLS + write constraints v2
--
-- This migration activates database defense-in-depth after the trusted
-- persistence service identity boundary in 0006. Policy Kernel authorization
-- remains authoritative; PostgreSQL RLS prevents accidental/cross-scope data
-- access even when an application query omits explicit tenant predicates.
--
-- Legacy rows with NULL tenant_id/data_domain remain inaccessible through the
-- protected persistence role. The scope check is NOT VALID so existing legacy
-- rows are not silently rewritten or stranded; it applies to new/changed rows.

ALTER TABLE anonymized_user_vitals
    ENABLE ROW LEVEL SECURITY;

-- The table owner must not bypass the policy accidentally. PostgreSQL
-- superusers remain outside RLS by design; production persistence must use the
-- dedicated non-owner somaos_persistence role established in 0006.
ALTER TABLE anonymized_user_vitals
    FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS anonymized_user_vitals_protected_scope ON anonymized_user_vitals;

CREATE POLICY anonymized_user_vitals_protected_scope
    ON anonymized_user_vitals
    FOR ALL
    TO somaos_persistence
    USING (
        current_setting('soma.tenant_id', true) = tenant_id
        AND current_setting('soma.data_domain', true) = data_domain
        AND tenant_id IS NOT NULL
        AND data_domain IS NOT NULL
    )
    WITH CHECK (
        current_setting('soma.tenant_id', true) = tenant_id
        AND current_setting('soma.data_domain', true) = data_domain
        AND tenant_id IS NOT NULL
        AND data_domain IS NOT NULL
    );

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conrelid = 'anonymized_user_vitals'::regclass
          AND conname = 'anonymized_user_vitals_protected_scope_required'
    ) THEN
        ALTER TABLE anonymized_user_vitals
            ADD CONSTRAINT anonymized_user_vitals_protected_scope_required
            CHECK (tenant_id IS NOT NULL AND data_domain IS NOT NULL)
            NOT VALID;
    END IF;
END
$$;

COMMENT ON POLICY anonymized_user_vitals_protected_scope ON anonymized_user_vitals IS
    'Defense-in-depth tenant/data-domain isolation. Trusted scope is established only through somaos_persistence; Policy Kernel remains authorization authority.';

COMMENT ON CONSTRAINT anonymized_user_vitals_protected_scope_required ON anonymized_user_vitals IS
    'New and modified protected rows require explicit tenant_id and data_domain. Existing legacy NULL-scope rows require verified classification before final NOT NULL validation.';
