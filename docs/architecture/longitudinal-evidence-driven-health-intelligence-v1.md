# SOMA Longitudinal Evidence-Driven Health Intelligence v1

**Status:** Architecture direction / future implementation baseline  
**Date:** 2026-09-10  
**Scope:** General health, acute/seasonal illness, prevention and immunisation, medication safety, adverse events, chronic/complex conditions, rehabilitation, performance, and research.

## 1. Purpose

SOMA should treat the same evidence-and-intelligence architecture as applicable across the full health spectrum—from an ordinary seasonal illness or routine immunisation to complex, multi-modal, longitudinal conditions.

The architecture must therefore not be cancer-specific. Cancer and advanced oncology are one demanding example of a broader problem: understanding an individual through heterogeneous evidence collected over time.

The objective is **evidence-aware health intelligence**, not autonomous diagnosis or treatment.

## 2. Core principle

> **Instrument broadly. Normalize carefully. Preserve provenance. Reason cautiously. Act only within evidence and authority.**

A second governing distinction is:

> **Claim ≠ Evidence ≠ Interpretation ≠ Recommendation.**

AI output is never promoted to clinical fact merely because a model produced it.

## 3. Universal health-intelligence loop

```text
Observe / Import
      ↓
Verify source and integrity
      ↓
Normalize into canonical health observations
      ↓
Preserve provenance and context
      ↓
Build longitudinal timeline / trajectory
      ↓
Retrieve relevant evidence
      ↓
Search contradictions and negative evidence
      ↓
Assess evidence quality, applicability and uncertainty
      ↓
Safety assessment
      ↓
Generate bounded interpretation / questions / options
      ↓
Human or clinician decision where appropriate
      ↓
Monitor outcome
      ↓
Learn / re-evaluate
```

## 4. The same architecture across health situations

### 4.1 Seasonal/common acute illness

SOMA may eventually combine symptoms, temperature, hydration, sleep, activity, exposure context, medications, prior history and relevant evidence to help the person understand a changing episode.

The system must distinguish observation from diagnosis and should surface uncertainty, red flags and appropriate care-seeking prompts rather than claim certainty.

### 4.2 Routine immunisation / vaccination

SOMA should be capable of maintaining an immunisation record and supporting evidence-aware questions such as schedule status, product/lot provenance where available, prior reactions, contraindication questions, and post-immunisation monitoring.

A vaccination record is an observation/history record. A recommendation must remain context- and authority-bound.

### 4.3 Adverse events following immunisation

The architecture must support **AEFI (Adverse Event Following Immunisation)** as a temporal safety-observation workflow.

Critical distinction:

```text
Event after immunisation
        ≠
Event caused by immunisation
```

SOMA should capture temporal association, product and administration provenance where available, symptoms/signs, onset, duration, severity, concurrent exposures/medications, relevant clinical evaluation, outcome, and evidence assessment. Causality must not be inferred from temporal proximity alone.

### 4.4 Adverse effects following medication

The same model should support adverse drug/medication events:

```text
Medication exposure
      ↓
Observed event
      ↓
Temporal relationship
      ↓
Alternative explanations / co-medications
      ↓
Known safety evidence
      ↓
Severity / outcome
      ↓
Causality assessment where appropriate
```

The system must not independently instruct a person to stop, start, increase or decrease prescribed medication merely because an association is suspected. High-risk medication decisions require appropriate clinical authority.

### 4.5 Chronic and complex conditions

Longitudinal evidence becomes more important as conditions involve multiple organs, therapies, specialists, comorbidities, biomarkers, imaging, lifestyle factors, or competing explanations.

### 4.6 Advanced biological evidence

Future modalities may include genomics, transcriptomics, proteomics, metabolomics, single-cell data, spatial data, ctDNA, pathology, imaging and functional assays. These are evidence modalities—not automatic indicators of evidence strength.

## 5. Longitudinal model

SOMA should evolve from isolated records toward a temporal evidence model:

```text
Observation(t1)
Observation(t2)
Observation(t3)
Observation(t4)
       ↓
Trajectory
       ↓
Change / persistence / recurrence
       ↓
Contextual interpretation
       ↓
Evidence assessment
```

Useful derived concepts may include trends, change points, persistence, recurrence, temporal relationships and intervention/outcome relationships. Derived signals must retain links to their source observations and processing version.

## 6. High-dimensional evidence principle

Large datasets do not automatically produce better intelligence.

