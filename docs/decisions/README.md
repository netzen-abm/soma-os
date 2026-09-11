# SOMA-OS Decisions

Durable architectural and product decisions live here. Decision records preserve rationale, alternatives, consequences, and status.

## Current decision records

- `2026-09-01-openclaw-mcp-agent-platform.md`
- `2026-09-02-health-intelligence-strategy.md`
- `2026-09-02-soma-os-scope-and-separation.md`
- `2026-09-02-soma-os-strategic-review.md`
- `2026-09-04-policy-kernel-resource-scope-v1.md`
- `2026-09-08-health-context-and-product-direction.md`
- `ADR-001-canonical-domain-runtime-polyglot-implementation.md`
- `ADR-003-trusted-db-service-identity-context.md`
- `ADR-004-optional-user-identity.md`

## Rule

Decision records are not substitutes for current architecture contracts. Once a decision has been implemented, the canonical architecture/contract document should describe the resulting system, while the decision record preserves why the choice was made.

Do not create a new ADR for an implementation detail that is already governed by an existing contract. Create one when a durable architectural choice or trade-off needs an explicit record.
