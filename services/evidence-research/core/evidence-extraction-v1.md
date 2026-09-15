# SOMA Structured Evidence Extraction Contract v1

## Status

Implemented as the bounded extraction boundary immediately after explicit evidence screening.

## Purpose

Convert an `INCLUDED` `ScreenedEvidenceCandidate` plus explicitly supplied source-grounded fields into a structured evidence record.

This is a data-structuring boundary, not an evidence-interpretation engine.

## Canonical position

```text
Evidence Research Core
        ↓
EvidenceCandidate
        ↓
Evidence Screening
        ↓
ScreenedEvidenceCandidate (INCLUDED)
        ↓
Structured Evidence Extraction  ← v1
        ↓
Directness Assessment
        ↓
Quality / Bias
        ↓
Evidence Synthesis
```

The canonical Evidence Research Engine already defines the study fields expected at this stage: study identity, authorship, year, journal, study type, registration, DOI/PubMed identifiers, population, sample size, intervention, comparator, duration, outcomes, effect estimates, findings, limitations, funding, conflicts and safety findings.

## Extraction contract

`EvidenceExtraction` contains explicitly supplied values for those fields. Missing values remain `None` or an empty tuple. The boundary never fabricates missing information.

`StructuredEvidenceRecord` retains the original immutable `EvidenceCandidate` alongside the extraction.

## Source-grounding rule

The extraction boundary accepts structured values through an explicit mapping. It performs shape and type validation only.

It does not:

- infer facts from an abstract;
- invent study details;
- infer causality;
- calculate an evidence grade;
- assign directness;
- assess risk of bias;
- determine clinical significance;
- synthesize findings.

An AI-assisted extraction workflow may be introduced later, but it must produce explicitly governed source-grounded fields and preserve source references. Model-generated interpretation must remain distinct from reported study facts.

## Screening gate

Only candidates with `screening_status == INCLUDED` may enter structured extraction.

`EXCLUDED`, `UNCERTAIN`, and `UNSCREENED` candidates remain outside the extraction boundary until an appropriate downstream workflow explicitly changes their state.

## Validation

The v1 boundary rejects:

- unknown extraction fields;
- invalid scalar types;
- negative sample sizes;
- malformed text collections;
- empty values inside declared collections;
- extraction attempts for non-included candidates.

Validation does not imply that a supplied value is scientifically correct; source verification remains necessary.

## Provenance

The original candidate is retained without mutation, including provider identity, provider record identity, source URL, verification URL, identifiers, source class, geography, publication year and source-derived summary.

The extraction layer therefore does not become a second source of truth.

## Privacy

The extraction boundary consumes normalized research output. It does not send personal health information to external research providers.

## Explicit non-goals

- no new database or evidence store;
- no Evidence Graph persistence;
- no graph engine;
- no second authorization or policy engine;
- no provider-specific extraction branches;
- no autonomous AI authority;
- no evidence-quality score;
- no risk-of-bias algorithm;
- no causal inference;
- no evidence synthesis;
- no recommendations or treatment decisions.

## Verification

Focused tests verify candidate preservation, explicit fact retention, missing-value preservation, unknown-field rejection and enforcement of the `INCLUDED` screening gate.
