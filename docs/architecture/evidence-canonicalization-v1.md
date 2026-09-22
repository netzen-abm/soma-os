# SOMA Evidence Canonicalization v1

**Status:** Audit baseline / migration decision
**Date:** 2026-09-02
**Branch:** `audit/evidence-canonicalization-v1`
**Base:** `main` at `7fffebcdc2e544adce2cb1ac4738bb514c3d40cd`

## 1. Decision

SOMA must use the **Health Evidence Graph v1** as the canonical semantic model for health evidence. The existing Evidence Registry is not discarded: it is retained as the current persistence and compatibility layer until an explicit migration is implemented and verified.

No competing evidence model should be introduced. Existing structures are mapped into the canonical graph rather than silently replaced.

## 2. Audit scope

The audit covers the principal evidence architecture already present in `main`:

- `docs/architecture/evidence-registry-schema.md`
- `database/migrations/0002_initialize_evidence_registry.sql`
- `docs/architecture/evidence-research-engine.md`
- `services/evidence-research/contracts/domain-model.md`
- `docs/food-life-evidence-model.md`
- the merged `Health Evidence Graph v1` architecture contract.

This is a semantic canonicalization audit, not a destructive database migration.

## 3. Existing-to-canonical mapping

| Existing concept | Health Evidence Graph v1 | Decision |
|---|---|---|
| `EvidenceSource` / `evidence_sources` | `Source` | **Canonicalize by mapping**. Preserve source type, title, publisher, dates, URL, provenance and license metadata. |
| `Claim` / `evidence_claims` | `Claim` | **Canonicalize by mapping**. Preserve source location, subject/predicate/object semantics and review state. Add explicit evidence/safety/applicability assessment through canonical relations. |
| `Protocol` / `management_protocols` | `Intervention` plus contextual protocol object | **Do not collapse protocol semantics into Claim.** Preserve protocol as an intervention/context record and link claims/studies through canonical relationships. |
| `ResearchEvidence` / `research_evidence` | `Study` + source relationship | **Canonicalize by mapping**. Preserve bibliographic, population, intervention, comparator, outcomes, findings, limitations, funding and conflicts. |
| `EvidenceAssessment` / `evidence_assessments` | `Evidence Assessment` | **Canonicalize by mapping**. Preserve directness, quality, consistency, precision, external validity, safety certainty, status and rationale. |
| `SafetyRecord` / `safety_records` | `Safety Assessment` | **Canonicalize by mapping**. Safety remains independent from efficacy. Preserve contraindications, interactions, adverse events, dose context and regulatory status. |
| `VerificationReference` | provenance / source references | **Canonicalize by mapping**. Preserve verification URL and link type; do not fabricate missing links. |
| `SearchContext` / `evidence_search_contexts` | provenance / research-run context | **Retain as research provenance infrastructure**. It should become a first-class provenance/run object rather than be dropped. |
| `ProtocolEvidenceRelation` | canonical relationships + Evidence Assessment | **Canonicalize** using `SUPPORTED_BY`, `CHALLENGED_BY`, `DERIVED_FROM`, `REPORTS_ON`, and explicit assessment fields. |
| Food–Life `FoodPlantIdentity` | `Intervention`/subject domain extension | **Retain as domain-specific extension**, not a replacement for the graph core. |
| Food–Life `NutritionProfile` / `PhytochemicalProfile` / `AnalyticalAssay` | evidence-linked domain observations | **Retain as specialized observation/measurement objects**. Analytical measurement is not clinical evidence by itself. |
| Food–Life `TraditionalKnowledgeRecord` | `Claim` + explicit source class / epistemic context | **Canonicalize by relation**, preserving traditional provenance and epistemic status. |
| Food–Life `ExperimentalEvidence` / `ClinicalEvidence` | `Study` with study design/type | **Canonicalize by mapping**. Do not create separate parallel evidence graphs. |
| Evidence Research Engine `SearchResult` / provider records | ingestion/provider boundary | **Keep outside canonical graph as retrieval-layer objects**; normalize into canonical Source/Study objects after verification. |

## 4. Semantic conflicts that require explicit handling

### 4.1 Evidence status vocabularies

The legacy research contract uses values such as `STRONG`, `MODERATE`, `LIMITED`, `MIXED`, `INDIRECT`, and `INSUFFICIENT`. Health Evidence Graph v1 uses contextual levels `E0_UNKNOWN` through `E4_WELL_SUPPORTED`.

These are **not interchangeable**.

Decision:

- retain legacy assessment values during compatibility/migration;
- define an explicit versioned mapping function before any conversion;
- never convert `STRONG` to `E4` mechanically without preserving the underlying assessment rationale and context;
- retain `MIXED`/contradictory evidence as a synthesis property, not as an unexplained evidence level.

### 4.2 Directness

The legacy directness vocabulary is compatible with the canonical contract and should be reused rather than duplicated.

