# SOMA Branch Lifecycle and Retirement Policy

**Status:** Active  
**Version:** 0.2.0

## Authority boundary

This is a repository governance policy. It governs branch lifecycle and retirement; it is not a SOMA technical architecture contract.

## Principle

`main` is the canonical verified ecosystem state. Branches exist to isolate
work, not to become permanent parallel product lines.

## Canonical branch set

SOMA maintains **exactly nine persistent branches**:

1. `main`
2. `development`
3. `security/current`
4. `architecture/current`
5. `feature/current`
6. `health/current`
7. `research/current`
8. `ai-agent/current`
9. `integration/current`

These branches are persistent coordination lanes, not independent products or
parallel architectural implementations.

All other branches are temporary work branches or historical cleanup
candidates. They must be merged into a canonical branch when their work is
still required, or retired safely when their work is already represented in
canonical history or an approved archive.

The nine-branch invariant is a repository governance requirement:

`persistent branches = 9`

No additional permanent branch may be introduced without an explicit governance
decision that also retires one of the existing persistent branches.

## Lifecycle

```text
Create
  ↓
Develop
  ↓
Audit
  ↓
Verify
  ↓
PR
  ↓
CI
  ↓
Merge
  ↓
Retire
```

## Temporary branch rule

A temporary implementation branch must be:

- created from a verified canonical base;
- scoped to one bounded change;
- audited before merge;
- verified by required CI;
- merged only after the exact head has been verified;
- retired immediately after successful merge.

Temporary branches must not accumulate into permanent `v1`, `v2`,
`-clean`, `-main`, or equivalent branch families.

## Retirement gate

A branch may be retired only when all of the following are verified:

- no open PR depends on it;
- it is not the default branch;
- no unique commits remain that are not represented in `main` or an approved
  archive/tag;
- no release, deployment, workflow, or documented procedure points to it;
- any useful work has been preserved in canonical history or an explicit
  archive;
- deletion is performed through a supported Git operation.

## Archive-first rule

Do not delete a branch merely because it appears old. Compare it with `main`,
inspect its unique commits when present, and preserve evidence of the decision.

A branch that is fully behind `main` with zero unique commits is normally safe
to retire after dependency verification.

For historical branches containing unique commits, do not force-merge the
branch solely to reduce branch count. First determine whether its substantive
work is already represented elsewhere. Preserve any genuinely unique useful
material in canonical files, commits, or an approved archive before retirement.

## Current cleanup policy

The repository currently contains historical branches beyond the nine
canonical branches. They are subject to an archive-first retirement pass.

The cleanup order is:

1. inventory every branch;
2. compare every non-canonical branch with `main`;
3. inspect unique commits and dependencies;
4. classify each branch as:
   - merge required,
   - archive/preserve then retire,
   - redundant and safe to retire;
5. preserve evidence of the classification;
6. retire only through a supported Git branch-deletion operation;
7. re-inventory the repository and require exactly the nine canonical branches.

Until a supported deletion operation is available, historical branches must
not be force-moved, force-merged, or otherwise destroyed as a workaround.

## Ecosystem branch strategy

Prefer bounded capability work through the canonical lanes:

```text
canonical lane
      ↓
temporary implementation branch
      ↓
audit + exact-head CI
      ↓
PR
      ↓
merge
      ↓
immediate retirement
```

The canonical lanes remain the only persistent organizational branches.

## Local cleanup

After remote retirement, contributors should run:

```text
git fetch origin --prune
git branch -d <branch>
```

Force deletion is prohibited unless an explicit recovery decision has been
recorded.
