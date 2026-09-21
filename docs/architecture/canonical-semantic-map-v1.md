# SOMA Canonical Semantic Map v1

## Decision

SOMA must consolidate existing Health State, longitudinal observation, relationship, evidence, intervention, response, outcome, safety, and research contracts into one shared semantic substrate. This map is a consolidation artifact, not a new domain model.

## Canonical ownership

| Semantic concern | Canonical owner | Boundary |
| --- | --- | --- |
| Health-domain entities | Health State Model v1 | `schemas/health-state-v1.json` |
| Protected persistence | Health Vault | existing Local Health Vault |
| Observation/timeline | Longitudinal Observation & Timeline v1 | governed repository/context |
| Relationships | Health State + bounded longitudinal vocabulary | no generic graph engine |
| Health↔Evidence linkage | explicit linkage contract / Evidence Graph | separate evidence domain |
| Research protocol↔study relation | Evidence Research domain contract | provider-neutral research service |
| Causality assessment | Evidence Causality Assessment v1 | after evidence/safety gates |
| Safety events | Health Safety Event Intelligence v1 | future canonical safety capability |
| Contexts | Health Context Framework v1 | references canonical entities |
| Authorization | Canonical Authorization Decision Boundary | single policy authority |
| Capability identity | Canonical Capability Registry | ID + version |
| Provenance/uncertainty | canonical health/evidence contracts | preserved end-to-end |

## Semantic spine

```
Observation
    ↓
Relationship / Hypothesis
    ↓
Evidence
    ↓
Safety / Uncertainty Assessment
    ↓
Causality Assessment where justified
    ↓
Action / Intervention
    ↓
Response
    ↓
Outcome
    ↓
New Observation
```

The repository must not collapse these layers.

## Non-equivalence rules

- Observation is not interpretation.
- Relationship is not causation.
- Temporal precedence is not causation.
- Intervention is not efficacy.
- Response is not proof of intervention effect.
- Evidence is not personal health fact.
- AI output is not canonical fact.
- Research consent is not implied by health-context activation.
- Context is not a separate health database.
- A Health State repository is not a graph database.
- Safety evidence is not efficacy evidence.
- Causality assessment is not diagnosis or treatment recommendation.

## Relationship vocabulary convergence

The existing contracts already define bounded relationship semantics. The canonical longitudinal vocabulary must be consolidated rather than duplicated.

Core longitudinal relationships include:

- `HAS_CONTEXT`
- `HAS_INTERVENTION`
- `HAS_RESPONSE`
- `HAS_OUTCOME`
- `DERIVED_FROM`
- `TEMPORALLY_PRECEDES`
- `ASSOCIATED_WITH`
- `MEASURED_BY`
- `EVIDENCE_LINKED`

Health State also defines entity-oriented relationships such as `HAS_OBSERVATION`, `HAS_GOAL`, `SUPPORTS_HYPOTHESIS`, and `REFUTES_HYPOTHESIS`. These must be treated as one governed vocabulary with explicit domain applicability, not as competing relationship systems.

`CAUSES` is not part of the foundational observation relationship vocabulary.

## Evidence boundary

Evidence Research remains provider-neutral. Provider adapters retrieve source material; the core retains normalization, directness, evidence assessment, contradiction handling, safety gating, and provenance.

The causal-assessment boundary accepts already safety-gated evidence and does not silently reweight or reinterpret the evidence.

## Authorization boundary

Every protected operation continues to use:

```
Principal / Actor
    ↓
Identity
    ↓
Subject
    ↓
Capability + Version
    ↓
Resource + Action
    ↓
Tenant + Data Domain
    ↓
Canonical Authorization Decision
    ↓
Protected Execution
```

No semantic layer may introduce a second authorization engine.

## Storage boundary

The semantic map does not authorize a new graph database or relationship datastore.

Existing protected storage remains authoritative. A future relationship persistence implementation must first prove that existing repository/provider boundaries cannot satisfy the contract.

## AI boundary

AI and agents may propose hypotheses, relationships, evidence summaries, or assessments. A proposal remains a proposal until it passes the relevant governed contract. Generated content must retain provenance and uncertainty and must never silently become an observation or causal fact.

## Ecosystem boundary

The semantic substrate is shared by:

- health;
- wellness;
- nutrition;
- research/evidence;
- AI and agents;
- device/sensor adapters;
- web/mobile and other client surfaces.

These are capabilities and surfaces of one SOMA ecosystem, not separate canonical health systems.

## Implementation gate

Before new semantic persistence is implemented:

1. consolidate vocabulary definitions;
2. map schema ownership;
3. define machine-readable relationship constraints;
4. define canonical capability IDs/versions for relationship operations;
5. add mechanical authorization invariants;
6. add provenance and uncertainty validation;
7. prove no duplicate storage or policy layer is introduced;
8. implement the minimum provider-neutral repository change.

## Decision

Treat this semantic map as the review baseline for the next longitudinal infrastructure milestone. No new relationship datastore, causal engine, or application-specific semantic model should be introduced until the map's ownership and invariants are satisfied.
