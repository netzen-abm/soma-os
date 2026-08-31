# SOMA Evidence Research — Domain Contract

## Purpose

Provider-neutral domain contract for the shared Evidence Research Service.

## Research request

```text
ResearchRequest
├── protocol_id
├── indication
├── source
├── components[]
├── preparation
├── administration
├── duration
├── claimed_outcomes[]
└── research_scope
```

## Provider adapter contract

Every research provider adapter must accept a normalized research query and return provider-neutral records.

```text
ResearchProvider
├── provider_id
├── search(query) -> SearchResult[]
├── get_record(provider_record_id) -> RawStudy
└── verification_url(provider_record_id) -> URL
```

The core engine must never depend on provider-specific response fields.

## Search result

```text
SearchResult
├── provider_id
├── provider_record_id
├── title
├── publication_year
├── study_type_hint
├── abstract_or_summary
├── source_url
└── verification_url
```

## Study record

A normalized study record contains only information supported by the source.

```text
StudyEvidence
├── research_id
├── title
├── authors[]
├── year
├── journal
├── DOI
├── PubMed ID
├── study_type
├── population
├── sample_size
├── intervention
├── comparator
├── duration
├── outcomes[]
├── main_findings
├── limitations
├── safety_findings
├── funding
├── conflicts_of_interest
├── source_url
└── verification_url
```

## Protocol relationship

A protocol-to-study relationship must contain:

```text
ProtocolEvidenceRelation
├── protocol_id
├── research_id
├── directness
├── supporting_or_contradictory
├── rationale
└── assessed_at
```

## Mandatory directness values

```text
DIRECT_EXACT_PROTOCOL
DIRECT_COMPONENT
RELATED_INTERVENTION
MECHANISTIC
OBSERVATIONAL_ASSOCIATION
PRECLINICAL
INDIRECT
NO_RELEVANT_EVIDENCE_FOUND
```

## Mandatory evidence states

```text
STRONG
MODERATE
LIMITED
MIXED
INDIRECT
INSUFFICIENT
```

The state is an evidence assessment, not a treatment recommendation.

## Safety result

```text
SafetyFinding
├── subject
├── finding
├── population
├── interaction
├── contraindication
├── dose_context
├── source_url
└── confidence
```

## No-evidence result

The engine must be able to return a valid package with zero studies.

```text
EvidencePackage
├── evidence_status = INSUFFICIENT
├── studies = []
├── no_evidence_declaration
├── search_context
├── safety
└── professional_review_required = true
```

This is a successful research outcome, not an error.

## Provider independence

The core engine owns:

- normalization;
- query generation;
- screening;
- directness classification;
- evidence assessment;
- contradiction handling;
- safety gating;
- user presentation contract.

Adapters own only provider-specific retrieval and source URL construction.

## AI boundary

AI may assist with query expansion, extraction and summarization, but every material claim must retain a source reference. Provider output must never be replaced by model-generated facts.

## Privacy boundary

No user identity, personal health information or sensitive personal data is required by this contract. Protocol research can be executed from source-derived protocol information alone.
