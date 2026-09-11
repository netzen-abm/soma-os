# SOMA-OS Documentation Authority Registry v1

**Status:** governing documentation map  
**Date:** 2026-09-11

## Purpose

This registry defines where a human, developer, or AI agent should look for a concept before creating or changing documentation. It is a navigation and authority map, not a duplicate specification.

## Authority hierarchy

| Level | Authority | Use |
|---|---|---|
| 1 | `schemas/*.json` + executable contract tests | Machine-readable domain/data semantics |
| 2 | `docs/architecture/` | System and domain architecture contracts |
| 3 | `docs/decisions/` | Durable architectural/product decisions and rationale |
| 4 | `docs/product/` | Product behavior and product contracts |
| 5 | implementation docs + source code | Current implementation realization |
| 6 | `docs/PRODUCT-READINESS-MASTER-CHECKLIST.md` | Readiness/status truth |
| 7 | `docs/research/` + `docs/evidence-sources/` | Research findings and source-level evidence |
| 8 | `docs/SOMA-OS-PROJECT-MEMORY.md` + `docs/archive/` | Historical context |

A lower-level document must not silently redefine a higher-level authority.

## Core canonical map

### System

- `README.md` — public repository entry point and product boundary.
- `docs/architecture/SOMA-SYSTEM-ARCHITECTURE-PRODUCT-SPEC-v1.md` — whole-system architecture baseline.
- `docs/architecture/shared-infrastructure-charter.md` — shared-first architectural rule.
- `docs/architecture/canonical-domain-runtime-polyglot-v1.md` — canonical/reference runtime and polyglot boundary.

### Security and governance

- `docs/architecture/identity-authorization-enforcement-v1.md` and its implementation specification — identity/authorization boundary.
- `docs/architecture/protected-data-access-enforcement-v1.md` and its implementation specification — protected-data access boundary.
- Policy Kernel documents — centralized authorization policy authority.
- `docs/architecture/automation-governance-gate-v1.md` — automation governance boundary.
- `docs/governance/branch-lifecycle.md` — branch lifecycle rules.
- `docs/governance/code-maintainability-policy.md` — maintainability rules.
- `docs/governance/cleanup-execution-plan.md` — repository cleanup, archive, deletion, CI, and merge-control process.

### Capability architecture

- `schemas/capability-registry-v1.json` / `services/shared/capability_registry.json` where present — machine-readable capability inventory.
- `docs/architecture/capability-adapter-contract.md` — adapter boundary.
- `docs/architecture/capability-assurance-requirements-v1.md` — assurance expectations.
- `docs/architecture/canonical-governed-capability-operation-v1.md` — governed operation lifecycle and authority gates.

### Health information architecture

- `docs/architecture/health-state-model-v1.md` — canonical personal health semantic model.
- `schemas/health-state-v1.json` — machine-readable Health State contract.
- `docs/architecture/health-evidence-graph-v1.md` — canonical general evidence model.
- `schemas/health-evidence-graph-v1.json` — machine-readable evidence contract.
- `docs/architecture/health-state-evidence-contracts-v1.md` — Health State/evidence separation and linkage.
- `docs/architecture/longitudinal-observation-timeline-v1.md` — longitudinal projection/timeline boundary; not a competing health-state model.
- `docs/architecture/personal-health-record-repository-v1.md` — local vault/index/reference boundary.
- `docs/architecture/longitudinal-evidence-driven-health-intelligence-v1.md` — future health-intelligence architecture baseline.
- `docs/architecture/health-context-framework-v1.md` — health-context contract.
- `docs/architecture/food-life-evidence-model.md` — Food–Life shared evidence/domain model; consumes canonical Health State and Evidence Graph semantics.

### Evidence and epistemics

- `docs/architecture/cross-paradigm-evidence-methodology-v1.md` — claim-level evidence assessment across knowledge systems.
- `docs/architecture/epistemic-context-mapping.md` — native knowledge-system/context mapping before comparison.
- `docs/architecture/behavioral-and-epistemic-evidence-layer-v1.md` — behavioral observation and epistemic metadata extension; it does not replace Health State or Evidence Graph.
- `schemas/epistemic-evidence-passport-v1.json` — machine-readable evidence-passport metadata contract.
- `schemas/behavioral-observation-v1.json` — machine-readable behavioral observation contract.

### Product and readiness

- `docs/PRODUCT-READINESS-MASTER-CHECKLIST.md` — authoritative readiness tracker.
- `docs/product/` — product-specific specifications.
- `docs/strategy/` — strategic direction; strategy is not architecture authority.
- `docs/ui-ux/` — interface behavior and presentation.

### History and decisions

- `docs/SOMA-OS-PROJECT-MEMORY.md` — consolidated project context/history.
- `docs/SOMA-OS-DECISION-LOG-2026-09-01-to-2026-09-08.md` — chronological decision history.
- `docs/decisions/` — durable decision records.
- `docs/archive/` — superseded historical material.

## Documentation citation hygiene

Repository documentation must use durable, independently resolvable references. Do not commit transient tool/chat citation artifacts such as `cite...`, `turn...` source identifiers, or other session-local references.

For external evidence, prefer:

- official source URLs;
- DOI;
- PMID/PMCID;
- ISBN or other stable bibliographic identifiers;
- repository-internal source IDs where applicable.

Repository citations must remain understandable and verifiable outside the chat/session in which they were created.

## Overlap rules

### Keep separate when authority differs

Examples:

- architecture baseline vs implementation specification;
- native epistemic context vs cross-paradigm evidence assessment;
- Health State semantics vs repository/storage boundary;
- project memory vs chronological decision log;
- product strategy vs technical architecture;
- architecture contracts vs governance/process controls.

### Consolidate when authority is actually duplicated

A document is a consolidation candidate when it:

1. defines the same concept;
2. has no distinct lifecycle or authority boundary;
3. duplicates the same normative rules;
4. has no unique implementation or historical purpose;
5. can be replaced safely by a link to the canonical document.

Do not consolidate solely because two documents use the same terminology.

## Required metadata for future canonical documents

Every new durable document should identify:

- Status
- Scope
- Authority level
- Canonical dependencies
- What it replaces, if anything
- What it explicitly does not replace
- Implementation status, if applicable
- Validation status, if applicable
- Supersession/archive status, if applicable

## Agent lookup protocol

Before creating a Markdown file:

```text
Search repository
  ↓
Search docs/architecture
  ↓
Search schemas and contract tests
  ↓
Search decisions
  ↓
Search product/research/governance
  ↓
Identify canonical authority
  ↓
Extend existing authority when appropriate
  ↓
Create new document only for a distinct authority boundary
```

## Reorganization safety

Do not rename, move, or delete a document merely to make the tree look cleaner. First verify:

- inbound links;
- CI/workflow references;
- code/test references;
- PR references;
- external links where known;
- historical significance;
- whether the destination already contains an authoritative document.

Archive before deletion.
