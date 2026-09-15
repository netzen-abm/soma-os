# SOMA Evidence Safety Gate v1

**Status:** Implemented contract
**Scope:** Bounded safety assessment after evidence synthesis.

## Purpose

The Safety Gate is a reusable SOMA evidence boundary that records an explicit safety assessment before a downstream action or experiment pathway can consume the assessed evidence.

It is a safety boundary, not a treatment, efficacy, causality, recommendation, or regulatory-reporting engine.

## Pipeline position

```text
Screening
  ↓
Structured Extraction
  ↓
Directness
  ↓
Quality / Bias
  ↓
Consistency
  ↓
Evidence Synthesis
  ↓
Safety Gate ← v1
  ↓
Future bounded action / experiment
```

## Canonical safety statuses

- `NO_IDENTIFIED_SAFETY_CONCERN`
- `SAFETY_CONCERN_IDENTIFIED`
- `INSUFFICIENT_SAFETY_INFORMATION`
- `REQUIRES_HUMAN_REVIEW`
- `NOT_ASSESSED`

These statuses describe the safety assessment only. They do not imply efficacy, appropriateness, indication, or causality.

## Assessment fields

The contract preserves:

- canonical evidence identifiers;
- safety findings;
- contraindication concerns;
- interaction concerns;
- exposure-risk concerns;
- unresolved questions;
- rationale;
- assessment method;
- assessment timestamp;
- optional assessor identity.

An explicit assessment is required. The boundary does not calculate a safety judgment automatically.

## Hard boundaries

The v1 gate does not:

- infer causality;
- determine treatment effectiveness;
- diagnose disease;
- recommend, prescribe, start, stop, or change treatment;
- perform meta-analysis;
- create a separate safety database;
- create a second authorization or policy engine;
- submit regulatory reports;
- convert temporal proximity into causality;
- silently resolve contradictory evidence.

Causality remains a separate future status/method contract as required by the health-safety architecture.

## Evidence identity

Safety assessments must reference the exact synthesized evidence records they assess. Provider and provider-record identifiers are preserved as the canonical evidence identity used by the preceding evidence pipeline.

## Governance

The gate remains within SOMA shared infrastructure. Surface-specific implementations must consume the shared contract rather than create competing safety logic. Authorization, privacy, provenance, and audit remain governed by existing shared infrastructure.

## Safety principle

> **Record broadly. Preserve the observation. Separate association from causation. Assess safety independently. Escalate according to evidence and authority.**
