# SOMA-OS System Architecture + Product Specification v1

**Status:** Architecture baseline  
**Scope:** Whole SOMA ecosystem and shared infrastructure  
**Authority:** System-level architecture baseline; machine-readable semantics are authoritative in schemas and executable contract tests. Durable rationale belongs in `docs/decisions/`.  
**Implementation note:** This document describes the intended/current architectural composition and must not be interpreted as proof of production readiness.

## 1. Purpose

SOMA-OS is a privacy-first, evidence-aware health intelligence infrastructure. It is one ecosystem built from shared capabilities and governed contracts. Product surfaces, providers, AI/agents, and protocols consume the shared foundation rather than implementing parallel health, evidence, privacy, safety, authorization, or provenance logic.

This specification establishes the whole-system architecture. Feature-specific implementation status belongs in the product-readiness master checklist and implementation documents.

## 2. Product principles

1. **Shared-first:** reusable capability belongs in shared infrastructure.
2. **One canonical domain:** domain contracts and invariants remain canonical even when implementation is polyglot.
3. **Canonical/reference runtime:** Rust is the canonical/reference runtime for security-sensitive and domain-critical behavior; other runtimes conform through explicit contracts where justified.
4. **Authorization before protected execution:** protected operations require validated identity/security context, canonical policy evaluation, and target-scope enforcement.
5. **Evidence integrity:** provenance, uncertainty, source status, safety, applicability, contradiction, and transformation lineage survive downstream processing.
6. **Fail closed:** ambiguous security state denies rather than guesses.
7. **Independent surfaces:** a transport, provider, model, agent, or protocol cannot redefine core policy or domain semantics.
8. **Optional capabilities remain optional:** AI, Nostr, Web3, DID/VC, MCP, and content-addressed storage are adapters/capabilities, not mandatory core dependencies.
9. **Archive before deletion:** historical material is preserved until safe removal is evidenced.
10. **No silent legacy promotion:** legacy data becomes protected only through explicit, verified, auditable promotion.

## 3. System context

```text
User / Operator / Service
        ↓
Product Surface / Adapter
        ↓
IdentityContext
        ↓
Identity → Authorization Enforcement
        ↓
Authorization + Policy Decision Boundary
        ↓
Policy Kernel
        ↓
Canonical Authorization Decision
        ↓
Governed Capability Operation / ProtectedDataAccess
        ↓
Trusted Persistence Identity
        ↓
Trusted Transaction Context
        ↓
PostgreSQL RLS / constraints or approved vault adapter
        ↓
Protected Data
        ↓
Provenance / Audit
```

Health/evidence capabilities compose alongside this security path:

```text
Observation / imported record
        ↓
Provenance + context + uncertainty
        ↓
Canonical Health State
        ↓
Explicit Health-State ↔ Evidence linkage
        ↓
Health Evidence Graph
        ↓
Bounded interpretation
        ↓
Human decision
        ↓
Outcome
        ↓
New observation
```

Optional adapters include AI, agents, MCP, messaging, Nostr, Web3/DID/VC, content-addressed storage, research providers, wearables, labs, and other health-data sources.

## 4. Authority boundaries

| Layer | Authority | Responsibility | Must not do |
|---|---|---|---|
| Product surfaces | none | UX, transport, request collection | redefine policy or security scope |
| IdentityContext | verified identity/security context | represent principal and explicit scope | contain untrusted authorization or credentials |
| Identity Authorization Enforcement | identity/scope security boundary | validate identity context and target scope | invent policy grants |
| Authorization + Policy Decision Boundary | authorization composition boundary | bind request, invoke Policy Kernel, preserve authoritative decision | become a second policy engine |
| Policy Kernel | policy evaluation authority | deterministic policy decision | read/write protected data directly |
| Governed Capability Operation | execution governance | operation lifecycle and authority gates | replace authorization decision |
| ProtectedDataAccess | persistence gate | bind authorized operation to protected target | grant access independently |
| Trusted DB identity/context | storage trust boundary | establish trusted DB identity and transaction scope | accept caller-controlled scope |
| PostgreSQL RLS/constraints | defense-in-depth | contain storage-layer mistakes | replace application authorization |
| Health State | personal-health semantic authority | represent observations/interventions/outcomes and related context | become population evidence |
| Health Evidence Graph | general evidence authority | represent claims/sources/studies/evidence/safety/applicability | become personal health fact automatically |
| Optional adapters | integration boundary | connect external systems | become core authority/dependency |

