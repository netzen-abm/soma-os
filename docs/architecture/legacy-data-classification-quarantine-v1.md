# Legacy Data Classification & Quarantine v1

## Status

Design baseline — implementation gate for protected PostgreSQL data isolation.

This document defines how pre-isolation rows are handled without inventing tenant or data-domain scope. It is intentionally provider-neutral in policy semantics and PostgreSQL-specific only where quarantine storage is described.

## Problem

`anonymized_user_vitals` predates canonical tenant/data-domain isolation. Existing rows may have `tenant_id` and `data_domain` unset. Their cryptographic or pseudonymous fields do not constitute trustworthy authorization scope.

Therefore:

> No legacy row may become protected-access eligible merely because a tenant, hash, transport, model, or caller claims a scope for it.

The transition must preserve uncertainty instead of converting unknown provenance into false certainty.

## Classification states

Each legacy record must resolve to exactly one transition disposition:

- `SCOPED_VERIFIED` — tenant and data-domain provenance are established by an approved authoritative source and independently verified.
- `SCOPED_REVIEW_REQUIRED` — a candidate scope exists, but provenance or consistency is insufficient for automatic promotion.
- `QUARANTINED_UNCLASSIFIED` — scope is absent or cannot be established without inference.
- `QUARANTINED_CONFLICT` — available provenance sources disagree or the record violates transition invariants.
- `REJECTED_INVALID` — the record is structurally invalid or fails required integrity checks.

Only `SCOPED_VERIFIED` may enter the protected access population.

## Required invariants

1. Unknown scope is never treated as a wildcard.
2. NULL tenant/data-domain is not equivalent to a public or global scope.
3. Hash equality is not evidence of shared tenant ownership.
4. Transport metadata, caller metadata, model output, AI inference, or search results cannot establish security scope.
5. A partial scope (`tenant_id` without `data_domain`, or vice versa) is not eligible for protected access.
6. Conflicting provenance is quarantined rather than resolved by guesswork.
7. Classification must be auditable and reproducible.
8. Quarantine must preserve the original record and its provenance needed for later review.
9. Promotion from quarantine to protected access requires an explicit, verifiable classification event.
10. No RLS policy may depend on an inferred fallback scope.

## Recommended data model

Prefer explicit transition metadata over overloading the protected row with ambiguous meaning. The implementation should introduce a classification/quarantine record containing at least:

- stable legacy record reference;
- classification state;
- candidate tenant/data-domain when known;
- classification reason/code;
- authoritative provenance reference(s);
- classifier identity;
- classified timestamp;
- verification status;
- optional review reference;
- immutable link to the original record.

A dedicated quarantine table is preferred for records that cannot safely enter the protected population. It prevents the protected table from becoming a mixture of trusted and untrusted records while preserving recoverability.

## Promotion rule

A row may be promoted only when both scope dimensions are present and the provenance establishing them is verified. Promotion must be an explicit state transition, not an UPDATE triggered by a normal application read.

Conceptually:

`LEGACY → CLASSIFY → VERIFY PROVENANCE → SCOPED_VERIFIED → protected access`

or

`LEGACY → CLASSIFY → insufficient/conflicting evidence → QUARANTINE`

## Migration sequence

1. Inventory and count existing rows by scope completeness and structural validity.
2. Create classification/quarantine metadata structures.
3. Classify existing rows without assigning inferred scope.
4. Quarantine unclassified, partial, conflicting, or invalid rows.
5. Verify that every protected-access-eligible row has explicit scope.
6. Add database constraints preventing partial protected scope.
7. Add RLS only after the trusted population is demonstrably scoped.
8. Add negative integration tests proving quarantined/legacy rows cannot be reached through protected queries.
9. Record migration evidence and update product-readiness gates.

## Rollback / recovery

Classification must be reversible at the metadata level without mutating the original health record solely to force a security classification. Quarantine records must retain enough provenance to support later manual review or an explicitly authorized reclassification.

## Relationship to RLS

RLS is a defense-in-depth enforcement layer. It must not be enabled as a substitute for legacy classification. If legacy rows remain scope-ambiguous, the safe behavior is to make them inaccessible to protected paths and quarantine them until classification is verified.

## Acceptance criteria

- No legacy row receives a fabricated tenant or data-domain.
- Protected access is limited to explicitly verified scope.
- Partial/conflicting/unknown records are quarantined or otherwise excluded from protected access.
- Classification decisions are auditable.
- Clean-database and legacy-data integration tests pass.
- Only after these conditions are satisfied may `NOT NULL`/CHECK constraints and RLS be introduced as the next enforcement gate.

## Security posture

This design deliberately favors false negatives over false positives during migration. A record that cannot be safely classified remains unavailable to protected access rather than being silently exposed under an assumed scope.
