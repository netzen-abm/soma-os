-- Final Protected Data NOT NULL Gate v1
--
-- Fail-closed schema finalization. The zero-NULL preflight is executed inside
-- the same migration transaction immediately before schema tightening.
-- No scope is inferred or repaired by this migration.

DO $$
DECLARE
    result RECORD;
BEGIN
    SELECT *
      INTO result
      FROM public.soma_legacy_promotion_preflight();

    IF NOT COALESCE(result.preflight_passed, FALSE) THEN
        RAISE EXCEPTION
            'protected-data final gate blocked: preflight failed (total_rows=%, null_scope_rows=%, partial_scope_rows=%, applied_event_scope_mismatches=%)',
            result.total_rows,
            result.null_scope_rows,
            result.partial_scope_rows,
            result.applied_event_scope_mismatches;
    END IF;

    IF result.null_scope_rows <> 0 OR result.partial_scope_rows <> 0 THEN
        RAISE EXCEPTION
            'protected-data final gate blocked: non-complete protected scope remains';
    END IF;
END
$$;

-- The check was deliberately introduced as NOT VALID while legacy NULL rows
-- existed. The preflight above proves there are now no violating rows, so the
-- existing constraint can be validated without rewriting or repairing data.
DO $$
BEGIN
    IF EXISTS (
        SELECT 1
          FROM pg_constraint c
          JOIN pg_class t ON t.oid = c.conrelid
          JOIN pg_namespace n ON n.oid = t.relnamespace
         WHERE n.nspname = 'public'
           AND t.relname = 'anonymized_user_vitals'
           AND c.conname = 'anonymized_user_vitals_protected_scope_required'
           AND NOT c.convalidated
    ) THEN
        ALTER TABLE public.anonymized_user_vitals
            VALIDATE CONSTRAINT anonymized_user_vitals_protected_scope_required;
    END IF;
END
$$;

-- Make the two protected scope dimensions structurally mandatory. PostgreSQL
-- DDL is transactional, so a failure rolls back both the preconditioned schema
-- changes and this gate as one unit.
ALTER TABLE public.anonymized_user_vitals
    ALTER COLUMN tenant_id SET NOT NULL,
    ALTER COLUMN data_domain SET NOT NULL;

COMMENT ON COLUMN public.anonymized_user_vitals.tenant_id IS
    'Required protected tenant scope; established only through the trusted persistence boundary.';
COMMENT ON COLUMN public.anonymized_user_vitals.data_domain IS
    'Required protected data-domain scope; established only through the trusted persistence boundary.';
