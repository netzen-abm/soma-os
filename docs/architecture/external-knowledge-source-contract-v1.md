# SOMA External Knowledge Source Contract v1

**Status:** Architecture contract / implementation boundary  
**Date:** 2026-09-14  
**Authority:** Architecture contract  
**Scope:** Shared, provider-neutral boundary for externally accessed knowledge and research sources  
**Canonical dependencies:** `capability-adapter-contract.md`, `capability-assurance-requirements-v1.md`, `evidence-research-engine.md`, `services/evidence-research/adapters/provider-adapter-contract.md`, `research-source-registry.yaml`, `evidence-registry-schema.md`, `health-evidence-graph-v1.md`, canonical authorization/policy contracts  
**Replaces:** Nothing  
**Explicitly does not replace:** Evidence Graph, Evidence Registry, Research Provider Adapter Contract, Research Source Registry, Policy Kernel, Authorization Decision Boundary, or any application surface  
**Implementation status:** Contract only; no provider integration in v1  
**Validation status:** Contract review pending exact-head CI  

## 1. Decision

SOMA SHALL access external knowledge through a shared, provider-neutral contract. External providers are adapters, not canonical SOMA authorities.

This contract creates one reusable boundary for external knowledge sources without creating a new application, database, evidence graph, authorization engine, or universal data API.

The existing `ResearchProviderAdapter` remains the specialized research-retrieval interface. This contract supplies the broader source identity, access, governance, provenance, semantics, and trust envelope that an adapter or future external-knowledge capability must satisfy.

## 2. Architectural position

```text
Identity / Authorization / Policy
              ↓
     Governed Capability Operation
              ↓
 External Knowledge Source Contract
              ↓
        Provider Adapter
        ┌─────┼─────┐
        ↓     ↓     ↓
     PubMed OpenAlex India sources ...
              ↓
       Source-derived data
              ↓
  Canonical Evidence / Knowledge models
              ↓
      Interpretation / Intelligence
```

No provider is allowed to become the source of truth for SOMA semantics.

## 3. Core invariants

### 3.1 Shared infrastructure

The contract is shared infrastructure. Web, Android, iOS, AI/RAG, agents, research workflows, practitioner tooling, and future surfaces consume the same governed capability rather than implementing private provider integrations.

### 3.2 Provider isolation

A provider outage, schema change, rate limit, licensing restriction, or deprecation must not require changes to canonical SOMA domain semantics or unrelated providers.

### 3.3 No provider authority

A provider supplies source-derived records. It does not determine:

- clinical significance;
- evidence quality;
- causal validity;
- recommendation status;
- safety clearance;
- applicability to an individual;
- SOMA policy decisions.

### 3.4 Privacy boundary

External research/knowledge retrieval MUST NOT require personal health information. Personal identifiers, medical records, contact information, or unnecessary sensitive data MUST NOT be sent to external providers.

Queries must be derived from the governed research/knowledge task and permitted source context.

### 3.5 Provenance is mandatory

External data is useful only when SOMA can retain enough information to identify what provider supplied it, what record was retrieved, when it was retrieved, and how it was transformed.

### 3.6 Unknown remains unknown

Missing publication dates, source versions, identifiers, licensing information, verification state, or semantic fields MUST remain explicitly unknown rather than being fabricated or silently inferred.

## 4. Canonical source envelope

Every configured external source SHOULD be representable by the following conceptual envelope:

```yaml
ExternalKnowledgeSource:
  identity:
    source_id: string
    provider: string
    source_version: string|null

  access:
    protocol: string
    endpoint: string|null
    authentication_mode: enum
    rate_policy_ref: string|null

  governance:
    license_status: enum
    attribution_required: boolean
    permitted_use: enum
    retention_policy_ref: string|null
    redistribution_status: enum

  provenance:
    source_record_id: string
    source_url: string|null
    verification_url: string|null
    retrieved_at: datetime
    published_at: datetime|null
    source_version: string|null

  semantics:
    entity_types: []
    identifier_systems: []
    units: []
    temporal_model: string|null

  trust:
    verification_status: enum
    transformation_history: []
    quality_status: enum
    uncertainty: enum
```

