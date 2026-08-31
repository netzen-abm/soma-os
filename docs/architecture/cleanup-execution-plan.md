# SOMA Cleanup and Merge Execution Plan

**Date:** 2026-08-31
**Branch:** `stabilization/baseline-audit`

## Objective

Clean the repository while building reusable shared infrastructure in parallel.
The stabilization branch becomes merge-ready only when the required gates pass.

## Parallel workstreams

### A — Repository hygiene

Audit every directory, generated file, duplicate configuration, obsolete
script, and legacy module.

No deletion without evidence.

### B — Shared infrastructure

Build reusable contracts and capabilities before client-specific features.
Priorities include privacy, safety, provenance, evidence, data handling,
transport boundaries, and provider adapters.

### C — Security hardening

Review cryptography, key lifecycle, vault import/export, authentication,
device sync, and recovery mechanisms.

Unsafe placeholders remain disabled until replaced and tested.

### D — Build and CI

Keep one canonical CI path on GitHub. Add tests for every new shared layer.
Avoid CI jobs that mutate source branches automatically.

### E — Evidence research

Continue multi-source research infrastructure independently of application
surfaces. Preserve provider provenance and distinguish evidence absence from
provider failure.

### F — Merge preparation

Keep `main` protected while stabilization proceeds. Compare refs before
merging and verify the final merge result.

## Deletion gate

A candidate can be deleted only when all are true:

- source and configuration references were searched;
- build/test references were checked;
- runtime relevance was checked;
- migration/deployment relevance was checked;
- security/privacy relevance was checked;
- replacement behavior exists where applicable;
- replacement was verified;
- archive/recovery exists when historical value is plausible;
- deletion is recorded in the cleanup manifest.

If any condition is unknown, classify as `UNKNOWN` and keep it.

## Shared-infrastructure gate

A reusable capability must be considered for the shared layer before being
implemented independently in a surface.

The review must identify:

- policy;
- contract;
- shared implementation;
- adapters;
- consuming surfaces;
- failure behavior;
- privacy boundary;
- safety boundary;
- tests.

## Security gate

Do not merge claims that exceed implementation.

Examples:

- signing is not automatically zero-knowledge proof;
- hashing is not encryption;
- Base64 is not encryption;
- a study citation is not evidence of efficacy by itself;
- traditional use is not automatically clinical evidence.

## Evidence gate

Research output must preserve:

- source;
- study identity;
- verification path;
- search context;
- supporting evidence;
- contradictory evidence;
- limitations;
- safety information where relevant.

## Merge-ready definition

The stabilization branch is merge-ready when:

1. no known high-severity security blocker remains active;
2. canonical CI passes;
3. repository cleanup has no unresolved deletion decisions that affect the
   active build;
4. shared infrastructure contracts are coherent;
5. database migrations are internally consistent;
6. Docker configuration matches the actual backend build;
7. evidence infrastructure tests pass;
8. the branch is reconciled with current `main` history;
9. the final diff has been reviewed;
10. no force-push is required to merge.

## Merge procedure

```text
Audit
  ↓
Clean
  ↓
Verify
  ↓
Reconcile main
  ↓
CI
  ↓
Security review
  ↓
Final diff review
  ↓
Merge
  ↓
Verify main
```

## Post-merge rule

After the merge, `main` becomes the canonical integration baseline.
Experimental work continues on feature branches and is merged through the same
verification gates.
