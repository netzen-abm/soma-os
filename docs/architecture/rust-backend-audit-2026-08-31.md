# Rust Backend Audit

**Date:** 2026-08-31
**Branch:** `stabilization/baseline-audit`

## Audit rule

No Rust source file is deleted from this pass solely because it is old,
unused, duplicated by name, or outside the current runtime path.

Each candidate requires reference, build, runtime, migration, and security
review before classification.

## Scope

Reviewed areas include:

- `services/backend-rust/src/`
- `services/backend-rust/archive/`
- `services/backend-rust/Cargo.toml`
- repository CI
- Docker build
- evidence-research integration boundary

## Initial classification

| Area | Decision | Reason |
| --- | --- | --- |
| Active backend source | KEEP | Runtime-critical until build verification proves otherwise. |
| Backend archive | KEEP | Historical recovery path; deletion requires separate evidence. |
| Crypto module | KEEP / SECURITY HOLD | Security-sensitive and requires standards review. |
| Vault exporter | SECURITY HOLD | Previous implementation was not real authenticated encryption. |
| Vault importer | SECURITY HOLD | Must remain aligned with the eventual secure format. |
| Key restoration | SECURITY HOLD | Current derivation is not a standards-based mnemonic implementation. |
| Evidence-research | KEEP | Shared cross-surface infrastructure. |
| Database migrations | KEEP | Schema history must remain reproducible. |

## Dependency audit status

Cargo dependencies are not removed merely because static inspection does not
find an obvious use. Before removal, verify:

1. direct source references;
2. feature-gated references;
3. build scripts;
4. test-only references;
5. transitive dependency assumptions;
6. Docker/build behavior.

## Security findings

### Vault export/import

The previous implementation used a marker and Base64 serialization rather
than authenticated encryption. It must not be described as AES-GCM encrypted
until a real implementation is present and tested.

**Decision:** security hold, not deletion.

### Mnemonic recovery

The previous implementation hashed concatenated mnemonic words. That is not a
replacement for a standards-compliant mnemonic-to-key derivation scheme.

**Decision:** security hold, redesign required.

## Evidence infrastructure

The multi-source evidence infrastructure remains active and must not be
removed as unused code. It is intentionally designed as shared infrastructure
for every SOMA surface.

## Code maintainability

Rust code should normally remain within the repository's configured format
width. Prefer:

- small functions;
- explicit types;
- early returns;
- meaningful names;
- semantic line breaks;
- isolated security operations.

Avoid dense one-line expressions and unnecessary abstraction.

## Deletion authorization

No Rust source deletion is authorized by this audit record yet.

A later deletion record must identify the exact path, evidence of non-use,
replacement, verification result, archive location, and deletion commit.
