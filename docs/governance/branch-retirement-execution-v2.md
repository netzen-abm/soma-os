# Branch Retirement Execution v2

## Target

Exactly nine persistent active branches:
- main
- development
- security/current
- architecture/current
- feature/current
- health/current
- research/current
- ai-agent/current
- integration/current

No obsolete branch is merged merely to reduce branch count.

## Retirement classification

Every non-canonical branch must be classified:
- MERGE_REQUIRED — unique, still-required implementation.
- PRESERVE_THEN_DELETE — unique historical/security/audit/research material; preserve provenance, then retire.
- REDUNDANT_DELETE — fully superseded or duplicate.
- UNKNOWN — insufficient evidence; do not delete.

Deletion requires comparison with main, review of unique commits/files, preservation of unique history, no open PR dependency, dependency confirmation, and an authorized GitHub branch-delete operation.

## Current state

All nine canonical branches exist.

The current repository inventory contains 33 non-canonical branches requiring controlled retirement:
arch/behavioral-evidence-epistemic-layer-v1
arch/canonical-governed-operation-contract-v1
arch/capability-registry-canonicalization-v1
architecture/health-media-intelligence-v1
architecture/local-health-vault-contract-v1
architecture/local-health-vault-storage-index-v1
audit/evidence-canonicalization-v1
design/canonical-domain-runtime-polyglot-v1-main
design/canonical-domain-runtime-polyglot-v1
design/db-constraints-trusted-context-rls-v1
design/hybrid-web-platform-v1
design/identity-authorization-enforcement-v1
design/legacy-scope-classification-quarantine-v1
design/protected-data-access-enforcement-v1
design/trusted-db-service-identity-context-v1
feat/canonical-principal-subject-scope
feat/canonical-principal-subject-scope-v2
feat/canonical-principal-subject-scope-v3
feat/canonical-principal-subject-scope-v4-clean
feat/python-authorization-subject-convergence-v1
feat/unpaywall-access-location-adapter-v1
feature/health-state-evidence-schema-v1
feature/policy-kernel-grant-identity-hardening
feature/shared-policy-kernel-hardening
feature/shared-policy-kernel-hardening-v2
fix/protected-db-context-rustfmt-v1
governance/automation-architecture-v1
impl/evidence-assessment-chain-hardening-v1
security/legacy-data-classification-quarantine-v1-clean
security/legacy-data-classification-quarantine-v1
security/protected-context-conformance-v1
security/protected-data-bypass-audit-v1-main
security/protected-data-bypass-audit-v1

## Safety constraint

The connected GitHub capability currently does not expose an authorized remote branch-delete mutation. Do not force-move refs to simulate deletion. Execute deletion only through an authorized branch-delete path and then verify the repository has exactly nine branches.

## Shared-infrastructure constraint

Retained work must converge into the single SOMA shared infrastructure. No parallel identity, authorization, policy, Health State, observation, evidence, safety, causality, intervention/response/outcome, provenance, or protected-data foundation may be introduced.


## Evidence register — 2026-09-21

The retirement candidates were compared against `main`. The following branches are materially behind `main` and diverged. They are therefore **not** candidates for blind fast-forward merging.

### Architecture / design history
- `arch/behavioral-evidence-epistemic-layer-v1` — 4 unique commits; behavioral/epistemic evidence contracts.
- `arch/canonical-governed-operation-contract-v1` — 7 unique commits; governed operation contract.
- `arch/capability-registry-canonicalization-v1` — 17 unique commits; capability registry implementation.
- `architecture/health-media-intelligence-v1` — 2 unique commits; health-media architecture.
- `architecture/local-health-vault-contract-v1` — 1 unique commit; vault contract.
- `architecture/local-health-vault-storage-index-v1` — 4 unique commits; vault storage index.
- `audit/evidence-canonicalization-v1` — 1 unique commit; evidence canonicalization.
- `design/canonical-domain-runtime-polyglot-v1-main` — 3 unique commits; canonical runtime/ADR.
- `design/canonical-domain-runtime-polyglot-v1` — 3 unique commits; same architectural generation as the `-main` variant; preserve provenance, do not merge blindly.
- `design/db-constraints-trusted-context-rls-v1` — 1 unique commit; database trusted-context design.
- `design/hybrid-web-platform-v1` — 1 unique commit; hybrid web architecture.
- `design/identity-authorization-enforcement-v1` — 1 unique commit; identity/authorization design.
- `design/legacy-scope-classification-quarantine-v1` — 1 unique commit; legacy scope design.
- `design/protected-data-access-enforcement-v1` — 1 unique commit; protected-data design.
- `design/trusted-db-service-identity-context-v1` — 2 unique commits; trusted DB identity design.

### Authorization / feature generations
- `feat/canonical-principal-subject-scope` — 4 unique commits.
- `feat/canonical-principal-subject-scope-v2` — 4 unique commits and the same unique file surface as the prior generation; preserve provenance, do not merge both.
- `feat/canonical-principal-subject-scope-v3` — 22 unique commits; broad authorization/protected-data generation.
- `feat/canonical-principal-subject-scope-v4-clean` — 3 unique commits; later cleanup generation.
- `feat/python-authorization-subject-convergence-v1` — 6 unique commits; Python authorization convergence.
- `feature/policy-kernel-grant-identity-hardening` — 2 unique commits.
- `feature/shared-policy-kernel-hardening` — 3 unique commits.
- `feature/shared-policy-kernel-hardening-v2` — 4 unique commits; later generation of the same Policy Kernel surface.
- `fix/protected-db-context-rustfmt-v1` — 2 unique commits.
- `security/protected-context-conformance-v1` — 8 unique commits.
- `security/protected-data-bypass-audit-v1-main` — 3 unique commits.
- `security/protected-data-bypass-audit-v1` — 6 unique commits; later generation of the same audit surface.

### Health / evidence / integration
- `feat/unpaywall-access-location-adapter-v1` — 4 unique commits; research-source adapter.
- `feature/health-state-evidence-schema-v1` — 5 unique commits; earlier Health State/Evidence schema generation.
- `governance/automation-architecture-v1` — 2 unique commits; automation governance.
- `impl/evidence-assessment-chain-hardening-v1` — 1 unique commit; evidence assessment implementation.
- `security/legacy-data-classification-quarantine-v1-clean` — 4 unique commits; legacy classification quarantine.
- `security/legacy-data-classification-quarantine-v1` — 8 unique commits; later generation of the same quarantine/protected-context surface.

### Retirement disposition

The evidence supports a **PRESERVE_THEN_DELETE / REDUNDANT_DELETE** strategy rather than mass merging. The older branches contain architectural and security history, but their active branch presence is no longer justified after convergence into `main`.

No candidate should be merged solely to make its branch deletable.

Before physical deletion, unique historical commits and design decisions must remain recoverable through repository history, documentation, or another explicitly authorized archival mechanism.

