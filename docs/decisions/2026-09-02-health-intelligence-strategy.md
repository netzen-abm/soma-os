# ADR — SOMA-OS Health Intelligence Strategy

**Date:** 2026-09-02  
**Status:** Accepted working direction

## Decision

SOMA-OS will pursue **evidence-aware health intelligence** as its product direction, with the working positioning **The Health Intelligence Infrastructure**.

The first product wedge will be **SOMA Metabolic & Lifestyle Health Intelligence**.

## Why

Market analysis indicates strong demand for personalized health, nutrition, biomarkers, coaching, lifestyle interventions and digital health. However, important structural gaps remain in evidence interpretation, provenance, safety, fragmentation, personalization certainty and outcome learning.

SOMA can add more value by addressing these infrastructure gaps than by copying individual consumer wellness features.

## Product thesis

The core question is:

> What actually works for this person, under these conditions, and how do we know?

The product should move from recommendation toward learning:

```text
Observe
  -> Understand
  -> Hypothesize
  -> Intervene
  -> Measure
  -> Learn
  -> Adapt
```

## Architectural consequence

Evidence, provenance, privacy, safety, policy, identity, capabilities, execution and audit remain shared infrastructure.

Applications and interfaces are consumers of these capabilities.

```text
Identity
  -> Capability
  -> Policy
  -> Gateway
  -> Execution
  -> Evidence
  -> Audit
```

## Evidence consequence

SOMA must preserve distinctions between:

- established clinical evidence;
- emerging evidence;
- observational evidence;
- mechanistic plausibility;
- expert opinion;
- traditional knowledge;
- historical claims;
- commercial claims;
- unsupported claims.

The system must not create false certainty by aggregating repeated weak claims.

## Safety consequence

Health information must not silently become diagnosis, individualized prescription, guaranteed cure, or advice to discontinue prescribed treatment.

High-consequence health outputs require stronger controls and appropriate professional oversight.

## Rejected strategic directions

The following are not the primary SOMA product identity:

- generic AI health chatbot;
- calorie tracker;
- supplement marketplace;
- influencer wellness platform;
- root-cause marketing engine without evidence;
- autonomous diagnostic system;
- unlimited testing platform;
- biohacking platform;
- traditional-medicine claim aggregator.

These may contain individual capabilities that could be integrated later only if they fit SOMA's governance and evidence model.

## India strategy

India will be treated as a high-value design and validation environment because it combines dietary diversity, chronic disease burden, affordability constraints, modern medicine, traditional medical knowledge, multilingual populations, family-centered health behavior and rapidly developing digital health infrastructure.

India is not a reason to weaken evidence standards. It is a reason to build stronger evidence and interoperability infrastructure.

## Consequence for development order

The recommended sequence is:

1. secure/stabilize foundation;
2. complete evidence/provenance infrastructure;
3. build one end-to-end metabolic/lifestyle reference flow;
4. add bounded health experiments and outcome tracking;
5. validate usefulness and safety;
6. add practitioner capabilities;
7. add research infrastructure;
8. expand external integrations and protocol adapters.

## Review trigger

Revisit this decision if evidence from real users, practitioners, research partners, regulatory requirements, safety reviews or technical validation materially changes the value proposition.
