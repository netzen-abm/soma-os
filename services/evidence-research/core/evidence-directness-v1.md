# SOMA Evidence Directness Assessment Contract v1

## Status

Implemented as the bounded assessment boundary after structured evidence extraction.

## Purpose

Record how closely an included study addresses a specific source protocol or research question. Directness is an applicability/relationship dimension; it is not a measure of study quality, evidence strength, causality, safety, or clinical significance.

## Canonical position

```text
Evidence Research Core
        ↓
Evidence Screening
        ↓
Structured Evidence Extraction
        ↓
Evidence Directness Assessment  ← v1
        ↓
Quality / Bias
        ↓
Evidence Synthesis
```

## Canonical vocabulary

The existing SOMA vocabulary is retained without duplication:

- `DIRECT_EXACT_PROTOCOL`
- `DIRECT_COMPONENT`
- `RELATED_INTERVENTION`
- `MECHANISTIC`
- `OBSERVATIONAL_ASSOCIATION`
- `PRECLINICAL`
- `INDIRECT`

`NO_RELEVANT_EVIDENCE_FOUND` remains a valid search outcome in the broader research contract, but is not a study-level directness classification.

## Decision contract

```text
EvidenceDirectnessDecision
├── directness
├── rationale
├── assessed_at
└── assessment_method
```

The decision must be explicit. v1 validates the supplied classification and metadata but does not infer directness from a title, abstract, source class, study type, or model output.

## Screening gate

Only an `INCLUDED` `ScreenedEvidenceCandidate` may enter this boundary. Excluded, uncertain, and unscreened candidates remain outside the study-assessment path.

## Meaning of directness

`DIRECT_EXACT_PROTOCOL` means the study directly evaluates the complete protocol or materially equivalent protocol conditions relevant to the research question.

`DIRECT_COMPONENT` means the study directly evaluates a component of the protocol but not the complete protocol.

`RELATED_INTERVENTION` means the study evaluates a related intervention or practice rather than the protocol/component itself.

`MECHANISTIC` means the evidence addresses a biological, biochemical, physiological, or other mechanism rather than a clinical protocol outcome.

`OBSERVATIONAL_ASSOCIATION` means the evidence reports an association from observational data and does not thereby establish intervention efficacy or causality.

`PRECLINICAL` means non-human or preclinical evidence and must not be silently promoted to human clinical evidence.

`INDIRECT` means the evidence has some relevance to the research question but material differences prevent a more direct classification.

These categories describe relationship to the question/protocol, not whether the study result is positive or credible.

## Independence from strength and quality

A strong study can be indirect. A weak study can be direct. Directness must therefore never be used as a proxy for evidence quality or certainty.

The canonicalization contract explicitly requires directness to remain independent of evidence level. Existing legacy evidence states such as `STRONG`, `MODERATE`, `LIMITED`, `MIXED`, `INDIRECT`, and `INSUFFICIENT` are not directness values.

## Independence from causality

No directness classification creates a causal relationship. In particular, `DIRECT_EXACT_PROTOCOL` does not mean the protocol caused the observed outcome. Causal assessment remains a separate governed step.

## Independence from safety

Directness does not establish safety. Safety evidence remains separately assessed and must not be inferred from an efficacy or applicability relationship.

## Provenance

The original `EvidenceCandidate` is retained unchanged inside the assessment result. The decision carries its rationale, assessment timestamp, and method so that a later reviewer can distinguish source facts from the explicit relationship assessment.

## AI boundary

AI may assist a future governed directness workflow, but the resulting classification must remain an explicit, reviewable decision with rationale. AI must not silently upgrade `RELATED_INTERVENTION`, `MECHANISTIC`, `OBSERVATIONAL_ASSOCIATION`, `PRECLINICAL`, or `INDIRECT` into exact-protocol evidence.

## No-evidence boundary

`NO_RELEVANT_EVIDENCE_FOUND` describes a search result rather than a study. It must remain distinguishable from a study-level `INDIRECT` classification and from an evidence state such as `INSUFFICIENT`.

## Explicit non-goals

- no evidence-quality score;
- no risk-of-bias assessment;
- no consistency assessment;
- no causal inference;
- no safety clearance;
- no recommendation;
- no clinical judgment;
- no new evidence database;
- no graph engine;
- no second authorization/policy engine;
- no provider-specific directness logic;
- no second directness vocabulary.

## Verification

Focused tests verify exact-protocol binding, preservation of indirect classification, screening-gate enforcement, rejection of the no-evidence search outcome as a study decision, and rejection of invalid classifications or missing rationale.
