# Protected-Data / Authorization Convergence Audit — 2026-09-13

## Baseline

Audit baseline: `cd6981e39d0f1d22642014f2263ba3aa81266b38` (`main`).

This audit covers executable Rust/Python code, database migrations, protected-data scripts, and authorization-related contracts found in the baseline tree. It deliberately searches for every authorization implementation and every direct protected-data path rather than treating `VaultAuthorizer` as the whole problem.

## Executive result

The repository is substantially converged at the policy-decision layer, but **protected persistence is not yet structurally bound to a canonical authorization proof at the Rust database boundary**.

The highest-value remaining architectural gap is:

> `ProtectedDbContext` is a caller-constructible scope carrier, while `db_layer.rs` accepts it as sufficient input to protected SQL execution. The context is documented as being produced after canonical authorization, but the type system does not enforce that provenance and there is no production Rust wiring that mints the context from the canonical authorization boundary.

This is more important than starting Health State implementation.

## Authorization implementations found

### 1. Canonical Python authorization boundary

`services/shared/authorization_policy_decision_boundary.py` is the canonical policy-decision composition boundary. `ProtectedDataAccess` delegates authorization to it and only reaches adapters on `ALLOW`.

### 2. Identity authorization enforcement

`services/shared/identity_authorization_enforcement.py` composes identity facts with policy evaluation and fails closed on invalid identity/scope conditions.

### 3. Rust canonical authorization contract

`services/backend-rust/src/canonical_authorization.rs` defines the Rust-side adapter contract. It is intentionally a decision consumer, not a second policy engine.

### 4. Local Vault canonical adapter

`services/backend-rust/src/canonical_vault_authorizer.rs` translates vault operations into the Rust canonical authorization request and accepts only `AuthorizationDecision::Allow`.

### 5. Legacy `VaultAuthorizer`

`services/backend-rust/src/local_health_vault_storage.rs` still contains the `VaultAuthorizer` compatibility trait. Search shows the production adapter implements it, while the concrete `SubjectAuthorizer` implementations are test-only. This is therefore a compatibility boundary, not evidence of a second production policy engine.

### 6. Test-only authorization implementations

`personal_health_record_repository.rs` and `local_health_vault_storage.rs` contain `SubjectAuthorizer` test doubles. They must remain test-only and must not become production constructors/wiring.

## Protected-data paths found

### A. Local encrypted Health Vault

`LocalFileVaultStore` is the main protected local file path. `put`, `get`, `list`, `tombstone`, and `verify` all invoke the authorizer before protected work. Listing additionally re-checks per-record read authorization before returning entries.

This path is therefore **converged at the adapter boundary**, subject to removal of the legacy compatibility interface after real production wiring exists.

### B. PostgreSQL protected vital data

`services/backend-rust/src/db_layer.rs` directly reads/writes `anonymized_user_vitals` using SQL. It correctly requires a `ProtectedDbContext`, starts a transaction, sets the trusted persistence role, binds transaction-local tenant/data-domain scope, and also includes explicit scope predicates on reads.

However, the Rust method does **not itself receive a canonical authorization decision/proof**. It trusts the caller to supply a valid `ProtectedDbContext`.

### C. PostgreSQL legacy promotion/preflight

`legacy_promotion.rs`, `legacy_promotion_preflight.rs`, and migrations `0008`–`0011` use dedicated SQL boundaries, separate roles, explicit provenance/classification, preflight, RLS and the final NOT NULL gate. These are privileged migration/control-plane paths and are not ordinary caller-facing data access.

They remain governed by their dedicated lifecycle and must not be generalized into application authorization.

### D. Longitudinal observation repository

`longitudinal_observation_repository.rs` is currently a provider-neutral contract only. It does not itself persist protected payloads and does not introduce another production authorization implementation.

### E. PHR repository

`personal_health_record_repository.rs` is a facade over the Local Health Vault and delegates protected operations to the vault. It does not introduce a second storage source of truth.

## Important negative findings

- No additional production `VaultAuthorizer` implementation was found beyond the canonical adapter.
- No second production Policy Kernel was found.
- No direct PostgreSQL access to `anonymized_user_vitals` was found outside the expected persistence/migration/test paths.
- No production `ProtectedDbContext::new(...)` caller was found; its current constructor is only exercised by its own tests.
- No production `ProtectedDbContext { ... }` literal was found outside the type definition.
- No longitudinal-health storage implementation was found that bypasses the existing protected vault boundary.

## Highest-value gap

The critical issue is not a duplicate policy engine. It is **authorization-to-persistence provenance**.

Current conceptual flow:

```text
Canonical authorization
        ↓
caller is expected to obtain a trusted scope
        ↓
ProtectedDbContext
        ↓
SET LOCAL ROLE + protected scope
        ↓
PostgreSQL RLS / protected SQL
```

The missing structural guarantee is:

```text
ALLOW decision
        ↓
non-forgeable authorization-bound execution context
        ↓
ProtectedDbContext
        ↓
protected persistence
```

A plain `ProtectedDbContext { tenant_id, data_domain }` is a scope value, not proof that the scope was authorized for the requested principal/capability/resource/action.

## Required next architectural target

Implement a **canonical authorization-bound protected execution context** (name to be finalized) that:

1. can only be issued through the canonical authorization boundary;
2. binds the principal, capability, action, resource, tenant and data-domain that were authorized;
3. cannot be created merely from caller-supplied tenant/data-domain strings;
4. is the only context accepted by protected PostgreSQL persistence APIs;
5. preserves fail-closed semantics for every non-`ALLOW` decision;
6. is compatible with the existing `Governed Capability Operation` contract rather than creating another operation model;
7. supports later Rust/AI/agent/MCP/Web3 adapters without giving those adapters direct protected-data authority.

## Explicitly deferred

Do **not** start Health State implementation as the next task.

Do **not** create another authorization engine.

Do **not** create another operation envelope; `schemas/governed-capability-operation-v1.json` already exists and is the canonical operation contract.

Do **not** remove the legacy `VaultAuthorizer` trait until actual production vault provider wiring exists and the canonical adapter is exercised end-to-end.

## Acceptance gates for the next implementation

- forged tenant/data-domain context cannot reach protected SQL;
- an `ALLOW` decision for resource A cannot be replayed for resource B;
- an `ALLOW` decision for action READ cannot be reused for WRITE/TOMBSTONE;
- principal/capability/resource/action/tenant/data-domain remain bound;
- `DENY`, `REQUIRE_CONSENT`, `REQUIRE_HUMAN_REVIEW`, and `DEGRADE` cannot mint protected execution context;
- transaction-local PostgreSQL scope remains enforced;
- RLS and explicit SQL predicates remain defense in depth;
- test-only authorizers remain test-only;
- exact-head CI is green before merge.
