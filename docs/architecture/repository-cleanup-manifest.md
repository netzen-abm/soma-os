# SOMA Repository Cleanup Manifest

**Date:** 2026-08-31
**Branch:** `stabilization/baseline-audit`
**Policy:** Audit → verify → classify → archive → verify → delete.

## Rules

1. Never delete a file solely because it is old, duplicated by name,
   or currently unused.
2. Establish whether the file is referenced by code, CI, Docker,
   documentation, migrations, deployment, or another active component.
3. Prefer archive before deletion when historical value is plausible.
4. Delete only after the replacement is verified and the archived copy
   is sufficient for recovery.
5. Do not remove unknown material during a cleanup pass.
6. Security, privacy, medical, and evidence-related material receives
   stricter review than ordinary documentation.
7. Keep code readable; avoid unnecessarily long lines and dense functions.

## Classification

| Status | Meaning |
| --- | --- |
| KEEP | Active or foundational component. |
| ARCHIVE | Historical/legacy material retained outside the active path. |
| DELETE-AFTER-VERIFY | Candidate for removal after dependency and recovery checks. |
| UNKNOWN | Insufficient evidence; do not remove. |

## Audited components

| Component | Current decision | Verification basis |
| --- | --- | --- |
| `services/evidence-research/` | KEEP | Shared evidence infrastructure under active development. |
| `database/migrations/` | KEEP | Database schema history must remain reproducible. |
| `docs/architecture/` | KEEP | Architectural decisions and contracts are active references. |
| `docs/evidence-sources/` | KEEP | Evidence-source provenance and policy are foundational. |
| `services/backend-rust/archive/` | KEEP | Explicit historical archive; no deletion without a separate audit. |
| `docs/archive/ci/` | KEEP | Archived superseded CI retained for traceability. |
| `.github/workflows/` | KEEP | Canonical CI/deployment control plane. |
| `.gitlab-ci.yml` | DELETE-AFTER-VERIFY | Duplicate CI was removed only after GitHub Actions was established as canonical. |
| `dockerfile` | KEEP | Active backend container build; corrected to use Cargo. |
| vault export/import implementation | UNKNOWN / BLOCKED | Security-sensitive; placeholder encryption was disabled rather than deleted. |
| mnemonic recovery implementation | UNKNOWN / BLOCKED | Security-sensitive; non-standard derivation must be redesigned before reuse/removal. |
| ZKP-labelled assets | UNKNOWN | Terminology does not prove implementation; each asset requires individual review. |

## Security-sensitive hold

The following categories must not be deleted until replacement behavior is
implemented and independently verified:

- encryption/key management;
- mnemonic/key recovery;
- device synchronization;
- health-vault import/export;
- authentication/authorization;
- privacy controls;
- medical safety controls.

## Evidence-sensitive hold

Evidence-related source records must retain:

- provenance;
- source identifier;
- retrieval context;
- verification URL;
- study-level identity;
- contradictory findings where available.

A duplicate database record is not automatically a duplicate underlying study.

## Cleanup procedure

### Step 1 — Inventory

List files, modules, dependencies, references and runtime entry points.

### Step 2 — Dependency verification

Search source, CI, Docker, scripts and documentation for references.

### Step 3 — Behavioral verification

Determine whether the component is required by build, runtime, deployment,
security, data migration, or user-facing behavior.

### Step 4 — Classification

Assign `KEEP`, `ARCHIVE`, `DELETE-AFTER-VERIFY`, or `UNKNOWN`.

### Step 5 — Archive

For plausible legacy material, preserve it under a clearly labelled archive
location before deletion.

### Step 6 — Verify replacement

Confirm the active replacement builds, tests, and preserves required behavior.

### Step 7 — Delete

Delete only when the evidence supports removal and recovery is preserved.

## Current conclusion

No broad destructive deletion is authorized by this manifest. The repository
contains security-sensitive and multi-generation components that require
component-level verification. Cleanup therefore proceeds incrementally.
