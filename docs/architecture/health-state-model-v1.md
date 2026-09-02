# SOMA Health State Model v1

**Status:** Architecture contract / design baseline  
**Date:** 2026-09-02  
**Scope:** Canonical longitudinal representation of health context for shared SOMA infrastructure

## 1. Purpose

The SOMA Health State Model represents a person's health-related state and its evolution over time without treating the model as a diagnosis, clinical truth, or complete representation of a human being.

It is a shared domain contract. Applications, agents, research workflows, nutrition intelligence, lifestyle intelligence, and future practitioner capabilities should consume this model rather than invent parallel health-state representations.

The model exists to support the SOMA loop:

```text
Observe -> Understand -> Hypothesize -> Intervene -> Measure -> Learn -> Adapt
```

## 2. Architectural position

The Health State Model sits above the governed infrastructure:

```text
Identity
  -> Capability
  -> Policy
  -> Gateway
  -> Execution
  -> Evidence
  -> Audit
             |
             v
       Health State Model
             |
             v
       Evidence Graph
             |
             v
   Intervention / Experiment
             |
             v
       Outcomes / Learning
```

The model must not bypass Policy, privacy, safety, provenance, or audit controls.

## 3. Core design principles

1. **Longitudinal:** health state is temporal; a current snapshot is insufficient.
2. **Observed before inferred:** observations and measurements must remain distinguishable from interpretations.
3. **Contextual:** the same observation may have different meaning in different populations and contexts.
4. **Provenance-first:** externally sourced or imported data must retain source and acquisition context.
5. **Uncertainty-preserving:** missing, estimated, conflicting, and low-confidence information must not be silently normalized into certainty.
6. **Non-diagnostic by default:** the model describes health information and state; it does not itself establish diagnosis.
7. **Composable:** capabilities reference stable entities instead of embedding domain logic in surfaces.
8. **Privacy-aware:** data classification, consent scope, and access policy are first-class relationships, not UI metadata.
9. **Versioned:** schema and semantic changes must be explicit and migratable.
10. **Evidence-linked:** interpretations and recommendations must be able to link to governed evidence objects.

## 4. Canonical domain

```text
PERSON
├── Biology
│   ├── biomarkers
│   ├── physiology
│   ├── symptoms/signals
│   └── laboratory observations
├── Behaviors
│   ├── nutrition
│   ├── physical activity
│   ├── sleep
│   └── substance exposure
├── Context
│   ├── environment
│   ├── schedule
│   ├── resources
│   └── social/contextual factors
├── Goals
├── Interventions
├── Responses
└── Outcomes
```

These are domain groupings, not necessarily database tables.

## 5. Core entity types

### Person

Represents the subject of a health record. A Person reference must use a SOMA-controlled identifier and must not expose unnecessary identifying attributes to downstream capabilities.

### Observation

A recorded fact or measurement at a known or bounded time.

Examples:
- laboratory value;
- wearable measurement;
- symptom report;
- food intake observation;
- sleep duration;
- activity measurement.

An observation should preserve:
- observation ID;
- subject ID;
- concept/type;
- value and unit where applicable;
- time or time interval;
- source;
- method/device where relevant;
- provenance;
- data quality;
- uncertainty where known;
- classification.

### Interpretation

A derived statement about one or more observations. It must remain separate from the underlying observations and preserve its derivation/evidence basis.

An interpretation should record:
- interpretation ID;
- inputs;
- method or rule/model;
- result;
- confidence/uncertainty;
- applicability context;
- evidence references;
- model/version;
- validation status.

### Goal

A user- or practitioner-defined desired outcome. Goals must not be converted into medical claims merely because they are stored in the health state.

### Intervention

A documented action, exposure, or management approach applied to the person or context. An intervention record describes what occurred; it does not imply efficacy.

### Response

A measured or reported change associated temporally with an intervention. Response is not automatically causation.

### Outcome

A defined result over an observation window. Outcomes should specify the metric, window, and source sufficiently to support later analysis.

### Context

Relevant environmental, behavioral, social, resource, temporal, or situational conditions that may modify interpretation or intervention feasibility.

## 6. Temporal model

Every time-sensitive object should support at least one of:

- `effective_at` for an instant;
- `effective_from` / `effective_to` for an interval;
- `observed_at` when observation time is distinct from recording time;
- `recorded_at` for ingestion/recording time.

SOMA must distinguish **when something happened** from **when SOMA learned about it**.

