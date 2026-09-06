# Database Constraints, Trusted DB Context, and RLS v1

**Status:** Design gate — not implementation
**Baseline:** `main` after PostgreSQL protected-data isolation v1
**Purpose:** Define the security boundary required before database-level enforcement is enabled.

## 1. Decision

SOMA will use PostgreSQL constraints and Row-Level Security (RLS) as **defense in depth**, not as the primary authorization authority.

The canonical authorization authority remains:

`IdentityContext → Identity Authorization Enforcement → Policy Kernel`

The protected persistence path remains:

`ProtectedDataAccess → trusted DB context → PostgreSQL`

RLS must never be treated as a substitute for application authorization.

## 2. Security problem

The current protected transaction binds `tenant_id` and `data_domain` with transaction-local PostgreSQL configuration. Transaction-local scope prevents connection-pool leakage, but transaction-local state is not inherently trustworthy.

If an untrusted database principal can directly connect and set the same GUC values, an RLS policy that trusts those values could be spoofed.

Therefore database-level enforcement requires a **trusted DB service identity boundary** before RLS is enabled.

## 3. Trusted DB context contract

Only an approved SOMA persistence service identity may establish protected transaction context.

Required properties:

- application authentication and authorization occur before persistence;
- the database connection uses a service identity not available to arbitrary end users;
- protected context is derived from the verified authorization boundary, never caller metadata;
- tenant and data-domain values are bound transaction-locally;
- partial, empty, malformed, or conflicting scope fails closed;
- transaction rollback clears the effective context;
- pooled connections cannot retain prior request scope;
- background jobs and service-to-service callers require their own verified identity path.

The database role itself is not an application principal. Database service identity and end-user identity remain separate concepts.

## 4. Scope invariant

For protected rows:

`tenant_id IS NOT NULL AND data_domain IS NOT NULL`

A row with either dimension missing is not protected-access eligible.

The database must eventually enforce this invariant for the protected table after legacy classification/quarantine is complete.

No default tenant or data domain may be introduced to make historical data appear classified.

## 5. Legacy data

Legacy rows currently carrying NULL scope remain transition data.

They must not become accessible through protected paths merely because:

- a hash matches;
- transport metadata supplies a tenant;
- a caller supplies a tenant;
- an AI/model infers a tenant;
- a search result suggests a tenant;
- a narrower resource scope is assumed to imply a broader scope.

Only the explicit, verified legacy-classification/promotion contract may establish protected eligibility.

## 6. Constraints

After legacy disposition is evidenced, the implementation should establish:

1. non-null tenant scope for protected rows;
2. non-null data-domain scope for protected rows;
3. a check that scope dimensions are jointly present;
4. scoped uniqueness of `(tenant_id, data_domain, anonymized_user_hash)`;
5. appropriate foreign-key/reference constraints where canonical tenant/domain registries exist;
6. constraints that prevent invalid scope values such as empty strings where appropriate.

Constraint design must be compatible with migration/backfill and must not silently rewrite legacy provenance.

## 7. RLS design

RLS is a database defense-in-depth layer.

The intended policy shape is conceptually:

`row.tenant_id = trusted_transaction_tenant AND row.data_domain = trusted_transaction_domain`

The exact implementation must additionally establish:

- which database role may set/read the trusted context;
- whether SECURITY DEFINER functions are required;
- how service roles are prevented from becoming an end-user bypass;
- whether table owners or migration roles bypass RLS and how that is controlled;
- how administrative and operational access is audited;
- how NULL/invalid context behaves;
- how INSERT, SELECT, UPDATE, and DELETE policies differ.

Missing or invalid trusted context must result in denial, not broad access.

## 8. Direct SQL and bypass resistance

The bypass audit establishes the repository-level inventory boundary. Runtime enforcement must additionally test:

- direct SQL through the application connection;
- SQL attempted without protected context;
- wrong tenant;
- wrong data domain;
- cross-tenant same hash;
- cross-domain same hash;
- partial context;
- forged caller metadata;
- connection-pool reuse;
- concurrent transactions;
- rollback after context establishment;
- background/service identity access;
- administrative access paths.

A clean static audit does not prove runtime isolation.

## 9. Transaction and Unit-of-Work boundary

Protected writes that modify multiple related records must eventually use a provider-neutral Unit-of-Work/transaction contract.

The transaction must encompass:

`authorization decision → protected context binding → persistence operations → commit`

Authorization must not be inferred from transaction success, and a database adapter must not independently grant access.

## 10. Authentication dependency

RLS cannot close the entire security chain while authentication/session/token trust remains undefined.

Before product-ready protected persistence, SOMA must establish:

- authenticated principal provenance;
- credential/session lifecycle;
- expiry and revocation;
- service-to-service identity;
- background-job identity;
- trusted context derivation;
- auditability of privileged operations.

## 11. Required acceptance tests

The implementation gate is not complete until all of the following are demonstrated:

### Positive

- authorized same-tenant/same-domain read succeeds;
- authorized scoped insert succeeds;
- transaction-local context survives the intended transaction only;
- concurrent requests remain isolated.

### Negative

- wrong tenant denied;
- wrong data domain denied;
- missing tenant denied;
- missing data domain denied;
- forged caller scope denied;
- unverified identity denied;
- expired/revoked identity denied;
- legacy NULL-scope row inaccessible through protected paths;
- direct SQL without trusted context denied by database defense-in-depth;
- connection-pool reuse cannot leak scope;
- rollback cannot leak scope;
- cross-tenant identical hashes cannot be used for existence inference through protected queries.

### Integrity

- scoped uniqueness is enforced;
- invalid/partial scope cannot be committed;
- migration/backfill preserves provenance;
- policy authority remains outside the database adapter;
- RLS bypass by table-owner/admin roles is explicitly governed and audited.

## 12. Rollout order

The recommended sequence is:

1. Merge and preserve the bypass-audit and legacy-classification gates.
2. Inventory actual production/legacy rows.
3. Classify/quarantine legacy rows using verified provenance.
4. Establish trusted database service identity and credential boundary.
5. Introduce final scope constraints using a migration-safe strategy.
6. Enable RLS in a controlled enforcement phase.
7. Add direct-SQL and adversarial integration tests.
8. Verify backup/restore, migration, rollback, and operational access.
9. Update product-readiness only from evidence.

## 13. Explicit non-goals

This design does not:

- replace the Policy Kernel;
- infer identity from database state;
- make RLS the authorization authority;
- classify historical records automatically;
- enable production RLS now;
- define end-user authentication providers;
- define delegation;
- declare protected persistence product-ready.

## 14. Gate status

**Design status: proposed.**

Implementation must not begin with RLS policies alone. The trusted database identity/context boundary and legacy disposition are prerequisites to safe database-level enforcement.
