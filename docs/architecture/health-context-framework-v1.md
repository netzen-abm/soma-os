# SOMA Health Context Framework v1

**Status:** Architecture contract / design baseline
**Date:** 2026-09-08
**Scope:** SOMA-OS only

## 1. Purpose

SOMA must support materially different health journeys without creating separate health databases, separate identity systems, or separate application silos.

A person may be a general health user, professional athlete, retired veteran/ex-service member, rehabilitation participant, or another supported context at different times in life. These are contexts over the same longitudinal person-centered health record, not separate products.

The Health Context Framework provides a reusable way to add context-specific observations, goals, workflows, analytics, sharing policies, and evidence without duplicating the canonical health domain.

## 2. Product decision

SOMA remains **Your Personal Health Intelligence System** and a research-oriented health and wellness environment.

Media/video/podcasts are supporting discovery and evidence-input capabilities. They are not the product identity.

The product loop is:

```text
Research
  -> Understand
  -> Personalize
  -> Monitor
  -> Learn
  -> Share
  -> Improve
```

The infrastructure loop remains:

```text
Identity
  -> Capability
  -> Policy
  -> Gateway
  -> Execution
  -> Evidence
  -> Audit
```

## 3. Architectural rule

**One person. One longitudinal health memory. Multiple governed contexts.**

Context modules MUST reference canonical Health State, Health Evidence Graph, Personal Health Record, Local Health Vault, identity, policy, provenance, safety, and audit infrastructure.

Context modules MUST NOT create parallel canonical health models merely because a domain has specialized terminology.

## 4. Initial context taxonomy

### 4.1 `general_health`

Default context for health, wellness, nutrition, lifestyle, prevention, monitoring, goals, and longitudinal personal health organization.

### 4.2 `athlete_performance`

For recreational, competitive, elite, and professional athletes.

The context combines health, physiology, training, recovery, injury, nutrition, performance, and longitudinal outcomes.

### 4.3 `service_veteran`

For serving or retired military/ex-service personnel where the user chooses to maintain a service-related health context.

It connects service history and occupational/deployment context to health records without automatically asserting causality between service and a current condition.

### 4.4 `rehabilitation_recovery`

Reusable across injury recovery, surgery, neurological rehabilitation, chronic functional recovery, post-illness recovery, and other appropriate rehabilitation journeys.

### 4.5 Future contexts

Potential future contexts include healthy aging, occupational health, pregnancy, pediatric health, caregiver support, expedition/extreme-environment health, and research participation.

These are not part of the v1 implementation unless justified by requirements and safety review.

## 5. Canonical data principle

Context-specific data is represented as observations, events, goals, interventions, responses, outcomes, documents, measurements, and relationships in the canonical domain.

A context may add a typed context envelope or index that references those canonical records.

Conceptually:

```text
Person
  |
  +-- Local Health Vault
  |
  +-- Personal Health Profile
  |
  +-- Health State / Observations / Events
  |
  +-- Evidence Graph
  |
  +-- Health Contexts
        |
        +-- general_health
        +-- athlete_performance
        +-- service_veteran
        +-- rehabilitation_recovery
```

## 6. Athlete Performance Context

### 6.1 Performance data

May include:

- training sessions;
- training load;
- intensity;
- duration;
- distance;
- speed;
- power;
- strength;
- repetitions;
- competition results;
- personal bests;
- technical or tactical performance observations where appropriate.

### 6.2 Physiology and recovery

May include:

- resting heart rate;
- HRV where available;
- sleep duration/quality;
- respiratory measurements;
- body weight/body composition;
- blood pressure;
- glucose or other measurements where available and appropriate;
- temperature;
- fatigue;
- soreness;
- perceived recovery;
- stress;
- readiness;
- rest/recovery days.

### 6.3 Injury and rehabilitation

May include:

- injury history;
- anatomical location;
- onset/date;
- severity as recorded;
- treatment;
- rehabilitation activities;
- return-to-training milestones;
- return-to-competition milestones;
- recurrence.

### 6.4 Nutrition

May include food records, hydration, nutrition strategy, supplements, and relevant measurements. Supplement information remains subject to evidence and safety evaluation.

### 6.5 Medical context

May include prescriptions, medications, allergies, labs, imaging references, clinician reports, procedures, vaccinations, and clearances where the user chooses to record them.

### 6.6 Athlete intelligence boundary

SOMA may identify patterns and relationships, for example:

```text
training load increased
        +
sleep decreased
        +
soreness increased
        +
performance decreased
        -> possible relationship requiring investigation
```

This is not a causal diagnosis. The output must distinguish:

- observed measurements;
- temporal relationships;
- evidence-supported relationships;
- hypotheses;
- uncertainty;
- questions for an appropriate professional.

### 6.7 Role-based sharing

An athlete may authorize different scopes for:

- athlete/individual;
- sports physician/team physician;
- physiotherapist;
- coach;
- nutrition professional;
- team medical director;
- research department.

Performance data and health data are both sensitive. Employment or team membership does not automatically establish authorization to access the user's vault.

## 7. Service Veteran Context

### 7.1 Longitudinal service-to-health record

The context may organize:

- service history;
- role/occupation;
- deployment or environmental context where voluntarily recorded;
- documented injuries;
- surgeries;
- rehabilitation;
- occupational/exposure history where documented;
- long-term medications;
- chronic health conditions as documented;
- pain and mobility observations;
- sleep and wellbeing observations;
- laboratory results;
- imaging/document references;
- current treatment/follow-up.

