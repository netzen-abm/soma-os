-- Legacy promotion preflight privilege isolation v1
--
-- The preflight is a migration/control-plane verification capability, not an
-- application persistence capability. The ordinary persistence role must not
-- be able to invoke the SECURITY DEFINER aggregate over protected data.
--
-- Migration 0011 has already consumed the preflight as part of the final
-- schema gate. This post-gate hardening removes the historical application
-- grant and assigns execution to the dedicated migration identity.

REVOKE ALL ON FUNCTION public.soma_legacy_promotion_preflight() FROM PUBLIC;
REVOKE ALL ON FUNCTION public.soma_legacy_promotion_preflight() FROM somaos_persistence;
GRANT EXECUTE ON FUNCTION public.soma_legacy_promotion_preflight() TO somaos_migrator;

COMMENT ON FUNCTION public.soma_legacy_promotion_preflight() IS
    'Read-only legacy promotion preflight for the migration/control-plane lifecycle. Ordinary application persistence must not invoke it.';
