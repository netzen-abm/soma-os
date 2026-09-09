# Protected Data Access Enforcement Boundary v1

**Status:** Design / review gate
**Baseline:** `main` at `3830d52bd94c49dc740589fe519b5dd0763509d1`
**Issue:** #27

## 1. Purpose

SOMA already has a canonical IdentityContext and a canonical Policy Kernel. The missing security boundary is the point where an authorized decision becomes an actual datastore operation.

The objective is to ensure that protected data cannot be reached through an alternate path that bypasses identity scope or authorization.

The canonical path is:

```text
verified authentication
  -> IdentityContext v1
  -> Identity -> Authorization Enforcement v1
  -> Policy Kernel v0.3
  -> Protected Data Access Contract
  -> provider adapter
  -> datastore
```

No database adapter, application surface, model, agent, tool, or transport may become a second authorization authority.

## 2. Current repository observations

The current Rust backend contains `SomaDatabaseManager`, which owns a `PgPool` and performs direct SQL reads/writes. The current vital-log table contains a pseudonymous user hash but no canonical tenant or data-domain columns. The evidence registry is explicitly described as source/scientific metadata rather than personal health records.

These observations mean the existing persistence layer cannot yet be treated as end-to-end tenant/data-domain enforced.

## 3. Data classification boundary

Every persisted resource must declare or be deterministically associated with:

- `tenant_id` — the isolation boundary;
- `data_domain` — the governed class/domain of data;
- `resource_id` — the concrete instance bound to the Policy Kernel request;
- classification/sensitivity where applicable;
- provenance and lifecycle metadata where required by the canonical contract.

Recommended initial classes:

| Class | Examples | Default posture |
| --- | --- | --- |
| Personal health data | health state, observations, interventions, outcomes | protected; explicit scope required |
| Evidence/scientific metadata | sources, studies, claims, assessments | shared/reference data; still integrity controlled |
| Operational data | jobs, audit records, runtime metadata | service-scoped; explicit service identity |
| Public/non-sensitive data | published public material | public-read where explicitly classified |

Classification must not be inferred from transport, endpoint, model output, or UI context.

## 4. Provider-neutral contract

The shared contract should be conceptually:

```text
ProtectedDataRequest {
    authorization_context
    capability_id
    resource_type
    resource_id
    action
    target_tenant_id
    target_data_domain
}
```

The access layer performs structural validation and target binding, invokes the existing Identity -> Authorization Enforcement boundary, and only then calls a provider adapter.

The provider adapter receives an already validated target. It does not evaluate grants and cannot broaden scope.

The contract must support at least `read`, `insert`, `update`, `delete`, and query/bulk operations. Bulk operations must be represented as bounded target sets or an explicitly authorized query scope; an arbitrary caller-supplied filter must never be treated as authorization.

## 5. Target binding

For an existing object, `resource_id` must identify the concrete resource and the datastore row must independently carry the tenant/data-domain attributes needed to verify that it belongs to the authorized target.

For creation, the target tenant and data domain must be supplied by trusted authorization context or an explicitly authorized server-side target. A caller cannot create an object in a tenant/domain merely by writing those values into the payload.

For update/delete, authorization must occur against the existing stored target before mutation.

For read/query, the datastore operation must be constrained by the same tenant/domain target rather than relying only on a pre-query application check.

## 6. PostgreSQL enforcement

PostgreSQL should provide a defense-in-depth enforcement layer for protected tables, using Row-Level Security or an equivalent database mechanism where practical.

The design should not turn PostgreSQL into SOMA's policy engine. Database enforcement is a constraint mechanism: it receives trusted, narrowly scoped session/request attributes and rejects rows outside that boundary. The Policy Kernel remains the authority for capability/resource/action decisions.

A database session must never accept tenant/domain scope directly from untrusted request payloads. The application must establish trusted transaction-local authorization attributes only after the shared authorization boundary has accepted the request.

Fail-closed expectations:

