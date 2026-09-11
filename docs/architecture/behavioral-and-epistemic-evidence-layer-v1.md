# SOMA Behavioral & Epistemic Evidence Layer v1

**Status:** architecture proposal / implementation baseline
**Scope:** shared SOMA ecosystem infrastructure

## Purpose

SOMA should be able to represent and evaluate behavioral, psychological, lifestyle, traditional, complementary, conventional, and emerging health knowledge without creating a separate application or granting automatic epistemic privilege to any paradigm.

The governing rule is:

> Investigate without prejudice. Conclude according to evidence.

No medical or knowledge paradigm receives automatic epistemic privilege, and no paradigm receives automatic exemption from scrutiny.

## 1. Evidence before interpretation

A SOMA operation must preserve the distinction between:

`Observation -> Claim -> Evidence -> Assessment -> Interpretation -> Recommendation`

A model-generated interpretation is never promoted to evidence merely because it is plausible, popular, or confidently stated.

## 2. Behavioral intervention representation

CBT-style thought records demonstrate a useful structured workflow: situation/context, automatic thought, emotion/intensity, alternative interpretation, and outcome/re-rating. The structure is useful as a behavioral observation protocol, not as permission for SOMA to diagnose a mental disorder or act as a therapist.

A future canonical behavioral record should preserve:

- event/context
- self-reported thought or belief
- reported affect/state and intensity
- user-selected or observed response
- intervention/exercise used
- immediate outcome
- later outcome when available
- provenance
- uncertainty
- source and method
- consent/purpose
- schema/version

The system must not silently infer a cognitive distortion, diagnosis, cause, or treatment response as fact. Model classifications are interpretations with provenance and confidence.

## 3. Knowledge-system neutrality

For cross-paradigm research, capture at least:

- knowledge_system
- claim_type
- epistemic_method
- population
- intervention_or_exposure
- comparator
- outcome
- context
- source/provenance
- evidence_design
- evidence_level
- bias/confounding considerations
- replication status
- contradiction status
- applicability
- safety assessment
- uncertainty
- epistemic status

This is not a ranking of medical traditions. It is metadata required to compare claims fairly.

## 4. Lateral-thinking protocol

SOMA may deliberately generate alternative hypotheses, but must keep hypothesis generation separate from evidence assessment.

For an unusual claim:

`Claim -> Decompose -> Identify assumptions -> Generate competing hypotheses -> Retrieve evidence -> Test discriminators -> Compare explanations -> Assess safety -> Preserve uncertainty`

Lateral thinking is therefore an exploration mechanism, not an evidence shortcut.

Examples of productive transformations:

- "Is this paradigm scientific?" -> "What exact claim, intervention, population, endpoint, and evidence are being evaluated?"
- "This worked for one person" -> "What observations changed, what alternatives exist, and what would discriminate among explanations?"
- "Mechanism is unknown" -> "What is observed independently of mechanism, and what evidence would test the proposed mechanism?"
- "Mainstream medicine rejects it" -> "What specific evidence, methodological objection, safety concern, or replication failure explains the disagreement?"

## 5. Unconventional-claim evidence passport

A reusable Evidence Passport should expose:

1. Claim
2. Claim source
3. Knowledge system
4. Claim type
5. Population/context
6. Intervention/exposure
7. Comparator
8. Outcome
9. Evidence design
10. Observed signal
11. Causal confidence
12. Bias/confounding
13. Replication
14. Contradictory evidence
15. Applicability
16. Safety
17. Mechanism status
18. Uncertainty
19. Epistemic status
20. Research priority

The passport must never collapse these fields into a single "scientific/unscientific" label.

## 6. Psychological/behavioral AI boundary

AI may assist with:

- structured journaling
- reflection prompts
- behavioral observation
- evidence retrieval
- hypothesis generation
- pattern summarization
- progress visualization
- user-directed experiments

AI must not, by default:

- diagnose mental disorders
- claim therapeutic efficacy from a single session
- infer hidden trauma or causes as facts
- make autonomous clinical decisions
- prescribe or modify treatment
- convert inferred psychological labels into canonical health facts

High-risk outputs must follow the existing SOMA policy, authorization, human-review, provenance, and audit contracts.

## 7. Relationship to existing SOMA architecture

This document does not replace the Health State Model, Health Evidence Graph, Policy Kernel, Identity Authorization Enforcement, Governed Capability Operation, or Capability Registry.

It is a semantic extension/projection that should consume those canonical contracts.

The target governed path remains:

`Identity -> Capability -> Policy -> Gateway -> Execution -> Evidence -> Interpretation -> Audit`

with epistemic metadata carried by evidence and interpretation records.

## 8. Lateral-thinking safety rule

SOMA should be open to hypotheses and strict about conclusions.

The system should be able to say:

- supported
- partially supported
- preliminary
- contested
- insufficient evidence
- contradicted
- unknown

without converting uncertainty into either endorsement or dismissal.

## 9. Implementation boundary

This document is an architecture baseline. It does not authorize implementation of a full behavioral-health product, autonomous therapy agent, universal causal engine, or broad alternative-medicine subsystem.

The first implementation artifact should be a small machine-readable contract for evidence-passport metadata and a contract test proving that behavioral observations, evidence claims, interpretations, and recommendations remain distinct.
