# SOMA-OS Current Operating Baseline — 2026-09-12

**Status:** Current operating baseline  
**Authority:** Governance navigation and execution context; canonical technical semantics remain in schemas and architecture contracts.  
**Audience:** Human owner, developers, reviewers, and AI agents.

## 1. Product identity

SOMA-OS is one health and wellbeing ecosystem built on shared infrastructure. It is not a collection of separate applications that independently implement the same capabilities.

Web, mobile, Telegram, WhatsApp, Messenger, desktop, AI, agents, MCP, Nostr, Web3/DID/VC, content-addressed storage, and future interfaces are adapters, surfaces, or optional capabilities around the shared SOMA foundation.

## 2. Shared-first rule

Every new capability must be evaluated and, where reusable, built through:

```text
Shared capability
  → shared contract
  → shared policy/security
  → shared data/evidence semantics
  → shared execution
  → provenance/audit
  → adapters/surfaces
```

Do not create surface-specific copies of authorization, privacy, evidence, safety, provenance, or health-domain logic.

## 3. Canonical security architecture

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

The Policy Kernel is the policy evaluator. The Authorization + Policy Decision Boundary composes existing identity/scope enforcement and policy evaluation into an authoritative decision contract. It is not a second policy engine.

AI, agents, MCP, transports, and external adapters may request capabilities but cannot self-authorize, widen scope, change the principal, or replace the canonical decision.

## 4. Canonical health/evidence architecture

```text
Observation
  ≠ Claim
  ≠ Evidence
  ≠ Interpretation
  ≠ Recommendation
```

The Health State Model is the canonical personal-health semantic model. The Health Evidence Graph is the canonical general evidence model. Longitudinal, behavioral, Food–Life, and other domain-specific records must consume or extend these authorities without creating competing health-state or evidence stores.

SOMA preserves provenance, uncertainty, contradiction, applicability, safety, and temporal context. Association is not silently promoted to causation.

## 5. Current implementation foundation

The repository already contains shared contracts and implementations for identity context, identity authorization enforcement, Policy Kernel, protected-data access, capability registry, governed capability operation, Health State, Health Evidence Graph, PHR repository, Local Health Vault, and longitudinal observation/timeline infrastructure.

These components are progressively being integrated and hardened; their presence in the repository is not itself proof of production readiness.

## 6. Authorization workstream

The Authorization + Policy Decision Boundary v1 architecture and decision envelope schema have been created on the dedicated authorization branch/PR. Runtime integration is intentionally deferred until the contract and existing Identity Authorization Enforcement, Policy Kernel, Governed Operation, and ProtectedDataAccess responsibilities are reconciled and validated together.

The decision vocabulary is:

```text
ALLOW
DENY
REQUIRE_CONSENT
REQUIRE_HUMAN_REVIEW
DEGRADE
```

A non-ALLOW result must never be silently converted into authorization by a downstream adapter or operation.

## 7. Documentation architecture

The repository documentation hierarchy is:

```text
Schemas + executable tests
        ↓
Architecture contracts
        ↓
Decisions / ADRs
        ↓
Product contracts
        ↓
Implementation documentation + source
        ↓
Product readiness
        ↓
Research / evidence sources
        ↓
Project memory / archive
```

Before creating documentation, consult `docs/governance/documentation-authority-registry-v1.md` and the content/organization audit.

## 8. CI truthfulness

GitHub Actions has repeatedly recorded PR #82 and PR #83 jobs as failures without assigning a runner, executing steps, or producing usable logs. Such a result is classified as a **CI execution/infrastructure failure**, not a code-test failure.

Do not weaken workflows, remove required checks, force merge, or report these runs as successful.

The repository workflow definitions use GitHub-hosted `ubuntu-latest` runners and contain real executable steps. The remaining CI blocker is runner/account execution availability and must be resolved independently of application architecture.

## 9. Current priority order

### P0 — Trustworthy CI

1. Verify Actions-specific usage/quota and account-level execution state.
2. Restore genuine runner execution.
3. Obtain real job steps, logs, and test results.

### P1 — Shared security foundation

1. Validate Capability Registry PR #80.
2. Validate Documentation Governance PR #82.
3. Validate Authorization Boundary PR #83.
4. Implement the smallest runtime authorization composition layer only after contract validation.

### P2 — First end-to-end SOMA vertical

After the foundation is trustworthy, build one complete health/evidence journey using the shared infrastructure rather than expanding the number of architectural subsystems.

### Deferred

Broad autonomous agents, Tool Gateway, prescribing/medication modification, universal causal inference, full evidence-to-action automation, mandatory decentralized protocols, and multiple separate SOMA applications remain deferred until justified by a validated product requirement.

## 10. Merge discipline

`main` is the canonical integration baseline.

Use:

```text
Audit
  → Compare with current main
  → Contract review
  → Security review
  → CI execution
  → Final diff review
  → Normal merge
  → Verify main
```

Never force a merge merely because a branch contains useful work.

## 11. Definition of done

A change is complete only when applicable gates are satisfied:

```text
Architecture
+ Contract
+ Implementation
+ Authorization
+ Security
+ Privacy
+ Failure semantics
+ Evidence/provenance
+ Auditability
+ Tests
+ CI
+ Documentation
+ Deployment implications
```

Speculative future architecture does not automatically require implementation.

## 12. Immediate operating instruction

Until CI execution is restored, prioritize documentation accuracy, contract review, and non-mutating audits. Do not add unrelated architecture or product features merely to fill the CI waiting period.

Once CI is trustworthy, move from architecture accumulation to **validate → integrate → prove → merge → build the first complete SOMA vertical**.
