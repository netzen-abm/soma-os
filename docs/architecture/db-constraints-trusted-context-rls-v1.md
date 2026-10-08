# Database Constraints, Trusted DB Context, and RLS v1

**Status:** Design gate — not implementation
**Purpose:** Define the security boundary required before database-level enforcement is enabled.

## Decision

SOMA will use PostgreSQL constraints and Row-Level Security (RLS) as defense in depth, not as the primary authorization authority.

Canonical authority remains:

`IdentityContext → Identity Authorization Enforcement → Policy Kernel`

Protected persistence remains:

`ProtectedDataAccess → trusted DB context → PostgreSQL`

RLS must never substitute for application authorization.

## Trusted DB context

Only an approved SOMA persistence service identity may establish protected transaction context.

Required properties:

- authentication and authorization occur before persistence;
- database identity is not exposed as an end-user principal;
- protected context is derived from the verified authorization boundary, never caller metadata;
- tenant and data-domain values are transaction-local;
- malformed, partial, conflicting, or missing scope fails closed;
- rollback clears effective context;
- pooled connections cannot retain prior request scope;
- background and service-to-service callers have their own verified identity path.

## Scope invariant

Protected rows require both `tenant_id` and `data_domain`. Legacy NULL-scope rows remain transition data and must not be promoted through hashes, transport metadata, caller metadata, AI inference, search results, or assumed scope.

After legacy disposition is evidenced, constraints should enforce non-null scope, jointly valid scope, scoped uniqueness, and valid references without silently rewriting provenance.

## RLS and bypass resistance

RLS is defense in depth. Missing or invalid trusted context must deny access.

Runtime acceptance testing must cover wrong tenant/domain, forged scope, direct SQL, pool reuse, concurrency, rollback, background/service identities, administrative paths, and cross-tenant existence inference.

The transaction boundary is:

`authorization decision → protected context binding → persistence operations → commit`

Authorization must not be inferred from transaction success, and a database adapter must not independently grant access.

## Rollout gate

1. Preserve bypass-audit and legacy-classification gates.
2. Inventory and quarantine legacy rows.
3. Establish trusted database service identity and credentials.
4. Introduce migration-safe scope constraints.
5. Enable RLS in controlled enforcement.
6. Add adversarial direct-SQL/integration tests.
7. Verify backup/restore, migration, rollback, and operational access.

**Gate status: proposed.** Do not enable production RLS from this design alone.
