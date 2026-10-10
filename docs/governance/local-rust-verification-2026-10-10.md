# SOMA Local Rust Verification — 2026-10-10

## Scope

Follow-up evidence for PR #154 (`feature/current` head `e3f7ff4683140f39735c3e09c499c5b1c01fe379`). This note records only supplied terminal output and the live GitHub review; it does not claim the PR head has changed or that checks passed.

## Observed results

- Local branch and remote branch were both at `e3f7ff4683140f39735c3e09c499c5b1c01fe379` before formatting.
- `cargo fmt --manifest-path services/backend-rust/Cargo.toml --all` was run.
- The resulting diff summary showed 34 files changed, 870 insertions, and 665 deletions.
- With end-of-line whitespace ignored, 33 files remained changed, with 867 insertions and 662 deletions. The diff therefore cannot be explained solely by CRLF/LF conversion.
- Git emitted repeated warnings that LF in the working copy would be replaced by CRLF.
- Local `cargo check --all-targets`, Clippy, and test compilation attempts stopped because the MSVC linker `link.exe` was not found. These checks are **blocked/unverified**, not passed.
- A subsequent pasted run executed `cargo fmt --check`, `cargo check`, Clippy, and tests in sequence. The captured output shows no rustfmt diagnostic before compilation began, but does not include explicit per-command exit codes; treat formatting as likely clean but not independently exit-code verified.
- In that subsequent run, compile/check, Clippy, and test attempts again failed because `link.exe` was unavailable. VS Code alone does not provide the linker.
- `branch-audit/` remains untracked and must be preserved; do not stage it as part of the Rust formatting change.
- Live GitHub recheck: PR #154 remains open and unmerged at the same head SHA. The exact-head Rust CI job failed at formatting, so compile and Clippy were skipped; cryptographic/integrity and PostgreSQL protected-data isolation jobs were skipped. Shared-infrastructure/health contract tests, the protected-data bypass audit, authorization invariant gate, evidence pipeline, and local vault key lifecycle gate passed. Those successes do not substitute for skipped gates.
- Live branch inventory remains 40 remote branches: nine canonical branches plus 31 noncanonical branches. No branch was deleted in this cycle.

## Code-review finding — PR #154

The PR patch changes `ProtectedDataAccess._execute` such that, when `request.permission is None`, it calls `authorize(request)` and invokes the adapter operation directly rather than passing through the governed capability executor and permission lifecycle. This is a review concern inferred from the patch, not a claim that CI demonstrated exploitation.

Before merge, explicitly resolve this path. Either route all protected-data operations through the canonical governed execution boundary, or prove that the path is outside protected-data access and move it to an accurately named boundary. Add regression tests for read/insert/update/delete that verify denial happens before adapter invocation. For permission-bound operations, test actor/principal and subject binding, scope/tenant/data-domain binding, and exactly-once execution of the adapter callback. Do not introduce a second policy evaluator.

## Required disposition

1. Keep PR #154 unmerged.
2. Review the formatter diff for unintended semantic edits, especially authorization, protected-data access, vault cryptography, longitudinal health repositories, and PHR contracts/tests.
3. Check repository and local Git line-ending configuration; do not normalize line endings broadly without a deliberate policy and reviewed diff.
4. Run checks using the repository's supported GitHub Actions/Linux environment, or a properly configured Visual Studio Developer PowerShell:
   - `cargo fmt --manifest-path services/backend-rust/Cargo.toml --all -- --check`
   - `cargo check --manifest-path services/backend-rust/Cargo.toml --all-targets`
   - `cargo clippy --manifest-path services/backend-rust/Cargo.toml --all-targets -- -D warnings`
   - `cargo test --manifest-path services/backend-rust/Cargo.toml`
   - Required PostgreSQL protected-data isolation, cryptographic/integrity, and all exact-head PR gates.
5. Commit/push only reviewed changes. If the local toolchain remains unavailable, use CI to validate the exact pushed head; never treat skipped jobs as success.

## Architecture guardrails

- Preserve one SOMA ecosystem with shareable health, wellness, nutrition, and research infrastructure.
- Keep actor/principal and health/data subject distinct.
- Maintain authorization → governed operation → ProtectedDataAccess → trusted persistence context → storage provider.
- The Policy Kernel remains the sole policy evaluator; adapters, models, agents, and transport surfaces do not become security authorities.
- Split responsibilities only where change, trust, persistence, provider dependency, or independent reuse creates a real boundary. Keep tightly coupled invariants together.
- Prove existing contracts with tests before introducing abstractions.
- Do not merge or delete branches merely to hit the nine-branch target. Archive and reconcile unique behavior/history first.
