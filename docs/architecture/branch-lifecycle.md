# SOMA Branch Lifecycle and Retirement Policy

**Status:** Active
**Version:** 0.1.0

## Principle

`main` is the canonical verified ecosystem state. Branches exist to isolate
work, not to become permanent parallel product lines.

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

## Ecosystem branch strategy

Prefer short-lived capability branches:

```text
main
  └── feature/<capability>
          ↓
        PR + CI
          ↓
        main
```

Use longer-lived integration branches only when the work cannot be safely
broken into independently verifiable capability increments.

## Local cleanup

After remote retirement, contributors should run:

```text
git fetch origin --prune
git branch -d <branch>
```

Force deletion is prohibited unless an explicit recovery decision has been
recorded.
