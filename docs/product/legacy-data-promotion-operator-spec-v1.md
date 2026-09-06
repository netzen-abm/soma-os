# Legacy Data Promotion — Operator, API & Data-Flow Specification v1

**Status:** Proposed implementation baseline  
**Depends on:** SOMA System Architecture + Product Specification v1; Legacy Data Classification v1; Protected Data RLS/Constraints v2.

## 1. Purpose

Define the implementation-grade operator workflow for legacy records that may eventually become protected-data eligible. This specification intentionally separates **classification**, **verification**, **promotion**, and the **final database constraint gate**.

## 2. State machine

```text
LEGACY_RECORD
    |
    v
INVENTORIED
    |
    v
CLASSIFICATION_RECORDED
    |
    +--> QUARANTINED_UNCLASSIFIED
    +--> QUARANTINED_CONFLICT
    +--> SCOPED_REVIEW_REQUIRED
    +--> REJECTED_INVALID
    |
    v
SCOPED_VERIFIED + VERIFIED + PROVENANCE_BOUND
    |
    v
PROMOTION_REQUESTED
    |
    v
PROMOTION_VERIFIED
    |
    v
PROMOTED
    |
    v
PRELIGHT_ZERO_NULL
    |
    v
FINAL_CONSTRAINT_GATE
```

No transition may skip verification or manufacture missing provenance.

## 3. Roles

| Role | View | Classify | Verify | Promote | Final gate |
|---|---:|---:|---:|---:|---:|
| observer | yes | no | no | no | no |
| classifier | yes | yes | no | no | no |
| verifier | yes | yes | yes | no | no |
| migration operator | yes | yes | yes | yes | no |
| database migration authority | limited | no | no | no | yes |

The database migration authority is separate from normal application persistence authority.

## 4. Canonical API contract

The API is provider-neutral. HTTP is only one possible adapter.

### `GET /legacy/records`

Filters:

- classification_state
- verification_status
- candidate_tenant_id
- candidate_data_domain
- provenance_reference
- page/cursor

Returns only metadata required for review; sensitive payload fields should be minimized.

### `GET /legacy/records/{legacy_log_id}`

Returns:

- legacy record identity
- current classification history
- provenance references
- verification status
- promotion status
- audit references

### `POST /legacy/records/{legacy_log_id}/classifications`

Request:

```json
{
  "classification_state": "SCOPED_REVIEW_REQUIRED",
  "candidate_tenant_id": "tenant-a",
  "candidate_data_domain": "health",
  "reason_code": "AUTHORITATIVE_SOURCE_REVIEW",
  "provenance_reference": "source-ref-123",
  "classified_by": "principal-123",
  "verification_status": "UNVERIFIED",
  "review_reference": "review-456"
}
```

Rules:

- both scope dimensions are present or both absent;
- classifier identity is explicit;
- classification history is append-only;
- this endpoint never promotes data;
- caller/model/transport metadata cannot substitute for provenance.

### `POST /legacy/records/{legacy_log_id}/verify`

Request:

```json
{
  "classification_reference": "classification-789",
  "authoritative_provenance_reference": "source-ref-123",
  "verification_status": "VERIFIED",
  "verified_by": "verifier-123",
  "verification_reference": "verification-101"
}
```

Rules:

- selected classification must be `SCOPED_VERIFIED`;
- classification must contain both candidate scope dimensions;
- authoritative provenance must exactly match the classification provenance;
- verification is explicit and auditable;
- conflicts or ambiguity deny verification.

### `POST /legacy/records/{legacy_log_id}/promotion`

Request:

```json
{
  "classification_reference": "classification-789",
  "verification_reference": "verification-101",
  "promotion_event_reference": "promotion-202",
  "authoritative_provenance_reference": "source-ref-123",
  "requested_by": "migration-operator-123"
}
```

Rules:

1. validate request schema;
2. load authoritative classification history;
3. require exactly one applicable verified classification or an explicit conflict denial;
4. require `SCOPED_VERIFIED`;
5. require `VERIFIED`;
6. require non-empty provenance;
7. require authoritative provenance equality;
8. derive tenant/domain only from the verified classification;
9. ignore caller-supplied transport/model/hash/agent scope;
10. execute promotion transactionally;
11. write an immutable promotion event;
12. return a stable promotion result.

### `POST /legacy/promotion/preflight`

Returns:

```json
{
  "status": "PASS",
  "checked_rows": 10000,
  "null_scope_rows": 0,
  "unresolved_classifications": 0,
  "conflicts": 0,
  "timestamp": "...",
  "preflight_reference": "preflight-303"
}
```

`PASS` requires zero NULL-scoped protected rows and no unresolved promotion blockers according to the migration gate definition.

### `POST /legacy/promotion/finalize`

This is a migration-authority operation, not a normal application operation.

It must:

- require a successful preflight reference;
- execute inside the controlled migration identity;
- refuse to proceed if the preflight is stale or the dataset changed materially since preflight;
- apply final validated constraint / `NOT NULL` only after zero NULL scope is proven;
- record migration and schema-version evidence.

## 5. Data-flow contract

