#!/usr/bin/env bash
set -euo pipefail

: "${SOMA_TEST_DATABASE_URL:?SOMA_TEST_DATABASE_URL must be set}"

psql_cmd=(psql "$SOMA_TEST_DATABASE_URL" -v ON_ERROR_STOP=1 -X)

# The PostgreSQL integration-test job intentionally exercises the same service
# database before this script runs. Start from a fresh public schema so this
# verification is a genuine replay of the repository migration sequence, not
# an accidental continuation of the earlier integration-test state. This
# script is test-only and the CI workflow points SOMA_TEST_DATABASE_URL at the
# dedicated ephemeral somaos_test service database.
"${psql_cmd[@]}" <<'SQL'
DROP SCHEMA public CASCADE;
CREATE SCHEMA public;
GRANT ALL ON SCHEMA public TO public;
SQL

# Replay every repository migration in numeric order. The final-gate assertion
# depends on the protected-data transition, but including 0002 ensures this
# harness does not silently test a partial migration history.
for migration in \
  database/migrations/0001_initialize_zk_logs.sql \
  database/migrations/0002_initialize_evidence_registry.sql \
  database/migrations/0003_protected_data_scope_transition.sql \
  database/migrations/0004_scoped_anonymized_hash.sql \
  database/migrations/0005_legacy_data_classification.sql \
  database/migrations/0006_trusted_db_service_identity.sql; do
  "${psql_cmd[@]}" -f "$migration"
done

# Seed adversarial legacy state before RLS/NOT VALID constraints are introduced.
# This models the real migration boundary: legacy NULL/partial rows may exist,
# while the later final gate must refuse to tighten the schema around them.
"${psql_cmd[@]}" <<'SQL'
INSERT INTO public.anonymized_user_vitals
    (anonymized_user_hash, public_verification_key_hex, verified_vitality_score,
     salud_schema_version, signature_proof_hex)
VALUES ('final-gate-legacy-null', 'pk', 1, '1.0', 'sig');

INSERT INTO public.anonymized_user_vitals
    (tenant_id, anonymized_user_hash, public_verification_key_hex,
     verified_vitality_score, salud_schema_version, signature_proof_hex)
VALUES ('partial-tenant', 'final-gate-partial', 'pk', 1, '1.0', 'sig');
SQL

for migration in \
  database/migrations/0007_protected_data_rls_constraints.sql \
  database/migrations/0008_legacy_data_promotion_events.sql \
  database/migrations/0009_legacy_promotion_executor.sql \
  database/migrations/0010_legacy_promotion_preflight.sql; do
  "${psql_cmd[@]}" -f "$migration"
done

# NULL and partial scope must block the final migration. The migration itself
# raises before any final schema tightening is reached.
if "${psql_cmd[@]}" -f database/migrations/0011_final_protected_data_not_null_gate.sql; then
  echo "ERROR: final NOT NULL gate unexpectedly succeeded with NULL/partial scope" >&2
  exit 1
else
  echo "PASS: final NOT NULL gate rejects NULL/partial scope"
fi

# Remove only the NULL fixture. RLS normally prevents access to an unscoped
# legacy row, so cleanup uses a tightly-scoped test-only transaction that
# disables RLS and restores it before commit. No production path uses this.
"${psql_cmd[@]}" <<'SQL'
BEGIN;
ALTER TABLE public.anonymized_user_vitals DISABLE ROW LEVEL SECURITY;
DELETE FROM public.anonymized_user_vitals
 WHERE anonymized_user_hash = 'final-gate-legacy-null';
ALTER TABLE public.anonymized_user_vitals ENABLE ROW LEVEL SECURITY;
COMMIT;
SQL

# Partial scope must independently block finalization even after NULL rows are
# gone. The existing NOT VALID constraint permits the pre-existing fixture, and
# the canonical preflight must detect it before schema tightening.
if "${psql_cmd[@]}" -f database/migrations/0011_final_protected_data_not_null_gate.sql; then
  echo "ERROR: final NOT NULL gate unexpectedly succeeded with partial scope" >&2
  exit 1
else
  echo "PASS: final NOT NULL gate rejects partial scope"
fi

# Remove the partial fixture using the same tightly-scoped test cleanup, then
# the exact final migration must succeed. No scope is inferred or repaired.
"${psql_cmd[@]}" <<'SQL'
BEGIN;
ALTER TABLE public.anonymized_user_vitals DISABLE ROW LEVEL SECURITY;
DELETE FROM public.anonymized_user_vitals
 WHERE anonymized_user_hash = 'final-gate-partial';
ALTER TABLE public.anonymized_user_vitals ENABLE ROW LEVEL SECURITY;
COMMIT;
SQL
"${psql_cmd[@]}" -f database/migrations/0011_final_protected_data_not_null_gate.sql

# Apply the post-gate privilege hardening. The application persistence role
# must lose access to the global SECURITY DEFINER preflight; the migration role
# retains the capability needed for migration/control-plane verification.
"${psql_cmd[@]}" -f database/migrations/0013_legacy_preflight_privilege_isolation.sql

"${psql_cmd[@]}" <<'SQL'
DO $$
DECLARE
    tenant_nullable TEXT;
    domain_nullable TEXT;
    constraint_valid BOOLEAN;
    persistence_can_execute BOOLEAN;
    migrator_can_execute BOOLEAN;
BEGIN
    SELECT is_nullable INTO tenant_nullable
      FROM information_schema.columns
     WHERE table_schema = 'public'
       AND table_name = 'anonymized_user_vitals'
       AND column_name = 'tenant_id';
    SELECT is_nullable INTO domain_nullable
      FROM information_schema.columns
     WHERE table_schema = 'public'
       AND table_name = 'anonymized_user_vitals'
       AND column_name = 'data_domain';
    SELECT convalidated INTO constraint_valid
      FROM pg_constraint
     WHERE conrelid = 'public.anonymized_user_vitals'::regclass
       AND conname = 'anonymized_user_vitals_protected_scope_required';

    SELECT has_function_privilege(
        'somaos_persistence',
        'public.soma_legacy_promotion_preflight()',
        'EXECUTE'
    ) INTO persistence_can_execute;
    SELECT has_function_privilege(
        'somaos_migrator',
        'public.soma_legacy_promotion_preflight()',
        'EXECUTE'
    ) INTO migrator_can_execute;

    IF tenant_nullable <> 'NO' OR domain_nullable <> 'NO' OR NOT COALESCE(constraint_valid, FALSE) THEN
        RAISE EXCEPTION 'final protected-data schema state is incomplete';
    END IF;

    IF persistence_can_execute OR NOT migrator_can_execute THEN
        RAISE EXCEPTION 'legacy preflight privilege boundary is incorrect';
    END IF;
END
$$;

DO $$
BEGIN
    BEGIN
        INSERT INTO public.anonymized_user_vitals
            (anonymized_user_hash, public_verification_key_hex,
             verified_vitality_score, salud_schema_version, signature_proof_hex)
        VALUES ('final-gate-post-null', 'pk', 1, '1.0', 'sig');
        RAISE EXCEPTION 'NULL protected scope insert unexpectedly succeeded';
    EXCEPTION WHEN not_null_violation THEN
        NULL;
    END;
END
$$;
SQL

echo "PASS: final protected-data NOT NULL gate and preflight privilege isolation are enforced"
