# SOMA Evidence Screening Contract v1

## Status

Implemented as a bounded, explicit screening boundary over the canonical `EvidenceCandidate` produced by Evidence Research Core v1.

## Purpose

Record an auditable screening decision for one evidence candidate without changing the source record and without making evidence-quality, causal, clinical, or recommendation judgments.

The contract operationalizes the existing Evidence Research Engine screening dimensions rather than creating a competing evidence model.

## Canonical position

```text
Governed Provider Adapters
        ↓
Multi-Source Orchestration
        ↓
Evidence Research Core
        ↓
EvidenceCandidate (UNSCREENED)
        ↓
Evidence Screening Boundary
        ↓
ScreenedEvidenceCandidate
        ↓
Structured Extraction
        ↓
Directness Assessment
        ↓
Quality / Bias
        ↓
Evidence Synthesis
```

The architecture already defines screening as the step after deduplication and before structured extraction. The executable boundary now makes that transition explicit.

## Screening decision

```text
EvidenceScreeningDecision
├── screening_status
├── relevance
├── protocol_match
├── population_match
├── outcome_match
├── safety_relevance
├── duplicate_of
├── exclusion_reason
└── decision_method
```

### Screening status

Allowed values:

- `UNSCREENED`
- `INCLUDED`
- `EXCLUDED`
- `UNCERTAIN`

`UNSCREENED` is the initial candidate state. A screening decision should normally transition an assessment-ready candidate to `INCLUDED`, `EXCLUDED`, or `UNCERTAIN`.

### Screening dimensions

For each dimension below, the contract accepts only:

- `YES`
- `NO`
- `UNCERTAIN`
- `NOT_ASSESSED`

Dimensions:

- `relevance`
- `protocol_match`
- `population_match`
- `outcome_match`
- `safety_relevance`

These values record an explicit assessment. They are not computed by this boundary.

## Exclusion and duplicate semantics

An `EXCLUDED` decision requires a non-empty `exclusion_reason`.

If a candidate is identified as a duplicate, `duplicate_of` must contain a non-empty identifier for the canonical retained record. A candidate cannot identify itself as its own duplicate.

The screening boundary does not decide which record is canonical; it records the supplied duplicate relationship.

## Immutability and provenance

`EvidenceCandidate` remains immutable and is retained intact inside `ScreenedEvidenceCandidate`.

The screening boundary does not rewrite:

- provider identity;
- provider record identity;
- source URL;
- verification URL;
- identifiers;
- source class;
- geography;
- publication year;
- source-derived abstract/summary.

This preserves the distinction between source-derived material and downstream assessment.

## Decision method

`decision_method` identifies how the explicit decision was supplied. v1 defaults to `EXPLICIT_ASSESSMENT`.

This field does not imply that a decision is human-reviewed, AI-generated, or clinically validated. A future governed workflow may define permitted methods and reviewer/provenance metadata without changing the candidate model.

## AI boundary

This contract does not perform semantic screening, AI classification, evidence grading, or clinical interpretation.

AI may later assist with screening only through an explicitly governed workflow that records the resulting assessment and retains the underlying source candidate. AI must not silently convert uncertainty into inclusion or exclusion.

## Relationship to later assessment

Screening is not evidence quality assessment.

This boundary does **not** determine:

- directness;
- risk of bias;
- evidence quality;
- consistency;
- causality;
- efficacy;
- clinical significance;
- safety clearance;
- recommendations.

Those remain downstream contracts in the canonical Evidence Research Engine flow.

## Failure and safety principles

Provider failure remains upstream provider/orchestration state. Screening must never convert provider failure into exclusion or no-evidence.

An evidence candidate marked relevant or included does not bypass the later safety gate.

## Privacy

Screening consumes normalized evidence candidates. No personal health information is required or sent to external research providers by this contract.

## Explicit non-goals

- no new database or evidence store;
- no Evidence Graph persistence;
- no graph engine;
- no second authorization or policy engine;
- no provider-specific screening branches;
- no automated clinical judgment;
- no evidence-quality score;
- no risk-of-bias algorithm;
- no causal inference;
- no recommendation engine;
- no synthesis;
- no second candidate/result model.

## Verification

Focused contract tests verify:

- source candidate preservation;
- explicit decision binding;
- allowed screening dimensions;
- exclusion-reason enforcement;
- duplicate self-reference rejection;
- prevention of applying the boundary to an already screened candidate.
