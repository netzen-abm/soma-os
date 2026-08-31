# SOMA Research Provider Adapter Contract

## Purpose

Defines the provider-neutral boundary for research retrieval. The core Evidence Research Engine must not depend on a specific external research provider.

## Adapter interface

```text
ResearchProviderAdapter
  provider_id() -> ProviderId
  search(query: NormalizedResearchQuery) -> List<SearchResult>
  fetch(record_id: ProviderRecordId) -> RawStudyRecord
  verification_url(record_id: ProviderRecordId) -> URL
```

## NormalizedResearchQuery

```yaml
query_id: string
terms: []
indication: string|null
components: []
outcomes: []
study_types: []
publication_start: date|null
publication_end: date|null
language: string|null
```

## SearchResult

```yaml
provider_id: string
provider_record_id: string
title: string
publication_year: integer|null
study_type_hint: string|null
abstract_or_summary: string|null
source_url: string
verification_url: string
```

## Adapter rules

1. Return source-derived metadata only.
2. Never generate or infer a study that the provider did not return.
3. Preserve provider identifiers.
4. Preserve the original verification URL.
5. Do not assign evidence quality or clinical significance; those belong to the core engine.
6. Do not access or require user personal health information.
7. Report provider errors explicitly rather than returning fabricated results.

## Core-engine responsibility

After adapter retrieval, the core engine performs:

```text
Deduplication
  ↓
Relevance screening
  ↓
Protocol matching
  ↓
Structured extraction
  ↓
Directness assessment
  ↓
Quality / bias assessment
  ↓
Supporting / contradictory classification
  ↓
Evidence synthesis
  ↓
Safety gate
```

## Verification requirement

Every included study must retain at least one user-verifiable primary reference where available:

- PubMed;
- DOI;
- publisher;
- government repository;
- primary repository.

If a provider supplies only secondary metadata, the record must remain labelled accordingly and must not be represented as a primary-source verification.

## Failure behavior

Provider unavailable:

```text
search_status = PROVIDER_UNAVAILABLE
```

The engine may retry or use another configured provider. It must not convert provider failure into `NO_RELEVANT_EVIDENCE_FOUND`.

No results after successful search:

```text
search_status = COMPLETED
results = []
evidence_status = INSUFFICIENT
reason = NO_RELEVANT_EVIDENCE_FOUND
```

These states must remain distinct.

## Privacy

Provider adapters must not send names, contact information, user identifiers, medical records, or other personal/sensitive personal data. Queries are constructed from the source-derived protocol and research question only.

## Free-tier / cost boundary

The initial adapter layer should prioritize publicly accessible, no-cost research sources and APIs. Paid providers must not become a dependency for the initial SOMA evidence capability.
