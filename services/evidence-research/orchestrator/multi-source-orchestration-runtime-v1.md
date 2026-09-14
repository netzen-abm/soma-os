# SOMA Multi-Source Orchestration Runtime v1

## Status

Implemented as a bounded runtime over the existing research adapter boundary.

## Purpose

Provide one shared SOMA execution boundary that routes normalized research queries to already-governed provider adapters, normalizes their retrieval results into one orchestration envelope, preserves provenance, deduplicates underlying studies, and exposes provider failure explicitly.

This runtime is shared infrastructure. Client surfaces must not implement private provider-routing logic.

## Existing contracts reused

- `services/evidence-research/adapters/provider-adapter-contract.md`
- `services/evidence-research/adapters/research-source-registry.yaml`
- `docs/architecture/evidence-research-engine.md`
- `docs/architecture/external-knowledge-source-contract-v1.md`

The runtime does not replace any of these contracts.

## Runtime flow

```text
Governed research query
        ↓
SearchRoute
        ↓
Existing provider adapters
        ↓
Provider result normalization
        ↓
Stable study deduplication
        ↓
OrchestrationResult
        ↓
Core Evidence Research Engine
```

## Query boundary

`ResearchQuery` carries only source-derived research terms and routing context:

- `query_id`
- `terms`
- `source_class`
- `geography`
- `retmax`

It must not contain personal health information.

The `retmax` field is intentionally compatible with the existing PubMed and OpenAlex normalized query shapes, allowing the orchestration layer to pass a structurally compatible query without introducing provider-specific routing code.

## Route boundary

`SearchRoute` declares:

- source class;
- ordered provider IDs;
- whether the route is required.

Provider IDs are resolved against the caller-supplied adapter set. The runtime does not create or discover providers implicitly.

The source registry remains the authoritative routing configuration; this v1 runtime intentionally accepts resolved routes rather than adding a second registry parser or configuration engine.

## Result normalization

Existing adapter result objects are converted into `ProviderResult` while preserving:

- provider ID;
- provider record ID;
- title;
- source class;
- geography;
- source URL;
- verification URL;
- publication year when supplied;
- abstract/summary when supplied;
- stable identifiers.

Provider-specific source-contract metadata remains owned by the adapter result and is not reinterpreted by this runtime.

## Deduplication

The runtime deduplicates underlying studies rather than counting provider copies as independent evidence.

Preferred identifiers:

1. DOI;
2. PMID;
3. NCT;
4. CTRI;
5. ISRCTN.

When none is available, normalized title plus publication year is used as a conservative fallback.

Contradictory records with different stable identifiers remain separate.

## Failure semantics

The runtime preserves the distinction between successful empty searches and provider failures.

Possible provider states include:

- `COMPLETED_WITH_RESULTS`
- `COMPLETED_NO_RESULTS`
- `PROVIDER_UNAVAILABLE`

Overall states include:

- `COMPLETED_WITH_RESULTS`
- `COMPLETED_NO_RELEVANT_RESULTS`
- `PARTIAL_PROVIDER_FAILURE`
- `PROVIDER_UNAVAILABLE`

If every provider on every required route is unavailable, the overall state is `PROVIDER_UNAVAILABLE`.

If at least one required route is incomplete but another required provider completes, the overall state is `PARTIAL_PROVIDER_FAILURE`.

A successful empty search is never converted into provider failure, and provider failure is never converted into no evidence.

## Epistemic boundary

This runtime does **not**:

- assess evidence quality;
- determine efficacy or safety;
- infer causality;
- classify clinical significance;
- synthesize recommendations;
- make autonomous clinical decisions;
- write to the Evidence Graph;
- create a second evidence store;
- create a generic graph engine;
- create a second authorization or policy engine.

Those responsibilities remain with the appropriate canonical SOMA layers.

## Privacy boundary

The runtime operates on normalized research questions. It must not require or transmit names, identifiers, medical records, or other personal/sensitive health information to research providers.

## Verification

Contract tests cover:

- cross-provider study deduplication;
- India/global multi-route composition;
- total provider failure;
- partial provider failure;
- successful empty searches;
- preservation of contradictory records;
- provider provenance and verification links;
- missing required providers;
- required routes without matching queries.

## Explicit non-goals for v1

This implementation does not add new external providers. Existing providers remain independently governed adapters. It does not parse the source registry into a new routing service, because doing so would duplicate configuration authority.

The next layer after v1 should be selected only after verifying that the core Evidence Research Engine can consume this orchestration envelope without creating a competing result model.
