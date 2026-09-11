# SOMA-OS Documentation

This directory is the canonical documentation home for SOMA-OS.

SOMA-OS is one ecosystem built on shared infrastructure. Documentation follows the same rule: each concept should have one canonical home, with other documents linking to it rather than redefining it.

## Start here

1. [`architecture/README.md`](architecture/README.md) — canonical architecture and contract map.
2. [`governance/README.md`](governance/README.md) — repository, documentation, AI-agent, and developer operating rules.
3. [`decisions/README.md`](decisions/README.md) — architectural decisions and durable choices.
4. [`strategy/README.md`](strategy/README.md) — product and ecosystem direction.
5. [`research/README.md`](research/README.md) — research-oriented material and research contributions.
6. [`evidence-sources/README.md`](evidence-sources/README.md) — source assessments and evidence-specific material.
7. [`product/README.md`](product/README.md) — product specifications and operational product contracts.
8. [`ui-ux/README.md`](ui-ux/README.md) — interface and experience specifications.
9. [`archive/README.md`](archive/README.md) — historical material retained for traceability.

## Canonical documentation rules

- **One concept, one canonical document.**
- Link to canonical documents instead of copying their definitions.
- A schema is authoritative for machine-readable data semantics when a schema exists.
- An architecture contract is authoritative for system-level semantics.
- An ADR/decision record is authoritative for a durable decision and its rationale.
- Project memory is historical/consolidated context, not a substitute for current contracts.
- Product-readiness tracking is the authoritative readiness status; architecture documents must not claim production readiness merely because they describe a design.
- Historical or superseded material must be archived before deletion.
- When a document is superseded, preserve the historical record and add a clear pointer to the replacement.

## Document lifecycle

```text
Idea / discussion
      ↓
Research / evidence
      ↓
Decision or architecture proposal
      ↓
Canonical contract
      ↓
Implementation
      ↓
Validation / CI
      ↓
Product-readiness status
      ↓
Superseded → archive with provenance
```

## Important distinction

Documentation describes the system; it does not grant implementation authority. A future architecture baseline must not be interpreted as permission to implement every capability it mentions.
