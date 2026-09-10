# SOMA Health Safety Event Intelligence v1

**Status:** Architecture direction / future implementation baseline  
**Date:** 2026-09-10  
**Scope:** AEFI, medication adverse effects, device/procedure events, and other health safety signals.

## 1. Purpose

SOMA should provide one reusable safety-event architecture rather than separate implementations for vaccines, medications, devices, procedures, or particular diseases.

The same contract must work from routine preventive care through complex longitudinal health journeys.

## 2. Fundamental distinction

> **An event following an exposure is not automatically an event caused by that exposure.**

Therefore SOMA must preserve at least four separate concepts:

1. exposure/intervention;
2. observed event;
3. temporal relationship;
4. causality assessment.

Temporal proximity alone must never be silently converted into causality.

## 3. Generic safety event

A future canonical safety event should be capable of representing:

```text
SafetyEvent
 ├─ subject
 ├─ exposure/intervention
 ├─ product/agent/device/procedure
 ├─ exposure time
 ├─ observed event
 ├─ onset time
 ├─ severity
 ├─ seriousness
 ├─ concurrent exposures
 ├─ relevant prior history
 ├─ clinical assessment
 ├─ outcome
 ├─ causality assessment
 ├─ source/provenance
 ├─ reporting status
 └─ follow-up status
```

The record should preserve the original observation even when a later review changes the interpretation.

## 4. AEFI

SOMA should support **Adverse Event Following Immunisation (AEFI)** as a safety-observation and evidence-assessment workflow.

Potential information includes:

- vaccine/product identity;
- dose and administration details where available;
- batch/lot and manufacturer where available;
- administration date/time;
- route/site where available;
- observed signs and symptoms;
- onset and duration;
- severity/seriousness;
- concurrent medicines, illnesses and exposures;
- relevant prior reactions/allergies;
- clinical evaluation;
- investigations;
- outcome;
- causality assessment and assessment method;
- source and provenance;
- follow-up.

SOMA should not label an AEFI as vaccine-caused solely because it happened after vaccination.

## 5. Medication adverse effects

The same architecture should support suspected adverse effects following medication exposure.

Relevant context can include:

- medication identity and formulation;
- dose/frequency/route as recorded;
- start/stop/change dates;
- concurrent medicines and supplements;
- observed event;
- onset relative to exposure;
- severity/seriousness;
- known safety evidence;
- alternative explanations;
- clinical assessment;
- outcome;
- causality assessment;
- provenance and follow-up.

SOMA may surface a possible safety relationship or help prepare questions for a clinician/pharmacist. It must not independently change prescribed treatment.

## 6. Evidence integration

Safety evidence is independent from efficacy evidence.

A product/intervention can have:

```text
strong efficacy evidence + important safety concern
strong safety evidence + uncertain efficacy
limited efficacy evidence + limited safety evidence
```

The Health Evidence Graph should therefore link safety assessments independently rather than deriving safety from efficacy.

Relevant evidence processing:

```text
Observed event
   ↓
Source verification
   ↓
Exposure verification
   ↓
Temporal relationship
   ↓
Alternative explanations / co-exposures
   ↓
Known safety evidence
   ↓
Severity / seriousness
   ↓
Causality assessment
   ↓
Uncertainty
   ↓
Monitoring / follow-up
```

## 7. No automatic regulatory reporting claim

Recording or analysing a safety event does not by itself authorize regulatory reporting.

Any future reporting capability must have its own:

- jurisdiction;
- reporting standard;
- consent/legal basis;
- identity requirements;
- human review;
- submission audit;
- correction/withdrawal handling;
- destination verification.

The architecture in this document does not authorize autonomous submission.

## 8. Longitudinal learning

Repeated safety observations can become longitudinal signals without prematurely becoming causal conclusions.

For example:

```text
Exposure A → Event X
Exposure A → Event X
Exposure A → Event X
```

may justify investigation, but the system must still assess confounding, background incidence, alternative explanations, data quality, independent evidence, and appropriate causal methodology.

## 9. Privacy and access

Safety-event records are health data and inherit SOMA's privacy/security model:

- local-first storage where appropriate;
- encryption;
- authorization before decryption;
- minimum-necessary disclosure;
- explicit sharing scope;
- provenance;
- auditability;
- controlled research contribution;
- no hidden secondary use.

## 10. Relationship to the wider SOMA model

Safety events are not a separate health database. They become canonical health/evidence entities linked to the same:

```text
Health Vault
Canonical Health Model
Health Contexts
Health Evidence Graph
Policy Kernel
Identity
Provenance
Audit
Sharing
Research Contribution
Health Intelligence
```

This permits one person to have vaccination history, medication exposure, seasonal illness, chronic conditions, rehabilitation, athletic activity and other records in one longitudinal model.

## 11. Future implementation order

Do not implement the full safety subsystem during the Local Vault Storage & Index milestone.

Recommended sequence:

1. define canonical safety-event schema;
2. define provenance and temporal semantics;
3. define causality status/method contract;
4. integrate medication and immunisation records;
5. implement safety-event repository operations;
6. add evidence/safety graph linkage;
7. add longitudinal signal detection;
8. add governed reporting adapters only where justified.

## 12. Safety principle

> **Record broadly. Preserve the observation. Separate association from causation. Assess safety independently. Escalate according to evidence and authority.**