## 5. Canonical authorization path

```text
Request
  ↓
Verified IdentityContext
  ↓
Identity → Authorization Enforcement
  ↓
Authorization + Policy Decision Boundary
  ↓
Policy Kernel
  ↓
Canonical Authorization Decision
  ↓
Governed Capability Operation / ProtectedDataAccess
  ↓
Trusted persistence boundary
  ↓
Protected operation
  ↓
Audit / provenance
```

The Policy Kernel is the sole policy evaluator. The Authorization + Policy Decision Boundary is the authoritative composition boundary around it. Consent, human review, authentication, grant administration, RLS, and execution remain distinct contracts.

## 6. Health information architecture

SOMA deliberately separates:

```text
Observation
  ≠ Claim
  ≠ Evidence
  ≠ Interpretation
  ≠ Recommendation
```

The Health State Model is the canonical personal-health semantic model. The Health Evidence Graph is the canonical general evidence semantic model. Explicit directional links connect them without making either source of truth for the other.

Unknown time remains unknown. Derived signals retain their source observations and processing lineage.

## 7. Longitudinal health intelligence

The future intelligence loop is:

```text
Observe / Import
  → Verify
  → Normalize
  → Preserve provenance/context
  → Longitudinal timeline
  → Retrieve evidence
  → Examine contradictions / negative evidence
  → Assess evidence quality / applicability / uncertainty
  → Apply safety gates
  → Produce bounded interpretation / questions / options
  → Human / clinician decision
  → Outcome
  → Learn from new observation
```

This is an architecture baseline, not authorization to implement autonomous clinical reasoning.

## 8. Legacy data lifecycle

```text
Legacy rows
  ↓
Inventory
  ↓
Classification evidence
  ↓
Authoritative provenance verification
  ↓
Quarantine / review / rejection OR explicit promotion
  ↓
Preflight: zero NULL protected scope
  ↓
Final constraints / NOT NULL gate
  ↓
Normal protected lifecycle
```

Promotion must be explicit, provenance-bound, idempotent, auditable, and rollback-safe. Untrusted metadata cannot establish protected scope.

## 9. Product capability families

| Capability family | Architectural status | Rule |
|---|---|---|
| Identity / authorization | foundational shared infrastructure | canonical path required |
| Protected health data | foundational | explicit tenant/data-domain isolation |
| Health State | foundational | canonical personal-health model |
| Evidence Graph | foundational | provenance + uncertainty + safety |
| Governed Operations | foundational | canonical operation lifecycle |
| Capability Registry | foundational | canonical capability inventory |
| Research/evidence adapters | shared infrastructure | provider-neutral and replaceable |
| AI | optional | user-controlled; cannot authorize itself |
| Agents | optional/shared infrastructure | capability-scoped and auditable |
| MCP | optional/shared infrastructure | discovery/invocation remains governed |
| Nostr/Web3/DID/VC/IPFS | future adapters | plug-and-play; never core dependencies |

## 10. Product roles

End users, research/knowledge operators, security/data operators, and system services receive only the capabilities and data for which they are authorized. Operational migration authority is not equivalent to normal application access.

## 11. Product acceptance boundary

A feature is not production-ready merely because its architecture or code exists. The applicable readiness gates are maintained in `docs/PRODUCT-READINESS-MASTER-CHECKLIST.md`.

At system level, the first meaningful product milestone is one complete health/evidence workflow that demonstrates shared capability composition, identity/authorization, evidence/provenance, safety, failure isolation, auditability, and a real user outcome.

## 12. Non-goals

This baseline does not itself implement:

- authentication provider integration;
- consent lifecycle;
- autonomous diagnosis or treatment;
- autonomous clinical authority;
- universal causal inference;
- mandatory AI;
- mandatory decentralized protocols;
- multiple independent SOMA applications;
- broad implementation of every future capability mentioned in this document.

## 13. Architectural rule for future work

Before implementing a feature:

1. find the existing canonical contract;
2. determine whether the capability is shared;
3. identify its policy/security boundary;
4. identify authoritative data/evidence schemas;
5. define provenance and failure semantics;
6. add contract/adversarial tests;
7. implement the smallest necessary shared primitive;
8. expose it through adapters only after the shared boundary is sound.

The objective is not maximum module count. The objective is a coherent shared infrastructure capable of safely supporting multiple SOMA use cases.
