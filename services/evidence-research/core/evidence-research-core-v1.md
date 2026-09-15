# SOMA Evidence Research Core v1

## Status

Implemented as a bounded consumption boundary over the existing multi-source orchestration runtime.

## Purpose

Convert one `OrchestrationResult` into a canonical, assessment-ready research package without making evidence judgments.

The core is shared SOMA infrastructure. Client surfaces, providers, and AI components must not create private evidence-result models.

## Boundary

```text
Governed research query
        ↓
Multi-Source Orchestration Runtime
        ↓
OrchestrationResult
        ↓
Evidence Research Core
        ↓
Assessment-ready EvidenceCandidate records
        ↓
Future screening / extraction / assessment contracts
```

The core preserves provider identity, provider record identity, source and verification links, stable identifiers, source class, geography, publication year, and retrieved source text when supplied.

## Assessment state

Every candidate starts as:

`screening_status: UNSCREENED`

Assessment fields remain unset until an explicit downstream contract supplies them:

- relevance;
- protocol match;
- population match;
- outcome match;
- safety relevance;
- exclusion reason;
- directness.

The core never upgrades directness, relevance, quality, safety, or efficacy.

## Failure semantics

The orchestration status is preserved exactly. `PROVIDER_UNAVAILABLE`, partial provider failure, successful empty retrieval, and successful results are not reclassified by the core.

## Provenance and verification

Candidates must preserve non-empty source and verification URLs. Provider-specific source-contract metadata remains owned by the upstream adapter/orchestration boundary; this core does not reinterpret source governance or quality metadata.

## Privacy

The core accepts normalized research output only. Personal health information must not be introduced into provider-facing research queries.

## Explicit non-goals

- no evidence-quality scoring;
- no risk-of-bias assessment;
- no causal inference;
- no evidence synthesis;
- no clinical significance determination;
- no recommendations or treatment decisions;
- no AI authority;
- no Evidence Graph persistence;
- no second evidence store;
- no new authorization or policy engine;
- no provider-specific branching;
- no universal graph runtime.

## Design rationale

This is deliberately smaller than the full Evidence Research Engine architecture. The architecture already specifies screening, structured extraction, directness, quality/bias, synthesis, safety discovery, reproducibility and human review. v1 establishes the executable convergence boundary first so those later capabilities can consume one canonical package rather than each inventing a competing result model.

## Verification

Focused tests verify:

- identity and verification provenance preservation;
- explicit preservation of provider failure state;
- rejection of missing verification links;
- query identifier validation and normalization.
