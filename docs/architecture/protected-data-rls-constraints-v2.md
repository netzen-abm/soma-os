# Protected Data PostgreSQL RLS + Constraints v2

**Status:** Implementation in progress on `security/protected-data-rls-constraints-v2`

## Purpose

This phase adds PostgreSQL defense-in-depth after the trusted persistence DB service identity boundary established in migrations 0006 / PR #47.

The canonical authorization chain remains:

```text
IdentityContext
  → Identity Authorization Enforcement
  → Policy Kernel
  → ProtectedDataAccess
  → trusted persistence DB identity
  → transaction-local protected scope
  → PostgreSQL RLS + constraints
```

PostgreSQL is not the policy authority. RLS exists to contain implementation mistakes, direct-query mistakes, and cross-scope access attempts at the storage boundary.

## Protected table

Current protected table:

`anonymized_user_vitals`

Required security scope:

- `tenant_id`
- `data_domain`

A row is protected-access eligible only when both dimensions are explicitly present and match the trusted transaction context.

## RLS model

Migration `0007_protected_data_rls_constraints.sql`:

1. enables row-level security;
2. forces RLS for the table owner so ownership does not silently bypass policy;
3. creates one `FOR ALL` policy for `somaos_persistence`;
4. requires both transaction-local scope values to equal the row scope;
5. explicitly rejects NULL scope in the policy;
6. applies the same invariant through `WITH CHECK` for inserts and updates.

The policy uses `current_setting(..., true)`. Missing settings therefore evaluate to NULL and cannot satisfy the equality predicates. This is fail-closed behavior.

The transaction-local GUC is **not** an authentication mechanism. Migration 0006 established the database-role privilege boundary that controls who can establish the trusted context.

## Constraint strategy

Existing legacy rows may still contain NULL scope. Therefore this phase does **not** blindly execute `SET NOT NULL` against a live legacy dataset.

Instead, it adds:

```text
CHECK (tenant_id IS NOT NULL AND data_domain IS NOT NULL) NOT VALID
```

This protects all new and modified rows immediately while allowing existing legacy rows to remain untouched until their provenance is verified.

Final migration to validated constraints / `NOT NULL` is a separate gate after legacy classification and explicit promotion are complete.

## Legacy safety

The following remain true:

- NULL-scope legacy rows are not visible through the protected persistence role.
- Scope cannot be inferred from anonymized hash, transport, caller metadata, model output, or resource type.
- Legacy classification records do not themselves promote a row.
- Promotion requires explicit verified classification and provenance.

## Adversarial verification

The integration suite verifies:

- RLS filters a query that omits application tenant predicates;
- missing scope returns no protected rows;
- wrong tenant/domain cannot mutate another scope;
- missing scope on insert is rejected;
- mismatched scope on insert is rejected;
- new NULL-scope rows are rejected by the write constraint;
- existing legacy NULL-scope rows remain inaccessible;
- prior pool-isolation, rollback, concurrency, role-boundary and scoped-uniqueness tests continue to run.

## Deployment implications

Production application traffic must use the dedicated persistence DB identity introduced by migration 0006. The normal application credential must not be the migration/owner identity.

No credentials are committed by this phase.

Migrations must be executed by the separately controlled migration identity. The persistence identity must remain non-owner and must not have DDL or administrative privileges.

## Rollback / recovery posture

RLS and constraints are security controls. A rollback must never silently remove isolation from an already protected deployment.

Operational rollback therefore requires an explicit reviewed migration plan. The preferred recovery action for application failures is to repair the application/persistence path while retaining RLS, rather than disabling RLS as an emergency shortcut.

## Non-goals

- Policy Kernel redesign.
- Authentication/session/token lifecycle.
- Production credential rotation.
- Moving authorization policy into PostgreSQL.
- Automatic legacy-data promotion.
- Surface-specific product features.

## Exit criteria

This phase is complete only after implementation, clean-database migration verification, adversarial PostgreSQL tests, bypass audit, full CI, deployment review, and documentation all pass. A green unit test alone is insufficient.
