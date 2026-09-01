# SOMA Capability Kernel

**Status:** Foundational contract
**Version:** 0.1.0

## Purpose

The Capability Kernel defines the shared vocabulary and lifecycle for
reusable SOMA capabilities. Application surfaces must consume capabilities
through this shared contract rather than implementing competing business,
safety, privacy, evidence, or provenance logic.

## Capability identity

Every capability has:

- a stable identifier;
- a semantic version;
- a lifecycle status;
- a maturity level;
- declared policy requirements;
- declared dependencies;
- supported adapters;
- supported surfaces;
- explicit failure behavior;
- automated tests.

## Lifecycle

```text
Proposed
  ↓
Contracted
  ↓
Implemented
  ↓
Tested
  ↓
Validated
  ↓
Production
```

A capability must not be represented as Production merely because its code
exists. Security-sensitive capabilities require additional review and tests.

## Shared-first rule

If a capability is reusable across two or more surfaces, its contract and
core behavior belong in shared infrastructure before surface integration,
unless a documented exception exists.

## Adapter rule

Adapters translate between a shared capability contract and an external
transport, provider, protocol, storage network, identity system, or AI
provider.

Adapters must not redefine the shared policy boundary.

## Failure rule

Optional capability failure must not disable unrelated capabilities.
Protected operations fail closed. Optional discovery and transport features
may degrade gracefully when the policy permits it.

## User-choice rule

Optional capabilities, including AI and decentralized capabilities, remain
user-controlled where user choice is relevant. SOMA core must remain usable
without a wallet, blockchain, decentralized identity, or AI provider.

## Health-information rule

Health-related capabilities must preserve the distinction between education,
management support, and clinical treatment. Evidence provenance, uncertainty,
limitations, safety information, and professional-review requirements belong
to shared infrastructure.

## Required review questions

Before integration, answer:

1. What reusable capability is being added?
2. What contract exposes it?
3. Which shared policies govern it?
4. Which data does it require?
5. Which adapters consume it?
6. Which surfaces consume it?
7. What happens when an external provider fails?
8. Can users operate without it where appropriate?
9. What tests prove the important success and failure states?
10. Does the implementation exceed any documented security or evidence claim?
