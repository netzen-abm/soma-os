# Protected Data Access Bypass Audit v1

## Purpose

This audit establishes a conservative repository-level control for detecting direct database/SQL access outside the canonical protected persistence boundary.

It is an **inventory and review gate**, not proof of runtime authorization. A clean static result does not replace IdentityContext validation, Policy Kernel authorization, ProtectedDataAccess enforcement, PostgreSQL constraints/RLS, integration tests, or service identity.

## Canonical boundary

Protected health persistence must pass through the shared authorization and protected-data boundary before reaching a persistence adapter. The current canonical Rust persistence implementation is:

- `services/shared/identity_authorization_enforcement.py`
- `services/shared/protected_data_access.py`
- `services/backend-rust/src/protected_db_context.rs`
- `services/backend-rust/src/db_layer.rs`

Migration SQL is separately allowlisted because schema definitions necessarily contain SQL primitives.

## Audit rule

`scripts/audit_protected_data_access.py` scans Rust, Python, and SQL source files for conservative indicators including:

- `sqlx::query` / `sqlx::query_as`
- `sqlx::raw_sql`
- PostgreSQL pool primitives
- protected-table references

Findings outside the explicitly allowlisted persistence boundary or migration directory fail the script and require manual review. The audit never auto-creates an exception.

## Security invariant

> No new protected-data access path may bypass the canonical Identity → Capability → Policy Kernel → Protected Data Access → persistence-adapter boundary.

A transport, model, agent, tool, caller-supplied metadata, or alternate runtime must not become an authorization authority merely because it can reach a database primitive.

## Current repository result

At the baseline audited before this control was added, repository search identified the protected table SQL access in `services/backend-rust/src/db_layer.rs`, with schema creation in `database/migrations/0001_initialize_zk_logs.sql`. The repository-wide `sqlx::query` search likewise resolved to the canonical Rust persistence layer. fileciteturn104file0L2-L16 fileciteturn105file0L2-L16

The baseline also showed `PgPool` ownership in `db_layer.rs`, while application startup constructs the pool and passes it to `SomaDatabaseManager`. fileciteturn106file0L2-L14 fileciteturn106file1L18-L30

## Limitations

This control does not prove:

- that all runtime data paths are scoped correctly;
- that PostgreSQL RLS is enabled or correctly configured;
- that a service cannot connect directly using credentials;
- that dynamic SQL constructed without these exact indicators is safe;
- that Python, external services, migrations, or operational tooling cannot bypass the boundary;
- that authentication/session/service identity is trustworthy;
- that legacy records have valid tenant/data-domain classification.

Those remain explicit product-readiness gates.

## Required review process

1. Run the static audit on every protected-data architecture change.
2. Treat every finding as a review item; do not silence it by widening the allowlist casually.
3. If a new legitimate persistence path is required, first define its boundary and tests, then add the narrowest possible allowlist entry with documentation.
4. Re-run negative-path integration tests after any boundary change.
5. Keep migration/schema SQL separate from application data-access authorization.

## Status

**Phase:** repository-level bypass detection v1.

**Not production-complete:** this is evidence-producing governance infrastructure. It must not be represented as proof that protected persistence is fully enforced.