```text
Operator
  ↓
Surface/API adapter
  ↓
Canonical identity + authorization
  ↓
Legacy classification service
  ↓
Append-only classification history
  ↓
Authoritative verification
  ↓
Promotion service
  ↓
ProtectedDataAccess / trusted persistence boundary
  ↓
Scoped database mutation
  ↓
Audit event

Finalization path:
Migration authority
  ↓
Preflight snapshot/check
  ↓
Zero NULL scope?
  ├─ No → STOP
  └─ Yes
       ↓
Controlled migration identity
       ↓
Final constraint gate
```

## 6. Idempotency

Promotion requests require a stable `promotion_event_reference`.

Expected behavior:

- same reference + same authoritative inputs → return the original successful result;
- same reference + different inputs → reject as conflict;
- already-promoted row + new event → do not duplicate the data mutation; record or reject according to the event policy;
- failed transaction → no partial promotion state may survive.

## 7. Concurrency

Promotion must serialize on the legacy record identity or use an equivalent database concurrency control mechanism. Two operators must not independently promote the same row with conflicting verified classifications.

A promotion transaction must verify the classification state inside the same transaction in which the protected mutation occurs, where practical.

## 8. Failure semantics

| Failure | Result |
|---|---|
| missing identity | deny |
| unverified identity | deny |
| expired/revoked identity | deny |
| missing classification | deny |
| review-required classification | deny promotion |
| conflict | deny promotion |
| provenance mismatch | deny |
| missing scope | deny |
| caller-supplied scope differs | ignore caller scope; derive from verified source |
| duplicate promotion reference with same inputs | idempotent success/result replay |
| duplicate reference with different inputs | conflict |
| database transaction failure | rollback |
| preflight finds NULL scope | finalization blocked |
| finalization authority unavailable | no schema change |

## 9. Audit events

Minimum events:

- `legacy.record.inventoried`
- `legacy.classification.created`
- `legacy.classification.verified`
- `legacy.classification.rejected`
- `legacy.promotion.requested`
- `legacy.promotion.completed`
- `legacy.promotion.conflict`
- `legacy.promotion.rollback`
- `legacy.promotion.preflight_passed`
- `legacy.promotion.preflight_failed`
- `legacy.constraint.finalization_started`
- `legacy.constraint.finalization_completed`
- `legacy.constraint.finalization_blocked`

Audit events should record principal, capability, target record, event reference, result, reason code, timestamps, and provenance references without copying unnecessary health payload.

## 10. Operator UI wireframe contract

### Dashboard

```text
+-------------------------------------------------------------+
| LEGACY DATA MIGRATION                                      |
+-------------------------------------------------------------+
| Total | Review | Verified | Conflicts | Quarantine | Ready |
+-------------------------------------------------------------+
| Search / Filters                                           |
|                                                             |
| ID       State               Verification    Action         |
| 1021     REVIEW_REQUIRED     UNVERIFIED      Review         |
| 1022     SCOPED_VERIFIED     VERIFIED        Promote        |
| 1023     CONFLICT            UNVERIFIED      Resolve        |
+-------------------------------------------------------------+
| Preflight: BLOCKED — 17 NULL-scope rows                    |
+-------------------------------------------------------------+
```

### Record review

```text
Record
  ↓
Original provenance
  ↓
Classification history
  ↓
Candidate scope
  ↓
Verification evidence
  ↓
Conflicts / warnings
  ↓
[Verify] [Quarantine] [Reject]
```

### Promotion confirmation

```text
PROMOTE RECORD

Verified provenance:        ✓
Classification:             SCOPED_VERIFIED
Verification:               VERIFIED
Tenant:                     tenant-a
Data domain:                health
Promotion reference:        promotion-202

This action changes protected-data eligibility.

[Cancel]                       [Confirm Promotion]
```

### Finalization gate

```text
FINAL DATABASE CONSTRAINT GATE

NULL-scoped rows:             0 ✓
Unresolved conflicts:         0 ✓
Promotion failures:           0 ✓
Preflight reference:          preflight-303
Migration identity:           VERIFIED ✓

[FINALIZE CONSTRAINT]
```

The final action must be unavailable while any blocking condition is non-zero.

## 11. Security invariants

- UI is not a security boundary.
- API adapter is not a policy authority.
- AI/model output is never authoritative scope evidence by itself.
- transport metadata is never authoritative scope evidence.
- a hash is not a tenant identifier.
- classification is not promotion.
- promotion is not authorization.
- database RLS remains defense-in-depth.
- migration authority is separate from normal application persistence authority.
- every state transition has an actor and provenance reference.

## 12. Implementation order

1. provider-neutral promotion domain contract;
2. authoritative classification selection;
3. promotion transaction and idempotency;
4. audit events;
5. preflight;
6. final migration gate;
7. adversarial tests;
8. operator API;
9. UI adapter;
10. documentation/runbook.

## 13. Acceptance test matrix

| Scenario | Expected |
|---|---|
| verified scope + matching provenance | promote |
| unverified classification | deny |
| review-required | deny |
| conflict | deny |
| provenance mismatch | deny |
| missing tenant/domain | deny |
| caller attempts broader scope | deny/ignore caller scope |
| same promotion reference replay | idempotent |
| same reference changed inputs | conflict |
| transaction failure | rollback |
| concurrent conflicting promotions | one succeeds; conflicting operation denied |
| NULL rows remain | final gate blocked |
| zero NULL rows | final gate may proceed if all other checks pass |

## 14. Definition of done

Implementation is not complete until the domain contract, migration, operator workflow, audit events, adversarial tests, clean-database verification, realistic legacy-state verification, and documentation all agree.
