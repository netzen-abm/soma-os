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

The canonical Authorization + Policy Decision Boundary is now implemented and integrated across the currently verified protected-data paths.

The completed convergence sequence is:

1. Canonical authorization decision envelope and boundary contract.
2. Canonical authorization decision composition.
3. ProtectedDataAccess integration.
4. Rust canonical authorization boundary.
5. Local Health Vault authorization adapter.

The Local Health Vault adapter is a translation boundary, not a second policy engine. It preserves the existing identity-scope precondition and permits protected vault execution only when the canonical decision is explicitly `ALLOW`. `DENY`, `REQUIRE_CONSENT`, `REQUIRE_HUMAN_REVIEW`, and `DEGRADE` are not silently converted into authorization.

The legacy `VaultAuthorizer` interface remains temporarily as a compatibility boundary. Its removal is deferred until an actual production vault provider/runtime wiring exists and can be verified. The remaining authorization audit must therefore distinguish test-only compatibility implementations from real production execution paths.

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

The repository workflow definitions use GitHub-hosted `ubuntu-latest` runners and contain real executable steps. The previously observed runner/execution blocker is not evidence that application architecture should be changed to accommodate CI anomalies.

For each merge, verify the exact PR head and all applicable executable jobs before merging. Jobs with no executable steps must not be treated as successful test evidence.

## 9. Current priority order

### P0 — Trustworthy CI

1. Continue verifying exact-head CI execution for every change.
2. Preserve real runner execution and truthful interpretation of workflow results.
3. Investigate account/runner execution anomalies independently when they occur.

### P1 — Shared security foundation

1. Re-audit remaining protected-data and authorization paths against the canonical decision boundary.
2. Reconcile compatibility authorization interfaces with real production wiring before removing them.
3. Verify authorization scope/resource intersection and fail-closed behavior across protected operations.

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

With the current authorization convergence now merged, the next work is a **non-mutating authorization/protected-data audit of `main`**. Identify every remaining authorization boundary, direct protected-data access path, compatibility authorizer, and production wiring point; classify each as canonical, adapter, test-only, compatibility, or unresolved.

Do not remove compatibility interfaces or introduce Tool Gateway, broad agent infrastructure, or unrelated product features until this audit establishes that the current protected execution paths converge on the canonical authorization boundary.

Once that audit is complete, move from architecture accumulation to **validate → integrate → prove → merge → build the first complete SOMA vertical**.
