-- Trusted legacy promotion executor v1
--
-- The persistence role is authorized to invoke this narrow SECURITY DEFINER
-- operation. The function owner is a dedicated NOLOGIN/BYPASSRLS database role;
-- this is required because the protected table is FORCE RLS and legacy NULL-scope
-- rows must remain inaccessible to ordinary protected-data queries until explicit
-- verified promotion.
--
-- The function derives tenant_id/data_domain exclusively from the selected,
-- verified classification row. Caller-supplied scope, hash, transport, model,
-- agent, or resource metadata is not accepted.

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'somaos_legacy_promotion_owner') THEN
        CREATE ROLE somaos_legacy_promotion_owner
            NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT
            NOREPLICATION BYPASSRLS;
    END IF;
END
$$;

ALTER ROLE somaos_legacy_promotion_owner
    NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT
    NOREPLICATION BYPASSRLS;

CREATE OR REPLACE FUNCTION public.soma_apply_legacy_promotion(
    p_legacy_log_id INTEGER,
    p_classification_id BIGINT,
    p_promotion_event_reference TEXT,
    p_authoritative_provenance TEXT,
    p_initiated_by TEXT
)
RETURNS BIGINT
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog
AS $$
DECLARE
    legacy_tenant TEXT;
    legacy_domain TEXT;
    classification_state TEXT;
    candidate_tenant TEXT;
    candidate_domain TEXT;
    classification_provenance TEXT;
    verification_status TEXT;
    existing_event_id BIGINT;
    existing_event_reference TEXT;
    existing_classification_id BIGINT;
    existing_provenance TEXT;
    existing_status TEXT;
    applied_event_id BIGINT;
BEGIN
    IF p_legacy_log_id IS NULL OR p_legacy_log_id <= 0 THEN
        RAISE EXCEPTION 'legacy_log_id must be positive';
    END IF;
    IF p_classification_id IS NULL OR p_classification_id <= 0 THEN
        RAISE EXCEPTION 'classification_id must be positive';
    END IF;
    IF p_promotion_event_reference IS NULL OR btrim(p_promotion_event_reference) = '' THEN
        RAISE EXCEPTION 'explicit promotion event reference is required';
    END IF;
    IF p_authoritative_provenance IS NULL OR btrim(p_authoritative_provenance) = '' THEN
        RAISE EXCEPTION 'authoritative provenance reference is required';
    END IF;
    IF p_initiated_by IS NULL OR btrim(p_initiated_by) = '' THEN
        RAISE EXCEPTION 'promotion initiator is required';
    END IF;
    IF p_promotion_event_reference ~ '[[:cntrl:]]'
       OR p_authoritative_provenance ~ '[[:cntrl:]]'
       OR p_initiated_by ~ '[[:cntrl:]]' THEN
        RAISE EXCEPTION 'promotion metadata contains control characters';
    END IF;

    -- Lock the legacy record first. This serializes concurrent promotion attempts
    -- for the same record and lets the following statements observe the committed
    -- outcome of an earlier promotion under PostgreSQL READ COMMITTED semantics.
    SELECT tenant_id, data_domain
      INTO legacy_tenant, legacy_domain
      FROM public.anonymized_user_vitals
     WHERE log_id = p_legacy_log_id
     FOR UPDATE;

    IF NOT FOUND THEN
        RAISE EXCEPTION 'legacy record % not found', p_legacy_log_id;
    END IF;

    SELECT promotion_event_id,
           promotion_event_reference,
           classification_id,
           provenance_reference,
           promotion_status
      INTO existing_event_id,
           existing_event_reference,
           existing_classification_id,
           existing_provenance,
           existing_status
      FROM public.anonymized_user_vitals_legacy_promotion_event
     WHERE legacy_log_id = p_legacy_log_id
       AND promotion_status IN ('REQUESTED', 'APPLIED')
     ORDER BY promotion_event_id DESC
     LIMIT 1
     FOR UPDATE;

    IF FOUND THEN
        IF existing_status = 'APPLIED'
           AND existing_event_reference = p_promotion_event_reference
           AND existing_classification_id = p_classification_id
           AND existing_provenance = p_authoritative_provenance
           AND legacy_tenant IS NOT NULL
           AND legacy_domain IS NOT NULL THEN
            RETURN existing_event_id;
        END IF;

        RAISE EXCEPTION 'legacy record % already has an active promotion event', p_legacy_log_id;
    END IF;

    -- A row that is already scoped without an authoritative promotion event is
    -- an invariant violation; never silently reinterpret it as a promotion.
    IF legacy_tenant IS NOT NULL OR legacy_domain IS NOT NULL THEN
        RAISE EXCEPTION 'legacy record % is already scoped without a matching promotion event', p_legacy_log_id;
    END IF;

    SELECT classification_state,
           candidate_tenant_id,
           candidate_data_domain,
           provenance_reference,
           verification_status
      INTO classification_state,
           candidate_tenant,
           candidate_domain,
           classification_provenance,
           verification_status
      FROM public.anonymized_user_vitals_legacy_classification
     WHERE classification_id = p_classification_id
       AND legacy_log_id = p_legacy_log_id
     FOR UPDATE;

    IF NOT FOUND THEN
        RAISE EXCEPTION 'classification % is not attached to legacy record %', p_classification_id, p_legacy_log_id;
    END IF;

    IF classification_state <> 'SCOPED_VERIFIED'
       OR verification_status <> 'VERIFIED'
       OR candidate_tenant IS NULL
       OR candidate_domain IS NULL
       OR classification_provenance IS NULL THEN
        RAISE EXCEPTION 'classification % is not eligible for protected promotion', p_classification_id;
    END IF;

    IF classification_provenance <> p_authoritative_provenance THEN
        RAISE EXCEPTION 'authoritative provenance does not match classification evidence';
    END IF;

    -- Scope comes only from the verified classification selected above.
    UPDATE public.anonymized_user_vitals
       SET tenant_id = candidate_tenant,
           data_domain = candidate_domain
     WHERE log_id = p_legacy_log_id
       AND tenant_id IS NULL
       AND data_domain IS NULL;

    IF NOT FOUND THEN
        RAISE EXCEPTION 'legacy record % could not be promoted from NULL scope', p_legacy_log_id;
    END IF;

    INSERT INTO public.anonymized_user_vitals_legacy_promotion_event (
        promotion_event_reference,
        legacy_log_id,
        classification_id,
        provenance_reference,
        tenant_id,
        data_domain,
        promotion_status,
        initiated_by,
        applied_at
    ) VALUES (
        p_promotion_event_reference,
        p_legacy_log_id,
        p_classification_id,
        classification_provenance,
        candidate_tenant,
        candidate_domain,
        'APPLIED',
        p_initiated_by,
        CURRENT_TIMESTAMP
    )
    RETURNING promotion_event_id INTO applied_event_id;

    RETURN applied_event_id;
END
$$;

ALTER FUNCTION public.soma_apply_legacy_promotion(INTEGER, BIGINT, TEXT, TEXT, TEXT)
    OWNER TO somaos_legacy_promotion_owner;

REVOKE ALL ON FUNCTION public.soma_apply_legacy_promotion(INTEGER, BIGINT, TEXT, TEXT, TEXT) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION public.soma_apply_legacy_promotion(INTEGER, BIGINT, TEXT, TEXT, TEXT)
    TO somaos_persistence;

COMMENT ON FUNCTION public.soma_apply_legacy_promotion(INTEGER, BIGINT, TEXT, TEXT, TEXT) IS
    'Atomic legacy promotion executor. Invoked only by the trusted persistence role; derives protected scope from an explicitly verified classification and records an immutable applied event.';
