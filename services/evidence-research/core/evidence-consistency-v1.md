# SOMA Evidence Consistency Assessment v1

## Purpose

Provide a bounded, reviewable cross-study consistency assessment after screening, structured extraction, directness assessment, and quality/bias assessment.

Consistency is an assessment of whether included evidence points in materially compatible directions for the same research question. It does not establish causality, clinical significance, safety, or recommendation status.

## Position in the research pipeline

```text
Evidence Search
  ↓
Screening
  ↓
Structured Extraction
  ↓
Directness Assessment
  ↓
Quality + Bias Assessment
  ↓
Consistency Assessment
  ↓
Future Evidence Synthesis
  ↓
Safety Gate
```

## Canonical judgments

The boundary records one explicit cross-study judgment:

- `CONSISTENT`
- `MIXED`
- `INCONSISTENT`
- `NOT_ASSESSED`

`MIXED` is the explicit state for materially conflicting or divergent findings. It must not be silently collapsed into a positive or negative conclusion.

## Boundary rules

1. At least two distinct evidence records are required for a cross-study consistency assessment.
2. Every supplied evidence record must have an `INCLUDED` screening status.
3. Every supplied evidence record must already have a directness assessment and quality/bias assessment.
4. Evidence records are identified by their provider and provider-record identifiers; identifiers must be non-empty and unique within the assessment.
5. The consistency judgment is explicit assessment input. This boundary does not infer consistency from titles, abstracts, effect estimates, or model output.
6. `MIXED` and `INCONSISTENT` findings remain represented as such; contradiction is not resolved here.
7. `NOT_ASSESSED` is valid when the available evidence is insufficient to assess consistency and must not be treated as consistent.
8. A non-empty rationale, assessment method, and timestamp are required.
9. The original evidence records and their prior assessments remain immutable.
10. No overall evidence-strength, certainty, causal, safety, or recommendation conclusion is produced.

## Relationship to study-level assessments

Consistency is a cross-study property and is therefore distinct from:

- directness of an individual study;
- methodological quality or bias of an individual study;
- source-reported risk-of-bias text;
- safety findings;
- causal inference.

A set of high-quality studies may still be inconsistent. A set of consistent studies may still be low quality or indirect.

## AI boundary

AI may assist a qualified reviewer by organizing findings for consistency review, but the canonical consistency judgment must remain explicit, reviewable, and traceable to the included evidence records. AI output does not become authoritative merely by being generated.

## Non-goals

This contract does not implement:

- automatic contradiction detection as a clinical conclusion;
- statistical heterogeneity or meta-analysis;
- effect-size recalculation;
- evidence synthesis or certainty grading;
- causal inference;
- safety gating;
- recommendations or treatment decisions;
- a second evidence store or graph;
- a second authorization or policy engine;
- provider-specific logic.

The purpose is to create the smallest canonical cross-study assessment boundary that later synthesis can consume.
