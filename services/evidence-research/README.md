# SOMA Evidence Research Service

Shared infrastructure boundary for evidence discovery and assessment.

## Purpose

This service will transform a source-derived disease-management protocol into a traceable research evidence package.

It is **not** a clinical decision engine.

## Contract

```text
Protocol
  -> decompose
  -> normalize
  -> generate research questions
  -> search evidence adapters
  -> screen
  -> extract structured studies
  -> classify directness
  -> assess quality/consistency
  -> search safety and contradictions
  -> produce evidence package
```

## Required output

The service must always return:

- protocol/source provenance;
- search context;
- evidence found or an explicit no-relevant-evidence declaration;
- supporting and contradictory evidence where identified;
- directness for every protocol-study relationship;
- structured study metadata and summary;
- safety findings;
- original verification links;
- professional-review requirement.

## Privacy boundary

The service accepts protocol/source research inputs. It must not require personal health information for evidence discovery. User-specific health data is outside this service boundary.

## Initial implementation boundary

The first implementation is deliberately provider-neutral. External research providers are adapters behind a shared interface so Web, Android, iOS, AI/RAG and research surfaces do not depend directly on one provider.

```text
Client surfaces
      |
      v
Evidence Research API
      |
      +-- Protocol Decomposer
      +-- Normalizer
      +-- Search Orchestrator
      +-- Provider Adapters
      +-- Screening/Extraction
      +-- Evidence Assessment
      +-- Safety Adapter
      +-- Verification Link Builder
      |
      v
Evidence Registry
```

## Non-goals

This service must not:

- diagnose;
- prescribe;
- claim a cure;
- replace professional care;
- tell users to stop medication;
- invent evidence or citations;
- upgrade indirect evidence into exact-protocol evidence.
