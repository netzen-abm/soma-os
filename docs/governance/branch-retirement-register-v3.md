# SOMA-OS Branch Retirement Register v3

## Branch disposition snapshot — 2026-10-10

- Repository: `netzen-abm/soma-os`
- Main/development SHAs above are historical register values, not a fresh ref snapshot; verify live SHAs before any branch mutation.
- Remote branches: 41 (live GitHub recheck, 2026-10-10)
- Canonical active branches: 9
- Noncanonical branches: 32
- Open pull requests: #156; PR #159 merged, with source-branch retirement still pending
- Branch protection status was not revalidated during this pass; inspect live settings before changing protections.
- No remote branch was deleted or force-moved during the 2026-10-10 cycle.

## Canonical active set — exactly nine

1. `main`
2. `development`
3. `security/current`
4. `architecture/current`
5. `feature/current`
6. `health/current`
7. `research/current`
8. `ai-agent/current`
9. `integration/current`

## Retirement policy

Every noncanonical branch must be classified before retirement:

- `MERGE_REQUIRED` — unique, required behavior is not yet represented in a canonical branch.
- `PRESERVE_THEN_DELETE` — useful history, design rationale, or audit evidence must first be preserved.
- `REDUNDANT_DELETE` — semantic equivalence and lack of required dependencies are demonstrated.
- `UNKNOWN` — evidence is insufficient; retain the branch.

Do not merge historical code merely to reduce branch count. Do not infer semantic equivalence from branch names, commit counts, ancestry, or matching filenames alone. Compare implementation behavior, tests, schemas, migrations, workflows, security properties, and provider contracts.

## Current noncanonical inventory

The following 32 noncanonical refs remain. Unless an entry below records a verified disposition, treat it as `UNKNOWN / HOLD` until its unique work is reconciled:

| Branch | Current disposition |
|---|---|
| `arch/behavioral-evidence-epistemic-layer-v1` | HOLD — evidence/epistemic contracts require semantic reconciliation |
| `arch/canonical-governed-operation-contract-v1` | HOLD — governed-operation contract requires comparison |
| `arch/capability-registry-canonicalization-v1` | HOLD — registry implementation, schema, tests, and CI require comparison |
| `architecture/health-media-intelligence-v1` | HOLD — health-media/research adapter work requires reconciliation |
| `architecture/local-health-vault-contract-v1` | HOLD — vault contract and key lifecycle require comparison |
| `architecture/local-health-vault-storage-index-v1` | HOLD — storage-index contract and tests require comparison |
| `audit/evidence-canonicalization-v1` | HOLD — evidence canonicalization and migration implications require review |
| `design/canonical-domain-runtime-polyglot-v1` | HOLD — diverged runtime/ADR history; preserve provenance |
| `design/canonical-domain-runtime-polyglot-v1-main` | HOLD — distinct diverged history; do not assume duplicate |
| `design/db-constraints-trusted-context-rls-v1` | HOLD — database trust/RLS requirements require verification |
| `design/hybrid-web-platform-v1` | PRESERVE_THEN_DELETE — design document copied to `docs/archive/branch-retirement/2026-10-10/hybrid-platform-design.md`; implementation and exact tip archive still required |
| `design/identity-authorization-enforcement-v1` | PRESERVE_THEN_DELETE — design document copied to `docs/archive/branch-retirement/2026-10-10/design-identity-authorization-enforcement-v1.md`; implementation and exact tip archive still required |
| `design/legacy-scope-classification-quarantine-v1` | PRESERVE_THEN_DELETE — design document copied to `docs/archive/branch-retirement/2026-10-10/design-legacy-scope-classification-quarantine-v1.md`; semantics and exact tip archive still required |
| `design/protected-data-access-enforcement-v1` | PRESERVE_THEN_DELETE — design document copied to `docs/archive/branch-retirement/2026-10-10/design-protected-data-access-enforcement-v1.md`; tests and exact tip archive still required |
| `design/trusted-db-service-identity-context-v1` | PRESERVE_THEN_DELETE — design document and ADR copied to `docs/archive/branch-retirement/2026-10-10/`; implementation and exact tip archive still required |
| `fix/governed-executor-no-bypass-v1` | RETIREMENT CANDIDATE — PR #159 merged into main; archive exact head `fffbf2d2b098612fe4dccd9b88c247773d339ab6`, verify no remaining dependencies, then delete source ref through reviewed local Git workflow |
| `feat/canonical-principal-subject-scope-v3` | HOLD — broad authorization/health/vault implementation; compare behavior and tests |
| `feat/canonical-principal-subject-scope-v4-clean` | HOLD — diverged later generation; compare authorization and outcome contracts |
| `feat/python-authorization-subject-convergence-v1` | HOLD — PR #156 open; do not delete before actor/subject semantics are re-derived |
| `feat/unpaywall-access-location-adapter-v1` | HOLD — research-source adapter and tests require reconciliation |
| `feature/health-state-evidence-schema-v1` | HOLD — schema/validator/CI equivalence requires verification |
| `feature/policy-kernel-grant-identity-hardening` | HOLD — grant identity and policy tests require comparison |
| `feature/shared-policy-kernel-hardening` | HOLD — earlier policy-kernel generation; compare with v2 and canonical code |
| `feature/shared-policy-kernel-hardening-v2` | HOLD — diverged later policy-kernel generation; verify all unique tests/workflows |
| `fix/protected-db-context-rustfmt-v1` | HOLD — database context and integration-test differences require review |
| `governance/automation-architecture-v1` | HOLD — governance rationale/readiness checklist needs preservation decision |
| `impl/evidence-assessment-chain-hardening-v1` | HOLD — evidence-assessment implementation and tests require reconciliation |
| `security/legacy-data-classification-quarantine-v1` | HOLD — classification/quarantine and Rust/CI changes require comparison |
| `security/legacy-data-classification-quarantine-v1-clean` | HOLD — later quarantine generation; test-level equivalence is unproven |
| `security/protected-context-conformance-v1` | HOLD — conformance tests and main entry point require comparison |
| `security/protected-data-bypass-audit-v1` | HOLD — audit scripts/workflows/security coverage require comparison |
| `security/protected-data-bypass-audit-v1-main` | HOLD — distinct migration/database/integration changes; not a name-only duplicate |

