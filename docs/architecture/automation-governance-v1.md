# SOMA Safe Automation Governance v1

**Status:** Design baseline
**Scope:** Engineering, security, data, CI/CD, product-readiness and operational automation

## 1. Purpose

Automation is an architectural capability of SOMA, not merely a convenience feature. It may reduce repeated human work, continuously verify invariants, detect drift, and surface evidence for decisions.

Automation must not silently become an authority that bypasses policy, identity, consent, human review, security controls, or release governance.

## 2. Core rule

> **Automate verification and bounded execution; do not automate away accountability.**

Every automation must have an explicit purpose, owner, scope, trigger, inputs, outputs, failure behavior, authorization boundary, auditability requirement, and stop condition.

## 3. Automation classes

### A. Observe

Read-only monitoring and evidence collection.

Examples:
- CI/workflow status monitoring
- dependency and vulnerability discovery
- architecture-drift detection
- direct database-access/bypass scanning
- documentation drift detection
- stale branch/PR detection

Default authority: read-only.

### B. Verify

Automated checks that evaluate explicit invariants.

Examples:
- security gates
- migration safety checks
- schema validation
- required-status verification
- product-readiness reconciliation
- release-gate verification

Default authority: may block progression, but must not silently alter protected state.

### C. Remediate — bounded

Automated changes are permitted only where the remediation is deterministic, reversible, low-risk, and within a pre-declared scope.

Examples:
- formatting fixes on dedicated branches
- generated documentation/index refreshes
- safe metadata reconciliation
- opening a follow-up issue for a detected violation

Default rule: branch/PR first; never direct production mutation unless separately approved by policy.

### D. Execute — controlled

Automation may execute operational actions only when an explicit policy authorizes them and the action is idempotent, auditable, bounded, and reversible where practical.

Examples may include controlled deployment, rollback, scheduled maintenance, or data lifecycle operations after their specific contracts are defined.

## 4. Safety invariants

1. **Identity-bound:** automation executes as an explicitly identified principal/service identity.
2. **Capability-bound:** automation can invoke only registered capabilities.
3. **Policy-bound:** Policy Kernel authorization remains authoritative.
4. **Scope-bound:** tenant/data-domain/resource scope is explicit; no inference is permitted.
5. **Fail-closed:** ambiguous authorization, scope, provenance, or safety state stops the action.
6. **Least privilege:** automation receives the minimum capability and scope required.
7. **Idempotent:** retries must not unintentionally duplicate protected effects.
8. **Auditable:** decisions and material actions have structured audit evidence without sensitive-data leakage.
9. **Reversible:** destructive or high-impact actions require an explicit recovery path and stronger approval.
10. **Independent failure:** an automation failure must not disable unrelated SOMA capabilities.
11. **Human accountability:** high-impact actions require explicit human approval unless a separately reviewed policy says otherwise.
12. **No silent escalation:** automation may never broaden its own capability, identity, tenant, data-domain, or approval scope.
13. **No hidden authority:** monitoring must not be treated as authorization merely because it produced a positive result.
14. **Versioned behavior:** automation contracts and policy dependencies are versioned and reviewable.
15. **Kill switch:** every recurring or long-running automation must have a documented disable path.

## 5. Automation lifecycle

```text
Propose
  ↓
Classify risk + authority
  ↓
Define trigger / scope / owner / failure behavior
  ↓
Implement behind shared capability + policy boundaries
  ↓
Test positive + negative + failure paths
  ↓
Observe in non-destructive mode
  ↓
Enable bounded execution if justified
  ↓
Continuously verify drift
  ↓
Retire when obsolete
```

## 6. Risk tiers

### Tier 0 — Informational

Read-only reporting. No state mutation and no blocking authority.

### Tier 1 — Guardrail

May fail a CI/release gate or open an issue, but cannot mutate protected application/data state.

### Tier 2 — Bounded remediation

May create commits/branches/PRs or perform narrowly scoped reversible maintenance. Requires explicit service identity and audit trail.

### Tier 3 — High-impact execution

Deployment, rollback, protected-data mutation, access changes, destructive operations, or actions affecting users require explicit authorization and, where appropriate, human approval. Tier 3 automation is not enabled by default.

## 7. Initial SOMA automation portfolio

| Automation | Class | Default authority | Safe to automate now? |
|---|---|---|---|
| PR/CI gate watcher | Observe/Verify | Read-only + notify | Yes |
| Architecture drift detector | Observe/Verify | Read-only + gate | Yes |
| Protected-data bypass scanner | Observe/Verify | Read-only + gate | Yes |
| Migration safety checker | Verify | Gate | Yes |
| Documentation drift checker | Observe/Verify | Read-only + PR suggestion | Yes |
| Product-readiness reconciliation | Verify | Evidence-based checklist update | Yes, with strict evidence rules |
| Dependency/security audit | Observe/Verify | Read-only + gate | Yes |
| Stale PR/branch watcher | Observe | Read-only + notify | Yes |
| Release readiness gate | Verify | Gate; no autonomous release | Yes |
| Autonomous merge | Execute | Write/merge | **No** |
| Autonomous authorization changes | Execute | Security authority | **No** |
| Autonomous protected-data mutation | Execute | Data authority | **No** |
| Autonomous destructive migration | Execute | Destructive authority | **No** |

## 8. Required shared infrastructure

Before automation becomes a broad platform capability, SOMA should provide:

- canonical automation identity/service principal model;
- capability registration for automation actions;
- Policy Kernel enforcement at invocation boundaries;
- tenant/data-domain/resource scoping;
- structured audit event contract;
- correlation/request IDs;
- idempotency keys for material operations;
- approval/human-review contract for high-impact actions;
- execution state and retry/cancellation semantics;
- secret/configuration management without committed credentials;
- automation health/readiness state;
- kill-switch/disable semantics;
- versioned automation contract;
- failure isolation and backoff.

## 9. Architecture placement

Automation is a cross-cutting infrastructure layer, not a replacement for the canonical domain runtime.

```text
Identity
   ↓
Capability
   ↓
Policy Kernel
   ↓
Automation Control / Scheduler
   ↓
Gateway / Execution Boundary
   ↓
Canonical Domain Runtime / Adapters
   ↓
Evidence + Audit
```

Schedulers, GitHub Actions, workers, cron jobs, agents, or external orchestration systems are execution mechanisms. They are not independent policy authorities.

## 10. Immediate implementation rule

For the current SOMA phase, automate **evidence-producing governance first**. Do not introduce autonomous production mutations while protected-data enforcement, authentication/session trust, service identity, consent/revocation, audit, and recovery contracts remain incomplete.

The existing PR #33 CI monitor is consistent with this model because it observes required gates and notifies only after the required condition is satisfied. It does not merge the PR or alter protected repository state.

## 11. Definition of done for an automation capability

An automation is product-ready only when:

- its purpose and risk tier are documented;
- identity and capability are explicit;
- policy enforcement is explicit;
- scope is explicit;
- positive and negative paths are tested;
- failure and retry behavior are defined;
- audit requirements are defined;
- secrets are externalized;
- a disable/kill path exists;
- documentation exists;
- operational ownership exists;
- deployment/recovery implications are checked.

## 12. Non-negotiable decision

SOMA should become **automation-native but not automation-dependent**: automation strengthens governance and execution, while the underlying product remains correct, secure, and usable when automation is unavailable.
