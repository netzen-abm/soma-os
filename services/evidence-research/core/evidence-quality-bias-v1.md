# SOMA Evidence Quality + Bias Assessment v1

## Purpose

Provide a bounded, reviewable assessment boundary for methodological quality and risk-of-bias considerations after screening, structured extraction, and directness assessment.

This boundary preserves the underlying dimensions rather than collapsing them into an opaque score.

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
Future Consistency / Synthesis
  ↓
Safety Gate
```

## Assessment dimensions

Each dimension uses an explicit judgment:

- `FAVORABLE`
- `SOME_CONCERNS`
- `SERIOUS_CONCERNS`
- `NOT_ASSESSED`

The contract records these judgments independently:

- study design;
- risk of bias;
- sample size / precision;
- comparator quality;
- outcome validity;
- follow-up duration;
- attrition;
- selective reporting;
- confounding;
- external validity.

`study_design` is retained as the source-grounded study-design description and is not itself converted into a quality score.

## Boundary rules

1. Only an `INCLUDED` screened candidate may be assessed.
2. Directness must already have been assessed; this boundary does not assign or change directness.
3. The original `EvidenceCandidate` remains immutable.
4. Missing or unavailable information remains `NOT_ASSESSED`; it is never silently treated as favorable.
5. Every assessment requires a non-empty rationale, assessment method, and timestamp.
6. An assessor may be recorded but is not fabricated.
7. No single overall numeric score is produced.
8. No automatic evidence-strength, certainty, causal, safety, or recommendation conclusion is produced.
9. Quality/bias is independent from directness: a directly relevant study can have serious concerns, and an indirect study can be methodologically favorable.
10. The boundary does not resolve contradictions between studies; that belongs to later synthesis.

## Relationship to extraction

The extraction layer may contain source-reported `risk_of_bias` text. This assessment layer is distinct: it records an explicit structured methodological judgment and its rationale. It must not silently convert extracted prose into a structured judgment.

## AI boundary

AI may assist with identifying candidate methodological concerns only when the resulting assessment is explicitly reviewable and retains its rationale and method. AI output is not authoritative clinical judgment and cannot silently assign a favorable assessment.

## Non-goals

This contract does not implement:

- GRADE or another external certainty framework;
- meta-analysis or statistical recalculation;
- evidence synthesis;
- consistency assessment across studies;
- causal inference;
- clinical significance;
- safety gating;
- recommendations;
- diagnosis or treatment decisions;
- a second evidence store or graph;
- a second authorization or policy engine;
- provider-specific logic.

The purpose is a transparent assessment boundary that later evidence synthesis can consume.
