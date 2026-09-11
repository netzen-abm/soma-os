# SOMA-OS Ecosystem Principles

**Date:** 2026-09-02  
**Status:** Strategic guidance  
**Authority:** Strategy guidance derived from the foundational architecture and governance contracts. Normative shared-infrastructure rules are owned by [`docs/architecture/shared-infrastructure-charter.md`](../architecture/shared-infrastructure-charter.md) and the canonical governance/architecture documents listed in [`docs/governance/documentation-authority-registry-v1.md`](../governance/documentation-authority-registry-v1.md).

> **Purpose:** Preserve the strategic principles that guide how the SOMA ecosystem should evolve without creating a competing architecture authority. Where this document conflicts with a canonical contract, schema, test, architecture document, or durable decision, the canonical authority prevails.

## 1. Shared infrastructure first

If a capability can be reused across multiple SOMA surfaces, its contract and core behavior should live in shared infrastructure first.

Applications should compose capabilities rather than duplicate them.

## 2. Thin surfaces

Web, mobile, professional, research, agent, MCP and other surfaces should remain replaceable clients/adapters around the SOMA core.

A failure in one surface must not become a failure of the underlying capability or another surface.

## 3. Central policy, distributed execution

The policy boundary is shared. Execution may occur in different services, devices or providers.

```text
Request
  -> Identity
  -> Capability
  -> Policy
  -> Gateway
  -> Execution
  -> Evidence
  -> Audit
```

## 4. AI is optional

AI is a capability and implementation option, not the authority layer and not the product identity.

Users should not be forced to use AI where a deterministic, local or human workflow is sufficient.

## 5. Evidence is first-class infrastructure

Evidence retrieval, provenance, classification, verification and uncertainty should be reusable capabilities rather than embedded separately in every application.

## 6. Safety is independent of intelligence

A more capable model does not automatically create a safer health system.

Safety policy must remain an independent governed layer capable of constraining any model, agent, service or application.

## 7. Privacy is architectural

Prefer local computation and storage. External data transfer must be explicit, minimized and protected.

Sensitive health data should not be transferred simply because a remote capability is convenient.

## 8. Adapters are replaceable

Provider-specific and protocol-specific implementations belong behind adapters.

The SOMA canonical model should not be redesigned around a single vendor or protocol.

## 9. Research and product must remain connected but distinct

Research infrastructure can inform product capabilities. Product telemetry must not automatically become research data.

Consent, governance, privacy and ethics determine whether and how real-world data can enter research workflows.

## 10. Build one complete proof before multiplying surfaces

The ecosystem should first demonstrate one reliable end-to-end health workflow.

Feature and protocol expansion follows validated utility.

## 11. Repository as institutional memory

The repository must preserve:

- architecture decisions;
- strategic decisions;
- market analysis;
- evidence-model decisions;
- security findings;
- workflow decisions;
- branch/cleanup rationale;
- product roadmap changes;
- rejected directions when their reasoning remains useful.

Future contributors should not need private chat history to understand why the system was built the way it was.

## 12. Governance rule

No implementation is considered complete merely because it compiles or passes a happy-path test.

Completion requires the relevant combination of:

- functional correctness;
- security review;
- evidence integrity;
- privacy review;
- safety behavior;
- operational observability;
- maintainability;
- provenance;
- regression coverage.

## Relationship to canonical architecture

This document is intentionally a strategic summary, not a second specification. Detailed invariants belong in the canonical architecture, schema, contract-test, security, product-readiness, and decision documents. This distinction allows the strategy to remain readable while keeping one authoritative definition for each system rule.
