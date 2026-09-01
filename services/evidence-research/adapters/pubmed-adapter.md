# SOMA PubMed Research Adapter

## Status

First free/public research-provider adapter specification.

## Provider

NCBI PubMed via Entrez E-utilities.

NCBI provides public APIs for Entrez databases, including PubMed and PMC. PubMed E-utilities support programmatic search and retrieval of PubMed records. The adapter must follow NCBI's current usage policies and identify the calling application with registered `tool` and `email` values. citeturn0search1turn0search3

## Why PubMed first

- public research infrastructure;
- broad biomedical literature coverage;
- programmatic E-utilities interface;
- stable PubMed identifiers;
- user-verifiable PubMed pages;
- no paid provider dependency for the first implementation.

## Supported operations

```text
search(normalized_query)
    ↓
ESearch(db=pubmed)
    ↓
PMID list
    ↓
ESummary / EFetch
    ↓
Normalized study records
    ↓
PubMed verification URL
```

NCBI documents ESearch for returning matching UIDs and ESummary/EFetch for retrieving records. citeturn0search1turn0search2

## Query construction

The adapter receives a normalized query from the SOMA core engine. It must not construct a query from personal user information.

Example conceptual query:

```text
(millet OR millets OR "finger millet")
AND
(diabetes OR glycemic OR glucose)
```

The core engine should create separate searches for:

1. exact protocol;
2. protocol components;
3. outcomes;
4. safety/interactions;
5. contradictory/null findings.

The adapter only transports the normalized query to PubMed.

## Provider-neutral result

```yaml
provider_id: pubmed
provider_record_id: PMID
source_url: https://pubmed.ncbi.nlm.nih.gov/{PMID}/
verification_url: https://pubmed.ncbi.nlm.nih.gov/{PMID}/
title:
publication_year:
study_type_hint:
abstract_or_summary:
```

The adapter must preserve the PMID as the provider record identifier.

## Retrieval policy

The adapter should use ESearch for discovery and ESummary/EFetch for metadata/full structured records as appropriate. E-utilities return structured XML/JSON forms and support batch retrieval; the core service should batch records where practical rather than making unnecessary individual requests. citeturn0search1turn0search12

## Rate limiting

NCBI's current documentation states that applications should identify themselves using `tool` and `email`. Without an API key, more than three requests per second from a single IP can trigger rate limiting; an API key raises the default supported rate to ten requests per second. citeturn0search1turn0search12

SOMA should therefore:

- throttle requests;
- cache provider responses where permitted;
- batch retrievals;
- never expose an API key to clients;
- treat provider rate limiting as `PROVIDER_UNAVAILABLE` or retryable provider failure, not as absence of evidence.

## Required configuration

```text
PUBMED_TOOL_NAME
PUBMED_CONTACT_EMAIL
PUBMED_API_KEY (optional)
```

The API key is optional for the initial low-volume implementation. If configured, it must be stored server-side and never committed to Git.

## Verification

Every included PubMed study receives:

```text
https://pubmed.ncbi.nlm.nih.gov/{PMID}/
```

This is the user-facing verification path.

## Copyright boundary

PubMed metadata and abstracts may be subject to copyright. NCBI explicitly notes that abstracts can contain protected material and that users of E-utilities must comply with NCBI disclaimer/copyright requirements. SOMA should therefore store structured metadata and concise evidence summaries rather than republishing entire copyrighted articles or abstracts without appropriate rights. citeturn0search12

For full-text research retrieval, use PMC or another source only through permitted APIs and respect article-specific licensing. PMC states that not all articles are available for text mining/reuse and that automated PMC retrieval should use its approved services. citeturn0search6

## Failure states

```text
PROVIDER_UNAVAILABLE
RATE_LIMITED
INVALID_QUERY
RETRIEVAL_ERROR
COMPLETED_WITH_RESULTS
COMPLETED_NO_RESULTS
```

`COMPLETED_NO_RESULTS` may contribute to the overall `NO_RELEVANT_EVIDENCE_FOUND` state after all required searches are completed.

`PROVIDER_UNAVAILABLE` and `RATE_LIMITED` must never be treated as evidence absence.

## Safety

PubMed retrieval itself does not establish safety or clinical validity. Safety classification remains a SOMA core-engine responsibility and requires separate evidence searches where appropriate.

## Privacy

The PubMed adapter receives only normalized research queries derived from public/source protocol information. It must not send names, contact details, user identifiers, medical records or sensitive personal data to PubMed.

## Implementation rule

This adapter is the first provider implementation, but PubMed is not the evidence authority. It is a **research retrieval source**. The SOMA evidence engine remains responsible for screening, directness, quality, contradiction handling, synthesis and user-facing evidence status.
