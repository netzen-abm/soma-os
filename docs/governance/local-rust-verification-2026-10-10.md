# SOMA Local Rust Verification — 2026-10-10

## Scope

Follow-up evidence for PR #154 (`feature/current` head `e3f7ff4683140f39735c3e09c499c5b1c01fe379`). This note records only the local terminal output supplied for this cycle; it does not claim the PR head has changed or that checks passed.

## Observed results

- Local branch and remote branch were both at `e3f7ff4683140f39735c3e09c499c5b1c01fe379` before formatting.
- `cargo fmt --manifest-path services/backend-rust/Cargo.toml --all` was run.
- The resulting diff summary showed 34 files changed, 870 insertions, and 665 deletions.
- With end-of-line whitespace ignored, 33 files remained changed, with 867 insertions and 662 deletions. The diff therefore cannot be explained solely by CRLF/LF conversion.
- Git emitted repeated warnings that LF in the working copy would be replaced by CRLF.
- Local `cargo check --all-targets`, Clippy, and test compilation attempts stopped because the MSVC linker `link.exe` was not found. These checks are **blocked/unverified**, not passed.
- `branch-audit/` remains untracked and must be preserved; do not stage it as part of the Rust formatting change.

## Required disposition

1. Do not merge PR #154 based on these local results.
2. Review the formatter diff for unintended semantic edits, especially authorization, protected-data access, vault cryptography, longitudinal health repositories, and PHR repository contracts/tests.
3. Check repository and local Git line-ending configuration; do not normalize line endings broadly without a deliberate policy and reviewed diff.
4. Run checks from a Visual Studio Developer PowerShell with the required C++ build tools installed, or use the repository's supported CI/Linux environment:
   - `cargo fmt --manifest-path services/backend-rust/Cargo.toml --all -- --check`
   - `cargo check --manifest-path services/backend-rust/Cargo.toml --all-targets`
   - `cargo clippy --manifest-path services/backend-rust/Cargo.toml --all-targets -- -D warnings`
   - `cargo test --manifest-path services/backend-rust/Cargo.toml`
   - Required PostgreSQL protected-data isolation, cryptographic/integrity, and all exact-head PR gates.
5. Commit/push only after diff review and successful applicable validation. If the local toolchain remains unavailable, use CI to validate the exact pushed head; never treat skipped jobs as success.

## Architecture guardrails

- Preserve one SOMA ecosystem and shared infrastructure.
- Keep actor/principal and health/data subject distinct.
- Maintain authorization → governed operation → ProtectedDataAccess → trusted persistence context → storage provider.
- Do not introduce new abstractions or split modules unless an independent change, trust, persistence, provider, or reuse boundary justifies it.
- Do not merge or delete branches merely to hit the nine-branch target. Archive/reconcile unique behavior and history first.
