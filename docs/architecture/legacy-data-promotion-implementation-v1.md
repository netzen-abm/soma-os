# Legacy Data Promotion Implementation v1

## Purpose

Implement Issue #50 as a controlled migration lifecycle without weakening the
existing identity, authorization, protected-data, or RLS boundaries.

## Canonical decision chain

```text
Legacy row
  -> classification history
  -> eligible verified classifications
  -> deterministic authoritative selection
  -> explicit promotion event
  -> protected transaction
  -> trusted DB service identity
  -> tenant/data-domain scope
  -> RLS + constraints
```

No transport, model, agent, hash, resource type, or caller metadata can create
or override scope.

## Authoritative classification selection

A classification is a candidate only when all of the following hold:

- `SCOPED_VERIFIED`
- `VERIFIED`
- tenant and data-domain candidates are both present
- provenance reference is present

The candidate with the newest `classified_at` is authoritative. If multiple
eligible candidates share that timestamp, selection fails closed as ambiguous;
the implementation must not resolve the tie using an arbitrary identifier.

All historical classifications remain preserved.

## Promotion event

Migration `0008_legacy_data_promotion_events.sql` introduces an append-only
event ledger. The event binds:

- explicit promotion event reference;
- legacy record;
- selected classification;
- verified provenance reference;
- selected tenant/data-domain;
- initiator;
- promotion lifecycle status.

The event ledger is evidence/audit infrastructure. It is not an authorization
mechanism and does not by itself make a legacy row protected-access eligible.

## Idempotency and rollback

A legacy record may have at most one active `REQUESTED` or `APPLIED` promotion
event. Repeated requests must reuse the same explicit event identity or return
the existing result; they must never create competing active promotions.

Promotion application must be one protected transaction: write the promotion
event and the scoped target mutation atomically. A failed transaction leaves
no partially applied protected state. Rollback is represented by a new audit
event, never by rewriting prior evidence.

## Preflight and final constraint

The final NOT NULL migration is a separate gate. It may execute only after a
verified preflight proves zero NULL-scoped rows in the protected table. Any
remaining NULL scope blocks the final migration.

## Security boundary

The implementation must continue to use:

`IdentityContext -> Identity Authorization Enforcement -> Policy Kernel -> ProtectedDataAccess -> trusted DB identity -> PostgreSQL`

The database remains defense-in-depth. It must not be treated as a replacement
for the Policy Kernel, and a promotion workflow must not bypass
`ProtectedDataAccess`.

## Verification boundary

Before integration, tests must cover:

1. no eligible classification;
2. quarantined/unverified classifications;
3. newest verified selection;
4. equal-timestamp ambiguity;
5. provenance mismatch;
6. explicit event requirement;
7. duplicate/idempotent promotion;
8. rollback atomicity;
9. cross-tenant/domain rejection;
10. zero-NULL preflight failure;
11. final NOT NULL on clean and realistic migrated state;
12. append-only event-history enforcement.

## Non-goals

This change does not redesign authentication, rotate production credentials,
infer missing scope, delete legacy evidence, or add application-surface
features.
