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

## Deliberately not completed yet

This phase does **not** claim end-to-end protected persistence security. In particular:

1. Rust `SomaDatabaseManager` still contains direct SQL access paths and has not yet been fully migrated behind the shared contract.
2. PostgreSQL RLS is not enabled by the transition migration because existing rows lack trustworthy scope.
3. Trusted transaction-local database context and connection-pool isolation are not yet wired.
4. Legacy record inventory, classification, quarantine and validated backfill are not yet implemented.
5. Authentication/session/token trust lifecycle is outside this phase.
6. Service-to-service identity and background-job enforcement remain open.
7. Export, backup, cache and administrative bypass paths require separate inventory and tests.

## Required next implementation gate

Before enabling database-level enforcement:

- migrate every protected caller to the shared access boundary;
- establish a trusted, transaction-local DB security context after authorization;
- implement RLS policies that fail closed when that context is absent or mismatched;
- prove connection-pool context cannot leak across requests;
- classify/quarantine legacy rows without inferred security scope;
- add integration tests that attempt direct SQL/bypass access;
- verify rollback and partial-failure behavior;
- update the product-readiness checklist only for evidence-backed completions.
