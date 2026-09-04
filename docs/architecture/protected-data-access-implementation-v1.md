# Protected Data Access Enforcement v1 — Implementation

Status: **Phase 1 implemented; not production-ready**.

The shared `ProtectedDataAccess` gate is now implemented as the provider-neutral bridge between the canonical Identity → Authorization Enforcement boundary and persistence adapters.

## Invariants

- Policy Kernel v0.3 remains the only grant evaluator.
- The access layer cannot broaden identity tenant/data-domain scope.
- Exact resource ID, action, capability, resource type, principal ID and principal type are passed to the existing authorization boundary.
- Denied requests never reach the provider adapter.
- Provider adapters are persistence mechanisms, not authorization authorities.

## Remaining implementation gates

1. Replace direct protected SQL in the Rust `SomaDatabaseManager` with a PostgreSQL adapter behind this contract.
2. Add trusted transaction-local PostgreSQL authorization context with connection-pool hygiene.
3. Add RLS/constraints as database defense-in-depth.
4. Inventory and classify existing rows before non-null scope enforcement.
5. Add end-to-end bypass, transaction rollback, and service-identity tests.
6. Run complete CI and security checks before merging.
