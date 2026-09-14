# SOMA United Nations Data Adapter

## Purpose

Bounded retrieval from the UNdata SDMX REST interface under the shared `ExternalKnowledgeSource` contract.

## Boundary

- Uses the existing provider-neutral external knowledge boundary.
- Retrieves one explicitly identified SDMX dataflow, optionally filtered by series key and period.
- Preserves the provider's SDMX agency/dataflow/series identity.
- Keeps returned statistical values source-derived; no evidence-quality or clinical interpretation is performed.
- Uses deterministic mocked tests; CI does not depend on a live UNdata request.

## Official interface

UNdata documents a public REST API at `http://data.un.org/ws/rest/`, powered by Eurostat's SDMX Reference Infrastructure. The API documentation states that data queries use SDMX artifacts such as Agency, DataFlow and DataStructure, and that XML, JSON and CSV response formats are supported. SOMA uses CSV for the bounded retrieval adapter. The provider documentation also states that API coverage varies by datamart according to available SDMX Data Structure Definitions.

## Provenance

Each result records the UNdata source identifier, source/verification URL, retrieval time, and shared source-contract metadata. Provider publication/version fields remain unknown unless a future verified mapping explicitly establishes them.

## Privacy

No personal health information is sent to UNdata. Queries contain only governed public-statistics identifiers and optional period filters.

## Failure semantics

Provider/network/response failures raise `UNDataProviderError`. A successful response with zero rows remains a successful retrieval and is not converted into provider failure.

## Non-goals

- no second statistical warehouse or cache;
- no universal data ontology;
- no Evidence Graph writes;
- no evidence-quality or clinical-significance assessment;
- no causal inference or recommendations;
- no personal-health-data transmission;
- no second authorization or policy engine;
- no scraping of UNdata HTML pages;
- no assumption that every UNdata datamart is SDMX-enabled.