This is a contract model, not a requirement to persist every field in a new database table.

## 5. Identity

`source_id` is the stable SOMA identifier for a configured source/provider relationship.

`provider` identifies the external organization or system.

`source_version` identifies a provider dataset/API/content version when the provider exposes one. It may be unknown.

Provider-native identifiers MUST be preserved. SOMA normalization MUST NOT erase the original identifier.

## 6. Access

The contract records how a source can be accessed without making access details part of canonical domain semantics.

The source declaration may include:

- protocol/API type;
- endpoint or canonical access location;
- authentication mode;
- rate-limit/rate-policy reference.

Secrets MUST NOT be stored in source registry content or committed to the repository.

Authentication requirements are capability/security concerns and MUST pass through SOMA's canonical authorization and policy chain where the operation is protected.

A public provider does not mean unrestricted SOMA use. Provider terms, rate limits, licensing, and permitted use remain governance inputs.

## 7. Governance

The source envelope MUST distinguish at least:

- license status;
- attribution requirement;
- permitted use;
- retention constraints;
- redistribution status.

A source may be technically accessible but not legally or operationally suitable for a particular SOMA use.

SOMA MUST NOT infer that public accessibility grants unrestricted copying, redistribution, training, or commercial use.

## 8. Provenance

At minimum, a retrieved source-derived record SHOULD retain:

```text
source_id
provider_record_id
source_url / canonical reference
verification_url when available
retrieved_at
published_at when available
source_version when available
```

For derived representations, SOMA SHOULD additionally retain:

```text
transformation_history
transformation_version
actor_or_process_ref
```

The representation classes are explicitly distinct:

```text
SOURCE_DERIVED
TRANSFORMED
INFERRED
COMPUTED
MODEL_GENERATED
HUMAN_VERIFIED
```

`MODEL_GENERATED` never means source-verified. `HUMAN_VERIFIED` must identify the verification actor/process where governance requires it.

## 9. Source → canonical separation

The boundary is:

```text
External provider record
        ↓
source-derived representation
        ↓
validated / normalized representation
        ↓
canonical SOMA domain object where justified
        ↓
assessment / interpretation
```

These stages MUST NOT be silently collapsed.

In particular:

```text
provider metadata != evidence assessment
provider ranking != SOMA relevance
provider category != clinical classification
provider claim != medical fact
provider availability != evidence sufficiency
```

The Evidence Graph remains the canonical general evidence model. Its source objects retain provenance and verification rather than delegating epistemic authority to a provider.

## 10. Semantics

External sources may expose different:

- entity models;
- identifier systems;
- units;
- temporal semantics;
- taxonomies;
- geographic classifications;
- versioning schemes.

Adapters MUST preserve source semantics before normalization. Normalization must be explicit and versioned.

If a mapping is uncertain, the uncertainty MUST be preserved.

For traditional, indigenous, or otherwise distinct knowledge systems, the existing Epistemic Context Mapping contract applies before cross-system interpretation. This contract does not flatten those systems into a universal evidence hierarchy.

## 11. Verification and trust

Initial trust states:

- `UNVERIFIED` — source record retrieved but verification has not been established;
- `SOURCE_VERIFIED` — the source/record has been independently verified according to the applicable source contract;
- `TRANSFORMATION_VERIFIED` — a transformation has been validated against its declared source representation;
- `HUMAN_VERIFIED` — an authorized human verification process has been completed;
- `WITHDRAWN` — the representation must not be treated as active.

A provider's response can be retained without being treated as verified evidence.

If a verification URL is unavailable, record that fact. Never fabricate one.

## 12. Failure contract

The source boundary MUST distinguish operational failure from an empty successful result.

