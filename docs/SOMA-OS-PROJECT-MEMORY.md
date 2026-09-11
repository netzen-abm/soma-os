# SOMA-OS Project Memory

**Purpose:** Preserve important architectural, product, security, evidence, and engineering context so future humans, developers, and AI agents can continue SOMA-OS without depending on private chat history.

**Last consolidated:** 2026-09-12

> **Authority note:** This is consolidated historical context, not the highest authority for current technical semantics. Consult schemas/executable tests, `docs/architecture/`, `docs/decisions/`, and `docs/PRODUCT-READINESS-MASTER-CHECKLIST.md` for current authoritative state.

## 1. Product identity

SOMA-OS is a privacy-first health and wellbeing infrastructure project. It is **one ecosystem**, not four separate applications.

Web, mobile, Telegram, WhatsApp, Messenger, desktop, AI, agents, MCP, Nostr, Web3/DID/VC, content-addressed storage, research providers, wearables, labs, and future interfaces are adapters, surfaces, or optional capabilities consuming the same shared foundation.

SOMA is distinct from Janavani. Janavani is a separate civic-infrastructure project. Do not combine their product roadmaps, terminology, repositories, or requirements.

## 2. Shared-first architecture rule

The governing architecture rule is:

```text
Shared capability
  → shared contract
  → shared policy/security
  → shared data/evidence semantics
  → shared execution
  → provenance/audit
  → adapters/surfaces
```

Do not duplicate health, evidence, privacy, safety, authorization, provenance, or policy logic in individual surfaces.

## 3. Canonical system/security spine

The broad SOMA capability spine is:

```text
Identity
  → Capability
  → Policy
  → Gateway
  → Execution
  → Evidence
  → Interpretation
  → Audit
```

Protected-data execution is more specifically:

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

The Policy Kernel is the policy evaluator. The Authorization + Policy Decision Boundary is the canonical composition/authority boundary around it. It must not become a second policy engine.

AI, agents, MCP, transports, and external adapters can request capabilities but cannot self-authorize, widen scope, change the principal without authorization, or replace the canonical decision.

## 4. Canonical health/evidence model

SOMA separates:

```text
Observation
  ≠ Claim
  ≠ Evidence
  ≠ Interpretation
  ≠ Recommendation
```

The Health State Model is the canonical personal-health semantic model. The Health Evidence Graph is the canonical general evidence model. Explicit links connect them without making one the source of truth for the other.

Longitudinal reasoning preserves effective/observed/recorded time, provenance, context, uncertainty, contradiction, evidence quality, applicability, and processing lineage.

Association is not silently promoted to causation.

## 5. Evidence and epistemic discipline

SOMA can investigate conventional, traditional, complementary, behavioral, psychological, lifestyle, mechanistic, emerging, and unconventional claims. Inclusion is not endorsement.

Operating rule:

> Investigate without prejudice. Conclude according to evidence. Preserve uncertainty where evidence is insufficient.

Lateral thinking expands hypotheses; it does not lower the evidence standard.

Research discovery is not diagnosis or treatment. Personal observations are not population evidence. Research claims are not personal facts.

## 6. Health safety and privacy

SOMA may support evidence discovery, education, synthesis, management information, research, policy analysis, and decision support where appropriate.

It must not silently turn informational evidence into diagnosis, individualized prescription, guaranteed treatment, cure claims, or instructions to discontinue prescribed treatment.

Privacy is architectural: prefer local-first processing/storage, explicit and minimized external transfer, appropriate encryption, authorization before protected access, and auditable provenance.

## 7. Core implemented foundation

The repository contains shared contracts/implementations for:

- IdentityContext;
- Identity Authorization Enforcement;
- Policy Kernel;
- ProtectedDataAccess;
- capability registry;
- Governed Capability Operation;
- Health State;
- Health Evidence Graph;
- PHR repository;
- Local Health Vault;
- longitudinal observation/timeline;
- evidence/research adapters and orchestration;
- cross-paradigm evidence methodology;
- epistemic context mapping.

Presence in the repository is not equivalent to production readiness.

## 8. Policy Kernel history and current state

