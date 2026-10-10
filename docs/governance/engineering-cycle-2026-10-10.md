# SOMA-OS Engineering Cycle — 2026-10-10

## Decision

Continue convergence and verification; do not merge a branch or delete a reference merely to satisfy the branch-count target. Preserve one SOMA ecosystem with shared health, wellness, nutrition, and research infrastructure. Decompose only where change, trust, persistence, provider dependency, or independent reuse creates a real boundary.

## Verified repository snapshot

- Repository: `netzen-abm/soma-os`
- Remote branch count: 40
- Canonical active branches: 9
- Noncanonical branches requiring disposition: 31
- Open PRs: #154 and #156
- All 40 branches were reported as unprotected by the GitHub branch listing.
- No branch was deleted or force-moved in this cycle.

Canonical active set:
`main`, `development`, `security/current`, `architecture/current`, `feature/current`, `health/current`, `research/current`, `ai-agent/current`, `integration/current`.

## PR #154 — governed authorization execution boundary

Head: `e3f7ff4683140f39735c3e09c499c5b1c01fe379`

Observed exact-head workflow evidence:
- Shared-infrastructure and health contract tests passed.
- Protected-data bypass audit passed.
- Authorization invariant, evidence pipeline, and local health-vault key lifecycle gates passed.
- Rust formatting check failed.
- Rust compilation and Clippy were skipped after formatting failed.
- The Personal Health Record repository workflow also failed at formatting; compile, repository tests, and lint were skipped.
- Cryptographic/integrity and PostgreSQL protected-data isolation jobs were skipped in the failing CI run.

Disposition: **do not merge yet**. Run the real formatter on the intended exact revision, review all resulting changes, and rerun the full required exact-head gates. Formatting is the observed blocker; unexecuted checks are not evidence of success or failure.

Review comment posted to PR #154 on 2026-10-10.

## PR #156 — Python actor/subject authorization convergence

Head: `160667d8aef542c6a1acb1dc35ede765eea33265`

Observed:
- PR remains open and is not mergeable.
- Its historical head is 364 commits behind current development and has no associated PR workflow runs.
- Actor/principal identity must remain distinct from the health/data subject.
- Grant identity and scope binding must be preserved across all protected-data call sites.

Disposition: **do not merge this historical patch as-is**. Re-derive the smallest semantic change against current `development`, inspect the canonical grant-key shape and every relevant call site, add focused regression tests, and require exact-head CI. Keep the branch until the security semantics have been recovered and reviewed.

Review comment posted to PR #156 on 2026-10-10.

## Architecture and verification invariants

1. One shared SOMA infrastructure; product surfaces and transport adapters do not create separate domain or policy authorities.
2. Authorization must precede governed execution and protected-data access.
3. Trusted persistence identity/context is established only after authorization; storage adapters do not independently grant access.
4. The Policy Kernel remains the sole policy evaluator.
5. Actor/principal and health/data subject remain distinct.
6. Personal health data remains separated from research data, with provenance, uncertainty, applicability, contradictions, and transformation lineage retained downstream.
7. Temporary permissions are purpose-bound and application-level; completion or failure revokes the lease, and future activation requires fresh consent/authorization. Operating-system permissions remain platform-controlled.
8. The health lifecycle remains observation → relationship/evidence → interpretation → action/intervention → outcome → new observation.
9. Split responsibilities only at independent boundaries of change, trust, persistence, provider dependency, or reuse. Do not split merely to reduce file or class size.
10. Prove existing contracts with tests before introducing new abstractions.

## Branch retirement policy

Each of the 31 noncanonical branches must be classified as:
- `MERGE_REQUIRED`: unique required behavior not yet represented canonically.
- `PRESERVE_THEN_DELETE`: useful historical/audit material has been preserved and active reference is no longer needed.
- `REDUNDANT_DELETE`: equivalent behavior and history are demonstrably redundant.
- `UNKNOWN`: evidence is insufficient; retain it.

Before retiring any branch:
1. Compare the complete file and commit history with canonical branches.
2. Verify behavior, tests, schemas, migrations, workflows, and security invariants—not just commit ancestry or filenames.
3. Preserve necessary history/provenance and confirm no open PR or dependency requires the branch.
4. Delete the local branch through VS Code/local Git as requested, then use an authorized GitHub branch-delete operation for its remote ref.
5. Re-query the remote inventory after each controlled batch.

The target is exactly the nine canonical branches. Never merge obsolete code merely to make a branch deletable. Never move a ref as a substitute for deletion.

## Current next actions

1. Resolve PR #154 formatting on the intended revision and complete all required Rust, integrity, database-isolation, and exact-head CI gates before considering merge.
2. Re-derive PR #156's actor/subject and grant-binding semantics against current development; test before merging.
3. Reconcile remaining security, capability-registry, health-state/evidence, vault, and research-adapter branch families; retire only evidence-backed redundant/superseded refs.
4. Configure branch protection for the nine canonical branches after confirming the required status-check names and integration workflow. Current GitHub inventory reports all 40 branches unprotected.

## Completion criteria

- Exactly the nine canonical branches remain on GitHub.
- Every retired branch's useful work is merged or intentionally preserved with provenance.
- No open PR or unique required implementation is orphaned by retirement.
- Exact-head CI and security review pass for merged changes.
- Shared authorization, protected-data, permission lifecycle, health/evidence, and persistence invariants are exercised by tests.
- No parallel authority or redundant orchestration is introduced.
