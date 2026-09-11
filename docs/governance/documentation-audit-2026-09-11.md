# SOMA-OS Documentation Audit — 2026-09-11

**Audit basis:** repository tree at `main` commit `41f1e151f080b8c27413e19dea27e9e0ba3a...` as inspected during this documentation pass.

## Audit objective

Prevent documentation duplication, preserve institutional memory, and establish clear authority for humans, developers, and AI agents.

## Repository observations

The repository already has meaningful documentation domains: architecture, decisions, evidence sources, research, strategy, product, UI/UX, operating model, and archive. The main weakness is not absence of documentation; it is **authority overlap and discoverability**. The architecture directory contains many highly specific documents, some of which describe adjacent lifecycle stages or implementation details.

The current repository tree also contains active code and schema areas for apps, services, database, health vault, protocols, operations, scripts, and schemas. This reinforces the need for documentation to explain ownership rather than reproduce code semantics. fileciteturn391file0

## Authority model adopted

| Information | Canonical authority |
|---|---|
| Machine-readable data shape | `schemas/*.json` + contract tests |
| System architecture | `docs/architecture/` canonical contract |
| Durable design decision | `docs/decisions/` |
| Product behavior | `docs/product/` |
| Research finding | `docs/research/` / `docs/evidence-sources/` |
| Product readiness | `docs/PRODUCT-READINESS-MASTER-CHECKLIST.md` |
| Historical project context | `docs/SOMA-OS-PROJECT-MEMORY.md` and archive |
| UI behavior | `docs/ui-ux/` |
| Contributor/AI workflow | `docs/governance/` |

## Duplicate/overlap analysis

### 1. Project memory vs decision log

These should **not** be merged. Project Memory is a consolidated historical context; the Decision Log preserves chronological decisions. The new documentation index treats both as complementary rather than competing authorities.

### 2. System architecture spec vs Shared Infrastructure Charter

These overlap on shared-first principles, but they have different scope. The charter is a foundational rule; the system architecture spec is the whole-system architecture baseline. Keep both and make the architecture index establish their relationship.

The system architecture explicitly defines shared-first, one canonical domain, policy-before-execution, evidence integrity, fail-closed behavior, independent surfaces, optional AI/protocol capabilities, and archive-before-deletion. fileciteturn392file0 The charter expresses the same principles at policy level and adds the reusable-capability admission rule. fileciteturn393file0

### 3. Cross-paradigm methodology vs Epistemic Context Mapping

Keep both. They are adjacent but distinct:

- **Epistemic Context Mapping:** understand a knowledge tradition, its native concepts, epistemic framework, and contextual meaning before comparison.
- **Cross-Paradigm Evidence Methodology:** evaluate a specific claim/intervention/outcome across paradigms using evidence, safety, replication, contradiction, applicability, and uncertainty.

The existing documents explicitly establish these different functions. fileciteturn414file0 fileciteturn395file0

### 4. Health State vs Longitudinal Observation

Do not create a second canonical health-state model. Health State remains canonical; longitudinal observation/timeline is a repository/projection concern. The repository already contains both the Health State contract and longitudinal observation contract/schema, so future work must prevent semantic drift.

### 5. Personal Health Record vs Personal Health Record Repository

Keep both only because they have different boundaries: the health-record semantic model versus the repository/storage/index boundary. The repository contract must not become a second clinical domain model.

### 6. Context/implementation documents

Several security areas have both context/contract and implementation documents. This is acceptable where the distinction is real. The rule is: the contract states invariants; the implementation document states how the current implementation realizes them. Do not duplicate the invariant text unnecessarily.

### 7. Legacy-data promotion documents

There are multiple documents for classification, preflight, implementation, executor, and operator specification. These should remain separate while each has a distinct lifecycle boundary. A future consolidation pass should add a single navigation map rather than flatten the documents into one oversized specification.

## New behavioral/epistemic material

The Behavioral & Epistemic Evidence Layer is correctly positioned as a semantic extension rather than a new application. Its purpose, lateral-thinking protocol, AI boundary, and non-goals are aligned with the existing shared-first architecture. The PR remains draft and must be validated against canonical Health State and Evidence Graph contracts before merge. fileciteturn411file0

The key architectural constraint is that behavioral observations must not become a competing canonical health-state model. The proposed contract is therefore subject to cross-contract testing before acceptance.

## Organization changes made in this pass

Added:

- `docs/README.md` — documentation entry point and lifecycle.
- `docs/architecture/README.md` — canonical architecture/contract map.
- `docs/governance/README.md` — governance for humans, developers, and AI agents.
- `docs/governance/SOMA-OS-AI-HUMAN-DEVELOPER-GUIDE.md` — consolidated operating context.
- `docs/governance/documentation-audit-2026-09-11.md` — this audit.
- `docs/decisions/README.md` — decision-record index.
- `docs/research/README.md` — research boundary and workflow.
- `docs/evidence-sources/README.md` — evidence-source boundary.
- `docs/product/README.md` — product-document boundary.
- `docs/strategy/README.md` — strategy-document boundary.
- `docs/ui-ux/README.md` — UI/UX boundary.
- `docs/archive/README.md` — archive lifecycle.

These additions improve discoverability without deleting or moving historical documents.

## What is deliberately NOT done yet

- No mass rename.
- No mass move.
- No deletion of existing Markdown files.
- No merging of documents merely because they share vocabulary.
- No change to canonical health/evidence schemas.
- No change to Policy Kernel authority.
- No forced merge of PR #81.

This is intentional. Repository organization must be evidence-driven; a premature mass move can break links, CI references, historical traceability, and agent navigation.

## Next organization phase

After the index is validated, perform a second pass that:

1. inventories every Markdown file and its canonical authority;
2. detects exact/semantic duplicate sections and stale references;
3. marks superseded documents explicitly;
4. moves only documents with a clear destination and verified link impact;
5. archives only when historical value requires preservation;
6. updates links and CI/documentation references;
7. validates the complete repository after the reorganization.

The target is not the fewest Markdown files. The target is one clear owner, one authority, one lifecycle, and one discoverable path for every concept.
