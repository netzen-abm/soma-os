# SOMA Health State ↔ Evidence Graph Contract v1

**Status:** Architecture decision / integration contract  
**Date:** 2026-09-02

## Decision

SOMA will maintain two distinct canonical domains:

1. **Health State Model** — what is known or recorded about a person and their context over time.
2. **Health Evidence Graph** — what is known about health claims and the evidence supporting, challenging, or contextualizing those claims.

Neither becomes the source of truth for the other.

## Boundary

```text
PERSON / HEALTH STATE
  observations
  goals
  interventions
  responses
  outcomes
       |
       | interpretation / hypothesis links
       v
EVIDENCE GRAPH
  claims
  studies
  sources
  assessments
  safety
  applicability
  uncertainty
       |
       v
GOVERNED CAPABILITY
       |
       v
POLICY KERNEL + AUDIT
```

## Critical semantic rules

- A personal observation is not automatically general evidence.
- A research claim is not automatically a personal fact.
- An intervention record is not evidence of efficacy.
- A temporal association is not causation.
- A model inference is not an observation.
- Evidence strength and safety are independent dimensions.
- Applicability must be assessed rather than assumed.
- Provenance must survive every transformation.

## Required cross-domain references

Health State may reference:

- evidence claim IDs;
- evidence assessment IDs;
- source IDs where direct provenance is relevant.

Evidence Graph may reference:

- anonymized or policy-scoped health-state observation IDs when a research workflow explicitly permits such linkage;
- intervention/outcome records when evaluating an individual response, subject to consent and policy.

Cross-domain references must be capability- and policy-governed. The graph must not embed unrestricted personal health data.

## Temporal relationship

The canonical causal-safe pattern is:

```text
Health observation A
       |
       | occurred before
       v
Intervention I
       |
       | observed during window W
       v
Response / Outcome O
       |
       | may be evaluated against
       v
Evidence Claim C
```

The existence of this chain does not prove that I caused O.

## Shared provenance

Both domains must be able to point into shared provenance/audit infrastructure so that a future answer can reconstruct:

```text
input
 -> transformation
 -> evidence used
 -> policy decision
 -> generated interpretation
 -> output
```

## Implementation rule

Do not introduce application-specific copies of either model. If a surface needs a simplified representation, it must be a view/DTO/adapter over the canonical contracts.

## Next implementation gate

Before writing production schemas:

1. inspect existing SOMA health/evidence data structures for overlap;
2. identify reusable fields and incompatible legacy models;
3. create JSON Schema v1 with explicit cross-domain references;
4. add validation fixtures and negative tests;
5. document migration requirements before replacing any legacy representation.
