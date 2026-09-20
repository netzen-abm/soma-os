# Protected Execution-Path Convergence Audit v1

## Purpose

Verify that protected execution paths preserve the canonical authorization contract from decision through the provider/persistence boundary.

## Canonical invariant

After an authoritative authorization decision, the following security meaning must not be widened or silently substituted:

- principal
- subject
- capability_id
- capability_version
- resource_type
- resource_id
- action
- tenant
- data_domain

## Current findings

### 1. Python protected-data path

`ProtectedDataAccess` constructs `PolicyRequest` from the explicit protected request and carries capability ID, capability version, resource, action, tenant, data-domain, and subject into the canonical decision boundary. Adapter execution occurs only after an ALLOW decision.

**Status: PASS at current contract level.**

### 2. Rust authorization-bound contexts

The protected DB, Health State, measurement, intervention, evidence-link, outcome, longitudinal-context, and longitudinal-observation contexts preserve the authorized capability ID and capability version alongside principal, subject, resource, action, tenant, and data-domain scope.

**Status: PASS at current context-minting level.**

### 3. Protected DB context validation

The protected DB context previously copied `capability_version` but did not include it in its non-empty/control-character validation set. This is corrected by this change.

**Status: FIXED in this branch.**

### 4. PostgreSQL execution scope

`begin_protected_transaction` accepts only an authorization-bound context, switches to `somaos_persistence`, and establishes transaction-local tenant/data-domain scope. The current SQL layer binds tenant/data-domain, while resource/action/capability semantics remain at the Rust operation boundary.

**Status: REVIEWED.** This is intentional for the current DB contract, but the legacy `SomaDatabaseManager` remains a separate review item because its SQL methods operate on the older anonymized-vitals schema.

### 5. Local Health Vault compatibility adapter

`CanonicalVaultAuthorizer` translates each vault operation into a fresh canonical authorization request. It preserves principal and subject and derives the vault capability/action from the concrete vault action. The adapter is therefore a second authorization invocation, but not an independent policy engine.

**Status: REVIEWED.** Future capability-version evolution must keep this compatibility mapping explicitly synchronized with the canonical registry.

### 6. Legacy PostgreSQL manager

`services/backend-rust/src/db_layer.rs` remains instantiated by the application runtime but has no repository call sites found by the current search. Its methods require an `AuthorizedProtectedDbContext`, but its SQL binds tenant/data-domain and caller-supplied anonymized hash rather than the full subject/resource/action contract.

**Status: LEGACY / NOT PROVEN AS AN ACTIVE REQUEST PATH.**

Do not extend this manager for new protected-data functionality. Any future use must first migrate it behind the canonical provider-neutral persistence contract or explicitly classify it as migration-only and archive it after evidence.

## Audit conclusion

The canonical authorization-to-context path is converged for the current active health contexts. The remaining material boundary is the legacy PostgreSQL manager and the compatibility semantics of the Local Health Vault adapter.

The next implementation gate is to prove that no active application request path reaches the legacy manager and then retire/archive that path only after repository evidence confirms it is unused.

## Required evidence before retirement

1. Repository-wide call-site search for `SomaDatabaseManager`, `append_anonymous_vital_log`, and `get_logs_by_user_hash`.
2. Runtime route/state reachability review.
3. CI/static guard preventing new production call sites.
4. Archive-first migration/removal decision.
5. Re-run the protected-data bypass audit and all exact-head required CI gates.
