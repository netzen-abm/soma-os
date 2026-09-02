# Evidence Canonical Mapping Contract v1

**Status:** Proposed canonicalization contract
**Version:** 1.0
**Date:** 2026-09-02

## 1. Purpose

This contract defines how SOMA-OS maps the existing Evidence Registry and related research/food-life evidence models into the canonical **Health Evidence Graph v1** without silently changing meaning, inflating evidence strength, losing provenance, or creating a competing evidence ontology.

The contract is deliberately non-destructive. Existing schemas, migrations, contracts, and documents remain available as compatibility and historical sources until a separately approved migration proves that removal is safe.

## 2. Canonical authority

`Health Evidence Graph v1` is the canonical semantic model for health evidence.

The existing Evidence Registry is treated as the current persistence/compatibility layer. Existing research-engine contracts, Food–Life evidence concepts, and legacy vocabulary remain reusable inputs and must map explicitly to the canonical graph.

No application surface may introduce an independent evidence domain model.

## 3. Mapping principles

1. **Preserve meaning.** A source concept is not renamed merely for cosmetic consistency.
2. **Preserve provenance.** Every migrated object must remain traceable to its source record and source system.
3. **No silent evidence inflation.** Legacy strength labels are not mechanically converted into E0–E4 without an explicit assessment rationale.
4. **Unknown stays unknown.** Missing information maps to an explicit unknown/insufficient state rather than an inferred value.
5. **Efficacy and safety remain independent.** Safety evidence must never be inferred from efficacy evidence.
6. **Directness is independent of strength.** A strong indirect study is still indirect.
7. **Traditional knowledge is not clinical efficacy.** Traditional reports retain their epistemic status.
8. **Analytical measurements are not clinical outcomes.** Laboratory/chemical observations remain measurements unless evidence establishes a clinical relationship.
9. **Association is not causation.** No `CAUSES` relationship is created by migration.
10. **Contradiction is preserved.** Conflicting evidence is represented explicitly, not resolved by majority vote during migration.
11. **Privacy boundary is preserved.** Public/scientific evidence records must not absorb personal health records.
12. **Migration is reversible/auditable.** Source IDs, mapping version, transformation status, and validation results are retained.

## 4. Entity mapping

| Existing concept | Canonical Health Evidence Graph v1 | Mapping rule |
|---|---|---|
| EvidenceSource | Source | Map identity, source type, title, URL, publication/retrieval metadata and provenance. Preserve original source ID. |
| Claim | Claim | Map claim text and provenance. Do not upgrade claim epistemic status during migration. |
| ResearchEvidence | Study | Map study identity, design/type, population, intervention/exposure, outcomes and source references where available. |
| EvidenceAssessment | Evidence Assessment | Map assessment rationale, directness, certainty/strength information and assessor provenance. Preserve legacy vocabulary alongside canonical classification where needed. |
| SafetyRecord | Safety Assessment | Map safety findings independently from efficacy/strength. Preserve condition/context and evidence basis. |
| VerificationReference | Source/provenance verification reference | Preserve verification URL/reference, retrieval date and verification status. |
| SearchContext | Research provenance / search run | Preserve query, provider, timestamp and retrieval context as research-process provenance, not as evidence itself. |
| Protocol | Intervention / protocol domain object | Preserve protocol as an intervention-context object that references evidence; do not turn protocol text into evidence automatically. |
| management_protocols | Intervention/Protocol | Map management protocol identity and contextual details. Evidence relations are mapped separately. |
| ProtocolClaimLink | Claim relation to intervention/protocol | Map as an explicit relationship connecting an intervention/protocol with a claim; preserve link provenance. |
| protocol_research_links | Intervention/Protocol ↔ Study relation | Map study relationship while preserving the legacy relation semantics. |
| ResearchProvider/SearchResult | Retrieval-layer object → Source/Study after verification | Search results remain provisional retrieval objects until verified; no search result becomes evidence merely by retrieval. |
| TraditionalKnowledgeRecord | Claim + Source with epistemic classification | Preserve traditional attribution and classify as traditional knowledge/reporting unless independently supported by higher-level evidence. |
| ExperimentalEvidence | Study / experimental evidence | Map study design and experimental context; retain non-human/preclinical status where applicable. |
| ClinicalEvidence | Study / clinical evidence | Map clinical study design, population and outcomes. |
| AnalyticalAssay | Measurement/observation linked to Source/Study where appropriate | Keep analytical result separate from clinical efficacy and therapeutic claims. |
| OutcomeRecord | Outcome | Map measured outcome and measurement context; preserve whether outcome is biochemical, physiological, clinical, or wellbeing-related. |

## 5. Field mapping rules

### 5.1 Identity and provenance

