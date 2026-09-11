# SOMA Longitudinal Observation & Timeline v1

**Status:** Implementation contract / bounded v1
**Date:** 2026-09-11

## 1. Decision

SOMA will build longitudinal health understanding on the existing canonical Health State Model and the Personal Health Record/Health Vault foundation. This layer does not create a competing clinical payload store or a second canonical health model.

The Observation contract represents recorded health-related facts or measurements. The Timeline is a governed view/query over observations ordered by their health-relevant time semantics.

## 2. Scope

v1 establishes:

- stable observation identity;
- subject ownership;
- observation concept/type;
- value and unit where applicable;
- observed time when known;
- recorded/ingestion time;
- source and provenance;
- classification and uncertainty;
- optional data quality and device/context references;
- bounded relationships;
- deterministic timeline ordering;
- explicit separation between observation and interpretation.

## 3. Time semantics

SOMA must distinguish:

```text
observed_at / observed interval = when the health event or measurement occurred
recorded_at                    = when the observation was recorded/ingested
```

Unknown observation time must remain unknown. It must never silently become the ingestion timestamp.

For an interval, both `observed_from` and `observed_to` are required together.

## 4. Provenance and uncertainty

Imported or externally sourced observations must preserve provenance sufficient to trace the observation to its source. The observation contract therefore requires provenance and classification.

Uncertainty remains explicit. A reported, estimated, inferred, missing, or conflicting observation must not be silently represented as known fact.

## 5. Relationship boundary

v1 supports only bounded relationship vocabulary needed for longitudinal organization and future evidence linking:

- `DERIVED_FROM`
- `HAS_CONTEXT`
- `TEMPORALLY_PRECEDES`
- `ASSOCIATED_WITH`
- `MEASURED_BY`
- `EVIDENCE_LINKED`
- `HAS_INTERVENTION`
- `HAS_RESPONSE`
- `HAS_OUTCOME`

`ASSOCIATED_WITH` and `TEMPORALLY_PRECEDES` do not imply causation. `CAUSES` is intentionally absent from the observation vocabulary.

Example:

```text
Medication started
      ↓ 8 days
Fatigue observed
```

This may be represented as a temporal/association relationship. It is not a causal conclusion.

## 6. Timeline semantics

The first implementation should provide deterministic ordering without pretending that an incomplete timestamp is precise.

Recommended ordering policy:

1. observations with known `observed_at` or complete observed interval first;
2. use the relevant observed timestamp for primary chronological order;
3. use `recorded_at` only as a deterministic secondary ordering key;
4. use stable observation ID as the final tie-breaker.

Future timeline capabilities may add recurrence, persistence, change points, trajectories, and intervention/outcome analysis. Those are derived capabilities and must retain links to source observations and processing/version metadata.

## 7. Storage boundary

The observation layer must follow the existing SOMA storage architecture.

It must not introduce a parallel plaintext clinical store. Where persistence is implemented, the provider must sit behind a shared contract and respect Identity, Capability, Policy, Gateway, execution, provenance, and audit controls.

The Health Vault remains authoritative for protected personal health records where the existing architecture requires it.

## 8. Authorization and privacy

Observation access is subject-scoped health-data access. Authorization must occur before protected data/key resolution.

The caller must not be able to:

- change the subject scope through query parameters;
- bypass policy by requesting a timeline directly;
- retrieve observations belonging to another subject;
- cause an external AI provider to receive the complete health record merely to construct a timeline.

Timeline generation should retrieve the minimum necessary observation context.

## 9. Canonical-model boundary

The existing `schemas/health-state-v1.json` remains the canonical Health State domain contract. This observation schema is a focused contract for the longitudinal observation repository boundary and must remain semantically aligned with Health State.

The PHR remains a reference/index composition over canonical health entities and vault references; it must not become the owner of observation semantics.

## 10. Non-goals

This v1 does not implement:

- automatic causal inference;
- diagnosis;
- prescribing or medication modification;
- autonomous clinical decisions;
- full Health Evidence Graph implementation;
- evidence-to-action compilation;
- generic safety-event intelligence;
- cloud synchronization;
- specialty-specific repositories;
- continuous device telemetry ingestion;
- an AI health assistant or agent layer.

## 11. Acceptance criteria

The bounded implementation is acceptable when:

- the observation schema is versioned and strict;
- subject identity is explicit;
- observation and recording time are distinct;
- interval semantics are explicit;
- provenance and uncertainty are mandatory;
- association cannot be encoded as `CAUSES`;
- observations remain distinguishable from interpretations;
- timeline ordering is deterministic;
- authorization remains outside and before protected storage/key access;
- no application surface introduces a competing observation model;
- contract tests cover positive and negative boundary cases.
