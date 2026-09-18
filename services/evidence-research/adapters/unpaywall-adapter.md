# SOMA Unpaywall Research Adapter

## Status

Bounded implementation v1.

## Purpose

Resolve open-access locations for an already identified scholarly work, normally by DOI. This adapter enriches research-work identity with access-location information; it does not assess evidence quality or clinical relevance.

## Boundary

```text
Governed evidence.research operation
        ↓
Research Source Adapter Contract
        ↓
Unpaywall Adapter
        ↓
Unpaywall
        ↓
ResearchAccessLocation
        ↓
Existing research/evidence infrastructure
```

## Contract

The adapter returns canonical `ResearchAccessLocation` records.

Each record preserves:

- Unpaywall provider identity;
- work identifier;
- resolved access URL;
- location type;
- open-access state;
- license when reported;
- version when reported;
- `ExternalKnowledgeSource` provenance.

## Privacy

The adapter accepts a scholarly identifier and provider-required contact email. It does not accept or transmit protected SOMA health records, personal health identifiers or internal authorization material.

## Failure semantics

A provider/network failure raises `UnpaywallProviderError`. A successful response with no access locations returns an empty list. These states remain distinct.

## Non-goals

- no evidence-quality assessment;
- no clinical recommendation;
- no license/legal determination beyond reporting provider metadata;
- no second evidence store;
- no provider-specific policy engine;
- no direct product-surface access;
- no automatic publication of research conclusions.
