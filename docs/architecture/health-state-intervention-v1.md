# SOMA Intervention Intent v1

**Status:** Bounded implementation baseline  
**Scope:** Explicit, governed intent to perform a health-state intervention.

## Purpose

Provide the smallest reusable boundary between an evidence-informed assessment and a later governed action or experiment workflow.

The contract records **what is intended**, not that the intervention is safe, effective, authorized for execution, or actually performed.

## Pipeline position

```text
Observation
  -> Relationship / Hypothesis
  -> Evidence
  -> Safety Assessment
  -> Causality Assessment where relevant
  -> Intervention Intent
  -> Future governed execution
  -> Measurement
  -> Outcome
  -> Adaptation
```

## Canonical contract

An `InterventionIntent` contains:

- stable identifier;
- schema version;
- subject reference;
- intervention type;
- bounded description;
- intended change;
- lifecycle status;
- rationale;
- evidence references;
- safety references;
- causality references where applicable;
- planned time window where known;
- creation time;
- actor reference where available.

## Governance boundary

The `AuthorizedInterventionContext` is minted only through the canonical authorization boundary after an explicit `ALLOW` decision.

Authorization context does **not** mean execution authorization. A later execution capability must perform its own appropriate policy, consent, safety, human-review and capability checks.

## Evidence and safety

Evidence, safety and causality references remain references to their canonical domains. This contract does not dereference, score, reinterpret, or synthesize them.

A reference to causality is not itself proof of causality; the existing explicit causality assessment remains authoritative for that assessment.

## Non-goals

This v1 capability does not:

- execute an intervention;
- prescribe treatment;
- authorize medication changes;
- infer efficacy;
- perform causal inference;
- create an experiment engine;
- create a measurement engine;
- create a recommendation engine;
- create a second store or graph;
- create a second authorization or policy engine;
- turn an AI-generated proposal into an executed action.

## Design principle

> **Define the change before attempting the change; measure what happens before claiming that it worked.**

This keeps intervention, response and outcome semantically distinct while enabling the future SOMA learning loop.
