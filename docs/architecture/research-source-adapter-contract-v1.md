# SOMA — Research Source Adapter Contract v1

## Status

Design contract. Provider implementations exist; this contract is now the canonical convergence boundary for them.

## Purpose

SOMA research infrastructure needs a canonical way to discover, identify, retrieve and preserve scholarly research metadata without creating provider-specific evidence models or a second evidence system.

The Research Source Adapter is shared SOMA infrastructure. Web, Android, iOS, AI/RAG, practitioner, research and future surfaces consume the same contract.

## Architectural position

```text
SOMA Surface / Agent / Workflow
          ↓
Governed Capability Operation
          ↓
Authorization + Policy
          ↓
Research Source Adapter Contract
          ↓
Provider Adapter
          ↓
External Research Provider
          ↓
Normalized Research Records
          ↓
Evidence Research Core
          ↓
Evidence / Provenance / Audit
```

Provider adapters are transport and normalization boundaries. They are not policy engines, evidence assessors or clinical authorities.

## Initial providers

The first implementation candidates are:

1. OpenAlex — scholarly metadata and relationship discovery.
2. Unpaywall — open-access location resolution.
3. Semantic Scholar — scholarly metadata and citation/related-work discovery.
4. DOAJ — open-access journal and article metadata.

Provider admission is not an endorsement of provider quality. Each provider remains independently attributable, versioned and failure-isolated.

## Canonical records

The adapter layer should normalize provider responses into these shared concepts:

### ResearchSource

Represents an external research provider.

```text
source_id
provider_name
provider_version_or_contract
source_class
base_reference
status
```

`source_id` is SOMA's stable identifier for the provider integration. Provider identifiers must not be used as SOMA authorization identities.

### ResearchWork

Represents a scholarly work or a provider's canonical representation of one.

```text
work_id
source_id
provider_record_id
title
authors
venue
publication_year
identifiers
source_url
verification_url
retrieved_at
```

A `ResearchWork` is a source-derived record, not an evidence judgment.

### ResearchAuthor

```text
author_id
source_id
provider_author_id
display_name
identifiers
```

### ResearchVenue

```text
venue_id
source_id
provider_venue_id
name
venue_type
identifiers
```

### ResearchIdentifier

Identifiers should preserve type and value without assuming that one identifier system is universally authoritative.

Supported initial types may include:

- DOI;
- PMID;
- PMCID;
- OpenAlex ID;
- Semantic Scholar ID;
- ISSN;
- provider-specific record ID.

### ResearchRelationship

Relationships describe source-derived relationships only.

Initial vocabulary:

- `CITES`
- `REFERENCES`
- `AUTHORED_BY`
- `PUBLISHED_IN`
- `RELATED_TO`
- `HAS_VERSION`
- `HAS_OPEN_ACCESS_LOCATION`

The adapter must not emit generic `CAUSES`, `PROVES`, `VALIDATES` or `RECOMMENDS` relationships. Those require separate evidence, causal or decision contracts.

### ResearchAccessLocation

Represents where a work can be accessed.

```text
location_id
work_id
source_id
url
location_type
is_open_access
license_if_reported
retrieved_at
```

An access location is not automatically a trusted evidence source. SOMA must preserve its provenance and verification state.

### ResearchProvenance

Every normalized record must retain enough information to reproduce its origin:

```text
source_id
provider_record_id
request_id
retrieved_at
source_url
verification_url
adapter_version
raw_or_normalized_reference
```

Provider failures and empty successful searches must remain distinguishable.

## Query contract

A provider query must be represented as structured data rather than an opaque model-generated instruction.

Minimum concepts:

```text
research_query_id
query_text_or_terms
filters
identifier_constraints
requested_record_types
pagination
requested_fields
created_at
```

The query contract must not contain unnecessary personal health information.

If a research question originated from protected SOMA health data, the workflow must first minimize/de-identify the information needed for public research discovery.

## Authorization and purpose

External research access remains governed SOMA execution.

```text
Principal
  → Subject / scope where applicable
  → Capability
  → Resource
  → Action
  → Purpose
  → Policy Decision
  → Bounded Permission / Operation
  → Provider Adapter
```

Examples of capabilities may include:

- `research.openalex.query`
- `research.unpaywall.resolve`
- `research.semantic_scholar.query`
- `research.doaj.query`

These are examples for capability design, not yet registry additions.

External providers never authorize SOMA access. A provider response cannot widen the authorized scope.

## Ephemeral external-access permission

Where provider access requires a permission lease, the same shared permission semantics apply:

```text
authorize
   ↓
activate bounded capability
   ↓
perform declared research operation
   ↓
release immediately after purpose completion
```

A later operation requires a new authorization decision and activation. Provider API credentials are infrastructure secrets and are not equivalent to user authorization.

## Privacy boundary