- no trusted authorization context -> no protected access;
- missing tenant/domain target -> no protected access;
- target mismatch -> no protected access;
- stale/expired/revoked identity context -> no protected access;
- Policy Kernel DENY -> no datastore operation;
- database context absent or malformed -> no protected row access.

## 7. Direct SQL and bypass analysis

`SomaDatabaseManager` is currently a direct SQL access point. It must become an adapter behind the shared access contract before protected production use.

Code review and tests must search for:

- direct `sqlx::query*` against protected tables outside the adapter;
- repository methods accepting caller-provided tenant/domain as trusted security facts;
- bulk queries without bounded target scope;
- administrative paths that bypass the same enforcement boundary;
- background jobs using ambient credentials without service identity;
- cache paths that can return protected data without authorization;
- exports/backups that expose protected rows outside their own governed boundary.

## 8. Transactions / Unit of Work

Authorization and mutation must occur in the same logical transaction boundary wherever a race could invalidate the target decision.

For mutation:

```text
begin transaction
  -> establish trusted authorization context
  -> authorize exact target/action
  -> perform constrained mutation
  -> commit
```

The implementation must not authorize one target and then permit a separately constructed SQL statement to mutate another target.

A future provider-neutral Unit-of-Work contract may be shared across PostgreSQL and other providers, but it must not introduce another authorization evaluator.

## 9. Migration strategy

Existing tables must not receive invented tenant/domain assignments merely to satisfy the new schema.

Migration should proceed in phases:

1. inventory and classify existing records;
2. add nullable/transition columns where needed;
3. establish an explicit backfill policy based on trustworthy provenance;
4. quarantine records whose scope cannot be established;
5. add constraints/RLS after backfill validation;
6. migrate callers to the shared access contract;
7. remove unsafe direct paths only after evidence of replacement;
8. preserve migration history and rollback/recovery procedures.

No silent default tenant is acceptable for personal health data.

## 10. Verification plan

Required tests include:

- authorized read of an in-scope resource;
- wrong tenant read denied;
- wrong data-domain read denied;
- wrong principal denied;
- wrong capability denied;
- wrong resource denied;
- wrong action denied;
- missing IdentityContext denied;
- unverified/expired/revoked context denied;
- caller metadata cannot broaden scope;
- insert cannot assign an unauthorized tenant/domain;
- update/delete cannot cross tenant/domain boundaries;
- bulk/query cannot escape target scope;
- direct adapter call without authorization context denied where the contract requires it;
- database/RLS denies missing or mismatched trusted context;
- transaction rollback leaves no partial protected mutation;
- service-to-service access requires service identity and explicit policy grants.

The strongest acceptance test should attempt to bypass the application authorization layer and demonstrate that the database boundary still rejects an out-of-scope protected row.

## 11. Security invariants

1. IdentityContext carries verified identity and scope; it does not grant authorization.
2. Policy Kernel v0.3 remains the sole grant evaluator.
3. Resource instance binding remains exact.
4. Data-store enforcement may strengthen but never weaken shared policy semantics.
5. Caller-controlled metadata cannot broaden security scope.
6. Missing or ambiguous security context fails closed.
7. Protected data cannot be accessed through a surface-specific bypass.
8. Existing data is never silently assigned a new security scope.
9. Evidence/scientific reference data remains distinct from personal health state.
10. Provider portability is preserved: PostgreSQL is one enforcement adapter, not the architecture itself.

## 12. Open review questions

- Which current tables are genuinely protected and which are public/reference-only?
- Which existing callers need migration from `SomaDatabaseManager`?
- Which operational tables need service identity and tenant scope?
- Which tables require RLS versus service-layer constraints?
- What is the trusted mechanism for setting transaction-local database context?
- How will connection pooling prevent authorization context leakage between requests?
- What audit event is emitted for allow/deny/data-access failure?
- What backup/export boundary is required for protected data?

These questions must be resolved from repository evidence before implementation is declared product-ready.
