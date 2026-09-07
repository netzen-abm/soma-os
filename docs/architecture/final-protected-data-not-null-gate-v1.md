# Final Protected-Data NOT NULL Gate v1

## Purpose

This migration is the final schema-tightening step for protected tenant/data-domain scope. It is intentionally separate from the zero-NULL preflight and from legacy-data promotion.

## Canonical lifecycle

`Legacy inventory → classification evidence → explicit verified promotion → zero-NULL preflight → final NOT NULL schema gate`

## Security boundary

The migration invokes the existing read-only `soma_legacy_promotion_preflight()` before changing the schema. The preflight must report `preflight_passed = true`; otherwise the migration raises an exception and stops.

The migration does not infer, repair, classify, promote, or delete any legacy data.

## Finalization

After a successful preflight:

1. The existing `anonymized_user_vitals_protected_scope_required` check is validated if it is still `NOT VALID`.
2. `tenant_id` is made `NOT NULL`.
3. `data_domain` is made `NOT NULL`.

PostgreSQL transactional DDL means failure of the gate rolls back the schema changes as one migration transaction.

## Invariants

- No NULL or partial protected scope may survive finalization.
- Existing RLS remains authoritative as defense-in-depth.
- Policy Kernel remains the application authorization authority.
- Scope is not inferred from hash, transport, model, agent, caller metadata, or application claims.
- Classification and promotion history remain intact.
- Normal persistence identity remains distinct from migration/owner authority.

## Verification requirement

The migration must be exercised against both a realistic zero-NULL state and adversarial NULL/partial-scope states. The latter must fail before final schema tightening. After successful finalization, inserts and updates attempting to create NULL protected scope must fail at the database boundary.
