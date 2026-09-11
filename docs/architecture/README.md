# SOMA-OS Architecture & Contracts

This directory contains canonical architecture contracts and implementation-boundary specifications. It is the technical source of truth for how the single SOMA ecosystem is composed.

Before creating a new architecture document, consult `../governance/documentation-authority-registry-v1.md` and `../governance/documentation-content-and-organization-audit-2026-09-12.md`.

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
Canonical Health State
  ↓
Health Evidence Graph
  ↓
Explicit Health-State ↔ Evidence linkage
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
| Whole-system baseline | `SOMA-SYSTEM-ARCHITECTURE-PRODUCT-SPEC-v1.md` | Whole-system architecture; does not imply production readiness |
| Shared-first architecture | `shared-infrastructure-charter.md` | Reusable capability belongs in shared infrastructure |
| Domain runtime | `canonical-domain-runtime-polyglot-v1.md` | Canonical/reference runtime and justified polyglot implementation |
| Authorization boundary | `authorization-policy-decision-boundary-v1.md` | Canonical authorization composition boundary; does not replace Policy Kernel |
| Identity authorization | `identity-authorization-enforcement-implementation-v1.md` and related identity contracts | Identity/scope enforcement boundary |
| Policy evaluation | `../services/shared/policy_kernel.md` | Current Policy Kernel implementation contract; v0.4 |
| Governed operations | `canonical-governed-capability-operation-v1.md` | Operation lifecycle and authority gates |
| Capability adapters | `capability-adapter-contract.md` | Adapter boundary |
| Health state | `health-state-model-v1.md` | Canonical personal-health semantic model |
| Health evidence | `health-evidence-graph-v1.md` | Canonical general evidence semantic model |
| Health-state/evidence boundary | `health-state-evidence-contracts-v1.md` | Explicit separation and linkage |
| Longitudinal reasoning | `longitudinal-evidence-driven-health-intelligence-v1.md` | Future health-intelligence architecture baseline |
| Longitudinal observations | `longitudinal-observation-timeline-v1.md` | Projection/timeline boundary; Health State remains canonical |
| Personal health repository | `personal-health-record-repository-v1.md` | Vault/index/reference boundary |
| Behavioral/epistemic layer | `behavioral-and-epistemic-evidence-layer-v1.md` | Extension; not a competing health-state model |
| Food–Life evidence | `food-life-evidence-model.md` | Shared Food–Life evidence/domain model |
| Cross-paradigm evidence | `cross-paradigm-evidence-methodology-v1.md` | Claim-level evidence assessment across knowledge systems |
| Epistemic context | `epistemic-context-mapping.md` | Native knowledge-system/context mapping before comparison |
| Protected data | `protected-data-access-implementation-v1.md` and enforcement specifications | Persistence/access boundary |
| Trusted DB identity | `trusted-db-service-identity-context-v1.md` and implementation specification | Database trust boundary |

## Canonical security chain

```text
IdentityContext
  → Identity Authorization Enforcement
  → Authorization + Policy Decision Boundary
  → Policy Kernel
  → Governed Capability Operation / ProtectedDataAccess
  → Trusted Persistence Identity
  → Trusted Transaction Context
  → PostgreSQL RLS / constraints
  → Protected Data
  → Provenance / Audit
```

No application surface, AI model, agent, protocol, or provider may bypass this chain.

## Canonical health/evidence chain

```text
Observation / imported record
  → provenance + context + uncertainty
  → canonical Health State
  → explicit evidence linkage when applicable
  → Health Evidence Graph
  → bounded interpretation
  → human decision
  → outcome
  → new observation
```

`Claim != Evidence != Interpretation != Recommendation`.

## Documentation discipline

Architecture documents define authority boundaries; they do not automatically authorize implementation. Prefer extending an existing canonical contract over creating another summary. Create a new architecture document only when the concept has a genuinely distinct authority boundary, lifecycle, or contract.

Do not use this README as a second architecture specification. It is an index and routing document.