The adapter must enforce data minimization before provider transmission.

Do not send:

- raw personal health records;
- unnecessary identifiers of a health subject;
- unrelated longitudinal observations;
- private notes;
- credentials or authorization material;
- internal SOMA policy decisions.

A public literature query should normally contain only the minimum research terms, identifiers or filters needed to answer the research question.

## Evidence boundary

The adapter may discover and normalize source records. It must not decide:

- evidence quality;
- risk of bias;
- clinical efficacy;
- causal effect;
- clinical significance;
- treatment suitability;
- safety sufficiency;
- recommendation status.

These remain downstream governed contracts. This preserves the existing Evidence Research Core boundary, whose candidates begin `UNSCREENED` and whose assessment fields are intentionally unset until downstream assessment contracts act. 

## Search and result semantics

The adapter must distinguish at least:

```text
SUCCESS_WITH_RESULTS
SUCCESS_EMPTY
PARTIAL_PROVIDER_FAILURE
PROVIDER_UNAVAILABLE
INVALID_QUERY
RATE_LIMITED
AUTHENTICATION_FAILURE
POLICY_DENIED
```

`SUCCESS_EMPTY` means the provider successfully processed the request but returned no matching records. It must not be reclassified as provider failure or as evidence of absence beyond the search performed.

`PARTIAL_PROVIDER_FAILURE` must identify which provider operation failed while preserving successful provider results.

## Identity resolution

Provider records may describe the same scholarly work. Deduplication and identity resolution should use stable identifiers where available, such as DOI, PMID, PMCID, OpenAlex ID and Semantic Scholar ID.

Identity resolution must preserve provider provenance and must not erase conflicting provider metadata silently.

A provider-specific identifier is not itself proof that two records are scientifically equivalent.

## Open-access resolution

Unpaywall and similar services provide access-location information. They do not determine evidence quality.

The normalized result should distinguish:

```text
work identity
access location
license information when reported
source provenance
verification state
```

## AI boundary

AI may help formulate structured queries, classify returned records, normalize metadata and summarize source-derived records.

AI may not:

- invent research records;
- invent identifiers or citations;
- silently alter provider provenance;
- convert a discovery result into evidence assessment;
- infer causality from citation relationships;
- bypass authorization;
- widen purpose or scope;
- conceal provider failure or contradictory research.

Every AI-generated research statement must remain traceable to normalized source records and, where applicable, extracted passages.

## Failure isolation

Provider adapters must fail independently.

```text
OpenAlex failure
      ≠
Unpaywall failure
      ≠
Semantic Scholar failure
      ≠
DOAJ failure
```

One provider being unavailable must not make the entire research infrastructure unavailable if the governing orchestration contract permits partial results.

Provider-specific retries, rate limits and authentication failures belong inside the adapter boundary and must not redefine SOMA authorization semantics.

## Relationship to Evidence Research Core

The Research Source Adapter produces source-derived normalized research records.

The existing Evidence Research Core consumes normalized orchestration results and converts them into assessment-ready `EvidenceCandidate` records without making evidence judgments.

Therefore:

```text
Research Source Adapter
        ↓
Orchestration Runtime
        ↓
Evidence Research Core
        ↓
EvidenceCandidate
        ↓
Screening / Extraction / Assessment
```

No second evidence store or parallel evidence graph is introduced.

## Audit and provenance

Research operations should produce audit/provenance events containing, as appropriate:

- principal reference;
- subject/scope when applicable;
- capability;
- purpose;
- provider/source;
- query identifier;
- operation result status;
- adapter version;
- timestamp;
- authorization decision reference;
- permission activation/release reference where applicable.

Audit records should minimize sensitive query content and should not become a copy of protected health data.

## Initial implementation boundary

This contract does not require new provider clients. Existing PubMed and OpenAlex implementations are being converged onto the boundary.

The first implementation should establish:

1. canonical Rust/domain types;
2. adapter trait and normalized result contract;
3. provider failure semantics;
4. provenance requirements;
5. capability identifiers after registry review;
6. OpenAlex adapter;
7. Unpaywall access-location resolver;
8. Semantic Scholar adapter;
9. DOAJ adapter;
10. contract tests shared by every adapter.

## Explicit non-goals

- no separate research application;
- no provider-specific evidence engine;
- no second authorization engine;
- no second policy engine;
- no second evidence store;
- no automatic clinical recommendation;
- no autonomous evidence interpretation;
- no causal graph inferred from citations;
- no automatic publication of clinical-facing conclusions.

## Design decision

SOMA should treat research providers as **replaceable source adapters behind one governed research infrastructure boundary**.

The canonical unit is not a provider. It is the traceable research operation:

`Research Question → Governed Query → Source Adapter → Normalized Work → Provenance → Evidence Research Core → Assessment`.
