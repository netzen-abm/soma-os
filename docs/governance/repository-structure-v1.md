# SOMA-OS Repository Structure v1

## Authority

This document defines repository organization. Architecture contracts remain authoritative for system behavior; this document only defines where artifacts belong.

## Core rule

SOMA-OS is one health, wellness, nutrition, research, and governed-intelligence ecosystem. Shared foundations are implemented once and consumed by surfaces.

## Canonical top-level layout

- apps/ — client and surface applications
- services/ — shared and domain service implementations
- database/ — schemas and migrations
- health-vault/ — vault-specific integration boundary
- protocols/ — protocol and transport adapters
- legal-shields/ — legal/compliance boundary artifacts
- ops/ — deployment and operational configuration
- scripts/ — audits, validators, migration and maintenance tools
- schemas/ — machine-readable canonical contracts
- docs/ — human, AI, and developer documentation
- public/ — public static assets
- .github/ — CI/CD and repository automation

## Service organization

- services/shared/ — cross-domain canonical primitives
- services/backend-rust/ — Rust execution/runtime boundary
- services/evidence-research/ — research and evidence capabilities
- services/ai-engine/ — AI capability adapters
- services/ai-prompts/ — prompt assets
- services/protocols/ — protocol implementations

services/shared/ owns reusable policy, authorization, capability, evidence, identity, and protected-data contracts. Application surfaces must not duplicate these foundations.

## Documentation organization

- docs/architecture/ — architecture and contracts
- docs/decisions/ — durable architectural decisions
- docs/governance/ — repository and operating rules
- docs/strategy/ — ecosystem and product strategy
- docs/research/ — research contribution and method
- docs/evidence-sources/ — source assessment and evidence governance
- docs/product/ — product specifications
- docs/ui-ux/ — interface specifications
- docs/operating-model/ — operating workflows
- docs/archive/ — superseded material with provenance

Use the documentation authority registry before creating a new document.

## Source-file placement rules

- Rust source belongs under Rust source directories; Markdown does not.
- Python source belongs with its service or in scripts/ for repository audits and maintenance.
- Schemas belong in schemas/.
- SQL migrations belong in database/migrations/.
- Tests remain adjacent to the implementation they validate unless the runtime requires a dedicated test tree.
- Historical artifacts move to docs/archive/ or an explicitly documented archive boundary before deletion.
- Generated or deployment-only artifacts must not become canonical source.

## Change discipline

Before moving or deleting an artifact:

1. Read its complete content.
2. Identify its canonical owner.
3. Search for references.
4. Preserve unique information.
5. Move only when the destination is semantically correct.
6. Verify the resulting tree and references.

## Shared-infrastructure rule

A reusable capability must have one canonical contract and one canonical implementation boundary. Surfaces may provide adapters, presentation, or transport-specific behavior, but may not create competing foundations.