Every canonical object produced by migration must support, directly or through the canonical provenance mechanism:

- canonical object ID
- source model/type
- source record ID
- mapping contract version
- source system/repository location
- provenance reference
- transformation status
- migration timestamp/run ID where applicable

If an existing source lacks a stable identifier, the migration must assign a deterministic migration ID while retaining the original identifying fields.

### 5.2 Source fields

Existing source URL, title, publisher, publication date, retrieval date, source class, attribution and verification information map to the canonical Source representation.

A URL stored only inside generic metadata must not be considered equivalent to a canonical source reference when downstream verification requires a first-class source. The migration must promote it explicitly and retain the original metadata representation for traceability.

### 5.3 Claim fields

Claim text maps to `Claim`.

Existing labels such as `medical fact`, `recommendation`, or similar descriptive language must not be interpreted as proof of clinical validity. Claim status must remain grounded in its evidence assessment.

### 5.4 Study fields

Study records map design/type, population, intervention/exposure, comparator where available, outcomes, publication/source identifiers and relevant study metadata.

Missing study fields remain unknown. The migration must not infer randomized design, causality, efficacy, or clinical relevance from titles, abstracts, source class, or legacy labels alone.

### 5.5 Assessment fields

The canonical Health Evidence Graph uses E0–E4:

- `E0_UNKNOWN`
- `E1_PLAUSIBLE`
- `E2_PRELIMINARY`
- `E3_SUPPORTED`
- `E4_WELL_SUPPORTED`

Existing Evidence Registry states are:

- `STRONG`
- `MODERATE`
- `LIMITED`
- `MIXED`
- `INDIRECT`
- `INSUFFICIENT`

These vocabularies are **not semantically interchangeable**.

Default migration rule: preserve the legacy value as `legacy_evidence_state` and set canonical evidence level to `E0_UNKNOWN` unless a deterministic, versioned assessment rule has sufficient evidence to assign E1–E4. Such an assignment must record its rule version and rationale.

In particular, `STRONG` must never be mechanically mapped to `E4_WELL_SUPPORTED`.

### 5.6 Directness

The existing research engine vocabulary is retained as the canonical directness vocabulary for v1:

- `DIRECT_EXACT_PROTOCOL`
- `DIRECT_COMPONENT`
- `RELATED_INTERVENTION`
- `MECHANISTIC`
- `OBSERVATIONAL_ASSOCIATION`
- `PRECLINICAL`
- `INDIRECT`
- `NO_RELEVANT_EVIDENCE_FOUND`

Directness and evidence level are separate dimensions.

### 5.7 Safety

Canonical safety classification:

- `LOW_CONCERN`
- `CONTEXT_DEPENDENT`
- `CLINICALLY_SIGNIFICANT_CAUTION`
- `CONTRAINDICATED`
- `INSUFFICIENT_SAFETY_EVIDENCE`

Existing safety records map independently. Absence of a safety record does not mean `LOW_CONCERN`; it maps to `INSUFFICIENT_SAFETY_EVIDENCE` or an explicit unknown state according to the canonical schema.

### 5.8 Applicability

Applicability must preserve population, context, intervention conditions, dose/intensity, duration, setting, and relevant exclusions where available.

No migration may convert population-level evidence into a person-specific diagnosis, treatment instruction, or certainty statement.

## 6. Relationship mapping

Existing relationships must map into canonical relationship vocabulary where semantics match:

- `SUPPORTED_BY`
- `CHALLENGED_BY`
- `DERIVED_FROM`
- `REPORTS_ON`
- `STUDIES`
- `APPLIES_TO`
- `INVOLVES_INTERVENTION`
- `MEASURES_OUTCOME`
- `HAS_SAFETY_ASSESSMENT`
- `HAS_APPLICABILITY`
- `HAS_LIMITATION`
- `HAS_UNCERTAINTY`
- `SUPERSEDES`
- `DUPLICATE_OF`
- `CONTRADICTS`

A legacy relation may map only when its semantics are demonstrably equivalent. Otherwise it is preserved as a legacy relation with `mapping_status = UNKNOWN` or `NOT_DIRECTLY_MAPPABLE`.

No migration creates `CAUSES` unless the source already contains an explicit causal assessment that satisfies the canonical causal-evidence requirements. Ordinary association, mechanistic plausibility, or intervention description is insufficient.

## 7. Loss and unknown rules

The migration must classify every source field/relationship as one of:

- `MAPPED_EXACT`
- `MAPPED_TRANSFORMED`
- `PRESERVED_LEGACY`
- `UNKNOWN`
- `NOT_APPLICABLE`
- `NOT_DIRECTLY_MAPPABLE`

Silent field dropping is prohibited.

