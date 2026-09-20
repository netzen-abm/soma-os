-- Legacy promotion executor privilege isolation v1
--
-- The legacy promotion SECURITY DEFINER operation is migration/control-plane
-- infrastructure, not an application persistence capability. The ordinary
-- somaos_persistence role must therefore not be able to invoke it directly.
--
-- A dedicated NOLOGIN executor role is used as the execution capability.
-- Operational migration credentials may be granted membership in this role
-- outside source control; the application persistence credential must not be.

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_roles
        WHERE rolname = 'somaos_legacy_promotion_executor'
    ) THEN
        CREATE ROLE somaos_legacy_promotion_executor
            NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT
            NOREPLICATION NOBYPASSRLS;
    END IF;
END
$$;

ALTER ROLE somaos_legacy_promotion_executor
    NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT
    NOREPLICATION NOBYPASSRLS;

REVOKE ALL ON FUNCTION public.soma_apply_legacy_promotion(
    INTEGER, BIGINT, TEXT, TEXT, TEXT
) FROM somaos_persistence;

GRANT EXECUTE ON FUNCTION public.soma_apply_legacy_promotion(
    INTEGER, BIGINT, TEXT, TEXT, TEXT
) TO somaos_legacy_promotion_executor;

COMMENT ON ROLE somaos_legacy_promotion_executor IS
    'Dedicated control-plane execution identity for verified legacy-data promotion; must not be granted to the ordinary application persistence credential.';
