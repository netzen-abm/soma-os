# Legacy Data Promotion Executor v1

## Purpose

This document defines the PostgreSQL execution boundary that applies an explicitly verified legacy-data promotion after the provider-neutral promotion contract has selected an authoritative classification.

The executor exists because the protected table is `FORCE ROW LEVEL SECURITY`, while legacy records intentionally remain NULL-scoped until verified promotion. Ordinary protected persistence access must therefore continue to deny those records; promotion requires a separate, narrowly scoped database execution capability.

## Canonical flow

```text
IdentityContext
  → Identity Authorization Enforcement
  → Policy Kernel
  → approved migration/promotion operation
  → somaos_persistence
  → soma_apply_legacy_promotion(...)
  → verified classification evidence
  → derive tenant_id/data_domain
  → atomic legacy-row update + APPLIED event
  → PostgreSQL
```

The executor does not replace Policy Kernel authorization. The caller must already have completed the canonical authorization boundary.

## Database privilege boundary

Migration `0009_legacy_promotion_executor.sql` creates:

- `somaos_legacy_promotion_owner`: `NOLOGIN`, non-superuser, `BYPASSRLS`, dedicated only to owning the narrow executor function.
- `soma_apply_legacy_promotion(...)`: `SECURITY DEFINER`, fixed `search_path = pg_catalog`, explicitly schema-qualified table references.
- `EXECUTE` granted only to `somaos_persistence`; `PUBLIC` receives no execute privilege.

PostgreSQL documents that `SECURITY DEFINER` executes with the function owner's privileges and that a secure `search_path` plus restricted `EXECUTE` privileges are required to avoid privilege escalation. citeturn2view0

The executor owner is deliberately not a login role and is not a general migration/admin identity. It has no application credential.

## Promotion invariants

The executor requires:

1. positive legacy record and classification identifiers;
2. non-empty, control-character-free promotion metadata;
3. the legacy row is locked before promotion state is evaluated;
4. an existing `APPLIED` event with the same event reference, classification and provenance is idempotently returned;
5. a different active event for the same record is rejected;
6. a pre-scoped legacy row without a matching promotion event is rejected rather than silently adopted;
7. classification belongs to the target legacy record;
8. classification state is `SCOPED_VERIFIED`;
9. verification status is `VERIFIED`;
10. candidate tenant and data-domain are both present;
11. classification provenance exactly matches the authoritative provenance supplied by the authorized promotion workflow;
12. scope is copied only from the verified classification row;
13. the row update and immutable `APPLIED` event insert occur in the same database transaction.

No scope is inferred from hash, transport, model output, agent metadata, resource type, or caller metadata.

## Concurrency and idempotency

The legacy row is acquired with `SELECT ... FOR UPDATE` before the active-event check. This serializes concurrent promotion attempts for the same record. PostgreSQL `FOR UPDATE` waits for an existing row lock and returns the updated version after the concurrent transaction commits, which allows the second attempt to observe an already-applied promotion and return the existing event when the request is identical. citeturn6search2turn6search9

## Rollback semantics

Promotion is atomic:

```text
BEGIN
  lock legacy row
  validate authoritative classification
  update NULL scope → verified scope
  insert immutable APPLIED event
COMMIT
```

If the transaction rolls back, both the scope update and event insertion roll back. Classification history remains unchanged.

## Why ordinary RLS access is not used for legacy rows

The protected RLS policy intentionally excludes NULL-scoped legacy records. PostgreSQL evaluates RLS policies for normal table access and supports role-specific policies; `FORCE ROW LEVEL SECURITY` also prevents the table owner from silently bypassing the policy. citeturn1view0turn3search0

Therefore promotion is not implemented by weakening the protected persistence policy or by granting ordinary persistence access to NULL-scoped records. Instead, a dedicated, non-login function-owner privilege boundary performs only the explicitly verified promotion operation.

## Security testing

The implementation must verify:

- verified promotion succeeds and derives scope from classification;
- same promotion event is idempotent;
- conflicting event is rejected;
- unverified classification is rejected;
- quarantined classification is rejected;
- provenance mismatch is rejected;
- rollback leaves both legacy row and event unchanged;
- untrusted database role cannot invoke the executor;
- immutable promotion history cannot be rewritten through UPDATE/DELETE.

## Deployment implications

Production must not expose credentials for `somaos_legacy_promotion_owner`. The normal application/persistence credential remains bound to the approved persistence service identity. Promotion invocation is an explicitly authorized migration capability, not a general-purpose SQL privilege.

The final zero-NULL preflight and final `NOT NULL` migration remain separate gates and are not introduced by this executor.