For a field that has no canonical destination, the migration must either preserve it under a documented legacy-extension/provenance mechanism or record an explicit loss decision with the source location and reason.

## 8. Duplicate and identity rules

Study/source identity must not rely on title-only matching.

Where identifiers exist, prefer stable identifiers such as DOI, PMID, registry ID, canonical URL, or source-system ID. If multiple legacy records appear to represent the same source/study, migration must preserve both source IDs and create an explicit `DUPLICATE_OF` relationship pending review.

Deduplication must not merge records merely because their claims appear similar.

## 9. Contradiction and null-evidence rules

The canonical graph must preserve:

- supporting evidence
- challenging evidence
- contradictory evidence
- null/no-relevant-evidence findings
- unresolved uncertainty.

`NO_RELEVANT_EVIDENCE_FOUND` is a valid research result and must not be converted to `INSUFFICIENT` evidence for efficacy without retaining the distinction between a search finding and an evidence assessment.

## 10. Research-engine compatibility

The existing evidence-research engine remains the retrieval, screening, extraction, assessment and synthesis infrastructure.

Its provider-neutral contracts remain useful at the retrieval/processing boundary:

`ResearchRequest → SearchResult → StudyEvidence → assessment/safety → EvidencePackage → canonical graph`

Search results are provisional. Canonical graph objects require appropriate verification and provenance.

The Evidence Passport is a presentation/view model derived from canonical evidence; it is never an independent source of truth.

## 11. Food–Life compatibility

The Food–Life evidence model is retained as a domain-specialized model that maps into the canonical graph.

Key boundaries:

- traditional use ≠ clinical efficacy
- analytical detection ≠ human therapeutic effect
- in vitro ≠ clinical outcome
- observational association ≠ intervention efficacy
- ML classification ≠ diagnosis

Food/plant observations may remain specialized measurement objects while linking to canonical claims, sources, studies, outcomes, and safety assessments.

## 12. Migration invariants

A migration is acceptable only if all of the following hold:

1. Every migrated object is traceable to a source object.
2. Every unmapped semantic element is explicitly classified.
3. No evidence level is silently inflated.
4. Directness is preserved.
5. Safety remains independent from efficacy.
6. Contradictory evidence remains represented.
7. Traditional knowledge remains correctly classified.
8. Analytical/preclinical evidence is not silently promoted to clinical evidence.
9. Study/source duplicates are traceable.
10. Source URLs required for verification are canonicalized, not buried only in metadata.
11. No personal health data crosses the public/scientific evidence boundary.
12. Existing research-engine behavior remains testable.
13. Record and relationship counts can be compared before/after migration.
14. Migration is reproducible from versioned inputs and mapping rules.
15. A migration failure cannot partially publish an apparently valid canonical record without an auditable status.

## 13. Required pilot fixtures

Before production migration, the non-destructive pilot must include at minimum:

1. exact protocol evidence
2. direct component evidence
3. indirect/related intervention evidence
4. contradictory evidence
5. traditional knowledge/report
6. analytical assay result
7. preclinical evidence
8. no-relevant-evidence search result
9. legacy `STRONG`, `MODERATE`, `LIMITED`, `MIXED`, `INDIRECT`, and `INSUFFICIENT` states
10. safety finding independent from efficacy
11. duplicate source/study candidates
12. missing provenance/URL edge case

Expected pilot outputs:

- canonical records
- mapping report
- unmapped/unknown report
- source-to-canonical traceability report
- count comparison
- vocabulary transformation report
- validation result

## 14. Implementation boundary

This contract does **not** authorize deletion of the existing Evidence Registry, its migrations, research-engine contracts, or Food–Life model.

It authorizes creation of a compatibility/mapping layer and a non-destructive migration pilot. Production migration, dual-read behavior, schema replacement, and archive/delete decisions require separate verification and approval.

## 15. Acceptance decision

The canonicalization sequence is:

```text
Existing Evidence Registry
        │
        ├── entities
        ├── fields
        ├── vocabularies
        └── relationships
                 │
                 ▼
Evidence Canonical Mapping Contract v1
                 │
                 ▼
      Non-destructive migration pilot
                 │
        ┌────────┴────────┐
        ▼                 ▼
 validation          traceability
        │                 │
        └────────┬────────┘
                 ▼
      production migration decision
```

Only after the pilot satisfies the migration invariants should the machine-readable Health Evidence Graph schema be rebased onto current `main` and considered for merge.

## 16. Decision

**Decision: APPROVED AS THE MAPPING BASELINE; NOT YET A PRODUCTION MIGRATION.**

The immediate next implementation artifact is the non-destructive migration pilot. The existing `feature/health-state-evidence-schema-v1` branch must not be merged as-is until it incorporates this canonicalization boundary.
