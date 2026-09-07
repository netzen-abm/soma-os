# Legacy Data Promotion Preflight v1

## Purpose

The preflight is the final verification boundary before the protected table can move from transitional nullable scope to a final `NOT NULL` constraint.

It is deliberately read-only. It does not promote records, rewrite classifications, alter RLS, or change constraints.

## Canonical sequence

```text
Legacy inventory
  → classification evidence
  → explicit verified promotion
  → transactional promotion executor
  → zero-NULL preflight
  → final NOT NULL migration
```

The preflight therefore answers a narrow question:

> Is the protected table in a state where every row has complete tenant/data-domain scope and promotion evidence is internally consistent enough for the final constraint gate?

## Blocking invariants

The preflight must fail when any of these conditions is true:

1. any row has both `tenant_id` and `data_domain` NULL;
2. any row has only one of the two scope fields populated;
3. an `APPLIED` promotion event has scope different from the protected row it records;
4. the database function reports `preflight_passed = false`.

The preflight must never infer or repair scope.

## Non-blocking diagnostics

The result also reports:

- total rows;
- fully scoped rows;
- scoped rows without an `APPLIED` promotion event;
- eligible verified classifications still attached to NULL-scoped records;
- records with more than one eligible verified classification.

These fields are diagnostic. A final zero-NULL pass necessarily drives the eligible-unpromoted count to zero because eligible-unpromoted records are defined only among NULL-scoped rows.

`scoped_without_applied_event` is not itself a blocker because not every protected row must originate from legacy promotion. It is retained to support reconciliation and anomaly review.

## Database boundary

Migration `0010_legacy_promotion_preflight.sql` creates a dedicated `NOLOGIN`, `BYPASSRLS` function-owner role and a `SECURITY DEFINER` read-only SQL function:

`public.soma_legacy_promotion_preflight()`

The function uses a fixed `search_path = pg_catalog`, schema-qualified protected tables, and no mutation statements. Execution is revoked from `PUBLIC` and granted only to `somaos_persistence`.

This is a verification capability, not an authorization mechanism. Invocation must remain behind the canonical identity → authorization → trusted persistence boundary.

## Why a privileged read-only function is required

The protected table is `FORCE ROW LEVEL SECURITY`, and NULL-scoped legacy rows are intentionally inaccessible to ordinary protected reads. A normal RLS-filtered query could therefore incorrectly report zero NULL rows simply because those rows were hidden.

The preflight must inspect the complete table, including transitional NULL-scoped rows, so it uses a narrowly scoped security-definer function rather than weakening the ordinary RLS policy.

## Provider-neutral contract

The Rust adapter exposes:

- `LegacyPromotionPreflight`
- `LegacyPromotionPreflightExecutor::run()`
- `LegacyPromotionPreflight::require_pass()`

The adapter accepts no tenant, data-domain, hash, transport, model, agent, or caller scope. It is read-only and delegates verification to the canonical PostgreSQL function.

## Final NOT NULL separation

This change intentionally does **not** add `NOT NULL`.

The final migration must be a separate PR and must be executed only after an independently captured successful preflight. The final migration must itself refuse to proceed when any NULL or partial scope remains.

The preflight result is evidence for the migration decision; it does not grant authority to alter schema.

## Failure behavior

- NULL scope → fail closed; identify remaining rows for classification/promotion.
- partial scope → fail closed; repair through an explicit verified workflow, never by inference.
- event/scope mismatch → fail closed; investigate immutable promotion evidence and do not overwrite it.
- ambiguous verified classifications → fail closed for promotion; preflight remains diagnostic until the record is explicitly resolved.
- database function unavailable or malformed → fail closed; no final constraint migration.

## Security posture

The architecture remains:

```text
IdentityContext
  → Identity Authorization Enforcement
  → Policy Kernel
  → ProtectedDataAccess / approved migration capability
  → trusted persistence identity
  → preflight verification
  → final schema gate
```

The Policy Kernel remains the authorization authority. PostgreSQL RLS and constraints remain defense-in-depth. The preflight does not promote evidence, reinterpret classifications, or create authorization.