### 7.2 Causality boundary

SOMA MUST NOT automatically infer that a current disease or condition was caused by military service.

It may organize documented facts, temporal relationships, exposure histories, research evidence, uncertainties, and questions for clinical or appropriate occupational/legal review.

### 7.3 Medication follow-up

For users taking multiple medicines over long periods, SOMA may support:

- medication reconciliation;
- duplicate-category detection;
- combination and interaction safety signals;
- missing medication information;
- timeline changes;
- symptom/measurement correlation review;
- clinician/pharmacist question generation;
- follow-up reminders where explicitly configured.

It MUST NOT autonomously prescribe, discontinue, substitute, or authorize medication changes.

### 7.4 Monitoring and test prompts

SOMA may identify candidate monitoring or test topics from recorded history, medication context, symptoms, measurements, and evidence.

These are candidate prompts for discussion with an appropriate clinician, not diagnoses, medical orders, or guarantees that a particular test is required.

## 8. Rehabilitation as a reusable context

Rehabilitation/recovery should not be implemented as an athlete-only feature.

The same infrastructure can support:

- sports injury recovery;
- post-operative recovery;
- neurological rehabilitation;
- mobility recovery;
- chronic functional rehabilitation;
- post-illness recovery;
- healthy aging where appropriate.

This reduces duplication and makes the architecture genuinely reusable.

## 9. Privacy and ownership

The default trust boundary remains the user's Local Health Vault.

Context data MUST inherit the same:

- classification;
- identity and authorization rules;
- consent requirements;
- policy enforcement;
- provenance;
- auditability;
- deletion semantics;
- export controls.

Context-specific analytics MUST NOT silently create new external data flows.

An athlete's performance profile must not become a team-owned data store by default. A veteran's service-health context must not become an institutional record by default.

## 10. Intelligence model

SOMA intelligence should operate over authorized context, not over an unrestricted vault.

A context-aware analysis should follow:

```text
Question / Goal
  -> authorized context selection
  -> minimum necessary records
  -> evidence retrieval
  -> contradiction search
  -> safety/policy evaluation
  -> analysis
  -> uncertainty assessment
  -> user-readable result
  -> provenance/audit
```

The distinction remains mandatory:

**Claim != Evidence != Interpretation != Recommendation.**

## 11. Research contribution

Context data can optionally contribute to research.

Examples include longitudinal athlete performance/recovery observations or long-term veteran health follow-up. Participation must be:

- voluntary;
- purpose-specific;
- explicitly consented;
- minimum-necessary;
- withdrawable where technically and legally possible;
- provenance-preserving;
- transparent about attribution and publication/use constraints.

No context creates automatic research consent.

## 12. Emergency profile

The Emergency Health Profile remains separate from the full vault.

Context modules may nominate user-selected emergency facts, but responders/family members must receive only the minimum necessary emergency disclosure artifact under its own policy.

Emergency access MUST NOT become an implicit pathway to general vault access.

## 13. Context lifecycle

```text
Discover context
  -> user enables context
  -> define goals
  -> collect/import authorized records
  -> normalize to canonical model
  -> monitor
  -> analyze
  -> review/share
  -> learn
  -> optionally contribute to research
```

A context may be enabled, paused, archived, or removed without deleting canonical health records unless the user explicitly requests deletion under the vault's deletion rules.

## 14. Product surface implications

SOMA should present one coherent product with context-aware experiences rather than separate apps.

Example navigation:

```text
SOMA
├── Research
├── Evidence
├── My Health
├── Timeline
├── Medications
├── Monitoring
├── Insights
├── Health Library
├── Share
├── Emergency
└── Research Contribution
```

Within My Health, a user can optionally enable contexts such as Athlete, Service/Veteran, or Rehabilitation.

## 15. Non-goals

This contract does not authorize:

- autonomous clinical diagnosis;
- autonomous prescribing;
- automated return-to-play clearance;
- automated disability/service-causality determination;
- insurance/employment decisions;
- military fitness determinations;
- covert team/employer monitoring;
- unrestricted clinician/team/institutional access;
- separate context-specific health databases;
- automatic research participation.

## 16. Implementation sequence

The recommended sequence is:

1. Local Health Vault cryptographic/security contract.
2. Health Context Framework contract and machine-readable schema.
3. General Personal Health Record implementation.
4. Medication/prescription model and longitudinal timeline.
5. Local health intelligence runtime with minimum-necessary context access.
6. Controlled sharing and Emergency Health Profile.
7. Athlete Performance Context.
8. Service Veteran Context.
9. Rehabilitation/Recovery Context.
10. Research contribution integration.
11. Deeper media/evidence discovery integrations.

This sequence deliberately makes privacy and canonical health storage foundational before specialized analytics.

## 17. Acceptance criteria for v1 architecture

- Contexts do not create parallel canonical health models.
- Context records can reference canonical Health State/PHR/Vault records.
- Identity and policy enforcement applies equally to every context.
- Context-specific sharing is explicit and scoped.
- Athlete performance data is treated as sensitive.
- Veteran service context does not imply causality.
- Medication intelligence is reconciliation/safety/question support, not prescribing.
- Test prompts are candidate prompts, not medical orders.
- Emergency data remains a separate minimum-necessary artifact.
- Research contribution is voluntary and purpose-bound.
- AI receives only authorized minimum-necessary context.
- Provenance and uncertainty survive contextual analysis.
- Context addition does not require a new product identity or database.
