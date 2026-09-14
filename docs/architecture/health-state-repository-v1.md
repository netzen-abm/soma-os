# SOMA Health State Repository v1

**Status:** Bounded implementation contract  
**Date:** 2026-09-14  
**Scope:** Shared SOMA ecosystem infrastructure

## 1. Decision

SOMA needs a provider-neutral repository boundary for canonical Health State entities so the existing Health State schema can be persisted and retrieved through the existing protected storage infrastructure.

This repository is **not** a new clinical database, Health State model, graph database, index, or authorization engine.

The canonical Health State schema remains authoritative:

`schemas/health-state-v1.json`

The Local Health Vault remains the protected persistence authority.

## 2. Boundary

```text
Identity / Authorization / Policy
              ↓
Authorized Health State Access Context
              ↓
Health State Repository
              ↓
Local Health Vault provider
              ↓
Encrypted protected record
```

The repository consumes an authorization-bound context and delegates protected storage operations to the existing vault.

## 3. Canonical ownership

- Health State semantics remain owned by `health-state-v1.json` and its architecture contract.
- Health State ↔ Evidence linkage remains owned by the explicit linkage contract and boundary.
- Evidence Graph remains a separate canonical domain.
- Personal Health Record remains a reference/view layer rather than a competing Health State store.
- The Local Health Vault remains responsible for encryption, integrity, tombstoning, key resolution, and protected persistence.
- Canonical authorization remains responsible for authorization decisions.

## 4. Entity boundary

The repository accepts only Health State entity types defined by the canonical schema:

- `person`
- `observation`
- `interpretation`
- `goal`
- `context`
- `intervention`
- `response`
- `outcome`

The repository does not reinterpret the entity payload or silently convert one entity type into another.

Schema validation remains a contract concern; storage must not become a second semantic validator with a competing model.

## 5. Access control

The repository must not accept caller-constructed authorization contexts.

An authorized context is minted only through the canonical authorization boundary after an authoritative `ALLOW` decision and structural validation.

Non-`ALLOW` decisions cannot produce a repository access context.

## 6. Query and timeline semantics

The repository may filter using protected metadata such as entity type and classification when the vault exposes that metadata through its governed index.

It must not introduce plaintext clinical indexing merely to support convenience queries.

The repository must not fabricate Health State effective time from vault recording time. If health-relevant time is unavailable through the protected metadata boundary, it remains unknown at the repository projection layer.

## 7. Relationship and evidence semantics

The repository stores canonical Health State entities and does not become a relationship engine.

Relationships are interpreted according to the Health State schema. Cross-domain evidence relationships continue through the explicit Health State ↔ Evidence linkage boundary.

Temporal association is not causation. An intervention followed by an outcome is not itself proof of efficacy or causality.

## 8. Non-goals

This v1 does not implement:

- a second Health State schema;
- a graph database;
- a generic relationship service;
- clinical inference;
- causal inference;
- recommendation logic;
- an intervention/experiment engine;
- AI or agent authority;
- a second authorization or policy engine;
- plaintext health-data indexes;
- distributed workflow orchestration.

## 9. Acceptance criteria

A conforming implementation must demonstrate that it:

1. accepts only canonical Health State entity types;
2. uses an authorization-bound access context;
3. cannot mint that context from caller-supplied scope alone;
4. delegates protected persistence to the existing Health Vault;
5. preserves vault encryption/integrity/tombstone semantics;
6. does not create a second clinical store or plaintext index;
7. preserves provenance and uncertainty by storing the canonical payload unchanged;
8. does not fabricate health-relevant time;
9. keeps Health State and Evidence Graph ownership separate;
10. remains compatible with the later longitudinal intelligence runtime.

## 10. Position in the SOMA loop

This repository is a persistence primitive, not the intelligence loop itself.

It enables the next bounded layer:

```text
Observe
  ↓
Canonical Health State
  ↓
Explicit Evidence Linkage
  ↓
Governed Interpretation / Hypothesis
  ↓
Future Governed Operation
  ↓
Response / Outcome
  ↓
New Observation
```

The implementation should proceed incrementally: repository boundary first, then a read-oriented longitudinal context assembly capability, then bounded intervention/experiment execution only after its own authority and safety contracts are established.
