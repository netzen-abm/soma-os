# SOMA Branch Retirement Manifest

**Status:** Active cleanup register  
**Persistent branch invariant:** exactly 9

## Canonical persistent branches

- main
- development
- security/current
- architecture/current
- feature/current
- health/current
- research/current
- ai-agent/current
- integration/current

## Retirement candidates

The 33 non-canonical branches currently present in the repository are retirement candidates. They must not become persistent development lanes.

## Required disposition

Each candidate must be classified as:

1. **MERGE_REQUIRED** — unique substantive work remains required and is not represented in canonical history.
2. **PRESERVE_THEN_DELETE** — historical material has value but is preserved elsewhere before retirement.
3. **REDUNDANT_DELETE** — no unique required work and no dependency remains.
4. **UNKNOWN** — retain until evidence resolves uncertainty.

No candidate may be force-merged merely to reduce branch count.

## Deletion gate

Before deletion verify:

- no open PR depends on the branch;
- no deployment/workflow references it;
- no documented procedure references it as active;
- unique commits have been compared with main;
- useful unique work is preserved;
- the decision is recorded.

## Operational constraint

The connected GitHub integration currently exposes no remote branch-delete mutation. Until supported deletion is available, candidates must remain intact rather than being destroyed through force-ref manipulation.

## Target state

**Exactly 9 persistent branches.**

Any persistent branch outside the canonical set is a governance violation.
