# SOMA Outcome Boundary v1

**Status:** Bounded implementation baseline  
**Scope:** Explicit definition of a defined result over an observation window within the canonical Health State domain.

## Purpose

Provide the smallest reusable boundary between recorded observations/responses and a later outcome representation.

An `Outcome` records a defined result over a bounded observation window. It is not itself proof of efficacy, causality, clinical significance, or an adaptation decision.

## Pipeline position

```text
Evidence
  -> Safety Assessment
  -> Causality Assessment where relevant
  -> Intervention Intent
  -> Future governed execution
  -> Measurement Plan
  -> Recorded Observation / Response
  -> Outcome
  -> Adaptation
```

## Canonical contract

An `Outcome` contains a stable identifier, schema version, subject reference, intervention reference, measurement reference, metric, recorded result, unit where applicable, observation window, baseline reference where available, uncertainty where available, status, interpretation reference where available, rationale, source reference, recording time, and actor reference where available.

## Governance boundary

Outcome authorization consumes the canonical authorization decision and mints an `AuthorizedOutcomeContext` only after an explicit `ALLOW`. It does not create a second policy engine.

Authorization does not establish clinical validity, intervention appropriateness, efficacy, causality, clinical significance, or an adaptation decision.

## Semantic separation

SOMA must keep these distinct:

- **Measurement plan:** what and how to measure.
- **Observation/response:** what was actually recorded.
- **Outcome:** the defined result over an observation window.
- **Interpretation:** what the observations may mean.
- **Causal assessment:** whether evidence supports a causal relationship.

A recorded change is not automatically an intervention effect. An intervention effect is not automatically causal. An outcome is not automatically clinically significant.

## Non-goals

This v1 capability does not calculate efficacy, establish clinical significance, infer causality, recommend adaptation, create an analytics engine, create a result warehouse or second clinical store, create a second graph/authorization/policy engine, or silently fabricate missing time, values, units, baselines, or uncertainty.

## Design principle

> **Record the defined result over its stated window; preserve the result before interpreting what caused it or what should happen next.**
