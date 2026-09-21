# SOMA-OS Branch Retirement Register v3

## Verified baseline

- Main: `2546202a436f0a3cc8b6ab9dd234095c422fc5fd`
- Physical branches: 36
- Canonical active branches: 9
- Non-canonical branches: 27
- Open pull requests: 0

## Canonical active set

1. main
2. development
3. security/current
4. architecture/current
5. feature/current
6. health/current
7. research/current
8. ai-agent/current
9. integration/current

## Retirement rule

A non-canonical branch may be retired only after its complete commit/file history has been inspected and its useful work is either:

- represented in a canonical branch and therefore no longer needs a branch ref;
- intentionally preserved as repository history or documentation;
- or explicitly classified as redundant.

Do not merge a historical branch merely to reduce the branch count.

## Verified duplicate

The following two branches currently point to the same commit and therefore contain no branch-tip distinction:

- feat/canonical-principal-subject-scope
- feat/canonical-principal-subject-scope-v2
- shared tip: `e1c15d39287029c989532e493abdc816cf7f04d1`

This is a strong retirement candidate, but physical deletion still requires an authorized GitHub branch-delete operation.

## Current limitation

The connected GitHub capability exposes branch creation, ref movement, search, and read operations but no safe branch-delete mutation. Ref movement must not be used as a substitute for deletion.

Therefore this register records the exact retirement state without falsely claiming that physical deletion has occurred.

## Completion invariant

The repository is complete only when the GitHub branch listing contains exactly the nine canonical names above.