## Previously retired duplicate pair

The following pair was verified to share tip `e1c15d39287029c989532e493abdc816cf7f04d1`, archived, and deleted from the remote:

- `feat/canonical-principal-subject-scope`
- `feat/canonical-principal-subject-scope-v2`

Archive tag: `archive/retired/feat-canonical-principal-subject-scope-v1-2026-10-09`.

This pair is historical evidence of the archive-first procedure, not authorization to delete other similarly named branches.

## Historical merged PR gate

### PR #154 — `feature/current` → `main` (historical; merged after subsequent review)

The details below describe the earlier exact-head findings and must not be interpreted as the current status of main.



Head: `e3f7ff4683140f39735c3e09c499c5b1c01fe379`.

Observed on exact head `e3f7ff4683140f39735c3e09c499c5b1c01fe379`: shared-infrastructure/health contract tests, protected-data bypass audit, authorization invariant gate, evidence pipeline, and local vault key lifecycle gates passed. Rust formatting failed with concrete rustfmt diffs across responsibility modules, PHR repository implementation/security tests, and PostgreSQL test support; compilation and Clippy were skipped, and cryptographic/integrity plus PostgreSQL isolation jobs were skipped because they depend on the failed Rust gate. Local Windows compile attempts are separately blocked by missing MSVC `link.exe`. Review finding: `services/shared/protected_data_access.py` routes `permission is None` through `authorize()` then directly invokes the adapter, bypassing the governed executor/permission lifecycle. Resolve that path and add read/insert/update/delete denial-before-adapter and exactly-once tests. Do not merge before reviewed code changes and all required exact-head gates pass.

## Open pull request gate

### PR #156 — Python actor/subject authorization

Head: `160667d8aef542c6a1acb1dc35ede765eea33265`, base `development`. Historical head is 364 commits behind current development and has no associated PR workflow runs. Keep the actor/principal distinct from the health/data subject and preserve grant-key binding. Re-derive the minimal semantic change on current development, add focused tests, and require exact-head CI; do not merge this historical patch as-is.

## Safe retirement procedure

1. Fetch the current remote refs and confirm the candidate is not one of the nine canonical branches.
2. Review the complete diff/history and classify the candidate. Resolve open PRs and confirm all dependencies.
3. Preserve the exact branch tip with an annotated archive tag and push/verify the tag before deletion.
4. Delete only explicitly approved refs through local Git in VS Code/PowerShell, as directed by the repository owner. Never force-move a ref as a substitute for deletion.
5. Re-query GitHub after each controlled batch and run `scripts/validate_branch_hygiene.py`.
6. Stop immediately if the archive tag, remote head, open-PR state, or semantic disposition differs from the reviewed evidence.

