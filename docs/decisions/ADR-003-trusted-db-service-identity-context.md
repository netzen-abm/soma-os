# ADR-003 — Trusted DB Service Identity and Transaction Context

**Status:** Accepted design baseline  
**Date:** 2026-09-06

## Context

SOMA's protected persistence path currently establishes tenant and data-domain context transaction-locally. Transaction-local state prevents connection-pool scope leakage, but it does not by itself prove that the state was established by a trusted actor.

Before database constraints and PostgreSQL RLS become enforcement layers, SOMA needs a database trust boundary that prevents arbitrary application SQL or caller-controlled metadata from forging protected scope.

## Decision

SOMA will separate **application identity** from **database service identity**.

The canonical chain remains:

`IdentityContext → Identity Authorization Enforcement → Policy Kernel → ProtectedDataAccess → approved persistence service identity → trusted transaction context → PostgreSQL`

The approved persistence identity will be a restricted application/database role. Schema ownership, migrations, and privileged operational access will use separately controlled roles.

The trusted transaction context may be established through a narrowly privileged database mechanism, including a controlled `SECURITY DEFINER` function if that is demonstrated to be safer than alternatives.

## Consequences

### Positive

- Caller metadata cannot directly become database trust.
- Database-level defense-in-depth can rely on a verifiable trust boundary.
- Application, migration, and operational privileges are separated.
- Connection pooling remains compatible with transaction-local isolation.
- The design supports later controlled RLS without making RLS the primary authorization authority.

### Costs

- Additional PostgreSQL role and privilege configuration is required.
- Integration tests must execute against configured PostgreSQL roles, not only a single unrestricted test account.
- Deployment configuration must manage separate credentials safely.
- Privileged database functions, if selected, require careful ownership and `search_path` hardening.

## Rejected alternatives

### Caller-provided trusted scope

Rejected because an untrusted caller could assert the same tenant/domain values.

### RLS-only authorization

Rejected because RLS cannot replace the canonical Policy Kernel and identity authorization boundary.

### One unrestricted database role

Rejected because it collapses application, migration, and administrative trust boundaries.

### Treating transaction-local GUC state as inherently trusted

Rejected because transaction-locality provides isolation semantics but not actor authenticity.

## Follow-on dependency

Only after this trust boundary is proven should SOMA introduce final protected-row constraints and controlled RLS enforcement. Adversarial direct-SQL tests are mandatory before those later gates are accepted.