```text
PROVIDER_UNAVAILABLE
RATE_LIMITED
AUTHENTICATION_REQUIRED
ACCESS_FORBIDDEN
SOURCE_SCHEMA_CHANGED
INVALID_REQUEST
COMPLETED
```

For research retrieval, these states continue to follow the existing provider adapter rule: provider failure is not equivalent to `NO_RELEVANT_EVIDENCE_FOUND`.

A source-specific failure SHOULD include a stable machine-readable reason without leaking secrets.

## 13. Cost and dependency boundary

Free/public sources are preferred during the foundation phase where they meet the quality and governance requirements.

Paid providers may be added later when there is a demonstrated product need, approved governance, and an adapter contract.

No paid provider may become a hidden mandatory dependency of the canonical SOMA runtime.

## 14. Authorization and policy

External knowledge retrieval is a capability operation, not an authorization exception.

Where authorization is required, the execution path remains:

```text
IdentityContext
  → Identity Authorization Enforcement
  → Authorization + Policy Decision Boundary
  → Policy Kernel
  → Governed Capability Operation
  → External Knowledge Source Adapter
```

An adapter MUST NOT implement a private authorization policy that can override the canonical decision.

Capability identity requirements continue to be evaluated by the Policy Kernel; this contract does not define a second identity or authorization model.

## 15. Research-provider compatibility

The existing `ResearchProviderAdapter` remains the research-specific interface:

```text
provider_id()
search(query)
fetch(record_id)
verification_url(record_id)
```

The new source envelope wraps governance and provenance around that provider-specific retrieval boundary; it does not replace the research adapter contract.

The research engine remains responsible for deduplication, screening, directness, quality/bias assessment, contradiction handling, synthesis, safety discovery, and user-verifiable evidence packaging.

## 16. No new persistence requirement

This contract does NOT require:

- a universal external-data database;
- a second evidence registry;
- a generic graph database;
- a universal knowledge graph;
- a new caching platform;
- a new API gateway;
- a new authorization engine.

Persistence is introduced only when a concrete capability requires it and must reuse existing SOMA storage/evidence ownership boundaries.

## 17. Security requirements

Provider adapters MUST:

- validate provider responses before canonical mapping;
- enforce response-size and resource limits appropriate to the adapter;
- avoid SSRF-prone unrestricted URL fetching;
- restrict outbound destinations to configured providers where practical;
- avoid forwarding credentials or sensitive user data to unrelated endpoints;
- preserve provider errors rather than fabricating successful records;
- fail closed for protected operations when authorization/policy cannot be established;
- record provenance without recording secrets.

Provider-supplied text is untrusted input and MUST NOT be treated as executable instructions.

External content MUST NOT be allowed to modify SOMA policy, authorization, safety, or capability configuration merely by being retrieved.

## 18. Non-goals

This contract does not:

- implement PubMed, OpenAlex, World Bank, UN, Indian government, legal, patent, or other adapters;
- define a universal data ontology;
- define a recommendation engine;
- define causal inference;
- define clinical evidence grading;
- replace the Evidence Graph;
- replace the Research Provider Adapter Contract;
- replace the Research Source Registry;
- make external sources mandatory for core SOMA operation;
- authorize AI agents to retrieve data autonomously.

## 19. Acceptance criteria

The contract is implementation-ready when:

- provider identity and native identifiers are preserved;
- source access metadata is separate from canonical domain semantics;
- licensing/attribution/permitted-use concerns are explicit;
- provenance is sufficient for user verification where available;
- source-derived, transformed, inferred, computed, model-generated, and human-verified representations remain distinguishable;
- provider failure is distinct from an empty successful result;
- privacy prevents unnecessary personal-health-data transmission;
- authorization/policy remains canonical;
- external content cannot alter SOMA authority boundaries;
- no second evidence/graph/auth infrastructure is introduced;
- the contract can support both research providers and future non-research public knowledge sources.

## 20. Implementation rule

Implement the smallest contract first. Do not add a provider adapter in the same change.

The first provider implementation should be selected only after the contract passes exact-head CI/security validation and source-specific governance review.
