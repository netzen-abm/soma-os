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
Dedicated PostgreSQL persistence identity
        ↓
Trusted transaction-local context
        ↓
PostgreSQL protected-data enforcement (future RLS)
```

The DB role is the trust boundary. The transaction-local GUCs are isolation state only.

## Implemented boundary

### 1. Dedicated roles

Migration `0006_trusted_db_service_identity.sql` establishes:

- `somaos_persistence` — `NOLOGIN`, non-superuser, no database creation, no role creation, no replication, no RLS bypass.
- `somaos_migrator` — separate `NOLOGIN` migration/ownership identity.
- `somaos_untrusted_test` — non-login adversarial test principal with no protected privileges.

No password or production credential is stored in the repository. Deployment must bind the actual persistence credential through the secret/configuration system.

### 2. Trusted context entry point

`public.soma_set_protected_scope(text, text)` is `SECURITY INVOKER` and executable only by `somaos_persistence`.

The function:

- rejects non-persistence callers;
- rejects empty scope;
- rejects control characters;
- establishes `soma.tenant_id` and `soma.data_domain` with transaction-local `set_config(..., true)`.

It deliberately does not perform application authorization. Authorization remains upstream in the canonical policy chain.

### 3. Rust persistence adapter

`ProtectedDbContext::begin_protected_transaction` now calls the canonical SQL function instead of issuing raw `set_config` statements.

This keeps the persistence implementation provider-aware while preserving a provider-neutral protected-data contract above it.

## Important security limitation — explicitly accepted

A database principal that is already the trusted `somaos_persistence` identity can issue arbitrary SQL with that identity. PostgreSQL cannot cryptographically distinguish approved application code from arbitrary SQL issued under the same role.

Therefore:

1. untrusted application/client identities must never receive the persistence DB credential;
2. untrusted roles must not receive protected table privileges or function `EXECUTE`;
3. migration/admin identities remain separate;
4. RLS remains defense-in-depth and is still required before production protected-data enforcement is considered complete.

The design intentionally does **not** claim that a GUC is trusted merely because it is transaction-local.

## Adversarial verification

The PostgreSQL integration suite verifies:

- untrusted DB role cannot establish trusted context;
- trusted persistence role can establish context;
- transaction-local scope is cleared after completion;
- concurrent transactions cannot cross-contaminate tenant/domain state;
- scoped hash uniqueness remains isolated;
- legacy NULL-scope rows remain excluded from scoped queries.

The tests use the PostgreSQL service database and a deliberately non-login adversarial role. Production credentials are not committed.

## Deployment requirement before RLS

The production service must connect using a credential mapped to `somaos_persistence`, while migrations/DDL execute under the separately controlled migration/owner identity. The existing administrative database credential must not remain the normal application credential once the protected-data enforcement cutover occurs.

This implementation therefore establishes the security mechanism but does not mark the deployment migration complete.

## Next gate

The next security gate is **protected-data enforcement v2**:

1. prove the real deployment role/credential separation in the deployment configuration;
2. enable PostgreSQL RLS using trusted transaction context plus role checks;
3. enforce final protected-row constraints only after legacy classification/quarantine is verified;
4. add end-to-end negative tests through the actual persistence service path.
