# Observation → Relationship Contract v1

## Status

Architecture contract for the shared SOMA longitudinal substrate. This document defines semantics only; it does not introduce a new datastore, clinical payload model, causal inference engine, or application surface.

## Purpose

SOMA already has a provider-neutral longitudinal observation repository and a canonical authorization boundary. The next reusable layer is a governed relationship contract that can connect observations and other explicitly referenceable entities without turning the relationship layer into a second Health State model.

## Core rule

**A relationship is not, by itself, a causal claim.**

A relationship records an explicitly typed association between source and target references. Causal interpretation requires separate evidence and assessment semantics and must not be inferred by repository persistence or timeline ordering.

## Relationship contract

A relationship has the following conceptual fields:

- relationship_id: stable identifier.
- source_ref: canonical reference to the source observation/entity.
- target_ref: canonical reference to the target observation/entity.
- relationship_type: controlled vocabulary value describing the asserted association.
- temporal_scope: explicit temporal bounds or temporal relation when applicable.
- subject_scope: subject binding for the relationship.
- provenance: who/what established the relationship and the originating source.
- evidence_refs: references to supporting evidence when available.
- uncertainty: explicit representation of uncertainty; absence of certainty must not be represented as certainty.
- capability_id: canonical capability used for the governed operation.
- capability_version: exact capability version authorized.
- authorization_scope: principal, subject, tenant, data-domain, resource and action scope inherited from the canonical authorization decision.

## Relationship-type semantics

Relationship types must be explicitly named, documented, non-ambiguous, distinguishable from causal claims, and extensible without changing existing meanings.

Examples of semantic classes that may be introduced later include temporal association, contextual association, correlation/assertion, derivation, composition, and reference. These are examples of contract categories, not an implementation commitment.

## Provenance and evidence

The relationship layer must preserve provenance rather than replacing it with a generated assertion.

Where evidence exists, evidence_refs point to the canonical evidence infrastructure. Evidence strength, assessment, and causal interpretation remain separate concerns.

A relationship without evidence is not automatically invalid; it must instead make its provenance and uncertainty explicit.

## Subject and scope rules

Relationships involving protected health observations are subject-bound.

A relationship must not widen access beyond the authorization scope that permitted its creation or retrieval.

Cross-subject relationships require an explicit governed authorization contract; they must never arise implicitly from a repository query.

## Temporal rules

Temporal association must distinguish:

- observation time;
- recording time;
- relationship assertion time;
- optional relationship validity interval.

The relationship layer must not manufacture an observation time from recording time.

Unknown temporal values remain unknown.

## Health State boundary

The relationship repository is not a replacement for canonical Health State.

Health State remains the canonical representation of health-domain observations. The relationship layer references canonical entities and provides governed association semantics around them.

## Persistence boundary

Do not create a relationship-specific datastore until the contract has been validated against existing repository/provider boundaries.

The first implementation should reuse the existing governed repository and protected-data infrastructure where technically appropriate.

No second authorization engine is permitted.

## AI and inference boundary

AI/agent systems may later propose candidate relationships, but a generated proposal must remain distinguishable from an accepted governed relationship.

AI output does not automatically become canonical fact.

Any promotion from proposal to governed relationship must pass the same authorization, provenance, uncertainty, and evidence requirements as other relationship creation paths.

## Required invariants

1. Every protected relationship operation is subject-bound.
2. Every protected relationship operation crosses the canonical authorization decision boundary.
3. Capability ID and capability version remain coupled.
4. Principal and subject remain distinct.
5. Relationship does not imply causation.
6. Provenance is preserved.
7. Uncertainty is preserved.
8. Unknown temporal information remains unknown.
9. Relationship storage does not create a competing Health State model.
10. Relationship infrastructure remains provider-neutral and reusable across SOMA surfaces.

## Implementation gate

Before implementation:

- review this contract against the existing longitudinal observation repository;
- review existing evidence/epistemic contracts for overlap;
- identify any existing relationship semantics and consolidate rather than duplicate;
- define canonical capability IDs for relationship operations;
- add mechanical authorization and registry invariants;
- only then implement the minimum persistence/provider changes.

## Ecosystem intent

This contract is infrastructure, not a separate application feature. Health, wellness, nutrition, research, AI/agents, device adapters, web, mobile, and future SOMA surfaces should consume the same governed relationship substrate.
