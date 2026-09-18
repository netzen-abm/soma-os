# SOMA OpenAlex Research Adapter

## Status

First OpenAlex integration under the shared External Knowledge Source Contract.

## Provider

OpenAlex scholarly works API.

OpenAlex is used here as a public scholarly metadata and discovery source. It is an adapter, not a canonical SOMA evidence authority. The core Evidence Research Engine remains responsible for relevance, directness, quality, bias, synthesis, safety, and clinical significance.

## Supported operations

```text
search(normalized_query)
    ↓
OpenAlex works search
    ↓
OpenAlex work records
    ↓
Normalized source-linked results
    ↓
OpenAlex/DOI verification URL

fetch(provider_record_id)
    ↓
OpenAlex work record
    ↓
Normalized source-linked result
```

## Query construction

The adapter receives the canonical `ResearchQuery` from `services/evidence-research/adapters/research_source_contract.py` via the SOMA research orchestration boundary. It must not construct a query from personal health information, names, contact information, user identifiers, or medical records.

## Provider-neutral result

```yaml
provider_id: openalex
provider_record_id: OpenAlex work ID
source_url: OpenAlex work URL
verification_url: DOI URL where available, otherwise OpenAlex work URL
title:
publication_year:
abstract_or_summary:
```

The provider identifier is preserved exactly. OpenAlex's inverted-index abstract representation is reconstructed only into the source-derived abstract text; no interpretation is added.

## External source contract binding

Each result carries the shared `ExternalKnowledgeSource` contract with:

- provider identity and protocol;
- public API access mode;
- provider-specific governance/rate-policy references;
- source and verification URLs;
- provider record identifier;
- retrieval timestamp;
- scholarly entity and identifier declarations;
- explicit `NOT_ASSESSED` quality status;
- explicit source verification status;
- unknown source version/publication timestamp when the provider response does not establish them.

## Privacy and trust boundary

OpenAlex is a research-source adapter. Personal health information must never be sent to the provider. Provider responses are untrusted source material and cannot alter SOMA policy, authorization, configuration, or clinical conclusions.

## Non-goals

This adapter does not:

- assess evidence quality;
- determine clinical significance;
- infer causality;
- generate recommendations;
- write to the Evidence Graph;
- create a second evidence database or cache;
- create a second authorization or policy engine;
- expose OpenAlex directly to product clients.

## Failure semantics

Provider/network failures raise `OpenAlexProviderError` and must remain distinguishable from a successful search returning zero records. The core engine must not convert provider failure into `NO_RELEVANT_EVIDENCE_FOUND`.