The earlier resource-scope Policy Kernel v0.3 decision remains part of project history. The current implementation declares `POLICY_VERSION = "0.4.0"` and adds/validates capability principal types and identity requirements including authentication status, identity mode, assurance level, and durable identity.

The implementation documentation was reconciled to v0.4 on the documentation-governance branch so it no longer incorrectly presents v0.3 as the current implementation contract.

The v0.3 decision record remains historical rationale and must not be deleted merely because the implementation advanced.

## 9. Authorization + Policy Decision Boundary

A new architecture contract was established for the missing shared composition boundary:

```text
IdentityContext
  → Identity Authorization Enforcement
  → Authorization + Policy Decision Boundary
  → Policy Kernel
  → Canonical Authorization Decision
  → Governed Capability Operation / ProtectedDataAccess
```

The canonical decision vocabulary is:

```text
ALLOW
DENY
REQUIRE_CONSENT
REQUIRE_HUMAN_REVIEW
DEGRADE
```

The boundary is responsible for binding principal, capability, resource, action, tenant/data-domain scope, invoking the Policy Kernel, preserving the decision, and preventing downstream reinterpretation. It does not authenticate, administer consent, execute protected data, manage RLS, or persist audit events itself.

The architecture contract and machine-readable decision envelope were created on the dedicated authorization branch/PR. Runtime integration is intentionally deferred until contract validation and CI are trustworthy.

## 10. Capability Registry

The Capability Registry is intended to be the canonical inventory of reusable SOMA capabilities. It includes evidence/research, agent, MCP, Nostr, Web3, decentralized identity, content-addressed storage, and AI capability boundaries.

Optional capabilities remain optional and must not become core dependencies.

The current registry contains some person/agent identity-requirement semantics that require a separate authorization/delegation contract rather than an ad-hoc redesign inside the registry PR.

## 11. Documentation architecture and organization

The documentation authority hierarchy is:

```text
Schemas + executable contract tests
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

The Documentation Authority Registry is the first lookup point for humans, developers, and AI agents.

The 2026-09-12 content/organization audit established the following rules:

- inspect content before moving a document;
- distinguish authority/lifecycle differences from superficial terminology overlap;
- one concept should have one canonical authority;
- extend an existing authority rather than create another summary when appropriate;
- archive before deletion;
- synchronize indexes after moves/renames;
- freeze organization after verified moves unless a concrete authority, lifecycle, broken-reference, or duplication problem is demonstrated.

Verified organization decisions include placing the Food–Life evidence model under `docs/architecture/` and the cleanup/merge execution plan under `docs/governance/`.

## 12. Documentation content audit findings

High-risk duplication areas requiring ongoing discipline are:

- Health State vs longitudinal/behavioral observations;
- Evidence Graph vs source-specific evidence records;
- Policy Kernel vs Authorization Decision Boundary;
- Capability Registry vs adapter/assurance documents;
- AI/agent architecture vs product surfaces.

These are legitimate separate boundaries when each has a distinct authority/lifecycle. They must not become parallel canonical models.

## 13. CI incident and validation truthfulness

PR #82 and PR #83 repeatedly produced GitHub Actions jobs with:

```text
runner_id = 0
runner_name = ""
steps = []
no usable logs
```

Jobs failed within seconds before executing their workflow steps. Attempts to retrieve job logs returned GitHub `BlobNotFound`.

The workflow YAML itself was audited and uses GitHub-hosted `ubuntu-latest` runners with actual executable steps. Repository Actions settings shown during the audit allow actions and have reasonable workflow permissions.

The self-hosted runner page showing no configured runners is not evidence of a problem because these workflows request GitHub-hosted runners.

The account billing overview showed `$92.21` metered usage and `$92.21` included usage; this is not sufficient to conclude that Actions minutes are exhausted because it is not Actions-specific usage.

**Current classification:** CI execution/runner/account infrastructure blocker. It must not be represented as a code-test failure.

Do not weaken workflows, remove required checks, force merge, add an unnecessary self-hosted runner, or create an Actions Policy merely to make these runs pass.

## 14. GitHub Actions policy decision

The repository Actions Policy feature is intentionally **not** being used as a workaround for runner execution. It is a governance control over workflow triggering/execution policy, not a mechanism for provisioning GitHub-hosted runners.

A deliberate SOMA Actions governance policy may be designed later after basic CI execution is trustworthy.

## 15. Branch and merge governance

`main` is the canonical integration branch.

Never force a merge merely because a branch contains useful work.

Before integrating:

```text
Audit
  ↓
