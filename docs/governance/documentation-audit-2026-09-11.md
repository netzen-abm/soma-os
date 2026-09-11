# SOMA-OS Documentation Audit — 2026-09-11

**Audit basis:** repository tree at `main` commit `41f1e151f080b8c27413e19dea27e9e0ba93fb21` as inspected during this documentation pass. Changes in the current documentation branch are audited against actual file content before organization decisions are made.

## Audit objective

Prevent documentation duplication, preserve institutional memory, and establish clear authority for humans, developers, and AI agents.

## Repository observations

The repository already has meaningful documentation domains: architecture, decisions, evidence sources, research, strategy, product, UI/UX, operating model, and archive. The main weakness is not absence of documentation; it is **authority overlap and discoverability**. The architecture directory contains many highly specific documents, some of which describe adjacent lifecycle stages or implementation details.

The current repository tree also contains active code and schema areas for apps, services, database, health vault, protocols, operations, scripts, and schemas. This reinforces the need for documentation to explain ownership rather than reproduce code semantics.

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

These should **not** be merged. Project Memory is a consolidated historical context; the Decision Log preserves chronological decisions. The documentation index treats both as complementary rather than competing authorities.

### 2. System architecture spec vs Shared Infrastructure Charter

These overlap on shared-first principles, but they have different scope. The charter is a foundational rule; the system architecture spec is the whole-system architecture baseline. Keep both and make the architecture index establish their relationship.

### 3. Cross-paradigm methodology vs Epistemic Context Mapping

Keep both. They are adjacent but distinct:

- **Epistemic Context Mapping:** understand a knowledge tradition, its native concepts, epistemic framework, and contextual meaning before comparison.
- **Cross-Paradigm Evidence Methodology:** evaluate a specific claim/intervention/outcome across paradigms using evidence, safety, replication, contradiction, applicability, and uncertainty.

### 4. Health State vs Longitudinal Observation

Do not create a second canonical health-state model. Health State remains canonical; longitudinal observation/timeline is a repository/projection concern. Future work must prevent semantic drift.

### 5. Personal Health Record vs Personal Health Record Repository

Keep both only because they have different boundaries: the health-record semantic model versus the repository/storage/index boundary. The repository contract must not become a second clinical domain model.

### 6. Context/implementation documents

Several security areas have both context/contract and implementation documents. This is acceptable where the distinction is real. The contract states invariants; the implementation document states how the current implementation realizes them. Do not duplicate the invariant text unnecessarily.

### 7. Legacy-data promotion documents

There are multiple documents for classification, preflight, implementation, executor, and operator specification. These should remain separate while each has a distinct lifecycle boundary. A future consolidation pass should add navigation rather than flatten the documents into one oversized specification.

### 8. Strategy vs foundational architecture

`docs/strategy/2026-09-soma-ecosystem-principles.md` contains a useful 12-point ecosystem summary, but its former label **Working architecture policy** incorrectly implied architectural authority. The document has been retained and relabeled **Strategic guidance**, with an explicit authority boundary pointing to the Shared Infrastructure Charter and canonical governance/architecture documents. Its substance was preserved rather than deleted or flattened.

### 9. Cleanup execution plan placement

`cleanup-execution-plan.md` describes repository hygiene, security/evidence gates, CI/merge procedure, and deletion controls. Those are governance/process concerns rather than technical architecture. It has therefore been relocated from `docs/architecture/cleanup-execution-plan.md` to `docs/governance/cleanup-execution-plan.md` on this branch, with the content preserved and its new placement explicitly marked as governance guidance.

### 10. Branch lifecycle and code maintainability policy placement

`branch-lifecycle.md` governs branch creation, verification, retirement, archive-first behavior, and branch cleanup. `code-maintainability-policy.md` governs engineering maintainability practices such as formatting, function structure, testing, and readability. Neither defines SOMA domain architecture or a runtime contract. Both have therefore been moved from `docs/architecture/` to `docs/governance/`, with explicit authority-boundary language added and substantive policy content preserved.

This is a controlled authority correction, not a content deletion. The original files remain recoverable through Git history and the destination paths are now the active locations.

## Citation and reference hygiene

A content audit found transient ChatGPT/tool citation artifacts in four Markdown documents:

- `services/evidence-research/adapters/pubmed-adapter.md`
- `docs/architecture/disease-management-protocol-evidence-policy.md`
- `docs/architecture/evidence-research-engine.md`
- `docs/evidence-sources/biswaroop-khadar-source-assessment.md`

These are not durable repository references. They have been replaced on the documentation branch with stable external references where the underlying source could be identified from the document content.

