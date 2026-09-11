# SOMA-OS Architecture & Contracts

This directory contains canonical architecture contracts and implementation specifications. It is the technical source of truth for how SOMA is composed.

## Canonical architecture hierarchy

```text
SOMA ecosystem
  ↓
Shared infrastructure
  ↓
Identity / Authorization / Policy
  ↓
Capability contracts
  ↓
Protected data + Health Vault
  ↓
Health State
  ↓
Evidence Graph
  ↓
Evidence ↔ Health-State linkage
  ↓
Governed Operations
  ↓
Intelligence / interpretation
  ↓
Adapters / protocols / surfaces
```

## Core canonical documents

| Concern | Canonical document | Rule |
|---|---|---|
| Whole-system baseline | `SOMA-SYSTEM-ARCHITECTURE-PRODUCT-SPEC-v1.md` | Overall architecture baseline; does not imply implementation completeness |
| Shared-first architecture | `shared-infrastructure-charter.md` | No duplicated reusable capability in surfaces |
| Health state | `health-state-model-v1.md` | Canonical personal-health semantic model |
| Health evidence | `health-evidence-graph-v1.md` | Canonical general evidence semantic model |
| Health-state/evidence boundary | `health-state-evidence-contracts-v1.md` | Explicit separation and linkage |
| Longitudinal reasoning | `longitudinal-evidence-driven-health-intelligence-v1.md` | Future intelligence architecture baseline |
| Longitudinal observations | `longitudinal-observation-timeline-v1.md` | Timeline/projection contract; Health State remains canonical |
| Personal health repository | `personal-health-record-repository-v1.md` | Vault/index/reference boundary |
| Governed operations | `canonical-governed-capability-operation-v1.md` | Canonical operation lifecycle and authority gates |
| Capability adapters | `capability-adapter-contract.md` | Adapter boundary |
| Cross-paradigm evidence | `cross-paradigm-evidence-methodology-v1.md` | Claim-level epistemic neutrality and evidence assessment |
| Behavioral/epistemic layer | `behavioral-and-epistemic-evidence-layer-v1.md` | Behavioral observations and epistemic metadata as an extension, not a competing domain model |
| Domain runtime | `canonical-domain-runtime-polyglot-v1.md` | Canonical domain/reference runtime and justified polyglot implementation |
| Security | security/identity/protected-data documents | Security contracts and implementation boundaries |

## Security chain

```text
IdentityContext
  → Identity Authorization Enforcement
  → Policy Kernel
  → ProtectedDataAccess
  → Trusted Persistence Identity
  → Trusted Transaction Context
  → PostgreSQL RLS / constraints
  → Protected Data
```

No application surface, AI model, agent, protocol, or provider may bypass this chain.

## Health information chain

```text
Observation / imported record
  → provenance + context + uncertainty
  → canonical Health State
  → evidence linkage when applicable
  → Health Evidence Graph
  → governed interpretation
  → bounded output
  → human decision
  → outcome
  → new observation
```

`Claim != Evidence != Interpretation != Recommendation`.

## Behavioral and epistemic extension

Behavioral and psychological observations may be represented through the behavioral contract, while evidence about behavioral/psychological claims is represented through the existing Evidence Graph and the epistemic passport. This does **not** create a separate behavioral-health application, therapy engine, or competing health-state model.

## Documentation discipline

Do not create a new architecture document merely because a new feature has appeared. First determine whether the concept already belongs in an existing canonical contract. Extend the canonical document when appropriate; create a new document only when it has a genuinely distinct authority boundary.
