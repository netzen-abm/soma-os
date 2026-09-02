# SOMA Health Evidence Graph v1

**Status:** Architecture contract / design baseline  
**Date:** 2026-09-02  
**Scope:** Canonical representation of health claims, evidence, provenance, uncertainty, safety, and applicability

## 1. Purpose

The SOMA Health Evidence Graph represents what is known, how it is known, who or what it applies to, how strong the evidence is, what remains uncertain, and what safety context must accompany a health claim.

It is shared infrastructure for research, health intelligence, nutrition, lifestyle, practitioner support, policy analysis, and governed agents.

The graph is not a recommendation engine and not a substitute for clinical judgment.

## 2. Architectural position

```text
Health State
     |
     v
Hypothesis / Question
     |
     v
Evidence Claim
     |
     +--> Study / Source
     |
     +--> Population / Context
     |
     +--> Intervention / Exposure
     |
     +--> Outcome
     |
     +--> Safety
     |
     +--> Uncertainty
     |
     v
Governed interpretation
     |
     v
Capability / intervention decision
```

All access and use remain subject to SOMA Identity, Capability, Policy, Gateway, and Audit infrastructure.

## 3. Core evidence principle

Evidence must answer more than:

> "Is there a study?"

The graph must preserve:

- what was claimed;
- what was actually studied;
- population;
- context;
- intervention/exposure;
- comparator where relevant;
- outcome;
- evidence design;
- strength/quality;
- safety;
- limitations;
- uncertainty;
- provenance;
- verification state.

## 4. Canonical entities

### Claim

A bounded proposition that can be evaluated against evidence.

Required conceptual fields:

- claim ID;
- statement;
- claim type;
- population/context;
- intervention/exposure;
- outcome;
- evidence status;
- evidence level;
- uncertainty;
- safety classification;
- review status.

Claims must be atomic enough to assess. Composite claims should be decomposed when different components have different evidence.

### Source

The originating publication, guideline, registry, institutional document, dataset, or self-published source.

Preserve:
- source ID;
- source class;
- title;
- publisher/institution;
- publication date;
- URL/identifier;
- retrieval date;
- provenance metadata.

### Study

A research unit associated with a claim. Multiple records referring to the same underlying study must not be counted as independent evidence.

### Population

The population actually represented by the evidence, including inclusion/exclusion characteristics where material.

### Intervention / Exposure

What participants received, consumed, practiced, or were exposed to. Dose/intensity, duration, route, and comparator should be captured when relevant and available.

### Outcome

The measured or reported endpoint. Outcome type, measurement method, timing, and direction should be retained where available.

### Evidence Assessment

The structured assessment of the evidence supporting or challenging a claim.

### Safety Assessment

A distinct assessment covering adverse effects, interactions, contraindications, special populations, and safety uncertainty.

### Applicability

A structured statement of how closely the evidence population/context matches the target use.

## 5. Evidence levels

SOMA's working evidence scale:

- `E0_UNKNOWN` — no meaningful evidence assessment yet;
- `E1_PLAUSIBLE` — plausible rationale without adequate outcome evidence;
- `E2_PRELIMINARY` — early or limited evidence;
- `E3_SUPPORTED` — meaningful supporting evidence with limitations;
- `E4_WELL_SUPPORTED` — strong and reasonably consistent evidence for the defined question/context.

Evidence level is contextual, not a universal score for an intervention.

A high evidence level for one population/outcome does not automatically transfer to another.

## 6. Safety classification

Safety is independent of efficacy.

Initial classification:

- `LOW_CONCERN`
- `CONTEXT_DEPENDENT`
- `CLINICALLY_SIGNIFICANT_CAUTION`
- `CONTRAINDICATED`
- `INSUFFICIENT_SAFETY_EVIDENCE`

A claim may be well-supported for an outcome while still requiring substantial safety controls in a particular population.

## 7. Review states

Align with the SOMA medical-data governance standard:

```text
UNREVIEWED
  -> SOURCE_FOUND
  -> SOURCE_VERIFIED
  -> EVIDENCE_ASSESSED
  -> SAFETY_REVIEWED
  -> PUBLISHED
```

`WITHDRAWN` is a terminal governance state when an object must no longer be published as active evidence.

No `UNREVIEWED` object may be presented as verified clinical evidence.

## 8. Provenance requirements

Every evidence object must retain enough provenance to independently trace the representation back to its source.

At minimum:

