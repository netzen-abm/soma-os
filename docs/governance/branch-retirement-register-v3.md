# SOMA-OS Branch Retirement Register v3

## Verified baseline — 2026-10-23

- Repository: `netzen-abm/soma-os`
- Main: `bb4bf255959620ff93c55acbca9c2229a9975b92` (post-PR #164 vault-contract merge)
- Physical branches: **44**
- Canonical active branches: **9**
- Non-canonical branches: **35**
- Open pull requests: **2** — PR #177 (permission-lifecycle enforcement) and PR #178 (branch-retirement workflow/manifest), both draft. PR #164 merged as `bb4bf255959620ff93c55acbca9c2229a9975b92`.
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

## PR #164 — merged preservation contract

PR #164 merged into `main` on 2026-10-23 as `bb4bf255959620ff93c55acbca9c2229a9975b92`. The merge commit changes only `docs/architecture/local-health-vault-v1.md`. The document's 15-gate evidence matrix records initial implementation/test evidence and remaining proof; it does not certify all vault requirements as implemented. The source branch remains in the live inventory until a post-merge retirement workflow archives and deletes it, or an authorized maintainer does so.

## PR #177 — permission-lifecycle enforcement

PR #177's current exact head is `a42934e466039464b2203fce67b4265939c7c893`. Its five-file change binds the existing registry flag to the canonical governed executor and adds fail-closed tests for missing permission and mismatched permission metadata. Exact-head Evidence Pipeline, authorization gate, key-lifecycle gate, SOMA CI, and PR analysis were reported successful on the latest completed run. Durable revocation persistence and production device-adapter reachability remain unproven; no claim is made that this PR physically disables operating-system permissions.

## PR-driven archival and branch deletion

The repository is adopting a reviewed-PR-driven path rather than ref rewrites. `.github/workflows/retire-merged-branch.yml` archives a merged PR's noncanonical same-repository source head in a uniquely named recovery tag before deleting that branch. It rechecks the exact head SHA and other open PR references immediately before deletion; mismatches fail closed. The nine canonical branch names are excluded in code and covered by unit tests.

For existing noncanonical branches without a relevant merged PR, the workflow does not sweep refs automatically. Each branch requires diff classification, preservation/porting decisions, and a small retirement PR when safe. The post-merge action's archive tag preserves the complete head ancestry for recovery and records the disposition in issue #161.

The live GitHub integration used to prepare this change does not offer direct branch deletion. The workflow uses the narrowly scoped `GITHUB_TOKEN` with `contents: write` only after a PR targeting `main` has successfully merged. Failure to archive, verify the current SHA, or confirm no open PR dependency prevents deletion.

**Completion invariant:** the live branch listing must contain exactly the nine canonical names above. At this baseline it contains 44 branches (9 canonical + 35 noncanonical), so the invariant is not satisfied.


## Fresh high-risk branch comparison — current `main`

A live compare pass against current `main` confirms that several security-sensitive branches are highly divergent and must not be merged wholesale:

| Branch | Ahead | Behind | Unique scope visible in compare | Disposition |
|---|---:|---:|---|---|
| `feat/canonical-principal-subject-scope-v3` | 22 | 390 | Authorization, vault, protected DB context, health state/outcome and longitudinal repositories; 17+ changed files | Highest-risk review; isolate invariants and tests, no whole-branch merge |
| `feat/canonical-principal-subject-scope-v4-clean` | 3 | 367 | Canonical authorization and health-state outcome | Compare specific behavior/tests with current `main`; likely overlapping lineage, not presumed duplicate |
| `feat/python-authorization-subject-convergence-v1` | 6 | 313 | Shared policy kernel, protected-data access, authorization schema/tests | Review test coverage and semantics; port only missing proven behavior through PR |
| `feature/policy-kernel-grant-identity-hardening` | 2 | 988 | Policy kernel and tests | Very stale; inspect exact patch and retain only non-redundant invariant |
| `feature/shared-policy-kernel-hardening` | 3 | 988 | Capability registry and policy kernel/tests | Very stale; do not merge wholesale |
| `feature/shared-policy-kernel-hardening-v2` | 4 | 987 | Workflow, capability registry and policy kernel/tests | Very stale; compare with sibling branch and current implementation before disposition |
| `fix/authorization-subject-context-contract-v1-1` | 109 | 36 | Authorization schemas/scripts, many architecture and governance documents | Broad, divergent branch; do not merge wholesale; extract documents/patches only after current-state comparison |
| `fix/governed-executor-no-bypass-v1` | 2 | 11 | Governed executor and tests | Candidate for targeted behavioral review; verify invariant is not already covered |
| `fix/protected-db-context-rustfmt-v1` | 6 | 898 | Protected DB context and PostgreSQL integration tests | Extremely stale; extract only unique tests/required correctness changes |
| `security/protected-context-conformance-v1` | 8 | 556 | Protected context conformance tests | Review test gaps against current context API before porting |
| `security/protected-data-bypass-audit-v1` | 6 | 915 | Workflow, protected-data audit script and architecture note | Compare with `-main` sibling and current CI before retaining |
| `security/protected-data-bypass-audit-v1-main` | 3 | 877 | Same audit script/workflow/doc family | Compare exact content to sibling; no automatic deletion until unique work is ruled out |
| `security/legacy-data-classification-quarantine-v1` | 8 | 898 | Migration, Python classifier/tests, protected DB context | Stale multi-layer branch; review migration compatibility and tests separately |
| `security/legacy-data-classification-quarantine-v1-clean` | 4 | 915 | Migration, Python classifier/tests | Compare to sibling; retain one canonical implementation only after behavior-level review |
| `security/vault-list-auth-order-v1` | 6 | 5 | Vault list authorization-ordering test and store changes | Main already contains the tested authorization-before-key-resolution behavior; verify branch-only changes are represented before retiring |
| `arch/capability-registry-canonicalization-v1` | 17 | 628 | Shared capability registry/schema, Rust infrastructure, Python tests and workflow | Architecture-level diff; review contract consistency and test evidence before selective port |
| `architecture/local-health-vault-storage-index-v1` | 4 | 700 | Index schema, Python contract test, workflow and architecture doc | Check for requirements not already implemented; avoid parallel index abstraction |
| `feat/unpaywall-access-location-adapter-v1` | 4 | 335 | Research source adapter and contract test | Separate provider boundary may justify an adapter; verify current contract and preserve only unique behavior |
| `feature/health-state-evidence-schema-v1` | 5 | 971 | Health evidence schemas, validator and workflow | Stale; compare schemas with current canonical health model and validator before selective recovery |
| `dependabot/cargo/services/backend-rust/rust-dependencies-0c8e3c0fe1` | 3 | 8 | Cargo manifest and crypto formatting changes | PR #173 merged; verify no unique commit/content remains before retirement |

The ahead/behind figures are relative to the `main` ref at comparison time. They are not proof that every listed commit is unique or still useful. The correct operation is selective patch/content reconciliation followed by tested PRs—not merge-first branch cleanup.

## Canonical branch tip observation

The live branch listing shows `ai-agent/current`, `integration/current`, and `research/current` all point to commit `aed45e8ce19fb9b7c409ad73c4422e5b7da4139b`. This is not itself a violation of the nine-name invariant, but it means these three branch names currently do not represent distinct branch tips. Do not move them merely to make them look different; first verify the intended branch-role contract and whether separate active work is expected.

## CI follow-up

The branch-register update commit is `89e555c3232eecddd0100a8c46d6c86b0dff80a1`. At the last live check, its check runs were queued/in progress and no completed success was established by that response. Recheck the commit's checks before treating the register update as fully gated.


## PR #178 — SHA-pinned branch-retirement batch

PR #178 adds the main-push retirement workflow and a SHA-pinned manifest for 33 noncanonical refs captured from the live inventory. The manifest excludes the two currently open PR source branches. The source branch of the merged retirement PR is handled separately from manifest targets.

Every manifest entry carries its expected tip SHA, disposition, reason, and evidence reference. The workflow checks exact live tip, explicit unprotected status, open PR references, and tracked automation/config references; creates and verifies a deterministic recovery tag for every target; rechecks all preconditions; then deletes eligible refs and verifies deletion. If any preflight target fails, no branch refs are deleted. The workflow is idempotent when an already deleted ref has its exact recovery tag.

Historical branches are not merged wholesale into `main`. Their reachable commit graphs remain recoverable through archive tags, and required functionality must be selectively extracted through ordinary reviewed PRs. The completion check is a fresh branch listing containing exactly the nine canonical names after PR #178 and PR #177 post-merge cleanup runs succeed.
