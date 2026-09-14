# SOMA Our World in Data Adapter

## Purpose

Bounded retrieval from Our World in Data's public Grapher data interface under the shared `ExternalKnowledgeSource` contract.

## Boundary

- Uses the existing provider-neutral external knowledge boundary.
- Retrieves one Grapher slug at a time as CSV plus metadata JSON.
- Preserves the OWID slug as the provider-native identifier.
- Keeps provider metadata source-derived; no evidence-quality or clinical interpretation is performed.
- Uses deterministic mocked tests; CI does not depend on a live OWID request.

## Provenance

Each result records the OWID source identifier, canonical Grapher URL, retrieval time, and the shared source-contract metadata. Provider publication/version fields remain unknown unless the provider response supplies values that SOMA explicitly maps.

## Privacy

No personal health information is sent to OWID. Queries contain only the governed dataset slug.

## Failure semantics

Provider/network/response failures raise `OwidProviderError`. A successful response with zero rows remains a successful retrieval and is not converted into provider failure.

## Non-goals

- no second data warehouse or cache;
- no universal data ontology;
- no Evidence Graph writes;
- no evidence-quality or clinical-significance assessment;
- no causal inference or recommendations;
- no personal-health-data transmission;
- no second authorization or policy engine.
