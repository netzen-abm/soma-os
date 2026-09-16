# SOMA Measurement Boundary v1

**Status:** Bounded implementation baseline  
**Scope:** Explicit definition of what an intervention response will be measured by, how it will be measured, and over what observation window.

## Purpose

Provide the smallest reusable boundary between a governed Intervention Intent and later recording of observations and outcomes.

A `MeasurementPlan` defines the measurement design. It is not itself a measurement result, proof of efficacy, or an outcome.

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

A `MeasurementPlan` contains:

- stable identifier;
- schema version;
- subject reference;
- intervention reference;
- metric;
- measurement method;
- unit where applicable;
- observation window start/end;
- baseline reference where available;
- uncertainty method where available;
- rationale;
- creation time;
- actor reference where available.

## Governance boundary

`AuthorizedMeasurementContext` is minted only from the canonical authorization boundary after an explicit `ALLOW` decision.

Authorization context does not mean that a measurement procedure is clinically appropriate or that an intervention is authorized for execution. Those decisions remain governed by their respective capability and policy boundaries.

## Semantic separation

SOMA must keep these distinct:

- **Measurement plan:** what and how to measure.
- **Observation/response:** what was actually recorded.
- **Outcome:** the defined result over an observation window.
- **Interpretation:** what the observations may mean.

A measured change is not automatically an intervention effect, and an intervention effect is not automatically causal.

## Non-goals

This v1 capability does not:

- record measurement results;
- calculate efficacy;
- infer causality;
- declare clinical significance;
- create a generic analytics engine;
- create an outcome engine;
- create an adaptation engine;
- recommend interventions;
- create a second store, graph, authorization, or policy engine;
- fabricate missing time, values, units, or uncertainty.

## Design principle

> **Specify the measure before observing the response; preserve what was observed before interpreting what changed.**
