-- Legacy Data Promotion Preflight v1
--
-- Read-only, fail-closed verification immediately before the final NOT NULL gate.
-- This does not mutate protected data or weaken the ordinary RLS boundary.
-- The SECURITY DEFINER owner is deliberately non-login and is used only for the
-- narrowly scoped aggregate verification required to inspect NULL-scoped legacy rows.

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'somaos_legacy_preflight_owner') THEN
        CREATE ROLE somaos_legacy_preflight_owner
            NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT
            NOREPLICATION BYPASSRLS;
    END IF;
END
$$;

ALTER ROLE somaos_legacy_preflight_owner
    NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT
    NOREPLICATION BYPASSRLS;

CREATE OR REPLACE FUNCTION public.soma_legacy_promotion_preflight()
RETURNS TABLE (
    total_rows BIGINT,
    null_scope_rows BIGINT,
    partial_scope_rows BIGINT,
    fully_scoped_rows BIGINT,
    scoped_without_applied_event BIGINT,
    applied_event_scope_mismatches BIGINT,
    eligible_unpromoted_rows BIGINT,
    ambiguous_eligible_rows BIGINT,
    preflight_passed BOOLEAN
)
LANGUAGE sql
SECURITY DEFINER
SET search_path = pg_catalog
AS $$
    WITH row_state AS (
        SELECT
            v.log_id,
            v.tenant_id,
            v.data_domain,
            EXISTS (
                SELECT 1
                  FROM public.anonymized_user_vitals_legacy_promotion_event e
                 WHERE e.legacy_log_id = v.log_id
                   AND e.promotion_status = 'APPLIED'
            ) AS has_applied_event,
            EXISTS (
                SELECT 1
                  FROM public.anonymized_user_vitals_legacy_promotion_event e
                 WHERE e.legacy_log_id = v.log_id
                   AND e.promotion_status = 'APPLIED'
                   AND (e.tenant_id IS DISTINCT FROM v.tenant_id
                        OR e.data_domain IS DISTINCT FROM v.data_domain)
            ) AS applied_event_scope_mismatch
        FROM public.anonymized_user_vitals v
    ),
    eligibility AS (
        SELECT
            v.log_id,
            COUNT(c.classification_id) FILTER (
                WHERE c.classification_state = 'SCOPED_VERIFIED'
                  AND c.verification_status = 'VERIFIED'
                  AND c.candidate_tenant_id IS NOT NULL
                  AND c.candidate_data_domain IS NOT NULL
                  AND c.provenance_reference IS NOT NULL
            ) AS eligible_count
        FROM public.anonymized_user_vitals v
        LEFT JOIN public.anonymized_user_vitals_legacy_classification c
          ON c.legacy_log_id = v.log_id
        WHERE v.tenant_id IS NULL AND v.data_domain IS NULL
        GROUP BY v.log_id
    ),
    aggregates AS (
        SELECT
            COUNT(*) AS total_rows,
            COUNT(*) FILTER (WHERE tenant_id IS NULL AND data_domain IS NULL) AS null_scope_rows,
            COUNT(*) FILTER (WHERE (tenant_id IS NULL) <> (data_domain IS NULL)) AS partial_scope_rows,
            COUNT(*) FILTER (WHERE tenant_id IS NOT NULL AND data_domain IS NOT NULL) AS fully_scoped_rows,
            COUNT(*) FILTER (WHERE tenant_id IS NOT NULL AND data_domain IS NOT NULL AND NOT has_applied_event) AS scoped_without_applied_event,
            COUNT(*) FILTER (WHERE applied_event_scope_mismatch) AS applied_event_scope_mismatches
        FROM row_state
    )
    SELECT
        a.total_rows,
        a.null_scope_rows,
        a.partial_scope_rows,
        a.fully_scoped_rows,
        a.scoped_without_applied_event,
        a.applied_event_scope_mismatches,
        COALESCE((SELECT COUNT(*) FROM eligibility WHERE eligible_count = 1), 0) AS eligible_unpromoted_rows,
        COALESCE((SELECT COUNT(*) FROM eligibility WHERE eligible_count > 1), 0) AS ambiguous_eligible_rows,
        (
            a.null_scope_rows = 0
            AND a.partial_scope_rows = 0
            AND a.applied_event_scope_mismatches = 0
        ) AS preflight_passed
    FROM aggregates a
$$;

ALTER FUNCTION public.soma_legacy_promotion_preflight()
    OWNER TO somaos_legacy_preflight_owner;

REVOKE ALL ON FUNCTION public.soma_legacy_promotion_preflight() FROM PUBLIC;
GRANT EXECUTE ON FUNCTION public.soma_legacy_promotion_preflight() TO somaos_persistence;

COMMENT ON FUNCTION public.soma_legacy_promotion_preflight() IS
    'Read-only zero-NULL legacy promotion preflight. Reports protected-scope completeness and promotion-event consistency without mutating data.';
