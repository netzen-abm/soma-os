# SOMA-OS Documentation Content & Organization Audit — 2026-09-12

**Status:** Governance audit and organization baseline  
**Scope:** Markdown documentation on the canonical repository baseline and the current documentation-governance branch  
**Purpose:** Establish one navigable documentation system for humans, developers, and AI agents without losing historical material or creating duplicate authorities.

## 1. Audit method

The audit was performed in this order:

1. enumerate repository Markdown paths from the Git tree;
2. inspect root README, project memory, product-readiness checklist, architecture baseline, authority registry, AI/human/developer guide, governance documents, decision records, evidence-source documents, research/strategy indexes, and security/policy documents;
3. compare documents by authority, scope, lifecycle, implementation status, and repeated normative rules;
4. identify true duplicates versus legitimate adjacent documents with different authority boundaries;
5. verify proposed moves before changing paths;
6. preserve historical content and avoid deletion unless the replacement and archival evidence are established.

This audit distinguishes **content ownership** from **file placement**. A document should move only when its authority and lifecycle are clearer in the destination.

## 2. Current documentation authority

The repository uses this hierarchy:

```text
Schemas + executable contract tests
        ↓
Architecture contracts
        ↓
Durable decisions / ADRs
        ↓
Product contracts
        ↓
Implementation documents + source
        ↓
Product readiness
        ↓
Research / evidence sources
        ↓
Project memory / archive
```

The authority registry is the first lookup point. It must be updated whenever a canonical document is moved, renamed, superseded, or replaced.

## 3. Organization decisions

### Keep

- `README.md` — public repository/product entry point.
- `docs/PRODUCT-READINESS-MASTER-CHECKLIST.md` — readiness/status authority.
- `docs/SOMA-OS-PROJECT-MEMORY.md` — consolidated historical/project context.
- `docs/SOMA-OS-DECISION-LOG-2026-09-01-to-2026-09-08.md` — chronological decision history.
- `docs/architecture/` — architecture contracts and implementation-boundary specifications.
- `docs/decisions/` — durable decisions and ADRs.
- `docs/evidence-sources/` — source-level evidence and source assessments.
- `docs/research/` — research contributions and research-specific material.
- `docs/strategy/` — strategic direction, explicitly subordinate to architecture authority.
- `docs/product/` — product contracts/specifications.
- `docs/ui-ux/` — interface/experience specifications.
- `docs/operating-model/` — operating-model artifacts with a distinct process purpose.
- `docs/archive/` — historical/superseded material.
- `services/shared/` documentation that is implementation-local and directly paired with shared runtime modules.
- `services/evidence-research/` documentation that is specifically provider/orchestrator implementation documentation.

### Move / normalize

- `docs/food-life-evidence-model.md` → `docs/architecture/food-life-evidence-model.md` because its content is an architecture/domain model, not merely a source assessment.
- `docs/architecture/cleanup-execution-plan.md` → `docs/governance/cleanup-execution-plan.md` because it governs repository cleanup, validation, archive/delete, and merge procedure rather than technical architecture.

Both moves were verified on the documentation-governance branch before being treated as authoritative destinations.

### Do not merge merely for naming similarity

The following pairs are intentionally separate:

- whole-system architecture baseline vs shared-infrastructure charter;
- Health State semantic model vs PHR repository/storage boundary;
- longitudinal observation/timeline projection vs Health State canonical model;
- cross-paradigm evidence methodology vs epistemic-context mapping;
- evidence graph vs source-level evidence assessments;
- architecture contracts vs implementation specifications;
- project memory vs chronological decision log;
- product strategy vs technical architecture;
- governance/process guidance vs architecture;
- provider adapter documentation vs provider-neutral evidence architecture.

The distinction is authority/lifecycle, not terminology.

## 4. High-risk overlap areas requiring continued discipline

### Health State duplication risk

`schemas/health-state-v1.json` is the machine-readable canonical Health State contract. Longitudinal observation and behavioral observation documents must remain projections/extensions or clearly scoped records and must not silently become competing health-state authorities.

### Evidence duplication risk