Compare with current main
  ↓
Contract review
  ↓
Security review
  ↓
Executable CI
  ↓
Final diff review
  ↓
Normal merge
  ↓
Verify main
```

If CI fails before execution, classify it as infrastructure failure and resolve that blocker rather than bypassing validation.

## 16. Known security findings retained

Important findings that remain visible until independently resolved include:

### Cryptography

Base64 is not encryption. Security-sensitive encryption must use an appropriately reviewed authenticated-encryption design.

### Key restoration

Direct hashing of a mnemonic into a secp256k1 key is not a standards-compliant wallet derivation scheme. Use an appropriate standard derivation design or fail closed.

### Nostr

Nostr broadcast remains disabled/fail-closed until real cryptographic signing and verification are implemented.

### Meta webhooks

WhatsApp/Messenger inbound webhooks must verify provider signatures over raw request bytes before accepting payloads. Sensitive content must not be logged.

### Secrets

A previously hardcoded Messenger page access token was identified during audit. It must be treated as compromised, rotated, and replaced through secure secret management.

### UniFFI

The Rust/UniFFI binding architecture requires reconciliation with the actual UDL/proc-macro approach before being considered production-ready.

## 17. Product direction

The immediate objective is no longer indefinite architecture expansion. The platform should move through:

```text
Validate
  → Integrate
  → Prove
  → Merge
  → Build one complete SOMA vertical
```

The first meaningful product milestone is one complete health/evidence workflow demonstrating shared capability composition, authorization, provenance, safety, failure isolation, and a real user outcome.

## 18. Deferred capabilities

The following remain intentionally deferred until validated product requirements justify them:

- broad autonomous agent runtime;
- Tool Gateway as a separate implementation track;
- autonomous clinical decisions;
- prescribing/medication modification;
- universal causal inference engine;
- full evidence-to-action compiler;
- mandatory AI;
- mandatory decentralized protocols;
- separate specialty repositories;
- multiple separate SOMA applications.

These are architectural possibilities, not current implementation commitments.

## 19. Current execution priority — 2026-09-12

### P0 — Trustworthy CI

1. Verify Actions-specific usage/quota/account execution state.
2. Restore genuine GitHub-hosted runner execution.
3. Obtain real executable steps, logs, and test results.
4. Establish a trustworthy green baseline.

### P1 — Foundation validation

1. Validate Capability Registry PR #80.
2. Validate Documentation Governance PR #82.
3. Validate Authorization Boundary PR #83.
4. Integrate runtime authorization only after contract/CI validation.
5. Complete adversarial security tests.

### P2 — First product vertical

Build one complete end-to-end SOMA health/evidence workflow from user question/input through governed capability execution to traceable result and outcome.

### P3 — Expansion

Only after the first vertical is proven, expand AI/agents/MCP, additional surfaces, providers, and decentralized adapters where justified.

## 20. Definition of a healthy SOMA ecosystem

A healthy SOMA ecosystem is one in which a new health capability can be delivered primarily by composing existing shared infrastructure rather than recreating policy, privacy, evidence, safety, provenance, identity, and audit logic.

Architectural success is therefore measured by safe reuse and validated end-to-end capability, not by the number of modules, protocols, or AI integrations.

## 21. Working rules for future contributors

1. Do not mix SOMA-OS with Janavani.
2. Treat SOMA as one ecosystem.
3. Build reusable capability shared-first.
4. Search existing contracts before creating new ones.
5. Do not duplicate canonical health/evidence models.
6. Do not use AI as a security authority or source of canonical health/evidence truth.
7. Preserve provenance and uncertainty.
8. Do not equate medical-stream integration with evidentiary equivalence.
9. Do not claim production security before verification.
10. Do not call Base64 encryption.
11. Do not enable insecure/mock cryptographic broadcast.
12. Do not log sensitive health/message payloads.
13. Do not force merges.
14. Archive before deletion.
15. Verify before claiming completion.
16. Record consequential decisions in the repository.
