# SOMA-OS Branch Retirement Register v3

## Verified baseline — 2026-10-10

- Repository: `netzen-abm/soma-os`
- Main: `78dd05e67b410264e85e929d4bb4a208b4fa7d6b`
- Development: `63b9dc6f76d33f5557e7ca293afe6dbe2160a13d`
- Remote branches: 40
- Canonical active branches: 9
- Noncanonical branches: 31
- Open pull requests: #154 and #156
- GitHub branch listing reports all 40 branches as unprotected.
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

The following 31 refs remain. Unless an entry below records a verified disposition, treat it as `UNKNOWN / HOLD` until its unique work is reconciled:

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
| `design/hybrid-web-platform-v1` | HOLD — architecture decision requires disposition |
| `design/identity-authorization-enforcement-v1` | HOLD — identity/authorization design requires reconciliation |
| `design/legacy-scope-classification-quarantine-v1` | HOLD — legacy classification semantics require verification |
| `design/protected-data-access-enforcement-v1` | HOLD — protected-data access requirements require verification |
| `design/trusted-db-service-identity-context-v1` | HOLD — trusted persistence context requirements require verification |
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

## Open pull request gates

### PR #154 — `feature/current` → `main`

Head: `e3f7ff4683140f39735c3e09c499c5b1c01fe379`.

Observed: shared-infrastructure/health contract tests, protected-data bypass audit, authorization invariant gate, evidence pipeline, and local vault key lifecycle gates passed. Rust formatting failed; compilation and Clippy were skipped, and cryptographic/integrity plus PostgreSQL isolation jobs did not complete. Run the real formatter on the intended revision, inspect the full diff, and rerun all required exact-head gates. Do not merge before they pass.

### PR #156 — Python actor/subject authorization

Head: `160667d8aef542c6a1acb1dc35ede765eea33265`, base `development`. Historical head is 364 commits behind current development and has no associated PR workflow runs. Keep the actor/principal distinct from the health/data subject and preserve grant-key binding. Re-derive the minimal semantic change on current development, add focused tests, and require exact-head CI; do not merge this historical patch as-is.

## Safe retirement procedure

1. Fetch the current remote refs and confirm the candidate is not one of the nine canonical branches.
2. Review the complete diff/history and classify the candidate. Resolve open PRs and confirm all dependencies.
3. Preserve the exact branch tip with an annotated archive tag and push/verify the tag before deletion.
4. Delete only explicitly approved refs through local Git in VS Code/PowerShell, as directed by the repository owner. Never force-move a ref as a substitute for deletion.
5. Re-query GitHub after each controlled batch and run `scripts/validate_branch_hygiene.py`.
6. Stop immediately if the archive tag, remote head, open-PR state, or semantic disposition differs from the reviewed evidence.

The connected GitHub mutation capability does not provide a safe remote branch-delete operation in this workflow. Do not claim exact-nine completion until the live remote branch listing contains exactly the nine canonical names.

## Completion invariant

The repository is complete only when exactly the nine canonical branch names remain on GitHub, retired work remains recoverable, no required implementation or PR is orphaned, and required exact-head security/CI gates pass.
