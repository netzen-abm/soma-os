# SOMA Shared Infrastructure Responsibility Boundaries v1

## Purpose

This document defines the decomposition gate for SOMA's shared health, wellness, nutrition, research, evidence, AI, and persistence infrastructure.

SOMA is one ecosystem. Reusable capability is implemented once in shared infrastructure and exposed through adapters/surfaces. Module count is not an architectural objective.

## Decomposition rule

Split a responsibility only when the split creates an independent architectural boundary of:

- change
- trust
- persistence
- provider dependency
- independent reuse

Do not split solely because a file, class, or module is large.

Keep tightly coupled responsibilities together when splitting them would:

- weaken a security or domain invariant
- duplicate orchestration
- create competing state
- require unnecessary translation
- introduce a second policy engine
- fragment a transaction/security boundary

## Canonical responsibility layers

Identity / Principal
→ Identity Authorization Enforcement
→ Authorization + Policy Decision Boundary
→ Policy Kernel
→ Governed Capability Operation
→ Protected Data Access
→ Trusted Persistence Context
→ Storage Provider

Health domain:
Observation
→ Health State
↔ Evidence Relationship
→ Interpretation
→ Action / Intervention
→ Outcome
→ New Observation

Evidence, provenance, uncertainty, safety, applicability, contradiction, and transformation lineage must survive downstream processing.

## Required boundaries

### Authorization
The Policy Kernel is the sole policy evaluator.

The Authorization + Policy Decision Boundary composes the request and authoritative policy decision. It must not become a second policy engine.

### Persistence
Trusted persistence context is established only after authorization. Storage adapters must not independently grant access.

### Domain contracts
Provider-neutral health/research contracts remain separate from provider implementations where independent providers are a real requirement.

### Adapters
External systems, models, agents, protocols, and surfaces are adapters. They may translate transport/provider semantics but must not redefine SOMA's canonical policy or domain contracts.

## Current implementation boundaries

The following are intentional architectural boundaries and should not be split further merely for file-size reasons:

- canonical authorization contract/boundary
- vault authorization adapter
- trusted protected database context
- provider-neutral longitudinal observation repository
- local longitudinal observation repository
- policy kernel
- governed capability execution
- protected-data access
- health state / evidence relationship contracts

## Security gate

Every new protected-data path must preserve:

Authorization
→ Governed operation
→ ProtectedDataAccess
→ Trusted persistence identity/context
→ storage provider.

Ambiguous authorization state fails closed.

AI and agents are never security authorities.

## Ecosystem gate

A new health, wellness, nutrition, research, AI, or agent capability must first determine whether the capability belongs in shared infrastructure. New product surfaces are adapters over shared capability; duplicated domain/security infrastructure is prohibited unless an explicit architectural decision establishes a genuinely independent trust or persistence boundary.