SOMA should progressively transform data while preserving traceability:

```text
Raw evidence
    ↓
Validated observation
    ↓
Derived feature
    ↓
Evidence relationship
    ↓
Relevant signal
    ↓
Bounded question / interpretation
```

The raw vault remains authoritative for the user's stored evidence. AI receives only the minimum necessary context through governed retrieval.

## 7. Evidence modalities are not evidence hierarchies

SOMA must not encode assumptions such as:

```text
omics > wearable > questionnaire
```

or:

```text
RCT > all other forms of knowledge for every question
```

Evidence quality is multidimensional and question-dependent. The Health Evidence Graph remains responsible for evidence assessment, including causal inference, applicability, safety, contradiction, negative evidence and provenance.

## 8. Safety event model

Future safety infrastructure should support a generic event model that can represent:

- medication adverse effects;
- AEFI;
- device-related adverse events;
- treatment complications;
- procedure-associated events;
- unexpected symptoms;
- laboratory abnormalities;
- other safety signals.

Minimum conceptual fields should include:

```text
subject
exposure/intervention
product or agent
exposure timestamp
observed event
onset timestamp
severity
seriousness
concurrent exposures
relevant history
clinical assessment
outcome
causality status
causality method
source/provenance
reporting status
follow-up status
```

**Causality status must remain distinct from temporal association.**

## 9. Privacy and security

Health intelligence must preserve the existing SOMA security architecture:

- local-first Health Vault where feasible;
- encrypted records and encrypted indexes;
- authorization before decryption;
- minimum-necessary AI context;
- explicit consent for research contribution and sharing;
- purpose/scope-bound sharing;
- provenance and auditability;
- no hidden secondary use;
- no autonomous clinical authority granted to AI.

## 10. Research contribution

When users voluntarily contribute data for research, the contribution must remain purpose-specific, consented, withdrawable where applicable, provenance-preserving and attribution-aware.

A personal event or observation may become a research signal only through an explicit governed pathway. Personal data must not silently become research data.

## 11. Agentic intelligence guardrails

SOMA may eventually use specialized agents for retrieval, evidence extraction, contradiction search, longitudinal analysis, summarisation, coding and workflow orchestration.

Agents must operate under:

```text
Identity
  ↓
Capability
  ↓
Policy
  ↓
Data minimization
  ↓
Evidence retrieval
  ↓
Reasoning
  ↓
Safety checks
  ↓
Provenance
  ↓
Audit
```

Autonomous self-healing is acceptable for bounded software-development workflows only when changes remain scoped, reviewable and verified. Health-facing agents must not acquire authority merely because they can reason or act.

## 12. Implementation boundary

This document is a **future architecture baseline**, not authorization to implement every capability immediately.

Current priority remains the Local Health Vault Storage & Index work. Future implementation should proceed incrementally through contracts, schemas, tests, security review and CI evidence.

Recommended future sequence:

1. Local Health Vault Storage & Index v1
2. Longitudinal Health Timeline / Observation model
3. Medication and immunisation record intelligence
4. Generic safety-event / adverse-event contract
5. Temporal evidence and signal detection
6. Local Health Intelligence runtime
7. Evidence-aware clinical/research retrieval
8. Advanced biological evidence adapters
9. Research contribution expansion

## 13. Non-goals

This architecture does not by itself establish:

- diagnosis;
- treatment efficacy;
- causality;
- clinical validity of an imported measurement;
- superiority of any medical paradigm;
- authorization to prescribe or alter medication;
- authorization to make vaccination decisions;
- authorization to submit regulatory safety reports automatically.

Those require their own evidence, safety, policy and governance contracts.

## 14. Design conclusion

The lesson from highly instrumented individual health journeys is not that every person needs maximal diagnostics. The lesson is that SOMA should be capable of **proportionate instrumentation**: simple for ordinary needs, richer when justified, and capable of scaling to complex evidence without changing its foundational model.

The same infrastructure can therefore support:

```text
Seasonal illness
      ↓
Routine prevention / immunisation
      ↓
Medication management and safety
      ↓
AEFI / adverse-event observation
      ↓
Chronic disease
      ↓
Rehabilitation
      ↓
Athletic performance
      ↓
Complex multi-system health
      ↓
Advanced biological research
```

One SOMA. One governed Health Intelligence Infrastructure. Different contexts, evidence modalities and capabilities over the same underlying health foundation.
