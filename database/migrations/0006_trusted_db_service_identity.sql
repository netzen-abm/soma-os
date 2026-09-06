-- Trusted DB service identity and transaction context v1
--
-- This migration establishes a real PostgreSQL privilege boundary around the
-- protected transaction context. It deliberately does not enable RLS or final
-- NOT NULL constraints yet.
--
-- The persistence role is a dedicated, non-login role. Deployment is expected
-- to bind the actual application database credential to this role through the
-- environment/secret manager. No credential is stored in source control.
--
-- PostgreSQL role privileges are the trust boundary. Transaction-local GUCs
-- provide isolation only; they are never treated as authentication by
-- themselves.

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'somaos_persistence') THEN
        CREATE ROLE somaos_persistence NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT NOREPLICATION NOBYPASSRLS;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'somaos_migrator') THEN
        CREATE ROLE somaos_migrator NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT NOREPLICATION NOBYPASSRLS;
    END IF;
END
$$;

-- The persistence role must never become an owner or gain DDL/admin authority.
ALTER ROLE somaos_persistence NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT NOREPLICATION NOBYPASSRLS;
ALTER ROLE somaos_migrator NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT NOREPLICATION NOBYPASSRLS;

-- Function is SECURITY INVOKER by design. It cannot manufacture authority for
-- callers: only the dedicated persistence role receives EXECUTE privilege.
CREATE OR REPLACE FUNCTION public.soma_set_protected_scope(p_tenant_id text, p_data_domain text)
RETURNS void
LANGUAGE plpgsql
SECURITY INVOKER
SET search_path = pg_catalog
AS $$
BEGIN
    IF current_user <> 'somaos_persistence' THEN
        RAISE EXCEPTION 'trusted protected scope may only be established by somaos_persistence';
    END IF;

    IF p_tenant_id IS NULL OR btrim(p_tenant_id) = ''
       OR p_data_domain IS NULL OR btrim(p_data_domain) = '' THEN
        RAISE EXCEPTION 'trusted protected scope must be non-empty';
    END IF;

    IF p_tenant_id ~ '[[:cntrl:]]' OR p_data_domain ~ '[[:cntrl:]]' THEN
        RAISE EXCEPTION 'trusted protected scope contains control characters';
    END IF;

    PERFORM pg_catalog.set_config('soma.tenant_id', p_tenant_id, true);
    PERFORM pg_catalog.set_config('soma.data_domain', p_data_domain, true);
END
$$;

REVOKE ALL ON FUNCTION public.soma_set_protected_scope(text, text) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION public.soma_set_protected_scope(text, text) TO somaos_persistence;

COMMENT ON FUNCTION public.soma_set_protected_scope(text, text) IS
    'Trusted transaction-local protected scope entry point. Authorization must occur before invocation; the DB role is the privilege boundary.';

-- Phase-1 protected table privileges. Ownership and migration privileges remain
-- separate. Existing deployments should explicitly grant these privileges to
-- the persistence role as part of the credential cutover.
GRANT SELECT, INSERT, UPDATE, DELETE ON TABLE anonymized_user_vitals TO somaos_persistence;
GRANT USAGE, SELECT ON SEQUENCE anonymized_user_vitals_log_id_seq TO somaos_persistence;