Unknown time must remain explicitly unknown rather than being replaced with ingestion time.

## 7. Observation versus interpretation

This distinction is mandatory.

```text
Observed:
  fasting glucose = X at time T

Interpreted:
  value may be relevant to metabolic-risk hypothesis H

Recommendation:
  consider intervention I under policy/evidence conditions P
```

These are three different semantic layers and must not be collapsed into one field.

## 8. Evidence and provenance linkage

A Health State object may reference evidence, but personal observations are not themselves clinical evidence for general populations.

Conversely, a research claim must not be rewritten as a personal fact.

The linkage model should support:

```text
Personal observation
      |
      +--> interpretation/hypothesis
                  |
                  +--> evidence claim(s)
                              |
                              +--> source(s)
```

Each link should preserve direction and provenance.

## 9. Uncertainty and data quality

The model should distinguish at minimum:

- `known`;
- `estimated`;
- `reported`;
- `inferred`;
- `missing`;
- `conflicting`;
- `not_applicable`.

Uncertainty should be represented where meaningful rather than forcing false precision.

A model-generated inference must never be stored as though it were an original observation.

## 10. Privacy and governance hooks

Health State objects should be addressable by policy without exposing policy implementation details in every domain object.

The architecture should support relationships to:

- data classification;
- consent scope;
- purpose of use;
- principal/access context;
- retention policy;
- provenance/audit record;
- jurisdiction where required.

Policy decisions remain the responsibility of the shared Policy Kernel.

## 11. Safety boundary

The Health State Model must not itself:

- diagnose disease;
- prescribe medication;
- direct medication cessation;
- guarantee treatment outcomes;
- convert a weak evidence claim into clinical instruction.

High-consequence use requires capability-level safety and human-review policy.

## 12. Systems-biology alignment

The model is deliberately compatible with systems-oriented research. It can represent interacting biology, behavior, context, interventions, responses, and outcomes.

However:

> A multidimensional health record is not automatically a systems-biology model.

Mechanistic models, pathway representations, perturbation models, and predictive models belong in appropriate research/evidence capabilities and must carry explicit assumptions, validation status, and uncertainty.

## 13. Minimal v1 object envelope

A future serialized object should conceptually contain:

```text
id
subject_ref
entity_type
schema_version
status
effective_time
recorded_time
concept
value
unit
source_ref
provenance_ref
classification
uncertainty
relationships
created_at
updated_at
```

Not every entity uses every field. The envelope is a design guide, not yet a final JSON Schema.

## 14. Relationship vocabulary

Initial relationships should include:

- `HAS_OBSERVATION`
- `HAS_CONTEXT`
- `HAS_GOAL`
- `HAS_INTERVENTION`
- `HAS_RESPONSE`
- `HAS_OUTCOME`
- `DERIVED_FROM`
- `SUPPORTS_HYPOTHESIS`
- `REFUTES_HYPOTHESIS`
- `TEMPORALLY_PRECEDES`
- `ASSOCIATED_WITH`
- `MEASURED_BY`
- `EVIDENCE_LINKED`

Causal relationships must not be implied by generic association edges.

## 15. Versioning and migration

The model must use explicit schema versions. Breaking semantic changes require a migration plan.

Migration follows SOMA's archive-first rule:

1. preserve the original record;
2. map fields explicitly;
3. identify unmapped/unknown values;
4. validate the migrated representation;
5. test consumers;
6. document the migration;
7. only then retire obsolete representations where justified.

## 16. Deliberate non-goals for v1

Do not make v1 a universal human digital twin.

Do not require:
- whole-body mechanistic simulation;
- multi-omics ingestion;
- continuous real-time telemetry;
- diagnosis coding for every observation;
- automatic causal inference;
- proprietary vendor-specific data models.

These can be future adapters/capabilities if validated use cases justify them.

## 17. Acceptance criteria for implementation

Health State Model v1 is implementation-ready only when:

- canonical entities are schema-defined;
- identifiers are stable;
- time semantics are defined;
- observation/interpretation separation is tested;
- provenance is mandatory for imported/external data;
- uncertainty states are testable;
- privacy/policy hooks are defined;
- evidence links are directional and traceable;
- versioning/migration rules are tested;
- no application surface owns a competing health-state schema.

## 18. Decision

Adopt this document as the architecture baseline for a canonical SOMA Health State Model v1. The next implementation artifact should be a machine-readable JSON Schema plus validation tests, created only after alignment with the Health Evidence Graph contract.
