# SOMA Cross-Paradigm Evidence Methodology v1

## Status

Proposed architecture contract. This document defines how SOMA evaluates health claims across different medical and knowledge systems without granting automatic epistemic privilege or exemption to any paradigm.

## Core principle

> No medical paradigm receives automatic epistemic privilege. No medical paradigm receives automatic exemption from scrutiny.

SOMA must not reduce a health system to a single `scientificity` score. It must evaluate a specific claim, intervention, diagnostic construct, outcome, or theory using methods appropriate to the question while preserving universal requirements for evidence integrity, safety, transparency, provenance, and uncertainty.

## Why this exists

A research method is an instrument for answering particular questions. Treating one method or evidence hierarchy as the definition of truth can create category errors when the object of study is systemic, relational, longitudinal, individualized, multi-component, experiential, or otherwise poorly represented by a single reductionist design.

This does **not** mean that unconventional or traditional claims are presumed true. It means that SOMA must distinguish:

- lack of evidence from evidence of absence;
- inability of a study design to establish causality from evidence that an observed phenomenon did not occur;
- empirical observation from causal explanation;
- causal explanation from ontological theory;
- paradigm coherence from external validation;
- methodological limitation from falsification.

## Universal integrity requirements

Regardless of paradigm, SOMA evaluates:

1. **Observation** — was something actually observed or recorded?
2. **Definition** — is the claim/intervention/outcome sufficiently specified?
3. **Measurement** — how was it measured and with what uncertainty?
4. **Provenance** — where did the information originate and can the chain be inspected?
5. **Internal validity** — what biases, confounding, selection effects, attrition, measurement error, or expectancy effects could explain the result?
6. **Causal inference** — what does the design justify claiming about causality?
7. **External validity** — for whom and under what conditions might the result apply?
8. **Replication** — has it been reproduced independently or consistently?
9. **Negative evidence** — were non-response, adverse outcomes, dropout, contradiction, and failed attempts preserved?
10. **Safety** — what harms, contraindications, interactions, and unknowns exist?
11. **Transparency** — are methods, outcomes, exclusions, conflicts, and limitations visible?
12. **Claim scope** — is the conclusion broader than the underlying evidence?
13. **Alternative explanations** — were plausible competing explanations examined?
14. **Uncertainty** — what remains unknown?

## Paradigm-aware dimensions

SOMA additionally records the context in which a claim is made:

- `knowledge_system`
- `epistemology`
- `diagnostic_model`
- `intervention_model`
- `outcome_model`
- `research_method`
- `evidence_framework`
- `claim_type`
- `context_of_use`

These dimensions describe the claim; they do not determine its truth.

## Evidence is multi-dimensional

SOMA must not use a single ladder such as `RCT > cohort > case report > traditional knowledge` as a universal measure of all knowledge.

Evidence quality is instead assessed across dimensions including:

- observational strength;
- measurement quality;
- causal inference;
- longitudinal consistency;
- external validity;
- replication;
- mechanistic support;
- safety evidence;
- applicability to the person/context;
- cross-paradigm convergence;
- contradiction;
- provenance and transparency.

A lower-causal-inference design can still contain valuable longitudinal observations. Conversely, a highly controlled experiment can answer a narrow question without validating a broad theory.

## Four claim layers

SOMA separates:

### 1. Phenomenological claim

Something appears to happen under specified conditions.

### 2. Empirical relationship

X is associated with Y under conditions Z.

### 3. Causal claim

X causes Y through mechanism or causal pathway M.

### 4. Ontological/theoretical claim

A proposed model of the body, mind, disease, or reality is correct.

Evidence supporting one layer must not automatically be promoted to the next.

## Cross-paradigm comparison

When comparing medical systems, SOMA compares **specific claims and outcomes**, not entire paradigms as though they were interventions.

Example:

```text
Question: Does intervention X improve outcome Y in population Z?

System A
  diagnostic model → intervention model → research method → outcome

System B
  diagnostic model → intervention model → research method → outcome

SOMA comparison
  observation
  measurement
  causal inference
  applicability
  safety
  replication
  contradictions
  uncertainty
```

The comparison must preserve each system's own diagnostic and intervention constructs while translating only the minimum needed for interoperable analysis.

## Traditional, complementary, integrative and emerging knowledge

SOMA may ingest and analyze:

- conventional biomedicine;
- Ayurveda;
- Siddha;
- Unani;
- traditional Chinese medicine;
- yoga-based interventions;
- nutrition and lifestyle interventions;
- functional medicine/nutrition;
- indigenous knowledge;
- emerging hypotheses;
- experiential or observational knowledge;
- mechanistic and laboratory research.

Inclusion in the evidence graph is **not endorsement**.

## Safety boundary

A claim may be interesting or promising while remaining unsafe to recommend. Evidence status and safety status are therefore independent fields.

SOMA must not use uncertainty as permission for high-risk experimentation.

High-consequence interventions require stronger safety controls and, where appropriate, qualified human review.

## Evidence status

SOMA retains the existing epistemic statuses:

- `ESTABLISHED`
- `SUPPORTED`
- `PRELIMINARY`
- `MIXED`
- `UNCERTAIN`
- `SPECULATIVE`
- `UNVERIFIED`
- `CONTRADICTED`
- `DISPROVEN`
- `SAFETY_RESTRICTED`

These statuses describe the current evidence state for a **specific claim**, not the reputation of an entire medical system.

## Anti-bias rule

SOMA must resist both forms of epistemic bias:

```text
Dominant-paradigm bias:
"Not demonstrated by our preferred method" → "false"

Counter-establishment bias:
"Rejected by the dominant paradigm" → "true"
```

Both are invalid shortcuts.

The correct sequence is:

```text
Claim
 ↓
Knowledge-system context
 ↓
Appropriate research question
 ↓
Evidence retrieval
 ↓
Integrity assessment
 ↓
Contradiction search
 ↓
Safety assessment
 ↓
Uncertainty
 ↓
Interpretation
```

## Padaav as a benchmark case

Padaav research is suitable as an early benchmark because its published material includes Ayurvedic clinical protocols, observational work, case reports, experimental/toxicity research, and research collaborations. SOMA must preserve both reported positive outcomes and limitations, including attrition, uncontrolled design, non-response, adverse outcomes, progression, and deaths where reported.

The benchmark purpose is **not** to validate Padaav or Ayurveda. It is to test whether SOMA can:

- represent a non-biomedical therapeutic model faithfully;
- preserve provenance;
- distinguish observed signal from causal conclusion;
- preserve negative outcomes;
- identify methodological limitations;
- compare related questions across paradigms;
- avoid both dismissal and endorsement by label.

## Research output

Every evidence assessment should be capable of producing:

- an Evidence Passport;
- claim-level provenance;
- methods and limitations;
- safety assessment;
- uncertainty;
- contradiction/replication status;
- research gaps;
- recommended next research design where appropriate.

## Non-negotiable rule

> Investigate without prejudice. Conclude according to evidence. Preserve uncertainty where evidence is insufficient.

This methodology is intended to make SOMA more scientifically disciplined, not less.
