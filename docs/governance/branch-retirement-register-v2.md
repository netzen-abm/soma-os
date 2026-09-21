# SOMA Branch Retirement Register v2

## Verified state: 2026-09-21

Persistent branches are exactly these nine:

- main
- development
- security/current
- architecture/current
- feature/current
- health/current
- research/current
- ai-agent/current
- integration/current

The repository currently contains additional historical branches. They are not persistent development lanes.

## Open historical PRs audited

| PR | Branch | Disposition |
| --- | --- | --- |
| #81 | arch/behavioral-evidence-epistemic-layer-v1 | PRESERVE_THEN_DELETE |
| #80 | arch/capability-registry-canonicalization-v1 | REDUNDANT_DELETE after preserving history; canonical registry is now represented in main |
| #78 | arch/canonical-governed-operation-contract-v1 | REDUNDANT_DELETE after preserving history; governed operation contract exists on main |
| #64 | architecture/health-media-intelligence-v1 | PRESERVE_THEN_DELETE |
| #42 | design/canonical-domain-runtime-polyglot-v1-main | REDUNDANT_DELETE after preserving history; canonical runtime document exists on main |
| #41 | security/protected-data-bypass-audit-v1-main | REDUNDANT_DELETE after preserving history; audit implementation exists on main |
| #38 | security/legacy-data-classification-quarantine-v1-clean | REDUNDANT_DELETE after preserving history; legacy classification contract exists on main |
| #37 | security/protected-data-bypass-audit-v1 | REDUNDANT_DELETE after preserving history; audit implementation exists on main |
| #36 | governance/automation-architecture-v1 | PRESERVE_THEN_DELETE |
| #35 | design/legacy-scope-classification-quarantine-v1 | PRESERVE_THEN_DELETE |
| #34 | security/legacy-data-classification-quarantine-v1 | REDUNDANT_DELETE after preserving history; canonical implementation exists on main |
| #32 | fix/protected-db-context-rustfmt-v1 | REDUNDANT_DELETE after preserving history; protected DB implementation is represented in main |
| #31 | design/hybrid-web-platform-v1 | PRESERVE_THEN_DELETE |
| #30 | design/canonical-domain-runtime-polyglot-v1 | REDUNDANT_DELETE after preserving history; canonical runtime document exists on main |
| #28 | design/protected-data-access-enforcement-v1 | PRESERVE_THEN_DELETE; implementation history must remain recoverable until convergence evidence is archived |
| #25 | design/identity-authorization-enforcement-v1 | REDUNDANT_DELETE after preserving history; current authorization architecture supersedes the design |

## Evidence rule

A branch marked REDUNDANT_DELETE must not be force-merged simply to close its PR. Its historical commit/ref remains evidence until deletion capability is available.

A PRESERVE_THEN_DELETE branch has at least one artifact not currently present on main and therefore requires explicit archival/convergence evidence before retirement.

## Exact-nine rule

No historical branch is an allowed persistent lane.

No new versioned permanent branch names are permitted.

Task work must use a temporary branch, merge into the appropriate canonical lane, then delete the task branch.

## Current tooling limitation

The connected GitHub capability provides branch creation, comparison, ref movement, and repository reads, but no safe remote branch-delete operation. Do not simulate deletion by force-moving a ref.

## Required final operation when delete capability is available

1. verify no open PR depends on branch;
2. verify archival/convergence evidence;
3. close superseded PR if still open;
4. delete remote branch;
5. re-list branches;
6. assert canonical set equals exactly nine;
7. fail the cleanup if any non-canonical persistent branch remains.