- source reference;
- source URL or persistent identifier when available;
- retrieval date;
- extraction/assessment method;
- source location or provenance span where practical;
- reviewer/agent provenance;
- transformation history for derived objects.

If a verification URL cannot be established, record that verification is unavailable. Never fabricate a URL.

## 9. Claim lifecycle

```text
Discovery
   -> Extraction
   -> Source verification
   -> Claim normalization
   -> Evidence assessment
   -> Safety assessment
   -> Applicability assessment
   -> Publication decision
   -> Monitoring / re-review
```

Discovery sources such as social media, WhatsApp channels, blogs, or commercial material can generate candidate claims. They do not automatically establish evidence.

## 10. Relationship vocabulary

Initial canonical edges:

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

Do not use `CAUSES` unless the underlying evidence and causal inference method explicitly justify that relation.

## 11. Evidence versus personal health state

The graph must preserve the boundary between general evidence and an individual's observations.

```text
Evidence Claim
     |
     | may inform
     v
Interpretation of Personal Observation
```

It must never silently transform:

- population evidence into personal diagnosis;
- a personal response into population efficacy;
- correlation into causation;
- mechanistic plausibility into clinical effectiveness.

## 12. Applicability model

An evidence object should support contextual dimensions such as:

- age band;
- sex where relevant;
- health status/condition;
- baseline state;
- geography/cultural context where relevant;
- intervention dose/intensity;
- duration;
- adherence;
- co-interventions;
- resource requirements.

Only dimensions material to the evidence need be populated. Unknown values remain unknown.

## 13. Contradictory evidence

Contradiction is a first-class state, not an error to be hidden.

Where credible studies disagree, SOMA should preserve both and represent possible reasons such as:

- population differences;
- intervention differences;
- outcome definitions;
- duration;
- methodological quality;
- statistical uncertainty;
- publication bias;
- biological heterogeneity.

A synthesis may conclude that evidence is uncertain rather than selecting one source by convenience.

## 14. Traditional and emerging knowledge

Traditional knowledge, expert material, mechanistic hypotheses, and emerging claims can be represented as knowledge objects.

They must retain their source class and epistemic status.

They must not be silently upgraded to clinical evidence merely because they are coherent, longstanding, popular, or mechanistically plausible.

## 15. Systems-biology compatibility

The graph should eventually support:

```text
Mechanism
   -> pathway/network
   -> intervention/perturbation
   -> biomarker response
   -> phenotype
   -> outcome
```

These relationships require stronger scientific qualification than ordinary informational links. Mechanistic hypotheses must remain explicitly labelled as hypotheses until appropriately validated.

## 16. Evidence Passport

A user-facing or capability-facing evidence summary may be generated as an **Evidence Passport** containing:

- intervention/question;
- defined population/context;
- evidence level;
- key outcomes;
- evidence basis;
- safety classification;
- limitations;
- uncertainty;
- applicability;
- last evidence review;
- source references.

The passport is a view over canonical graph objects, not a separate source of truth.

## 17. Versioning

Every schema and assessment model must be versioned.

Changes to evidence level definitions, safety classes, or semantic relations require explicit migration/version handling.

Historical assessments should remain reconstructable rather than silently overwritten.

## 18. Minimal v1 object envelope

Conceptually:

```text
id
entity_type
schema_version
status
claim_or_subject_ref
source_refs
study_refs
population_ref
context
intervention_or_exposure
outcome
assessment
safety
applicability
limitations
uncertainty
relationships
provenance
created_at
updated_at
```

This is a design envelope, not yet the final JSON Schema.

## 19. Non-goals for v1

Do not attempt to solve all of:

- automated systematic review replacement;
- fully autonomous clinical evidence grading;
- universal causal inference;
- complete biomedical knowledge graph coverage;
- whole-body mechanistic simulation;
- automatic medical recommendation generation without policy/safety controls.

## 20. Acceptance criteria

The Evidence Graph is implementation-ready when:

- claims are atomic and versioned;
- source classes are explicit;
- study deduplication is represented;
- provenance is mandatory;
- evidence and safety are independently represented;
- applicability is contextual;
- contradictions are representable;
- uncertainty is preserved;
- personal observations remain distinct from general evidence;
- traditional/emerging sources retain epistemic status;
- every published claim is traceable to source evidence;
- policy and audit integration points are defined.

## 21. Decision

Adopt this document as the architecture baseline for the SOMA Health Evidence Graph v1.

The next implementation step is a machine-readable schema that explicitly links Health State entities to Evidence Graph entities without making either the source of truth for the other.
