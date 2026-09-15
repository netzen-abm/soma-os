# SOMA Evidence Causality Assessment v1

**Status:** Bounded implementation baseline  
**Scope:** Explicit, reviewable causality assessment after evidence synthesis and safety assessment.

## Purpose

Provide the smallest reusable causality boundary without turning temporal association into causation or creating an automatic causal-inference engine.

## Pipeline position

```text
Screening
  ↓
Structured extraction
  ↓
Directness
  ↓
Quality / bias
  ↓
Consistency
  ↓
Synthesis
  ↓
Safety Gate
  ↓
Causality Assessment
```

Causality remains distinct from directness, quality, consistency, synthesis, safety, recommendation, diagnosis, and action.

## Canonical status vocabulary

- `CAUSALITY_SUPPORTED`
- `CAUSALITY_PLAUSIBLE`
- `CAUSALITY_UNLIKELY`
- `CAUSALITY_NOT_SUPPORTED`
- `INSUFFICIENT_CAUSALITY_INFORMATION`
- `REQUIRES_HUMAN_REVIEW`
- `NOT_ASSESSED`

`CAUSALITY_UNCERTAIN` is represented operationally by the explicit insufficient-information or human-review states rather than by a forced binary conclusion.

## Required assessment context

Each assessment explicitly records:

- canonical evidence identifiers;
- temporal relationship;
- alternative explanations;
- co-exposures;
- relevant prior history;
- causal factors considered;
- unresolved questions;
- rationale;
- assessment method;
- assessment timestamp;
- assessor where available.

## Boundary rules

The boundary accepts only `SafetyGatedEvidence`. The assessment evidence identifiers must exactly match the underlying synthesized evidence identifiers. No evidence is dereferenced, weighted, reclassified, or automatically judged by this boundary.

## Non-goals

This capability does not:

- infer causality automatically;
- calculate causal effect sizes;
- perform statistical causal inference;
- resolve contradictory evidence by majority vote;
- establish diagnosis or treatment efficacy;
- generate treatment recommendations;
- authorize medication changes;
- submit regulatory reports;
- create a new database, graph, authorization engine, or policy engine.

AI may assist with evidence retrieval or draft assessment material, but a generated assertion is not promoted to causal fact without an explicit governed assessment.

## Design principle

> **Temporal association is evidence about sequence, not proof of causation.**

Causality assessment must preserve competing explanations and uncertainty rather than manufacture certainty from incomplete longitudinal data.
