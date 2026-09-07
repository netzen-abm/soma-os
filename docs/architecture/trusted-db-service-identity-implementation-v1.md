# Trusted DB Service Identity & Transaction Context — Implementation v1

## Status

Implemented on branch `security/trusted-db-service-identity-context-v1` from current `main`.

This phase establishes the PostgreSQL privilege boundary required before final protected-data RLS and `NOT NULL` constraints. It does **not** enable final RLS or mutate legacy scope.

## Canonical security chain

```text
Verified IdentityContext
        ↓
Identity → Authorization Enforcement
        ↓
Policy Kernel
        ↓
ProtectedDataAccess
        ↓
LOGIN application role
        ↓ SET LOCAL ROLE
Dedicated PostgreSQL persistence role
        ↓
Trusted transaction-local context
        ↓
PostgreSQL protected-data enforcement
```

The DB role is the trust boundary. The transaction-local GUCs are isolation state only.

## Implemented boundary

### 1. Dedicated roles

Migration `0006_trusted_db_service_identity.sql` establishes:

- `somaos_persistence` — `NOLOGIN`, non-superuser, no database creation, no role creation, no replication, no RLS bypass.
- `somaos_migrator` — separate `NOLOGIN` migration/ownership identity.
- `somaos_untrusted_test` — non-login adversarial test principal with no protected privileges.

No password or production credential is stored in the repository.

### 2. Trusted context entry point

`public.soma_set_protected_scope(text, text)` is `SECURITY INVOKER` and executable only by `somaos_persistence`.

The function:

- rejects non-persistence callers;
- rejects empty scope;
- rejects control characters;
- establishes `soma.tenant_id` and `soma.data_domain` with transaction-local `set_config(..., true)`.

It deliberately does not perform application authorization. Authorization remains upstream in the canonical policy chain.

### 3. Rust persistence adapter

`ProtectedDbContext::begin_protected_transaction` now enters `somaos_persistence` with `SET LOCAL ROLE` before invoking the canonical SQL function. This matches PostgreSQL's `NOLOGIN` design: the application connection uses a separately provisioned LOGIN role that is explicitly granted membership in `somaos_persistence`.

The adapter does not create, grant, alter, or otherwise provision roles at runtime.

## Required production role provisioning

A deployment must create a dedicated LOGIN role for the application service and grant it membership in the non-login persistence role. Conceptually:

```sql
CREATE ROLE somaos_app LOGIN PASSWORD '<managed-secret>' NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS;
GRANT somaos_persistence TO somaos_app;
```

The actual credential must be generated and stored only in the deployment secret manager. The placeholder above is documentation only and must never be copied as a real password.

The application LOGIN role must not own protected tables, must not be a superuser, must not have `BYPASSRLS`, and must not retain migration/admin privileges. The migration/owner identity remains separate.

Because `somaos_persistence` is `NOINHERIT`, the adapter's explicit `SET LOCAL ROLE somaos_persistence` is required for the protected transaction. A connection that cannot assume this role must fail closed rather than falling back to direct table access.

## Important security limitation — explicitly accepted

A database principal that is already the trusted `somaos_persistence` identity can issue arbitrary SQL with that identity. PostgreSQL cannot cryptographically distinguish approved application code from arbitrary SQL issued under the same role.

Therefore:

1. untrusted application/client identities must never receive the persistence DB credential;
2. the application LOGIN role must only have the minimum role membership required to assume `somaos_persistence`;
3. untrusted roles must not receive protected table privileges or function `EXECUTE`;
4. migration/admin identities remain separate;
5. RLS remains defense-in-depth and is required before production protected-data enforcement is considered complete.

The design intentionally does **not** claim that a GUC is trusted merely because it is transaction-local.

## Adversarial verification

The PostgreSQL integration suite must verify:

- an untrusted DB role cannot establish trusted context;
- the provisioned application LOGIN role can assume `somaos_persistence` but cannot directly exercise protected operations before role elevation;
- trusted persistence role can establish context;
- transaction-local scope is cleared after completion;
- concurrent transactions cannot cross-contaminate tenant/domain state;
- scoped hash uniqueness remains isolated;
- legacy NULL-scope rows remain excluded from scoped queries;
- direct protected-table access without the trusted role/context is denied after RLS enforcement.

The tests use the PostgreSQL service database and deliberately non-production principals. Production credentials are never committed.

## Deployment requirement before RLS

The production service must connect using a dedicated LOGIN credential that is explicitly mapped to `somaos_persistence`, while migrations/DDL execute under the separately controlled migration/owner identity. The existing administrative database credential must not remain the normal application credential once the protected-data enforcement cutover occurs.

This implementation establishes the mechanism but does not mark the production deployment migration complete until role provisioning and the actual service path are verified.

## Next gate

The next security gate is **protected-data enforcement v2**:

1. prove the real deployment role/credential separation in deployment configuration;
2. verify `SET LOCAL ROLE somaos_persistence` works for the provisioned application identity;
3. enable PostgreSQL RLS using trusted transaction context plus role checks;
4. enforce final protected-row constraints only after legacy classification/quarantine is verified;
5. add end-to-end negative tests through the actual persistence service path.
