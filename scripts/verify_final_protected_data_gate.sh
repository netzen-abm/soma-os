#!/usr/bin/env bash
set -euo pipefail

: "${SOMA_TEST_DATABASE_URL:?SOMA_TEST_DATABASE_URL must be set}"

psql_cmd=(psql "$SOMA_TEST_DATABASE_URL" -v ON_ERROR_STOP=1 -X)

for migration in \
  database/migrations/0001_initialize_zk_logs.sql \
  database/migrations/0003_protected_data_scope_transition.sql \
  database/migrations/0004_scoped_anonymized_hash.sql \
  database/migrations/0005_legacy_data_classification.sql \
  database/migrations/0006_trusted_db_service_identity.sql; do
  "${psql_cmd[@]}" -f "$migration"
done

# Seed an intentionally unclassified legacy row before the protected-data
# constraint exists, matching the production migration lifecycle.
"${psql_cmd[@]}" <<'SQL'
INSERT INTO public.anonymized_user_vitals
    (anonymized_user_hash, public_verification_key_hex, verified_vitality_score,
     salud_schema_version, signature_proof_hex)
VALUES ('final-gate-legacy-null', 'pk', 1, '1.0', 'sig');
SQL

for migration in \
  database/migrations/0007_protected_data_rls_constraints.sql \
  database/migrations/0008_legacy_data_promotion_events.sql \
  database/migrations/0009_legacy_promotion_executor.sql \
  database/migrations/0010_legacy_promotion_preflight.sql; do
  "${psql_cmd[@]}" -f "$migration"
done

# NULL scope must block the final migration. The migration itself raises an
# exception before any final schema tightening is reached.
if "${psql_cmd[@]}" -f database/migrations/0011_final_protected_data_not_null_gate.sql; then
  echo "ERROR: final NOT NULL gate unexpectedly succeeded with NULL scope" >&2
  exit 1
else
  echo "PASS: final NOT NULL gate rejects remaining NULL scope"
fi

# Partial scope must also block finalization. The existing check is temporarily
# removed only inside a transaction so this adversarial fixture is rolled back.
if "${psql_cmd[@]}" <<'SQL'; then
  echo "ERROR: final NOT NULL gate unexpectedly succeeded with partial scope" >&2
  exit 1
else
  echo "PASS: final NOT NULL gate rejects partial scope"
fi
BEGIN;
ALTER TABLE public.anonymized_user_vitals
    DROP CONSTRAINT anonymized_user_vitals_protected_scope_required;
INSERT INTO public.anonymized_user_vitals
    (tenant_id, anonymized_user_hash, public_verification_key_hex,
     verified_vitality_score, salud_schema_version, signature_proof_hex)
VALUES ('partial-tenant', 'final-gate-partial', 'pk', 1, '1.0', 'sig');
\i database/migrations/0011_final_protected_data_not_null_gate.sql
COMMIT;
SQL

# Remove the intentionally NULL-scoped legacy record, then the exact same final
# migration must succeed. No scope is inferred or repaired.
"${psql_cmd[@]}" <<'SQL'
DELETE FROM public.anonymized_user_vitals
 WHERE anonymized_user_hash = 'final-gate-legacy-null';
SQL
"${psql_cmd[@]}" -f database/migrations/0011_final_protected_data_not_null_gate.sql

"${psql_cmd[@]}" <<'SQL'
DO $$
DECLARE
    tenant_nullable TEXT;
    domain_nullable TEXT;
    constraint_valid BOOLEAN;
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

    IF tenant_nullable <> 'NO' OR domain_nullable <> 'NO' OR NOT COALESCE(constraint_valid, FALSE) THEN
        RAISE EXCEPTION 'final protected-data schema state is incomplete';
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

echo "PASS: final protected-data NOT NULL gate is enforced"
