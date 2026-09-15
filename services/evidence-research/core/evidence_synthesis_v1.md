# SOMA Evidence Synthesis v1

## Purpose

Provide the smallest explicit, reviewable synthesis boundary after screening, structured extraction, directness assessment, quality/bias assessment, and consistency assessment.

Synthesis combines already-assessed evidence into a traceable conclusion. It does not silently recalculate study quality, directness, consistency, certainty, causality, safety, or clinical significance.

## Canonical flow

```text
Screening
  ↓
Structured Extraction
  ↓
Directness
  ↓
Quality + Bias
  ↓
Consistency
  ↓
Evidence Synthesis  ← v1
  ↓
Safety Gate
```

## Synthesis status

The v1 boundary records an explicit reviewer/rule-supplied synthesis status:

- `SUPPORTIVE`
- `NULL`
- `MIXED`
- `CONTRADICTORY`
- `INSUFFICIENT`
- `NOT_ASSESSED`

These statuses describe the synthesized evidence pattern. They are not equivalent to evidence-quality grades or clinical recommendations.

## Boundary rules

1. Only `INCLUDED` evidence may enter synthesis.
2. Each evidence record must already have quality/bias assessment.
3. A synthesis assessment requires at least one evidence record.
4. Evidence identity is preserved through provider and provider-record identifiers.
5. Supporting, null, contradictory, indirect, and unresolved evidence remain separately traceable.
6. `MIXED` and `CONTRADICTORY` findings are preserved rather than resolved by majority vote or source count.
7. Missing information remains explicit; it is never silently treated as supportive.
8. Every synthesis assessment requires rationale, method, and timestamp.
9. The original evidence records and prior assessments remain immutable.
10. Synthesis does not produce a causal claim, safety clearance, treatment recommendation, diagnosis, or prescription.
11. Synthesis does not calculate meta-analytic effect estimates, heterogeneity, or an opaque confidence score.
12. A synthesis status must not be presented as proof that a protocol works or is safe.

## AI boundary

AI may organize already-assessed evidence or draft a reviewable synthesis rationale. The canonical synthesis status remains explicit and traceable to the supplied evidence records. AI output cannot silently become authoritative clinical judgment.

## No-evidence state

`INSUFFICIENT` is valid when the assessed evidence set does not establish a sufficiently informative synthesis. `NO_RELEVANT_EVIDENCE_FOUND` remains the canonical search outcome and is not redefined here.

## Non-goals

- automatic clinical conclusion;
- GRADE or another certainty framework;
- meta-analysis;
- effect-size recalculation;
- causal inference;
- safety gating;
- recommendation or treatment logic;
- diagnosis;
- second evidence store or graph;
- second authorization or policy engine;
- provider-specific synthesis logic.