The Health Evidence Graph is the canonical general evidence semantic model. Evidence-source documents, provider adapters, research orchestrators, and evidence passports must reference it rather than redefine its entity semantics.

### Authorization duplication risk

The Policy Kernel is the policy evaluator. Identity Authorization Enforcement is the identity/scope enforcement boundary. The Authorization + Policy Decision Boundary is the composition/authority boundary. None of these documents should introduce a second policy engine.

### Capability duplication risk

The capability registry is the canonical inventory. Capability adapter and assurance documents define boundaries; individual surfaces must not create private capability registries.

### AI/agent duplication risk

AI and agents are optional capabilities/adapters. They are not independent product authorities and must not create separate authorization, health-state, evidence, or privacy systems.

## 5. Policy Kernel documentation mismatch found during audit

The implementation currently declares `POLICY_VERSION = "0.4.0"`, while `services/shared/policy_kernel.md` still described itself as v0.3. This is a real documentation/runtime mismatch.

The documentation must be updated to describe the implemented v0.4 contract. The historical v0.3 resource-scope decision remains valuable as a decision record and should not be erased.

## 6. CI documentation and truthfulness rule

Repeated PR #82/#83 Actions runs failed before executable steps were assigned: jobs reported no runner identity, `steps: []`, and no usable logs. Such a run is an infrastructure/execution failure, not evidence that the underlying tests failed.

Documentation must therefore distinguish:

```text
CI passed
CI test/build failed
CI did not execute
CI infrastructure unavailable
```

Never report a pre-execution failure as a code-test failure or as a green validation result.

## 7. Current architecture/security baseline to preserve

The current shared security chain is:

```text
IdentityContext
  → Identity Authorization Enforcement
  → Authorization + Policy Decision Boundary
  → Policy Kernel
  → Governed Capability Operation / ProtectedDataAccess
  → Trusted Persistence Identity
  → Trusted Transaction Context
  → PostgreSQL RLS / constraints
  → Protected Data
  → Provenance / Audit
```

The authorization decision boundary composes existing authorities; it does not replace them.

## 8. Current SOMA operating rule

SOMA is one ecosystem. New work must follow:

```text
Shared capability
  → shared contract
  → shared policy/security
  → shared data/evidence semantics
  → shared execution
  → provenance/audit
  → adapters/surfaces
```

No separate application should duplicate a shared health, evidence, privacy, safety, authorization, provenance, or policy capability.

## 9. Required document metadata

Future durable documents should state, where applicable:

- Status
- Scope
- Authority
- Canonical dependencies
- Replaces / supersedes
- Explicit non-replacements
- Implementation status
- Validation status
- Archive/supersession status

## 10. Organization freeze

After the verified moves above, documentation organization should be **frozen** unless a concrete authority conflict, broken reference, duplicate canonical concept, or lifecycle problem is demonstrated.

Do not reorganize Markdown simply to make the directory tree visually cleaner.

## 11. Agent / developer lookup procedure

Before creating or editing a Markdown document:

```text
Search repository
  ↓
Read documentation authority registry
  ↓
Search schemas + executable tests
  ↓
Search architecture
  ↓
Search decisions
  ↓
Search product/research/governance
  ↓
Identify existing authority
  ↓
Extend existing document if authority is the same
  ↓
Create a new document only for a genuinely distinct authority boundary
```

Before moving a document:

```text
Content audit
  ↓
Authority audit
  ↓
Inbound/outbound reference audit
  ↓
CI/code/PR reference audit
  ↓
Destination conflict audit
  ↓
Atomic move
  ↓
Index/registry synchronization
  ↓
Verification
```

## 12. Final assessment

The repository has enough documentation to serve as institutional memory, but the primary remaining risk is not missing prose. It is **authority drift**: the same concept being described differently by a schema, architecture document, implementation note, checklist, or historical document.

The remedy is therefore not more documentation. It is:

1. one authority per concept;
2. explicit boundaries between adjacent concepts;
3. synchronized indexes;
4. truthful status reporting;
5. archive-before-delete;
6. periodic content audits when architecture materially changes.

**Decision:** use this audit plus the Documentation Authority Registry as the organization baseline. Freeze structure after the verified moves and concentrate future effort on implementation, validation, and product readiness.
