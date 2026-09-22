# SOMA Shared Infrastructure Convergence Gate v1

**Status:** Active governance baseline  
**Scope:** SOMA-OS shared infrastructure and ecosystem surfaces

## Architectural rule

Architecture determines the file boundary. File size triggers review; it does not determine the boundary.

Split code only when separation creates an independent boundary of:
- change;
- trust/security;
- persistence;
- provider dependency;
- independent reuse.

Keep tightly coupled responsibilities together when splitting would:
- weaken invariants;
- duplicate orchestration;
- duplicate authorization;
- obscure lifecycle semantics;
- create competing sources of truth.

## Canonical shared execution path

```text
Principal / Actor
  -> Identity
  -> Subject
  -> Capability + Version
  -> Resource + Action
  -> Tenant / Data Domain
  -> Canonical Authorization Decision
  -> Governed Operation
  -> Repository Contract
  -> Provider / Persistence Adapter
  -> Protected Execution
```

No surface, provider, AI model, or agent may create a parallel authorization path.

## Canonical semantic path

```text
Observation
  -> Relationship / Hypothesis
  -> Evidence
  -> Safety / Uncertainty
  -> Causality where justified
  -> Intervention
  -> Response
  -> Outcome
  -> New Observation
```

A relationship is not causation. An outcome does not by itself establish efficacy. AI output is not automatically an observation.

## Ecosystem rule

Web, mobile, device, bot, practitioner, research, AI, and agent surfaces are adapters over shared SOMA capabilities. A surface failure must not become a shared-infrastructure failure.

## Permission lifecycle

Sensitive capabilities such as camera, microphone, location, contacts, or device access are purpose-bound. Access must be granted only for the declared operation, released/disabled when that purpose ends where technically applicable, and require renewed authorization when later needed.

## Repository hygiene

Exactly nine branches are persistent architecture lanes:

1. `main`
2. `development`
3. `security/current`
4. `architecture/current`
5. `feature/current`
6. `health/current`
7. `research/current`
8. `ai-agent/current`
9. `integration/current`

Historical branches are not merged merely to reduce branch count. Unique work must first be classified as:
- already canonical;
- extractable into a canonical lane;
- historical provenance only;
- redundant.

Deletion occurs only after that evidence is preserved and no open dependency remains.

## Implementation gate

Before adding a capability:

1. Define/reuse the canonical domain contract.
2. Reuse the shared capability registry.
3. Reuse canonical authorization.
4. Route protected data through the protected-data boundary.
5. Route executable work through the governed operation boundary.
6. Preserve provenance, safety, uncertainty, and audit where applicable.
7. Keep provider-specific behavior behind provider adapters.
8. Keep AI/agents as clients of governed capabilities, never security authorities.
9. Split code only where the architectural boundary is independently meaningful.
10. Verify exact-head CI before integration.

## Current repository constraint

The GitHub integration used for this governance pass does not expose an authorized remote branch-deletion mutation. Therefore branch retirement must not be simulated by force-moving refs. The target remains exactly nine physical branches; deletion must be performed through an authorized GitHub operation when available.

