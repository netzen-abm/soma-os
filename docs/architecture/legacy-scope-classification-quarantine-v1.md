# Legacy Scope Classification & Quarantine v1

## Status

**Design gate — not production enforcement**

This document defines how SOMA-OS must handle existing `anonymized_user_vitals` rows that predate explicit tenant/data-domain scope.

The governing rule is simple:

> **No legacy row may become protected data merely because the system can guess its scope.**

Scope must be established from trustworthy provenance or the row must remain outside protected access.

## 1. Current state

The original `anonymized_user_vitals` table contains pseudonymous health-proof data but no tenant or data-domain fields. The transition migration adds nullable `tenant_id` and `data_domain`; NULL means legacy/unclassified and is explicitly outside the protected-access path.

The protected application path now requires an explicit tenant/data-domain binding before persistence operations. Database-level RLS is intentionally deferred until the legacy population has a trustworthy disposition.

## 2. Legacy classification states

Every pre-scope row must be assigned one of these dispositions before database enforcement is tightened:

| State | Meaning | Protected access |
| --- | --- | --- |
| `LEGACY_UNCLASSIFIED` | No trustworthy scope evidence exists | **Denied** |
| `CLASSIFICATION_PENDING` | A documented source of scope is being reviewed | **Denied** |
| `CLASSIFIED` | Tenant and data domain were established from approved provenance | **Allowed after validation** |
| `QUARANTINED` | Row is retained for controlled investigation/retention but is outside the protected dataset | **Denied** |
| `RETIRED` | Row is no longer operationally required and has an approved retention disposition | **Denied** |

The exact storage representation may be a classification table, quarantine table, or controlled metadata columns. The implementation decision must preserve immutable original row identity and provenance.

## 3. Classification evidence hierarchy

A legacy row may be classified only from an explicitly approved evidence source, for example:

1. a trusted migration manifest with independently verified scope;
2. an authoritative application record whose tenant/domain binding is itself already governed;
3. a controlled operator review with recorded evidence and approval;
4. another source explicitly approved by the data-governance policy.

The following are **not** valid scope evidence by themselves:

- anonymized hash value;
- hash collision/uniqueness behavior;
- transport channel;
- HTTP/Telegram/agent metadata;
- AI inference;
- current caller identity;
- database connection identity;
- row ordering or timestamps;
- guessed tenant/domain from naming conventions.

If provenance is ambiguous, classification must fail closed and the row remains quarantined/unclassified.

## 4. Quarantine strategy

The preferred safety model is **logical quarantine before destructive migration**.

A quarantine record should retain:

- stable reference to the original row;
- classification state;
- reason code;
- original tenant/data-domain values, if any;
- proposed scope, if any, separately from authoritative scope;
- evidence/provenance reference;
- reviewer/actor reference;
- classification timestamp;
- migration batch/version;
- disposition notes.

A proposed scope must never be written into authoritative `tenant_id` or `data_domain` until the classification gate succeeds.

## 5. Two-dimensional scope invariant

The protected dataset requires a complete scope pair:

```text
tenant_id != NULL
AND data_domain != NULL
```

Partial scope is not protected scope. A row with only one dimension populated must remain outside protected access until the missing dimension is established from trustworthy provenance.

Before RLS is enabled, the migration sequence must therefore establish an invariant equivalent to:

```text
(tenant_id IS NULL AND data_domain IS NULL)
OR
(tenant_id IS NOT NULL AND data_domain IS NOT NULL)
```

The transition period may temporarily contain NULL-scope legacy rows, but it must not silently reinterpret them as belonging to a tenant or domain.

## 6. Migration phases

### Phase A — Inventory

Produce a repeatable inventory of:

- total legacy rows;
- fully scoped rows;
- partially scoped rows;
- NULL/unclassified rows;
- duplicate hashes within each intended scope;
- rows with invalid field shapes.

The inventory must be auditable and reproducible.

### Phase B — Evidence classification

For each candidate row, attach the evidence that establishes the proposed tenant/domain. No automated inference is permitted.

### Phase C — Quarantine

Rows without sufficient evidence are moved or marked as `QUARANTINED` while retaining the original data and an auditable reference. Quarantine access is a separate capability and must not be reachable through ordinary protected-data reads.

### Phase D — Controlled promotion

Only validated rows may receive authoritative tenant/domain scope. Promotion must be idempotent and must fail closed on conflicting evidence.

### Phase E — Constraint gate

After classification:

- reject partial scope;
- require complete scope for operational protected rows;
- preserve an explicit disposition for legacy/quarantined rows;
- verify scoped uniqueness;
- verify no protected query can return quarantined/unclassified rows.

### Phase F — RLS gate

Only after the preceding invariants are proven should PostgreSQL RLS be introduced as defense-in-depth. RLS must use trusted transaction-local scope established by the application authorization boundary, not caller-controlled metadata.

## 7. Required negative tests

The implementation must prove that:

1. a NULL-scope row cannot be returned by a protected query;
2. a partially scoped row cannot be returned by a protected query;
3. a quarantined row cannot be returned by a protected query;
4. a row cannot be promoted without authoritative evidence;
5. conflicting classification evidence fails closed;
6. a caller cannot supply or override classification scope through request metadata;
7. the same anonymized hash can exist in different tenant/domain scopes;
8. the same anonymized hash cannot be duplicated inside one complete scope;
9. classification is idempotent;
10. failed promotion leaves the original row unchanged.

## 8. Operational safety

Classification must be performed as a controlled migration process, not opportunistically during normal reads/writes.

Every batch must provide:

- deterministic batch identifier;
- dry-run/inventory mode;
- explicit approval boundary before promotion;
- transaction-safe updates;
- resumability;
- before/after counts;
- audit records;
- rollback or compensating procedure;
- post-migration verification.

No destructive deletion is part of this design gate. Deletion requires a separate retention decision and evidence that the data is no longer needed.

## 9. Relationship to protected data enforcement

This gate sits between the current application-level protected access boundary and future database-level enforcement:

```text
IdentityContext
      ↓
Authorization Enforcement
      ↓
Protected Data Access
      ↓
Classification / Quarantine Gate
      ↓
Complete trusted scope
      ↓
Database constraints
      ↓
PostgreSQL RLS (defense-in-depth)
```

The Policy Kernel remains the authorization authority. PostgreSQL RLS is a secondary enforcement mechanism and must never become an independent authorization model.

## 10. Acceptance criteria

This design is ready for implementation when:

- classification states and transitions are explicit;
- authoritative evidence sources are defined;
- inference is explicitly prohibited;
- quarantine isolation is defined;
- complete two-dimensional scope is required for protected rows;
- migration and rollback behavior are defined;
- negative tests cover every fail-closed condition;
- RLS prerequisites are explicit;
- audit/provenance requirements are implementable.

**Current recommendation:** implement this as a separate design/implementation gate before adding RLS or making `tenant_id`/`data_domain` universally `NOT NULL`. Preserve legacy data until its disposition is independently justified.