Live recheck on 2026-10-10: GitHub lists 41 remote branches (the nine canonical names plus 32 noncanonical refs, including `fix/governed-executor-no-bypass-v1`). The connected GitHub mutation capability in this workflow does not provide a safe remote branch-delete operation; use VS Code/PowerShell for candidate local deletion as directed, archive and verify history first, then retire remote refs through the reviewed local Git workflow. Do not claim exact-nine completion until the live remote listing contains exactly the nine canonical names.

## Completion invariant

The repository is complete only when exactly the nine canonical branch names remain on GitHub, retired work remains recoverable, no required implementation or PR is orphaned, and required exact-head security/CI gates pass.


## Live recheck addendum — 2026-10-15

This addendum supersedes earlier snapshot counts and PR-state statements above for current-state decisions. Earlier sections are retained as historical audit evidence.

- Remote branch inventory: **42 refs**.
- Canonical active branches: **9**.
- Noncanonical refs requiring disposition: **33**.
- `main` head observed by compare: `cb9c906d24cc3b51e4f0b6b24478840ee0ad22c2`.
- `development` head observed by compare: `32bcc8e7a4207d279743d62ea4c8ad171e3a04a7`.
- Merge base: `13b0b4ac2954f2d0ad2171abeb24e9f12adf9444`.
- Divergence: development is 155 commits ahead of main; main is 26 commits ahead of development.
- Latest open-PR search returned no open PRs. PR #156 is **closed, not merged**; its historical branch must not be treated as an active open-PR dependency, but its actor/subject semantics still require recovery into the current contract if not already present. PR #159 is merged; its source branch remains a retirement candidate only after exact-tip archival and dependency verification.
- The live inventory is the authority for branch names; this register is not itself a deletion authorization.

### Live noncanonical branch inventory

Every branch below remains `UNKNOWN / HOLD` until exact tip, unique diff, dependencies, archival evidence, and disposition are verified:

- `arch/behavioral-evidence-epistemic-layer-v1`
- `arch/canonical-governed-operation-contract-v1`
- `arch/capability-registry-canonicalization-v1`
- `architecture/health-media-intelligence-v1`
- `architecture/local-health-vault-contract-v1`
- `architecture/local-health-vault-storage-index-v1`
- `audit/evidence-canonicalization-v1`
- `design/canonical-domain-runtime-polyglot-v1-main`
- `design/canonical-domain-runtime-polyglot-v1`
- `design/db-constraints-trusted-context-rls-v1`
- `design/hybrid-web-platform-v1`
- `design/identity-authorization-enforcement-v1`
- `design/legacy-scope-classification-quarantine-v1`
- `design/protected-data-access-enforcement-v1`
- `design/trusted-db-service-identity-context-v1`
- `feat/canonical-principal-subject-scope-v3`
- `feat/canonical-principal-subject-scope-v4-clean`
- `feat/python-authorization-subject-convergence-v1`
- `feat/unpaywall-access-location-adapter-v1`
- `feature/health-state-evidence-schema-v1`
- `feature/policy-kernel-grant-identity-hardening`
- `feature/shared-policy-kernel-hardening`
- `feature/shared-policy-kernel-hardening-v2`
- `fix/authorization-subject-context-contract-v1-1`
- `fix/governed-executor-no-bypass-v1`
- `fix/protected-db-context-rustfmt-v1`
- `governance/automation-architecture-v1`
- `impl/evidence-assessment-chain-hardening-v1`
- `security/legacy-data-classification-quarantine-v1`
- `security/legacy-data-classification-quarantine-v1-clean`
- `security/protected-context-conformance-v1`
- `security/protected-data-bypass-audit-v1`
- `security/protected-data-bypass-audit-v1-main`

### Next execution gates

1. Reconcile main-only changes by file semantics and behavior; do not merge based on commit counts.
2. Add evidence-link repository write/readback to the existing longitudinal integration test, reusing the existing encrypted vault and canonical authorization boundary.
3. Replace the temporary Rust formatting workflow's direct security-branch push with a non-mutating check through a reviewed change.
4. Archive and verify exact tips before retiring any noncanonical branch.
5. Run `scripts/validate_branch_hygiene.py` against freshly fetched remote refs; only declare completion when the live remote count is exactly nine and all required checks pass.

No branches were deleted or merged during this recheck. No code tests were run by the GitHub connector.
