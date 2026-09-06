# Protected Data Access Enforcement v1 — Implementation Boundary

**Status:** Implementation phase 1 — not production-complete  
**Issue:** #27  
**Design:** `docs/architecture/protected-data-access-enforcement-v1.md`

## Implemented in this phase

- Provider-neutral `ProtectedDataRequest` contract.
- Exact binding of capability, resource type, resource ID, action, tenant and data domain.
- Composition with the canonical Identity → Authorization Enforcement boundary.
- Policy Kernel v0.3 remains the sole grant evaluator.
- Caller metadata cannot broaden identity scope.
- Fail-closed validation for missing/invalid authorization context and malformed targets.
- Negative tests for tenant, domain, resource, action, caller metadata, identity and policy failures.
- Database transition migration adds explicit `tenant_id` and `data_domain` fields without inventing legacy scope.
- Rust protected persistence methods now require `ProtectedDbContext` and perform scoped writes/reads.
- PostgreSQL transaction-local scope binding uses `set_config(..., true)` so pooled connections do not retain request scope after commit/rollback.
- PostgreSQL integration harness verifies pool reuse, concurrent transaction isolation, scoped hash uniqueness, and exclusion of legacy NULL-scope rows.
- Legacy global `anonymized_user_hash` uniqueness is replaced by tenant + data-domain + hash uniqueness during the transition period.

## Deliberately not completed yet

This phase does **not** claim end-to-end protected persistence security. In particular:

1. The full provider-neutral adapter replacement for all persistence access is not yet complete; the existing `SomaDatabaseManager` remains the concrete PostgreSQL implementation boundary.
2. PostgreSQL RLS is not enabled because existing rows lack trustworthy scope.
3. Legacy record inventory, classification, quarantine and validated backfill are not yet implemented.
4. Authentication/session/token trust lifecycle is outside this phase.
5. Service-to-service identity and background-job enforcement remain open.
6. Export, backup, cache and administrative bypass paths require separate inventory and tests.
7. Database constraints requiring complete scope remain deferred until legacy disposition is established.

## Required next implementation gate

Before enabling database-level enforcement:

- complete the repository-wide direct SQL/persistence bypass audit;
- establish the legacy classification/quarantine process without inferred security scope;
- establish a complete two-dimensional scope invariant;
- implement constraints and RLS only after trustworthy scope exists;
- add integration tests for direct SQL/bypass attempts and partial scope;
- verify rollback, recovery and migration behavior;
- complete authentication/session/service identity enforcement;
- update the product-readiness checklist only for evidence-backed completions.
