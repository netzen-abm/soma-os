# SOMA-OS Branch Retirement Register v3

## Verified baseline — 2026-10-22

- Repository: `netzen-abm/soma-os`
- Main: `fd43b98be1583f2d13fc0f96eeabaeee71ad9c95` (post-PR #173 merge)
- Physical branches: **44**
- Canonical active branches: **9**
- Non-canonical branches: **35**
- Open pull requests: **1** — PR #164, draft, `architecture/local-health-vault-contract-v1` → `main`
- Retirement completion: **NOT COMPLETE**. No branch has been deleted in this pass.
- GitHub branch-list response marks the listed branches `protected: false`; rulesets query previously returned none. The branch-protection endpoint was denied by the connected integration, so protection settings are not independently verified.

## Canonical active set — exactly these nine

1. `main`
2. `development`
3. `security/current`
4. `architecture/current`
5. `feature/current`
6. `health/current`
7. `research/current`
8. `ai-agent/current`
9. `integration/current`

## Retirement rule

A non-canonical branch may be retired only after:
1. its tip SHA and full diff against current `main` are recorded;
2. open PRs and CI/deployment references are checked;
3. useful, non-duplicated work is either merged through a reviewed PR or preserved in repository history/docs;
4. security-sensitive changes are reconciled against current code and tests;
5. the branch is confirmed not to contain the only reference to required work.

Do not merge a historical branch merely to reduce the branch count. Do not force-move refs or use ref updates as a substitute for deletion. After authorized deletion, re-list branches and verify the exact nine-name invariant.

## Current non-canonical inventory (35)

### Authorization, protected-data and policy-kernel priority review
- `feat/canonical-principal-subject-scope-v3`
- `feat/canonical-principal-subject-scope-v4-clean`
- `feat/python-authorization-subject-convergence-v1`
- `feature/policy-kernel-grant-identity-hardening`
- `feature/shared-policy-kernel-hardening`
- `feature/shared-policy-kernel-hardening-v2`
- `fix/authorization-subject-context-contract-v1-1`
- `fix/governed-executor-no-bypass-v1`
- `fix/protected-db-context-rustfmt-v1`
- `security/legacy-data-classification-quarantine-v1`
- `security/legacy-data-classification-quarantine-v1-clean`
- `security/protected-context-conformance-v1`
- `security/protected-data-bypass-audit-v1`
- `security/protected-data-bypass-audit-v1-main`
- `security/vault-list-auth-order-v1`
- `feature/health-state-evidence-schema-v1`
- `arch/capability-registry-canonicalization-v1`
- `architecture/local-health-vault-storage-index-v1`

### Health, evidence, research and capability work requiring diff-level disposition
- `arch/behavioral-evidence-epistemic-layer-v1`
- `arch/canonical-governed-operation-contract-v1`
- `architecture/health-media-intelligence-v1`
- `architecture/local-health-vault-contract-v1` — source of open draft PR #164; do not retire until PR disposition and useful contract content are preserved
- `audit/evidence-canonicalization-v1`
- `feat/unpaywall-access-location-adapter-v1`
- `impl/evidence-assessment-chain-hardening-v1`

### Design/governance documentation requiring preservation check
- `design/canonical-domain-runtime-polyglot-v1`
- `design/canonical-domain-runtime-polyglot-v1-main`
- `design/db-constraints-trusted-context-rls-v1`
- `design/hybrid-web-platform-v1`
- `design/identity-authorization-enforcement-v1`
- `design/legacy-scope-classification-quarantine-v1`
- `design/protected-data-access-enforcement-v1`
- `design/trusted-db-service-identity-context-v1`
- `governance/automation-architecture-v1`

### Dependency branch requiring post-squash retirement verification
- `dependabot/cargo/services/backend-rust/rust-dependencies-0c8e3c0fe1` — PR #173 is merged, but the branch ref remains. Confirm no unique unmerged content before retirement.

## Verified implementation evidence — vault list authorization

On `main`, `services/backend-rust/src/local_health_vault_storage_store.rs` checks per-record `VaultAction::Read` before resolving that record's key and decrypting its index in `list()`, then validates the decrypted index binding and subject. The regression test is present through `services/backend-rust/src/local_health_vault_storage_tests.rs`, which imports `local_health_vault_storage_list_authorization_tests.rs`; that file tests that denied records do not trigger key resolution. This is evidence for the tested authorization-ordering invariant, not proof of all vault security objectives.

## PR #164 — keep draft pending evidence mapping

PR #164 adds `docs/architecture/local-health-vault-v1.md`. A review comment has been posted asking for a requirement-to-implementation/test/CI matrix and for each claim to be labeled implemented, partial, or unproven. Existing code already provides encrypted index storage, AES-256-GCM authenticated encryption for the index, record/index binding checks, and authorization-before-key-resolution paths. The contract still needs evidence mapping for key lifecycle/recovery, durable audit consistency, backup/restore, deletion semantics, sharing/export, and external-AI policy enforcement. Do not merge solely because the document describes a MUST.

## Tooling limitation and completion invariant

The connected GitHub tools available in this session expose branch listing/search and ref updates but no safe branch-delete mutation. Ref movement must not be used as a substitute for deletion. Therefore physical branch cleanup is blocked until an authorized GitHub branch-delete capability or an authorized maintainer operation is available.

**Completion invariant:** the live GitHub branch listing must contain exactly the nine canonical names above. At this baseline it contains 44 branches, so the invariant is not satisfied.