Decision: make the existing directness vocabulary a canonical controlled vocabulary under Health Evidence Graph v1.

### 4.3 Safety

Legacy `SafetyRecord` is structurally compatible with the canonical `Safety Assessment` but currently has text-oriented fields and no universal safety-classification vocabulary.

Decision: preserve existing fields and add a versioned safety classification layer. Efficacy/evidence level must never imply safety.

### 4.4 Protocol versus intervention

The legacy registry models a `Protocol` as a first-class object. The Health Evidence Graph models `Intervention / Exposure` as a canonical entity.

Decision: protocol remains a legitimate higher-level domain object containing intervention details, source-derived claims and context. It must reference canonical interventions and evidence objects rather than becoming a second evidence ontology.

### 4.5 Personal health state

The evidence registry explicitly excludes personal health records. The Health Evidence Graph also requires separation from personal observations.

Decision: preserve this boundary. Personal observations belong to Health State infrastructure and may inform interpretation, but they do not enter the public/scientific evidence registry as if they were population evidence.

## 5. Canonical architecture after migration

```text
                         SOMA Evidence Infrastructure
                                      |
                    +-----------------+------------------+
                    |                                    |
             Retrieval / Providers                 Canonical Graph
                    |                                    |
        PubMed / publishers / social          Source / Claim / Study
        / government / datasets                Population / Intervention
                    |                           Outcome / Assessment
                    v                           Safety / Applicability
              verification                          Uncertainty
                    |                                    |
                    +------------ provenance ------------+
                                      |
                                      v
                              Evidence Passport
                                      |
                                      v
                            Governed capabilities
```

## 6. What is retained

The following legacy capabilities must not be lost during canonicalization:

- source provenance and verification references;
- protocol decomposition;
- directness assessment;
- contradiction/null evidence handling;
- safety discovery;
- no-evidence declarations;
- search reproducibility;
- traditional knowledge context;
- food/plant analytical evidence;
- funding/conflict-of-interest information;
- professional-review requirements;
- privacy boundary excluding personal health records.

These capabilities are already consistent with the direction of Health Evidence Graph v1 and should be migrated, not rewritten from scratch.

## 7. Required migration controls

Before changing persistence structures:

1. Create machine-readable mapping specifications for every legacy entity.
2. Define controlled-vocabulary mappings and version them.
3. Build fixtures covering exact, indirect, contradictory, traditional, analytical and no-evidence cases.
4. Run dual-read/compatibility validation where practical.
5. Compare record counts and relationship counts before and after transformation.
6. Verify that every migrated published claim remains traceable to its original source.
7. Verify that no personal health information is introduced into the public evidence registry.
8. Only after migration evidence is complete may obsolete structures be archived and later removed.

## 8. Deletion policy

No legacy evidence schema, migration, contract, or document is deleted in this phase.

If a legacy artifact becomes obsolete after verified migration:

```text
Current artifact
      -> archive with provenance
      -> migration evidence
      -> compatibility verification
      -> deletion decision
```

This follows SOMA's archive-before-delete rule.

## 9. Implementation sequence

### Phase A — completed by this audit

- verify current `main` baseline;
- identify overlapping evidence models;
- establish Health Evidence Graph v1 as semantic canonical target;
- identify compatible, conflicting and unique legacy semantics;
- preserve all legacy capabilities pending migration.

### Phase B — next

Build a machine-readable **Evidence Canonical Mapping Contract v1** containing:

- entity mappings;
- field mappings;
- vocabulary mappings;
- relationship mappings;
- loss/unknown rules;
- provenance requirements;
- migration invariants.

### Phase C

Build a non-destructive migration pilot against representative fixtures. The pilot must prove round-trip traceability before any production persistence migration.

### Phase D

Only after pilot verification, update canonical persistence and service contracts, archive superseded artifacts, and remove duplicates where evidence supports removal.

## 10. Acceptance criteria

Canonicalization is complete only when:

- one canonical evidence semantic model exists;
- legacy records map deterministically or remain explicitly unknown;
- no evidence strength is silently inflated;
- contradictions remain representable;
- safety remains independent;
- source provenance is preserved;
- study duplication is controlled;
- traditional/emerging evidence retains epistemic status;
- no personal health data enters the public evidence registry;
- existing Evidence Research Engine capabilities continue to function;
- Health State ↔ Evidence links remain semantically bounded;
- migration can be audited and reproduced;
- deletion, if any, occurs only after archive and evidence.

## 11. Final decision

**Do not merge the existing `feature/health-state-evidence-schema-v1` implementation branch as-is.** Its conceptual target is correct, but the repository now contains enough overlapping evidence infrastructure that the machine-readable contract must first incorporate the canonicalization mapping and compatibility boundary identified here.

The correct next artifact is therefore the **Evidence Canonical Mapping Contract v1**, followed by a migration pilot. This preserves the user's requirement that `main` become capable of the new model before integration and prevents duplicate evidence infrastructure.
