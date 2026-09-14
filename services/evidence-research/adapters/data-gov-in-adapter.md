# SOMA Data.gov.in Adapter

## Status

Bounded India-focused public-dataset retrieval adapter under the shared External Knowledge Source Contract.

## Role

Data.gov.in is treated as a catalog and access layer for heterogeneous Indian government datasets. It is not a canonical SOMA evidence authority. The adapter preserves dataset/resource identity and provenance; the SOMA research/intelligence layer remains responsible for relevance, interpretation, quality assessment, and any downstream evidence use.

## Access boundary

The adapter accepts a SOMA-normalized resource identifier and explicit filters. It does not discover arbitrary resources on behalf of product clients and does not construct queries from names, contact information, user identifiers, medical records, or other personal health information.

API credentials, when required by a resource, are supplied through runtime environment configuration and are never stored in source, registry files, fixtures, or documentation.

## Provider-neutral result

```yaml
provider_id: data_gov_in
provider_record_id: Data.gov.in resource identifier
source_url: Data.gov.in resource URL
verification_url: Data.gov.in resource URL
data:
  records: []
  count: 0
```

The provider resource identifier is preserved. Provider metadata is not promoted to evidence quality, clinical significance, or causal meaning.

## External source contract binding

Each successful resource retrieval carries the shared `ExternalKnowledgeSource` envelope with provider identity, access mode, governance references, resource identifier, source and verification URLs, retrieval time, declared entity/identifier systems, verification status, `NOT_ASSESSED` quality status, and explicit unknown semantics where the provider response does not establish a value.

## Failure semantics

Missing credentials, network/provider errors, malformed responses, and other provider failures raise `DataGovProviderError`. A successful response containing zero records remains a successful empty result. The core layer must not translate provider failure into `NO_RELEVANT_EVIDENCE_FOUND`.

## Privacy and trust

No personal health information is sent to Data.gov.in by this adapter. Provider responses are untrusted source material and cannot modify SOMA authorization, policy, configuration, or clinical conclusions.

## Non-goals

This adapter does not:

- create a second data warehouse or cache;
- create a universal government-data ontology;
- write directly to the Evidence Graph;
- assess dataset quality or clinical significance;
- infer causality;
- generate recommendations or safety decisions;
- bypass the canonical authorization/policy boundary;
- make Data.gov.in part of the biomedical literature route.

## CI strategy

Contract tests use deterministic fixtures/mocks for provider responses. CI must not depend on a live Data.gov.in service or a real API key.
