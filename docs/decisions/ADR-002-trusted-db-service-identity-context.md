# ADR-002: Trusted DB Service Identity and Transaction Context

**Status:** Proposed
**Date:** 2026-09-06
**Decision scope:** Protected PostgreSQL persistence

## Context

SOMA currently binds protected tenant/data-domain scope transaction-locally. This prevents connection-pool leakage but does not, by itself, establish that the database context is authoritative. An application principal able to connect directly to PostgreSQL must not be able to self-assert the same scope and thereby defeat future RLS policies.

## Decision

SOMA will establish a distinct trusted database service identity boundary before enabling final protected-data constraints or RLS.

The canonical chain is:

`Authenticated Principal → IdentityContext → Policy Kernel → ProtectedDataAccess → Trusted DB Service Identity → PostgreSQL`

The database role is not an end-user identity. Tenant/data-domain context is derived from the verified authorization boundary and bound to the transaction through an approved persistence mechanism.

## Consequences

### Positive

- Prevents database context from becoming a caller-controlled authorization input.
- Preserves the Policy Kernel as the authorization authority.
- Provides a clean boundary for RLS defense in depth.
- Separates application, migration/owner, and operational privileges.
- Makes background and service-to-service access explicit rather than implicit.
- Provides a testable boundary for direct-SQL and adversarial verification.

### Costs

- Requires explicit PostgreSQL role/privilege provisioning.
- Requires a trusted context mechanism, likely a narrowly scoped database function or equivalent.
- Requires separate operational credential handling.
- Requires additional integration tests for privilege boundaries, direct SQL, rollback, pooling, and concurrency.

## Rejected alternatives

### Let the application set arbitrary GUC values

Rejected because knowledge of the context key names would be sufficient to impersonate protected scope.

### Trust caller metadata

Rejected because transport, client, model, agent, or request metadata is not authoritative security provenance.

### Make RLS the primary authorization system

Rejected because database policy cannot replace the canonical IdentityContext → Policy Kernel decision model.

### Use one superuser/owner role for everything

Rejected because it collapses application and administrative trust boundaries and can create RLS bypass paths.

## Required follow-up

1. Define exact PostgreSQL roles and privileges.
2. Define the trusted context establishment mechanism.
3. Prove direct-SQL resistance and privilege separation.
4. Verify transaction/pool/concurrency/rollback isolation.
5. Only then implement final scope constraints and RLS.