The repository-wide rule is now recorded in `docs/governance/documentation-authority-registry-v1.md`: repository documentation must not contain session-local citation markers such as `cite...` or `turn...` identifiers. External evidence should use durable URLs, DOI/PMID/PMCID, ISBN, or repository-internal source IDs.

A follow-up search against the default `main` branch may continue to show the old artifacts because the cleanup is currently on the documentation branch; that is expected until the branch is validated and merged.

## New behavioral/epistemic material

The Behavioral & Epistemic Evidence Layer is correctly positioned as a semantic extension rather than a new application. Its purpose, lateral-thinking protocol, AI boundary, and non-goals are aligned with the existing shared-first architecture. The PR remains draft and must be validated against canonical Health State and Evidence Graph contracts before merge.

The key architectural constraint is that behavioral observations must not become a competing canonical health-state model. The proposed contract is therefore subject to cross-contract testing before acceptance.

## Organization changes made in this pass

Added:

- `docs/README.md` — documentation entry point and lifecycle.
- `docs/architecture/README.md` — canonical architecture/contract map.
- `docs/governance/README.md` — governance for humans, developers, and AI agents.
- `docs/governance/SOMA-OS-AI-HUMAN-DEVELOPER-GUIDE.md` — shared operating context for humans, developers, and AI agents.
- `docs/governance/documentation-authority-registry-v1.md` — canonical ownership/navigation map.
- `docs/governance/documentation-audit-2026-09-11.md` — this audit.
- `docs/decisions/README.md` — decision-record index.
- `docs/research/README.md` — research boundary and workflow.
- `docs/evidence-sources/README.md` — evidence-source boundary.
- `docs/product/README.md` — product-document boundary.
- `docs/strategy/README.md` — strategy-document boundary.
- `docs/ui-ux/README.md` — UI/UX boundary.
- `docs/archive/README.md` — archive lifecycle.
- `docs/governance/cleanup-execution-plan.md` — governance/process placement for repository cleanup and merge controls.
- `docs/governance/branch-lifecycle.md` — branch lifecycle and retirement governance.
- `docs/governance/code-maintainability-policy.md` — engineering maintainability governance.

Relocated:

- `docs/architecture/cleanup-execution-plan.md` → `docs/governance/cleanup-execution-plan.md`.
- `docs/architecture/branch-lifecycle.md` → `docs/governance/branch-lifecycle.md`.
- `docs/architecture/code-maintainability-policy.md` → `docs/governance/code-maintainability-policy.md`.
- `docs/food-life-evidence-model.md` → `docs/architecture/food-life-evidence-model.md`.

These changes improve discoverability without discarding substantive content.

## Content-level changes made in this pass

- Reclassified the ecosystem-principles document from architecture policy to strategic guidance.
- Added an explicit strategy-to-architecture authority boundary.
- Replaced transient PubMed citation markers with durable NCBI/PubMed references.
- Replaced transient evidence citations in the disease-management policy with stable PubMed identifiers/URLs for the specific studies described.
- Replaced transient WHO citation markers in the Evidence Research Engine with durable official WHO URLs.
- Replaced the transient citation marker in the Biswaroop/Khadar source assessment with the canonical source URL already identified by that document.
- Added repository-wide citation hygiene rules to the authority registry.
- Relocated the cleanup execution plan to the governance domain after confirming that its content is process/governance guidance rather than a technical architecture contract.
- Relocated branch lifecycle and code maintainability policies to governance after confirming they govern repository/engineering process rather than SOMA technical architecture.
- Relocated the Food–Life evidence model into architecture after confirming that it defines shared evidence infrastructure, domain entities, provenance, safety, and analytical boundaries.

## What is deliberately NOT done yet

- No mass rename.
- No mass move.
- No deletion of unrelated existing Markdown files.
- No merging of documents merely because they share vocabulary.
- No change to canonical health/evidence schemas.
- No change to Policy Kernel authority.
- No forced merge of PR #81 or PR #82.

The relocations above are controlled, content-preserving authority corrections with clear destinations. Broader reorganization remains evidence-driven; premature mass moves can break links, CI references, historical traceability, and agent navigation.

## Next organization phase

After the index is validated, perform a second pass that:

1. inventories every Markdown file and its canonical authority;
2. detects exact/semantic duplicate sections and stale references;
3. marks superseded documents explicitly;
4. verifies inbound links, CI/workflow references, code/test references, PR references, and known external references before each move/rename;
5. moves only documents with a clear destination and verified link impact;
6. archives only when historical value requires preservation;
7. updates links and documentation references;
8. validates the complete repository after the reorganization.

The target is not the fewest Markdown files. The target is one clear owner, one authority, one lifecycle, and one discoverable path for every concept.
