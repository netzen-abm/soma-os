# SOMA — Canonical Governed Capability Operation v1

**Status:** Architecture contract / implementation boundary  
**Date:** 2026-09-11  
**Scope:** Shared SOMA ecosystem infrastructure

## 1. Decision

SOMA needs one provider-neutral operation contract that composes the existing shared security and capability boundaries into a single governed execution envelope.

This is **shared SOMA infrastructure**, not a new application, service product, or surface. Every SOMA surface, adapter, AI component, agent, protocol, or future runtime may consume the operation contract, but none may redefine its authority semantics.

The operation contract binds:

```text
Identity
  → Capability
  → Policy
  → Protected Data / Evidence
  → Execution
  → Result
  → Provenance
  → Audit
```

The contract composes existing primitives. It does not replace the IdentityContext, Identity → Authorization Enforcement, Policy Kernel, Protected Data Access, Health State, Health Evidence Graph, provenance, or audit contracts.

## 2. Architectural rule

SOMA follows:

> **Shared capability → shared contract → shared policy/security → shared data/evidence contract → shared execution → provenance/audit → adapters/surfaces.**

The prohibited pattern is:

> Surface → private business logic → private authorization → private data access → provider-specific result.

A Telegram bot, web application, mobile client, MCP adapter, AI provider, agent, Nostr adapter, or future Web3/Web4/Web5 adapter is a consumer of governed capabilities—not a new authority layer.

## 3. Why this contract is needed

The current repository already has strong individual foundations:

- Capability Kernel defines reusable capability identity, lifecycle, policy requirements, adapters, surfaces, and failure behavior.
- IdentityContext and Identity → Authorization Enforcement define the security context and policy decision boundary.
- Policy Kernel provides deterministic, fail-closed policy decisions.
- Protected Data Access is the intended data-access boundary.
- Health State and Health Evidence Graph define canonical health/evidence semantics.
- Local Health Vault / PHR define protected personal-health storage boundaries.

The missing shared primitive is the **composition boundary** that records which governed capability operation was requested, under whose authority, against what bounded resource, under which policy decision, with what execution/result, and with what provenance/audit references.

## 4. Operation envelope

Every governed operation should have a stable operation identifier and correlation metadata. The operation envelope contains references and control metadata, not unrestricted health payloads.

Required semantic groups:

1. **Identity** — principal and verified security-context reference.
2. **Capability** — stable capability ID and contract version.
3. **Action** — bounded operation/action requested.
4. **Resource** — resource type and bounded resource reference/scope.
5. **Purpose** — declared purpose for policy evaluation.
6. **Policy** — policy decision, policy version, and decision reference.
7. **Consent / human authority** — explicit requirement/state when policy demands it.
8. **Execution** — lifecycle state, executor/runtime reference, timestamps, and bounded failure semantics.
9. **Result** — result status plus reference/digest; sensitive payloads remain behind the governed data boundary.
10. **Provenance** — references to source/transform provenance where the operation produces or transforms health/evidence information.
11. **Audit** — immutable audit reference for the governed operation.
12. **Replay/idempotency** — caller-supplied idempotency key where the operation has side effects.

## 5. Authority rules

The operation envelope is descriptive and authoritative only for the governed operation lifecycle; it must not become a second authorization engine.

- Identity authority remains IdentityContext / canonical identity enforcement.
- Authorization authority remains the Identity → Authorization Enforcement + Policy Kernel boundary.
- Data authority remains the canonical data contract and governed storage/access boundary.
- Health semantics remain Health State / Health Evidence Graph authority.
- AI and agent runtimes may propose or request operations but cannot grant themselves authority.
- A client-supplied policy decision is never trusted as authorization.
- A result cannot retroactively authorize the operation that produced it.
- A surface cannot widen subject scope, resource scope, capability scope, or purpose after authorization.

## 6. Policy and consent semantics

An operation may be:

- allowed;
- denied;
- waiting for explicit consent;
- waiting for human review;
- degraded where the policy explicitly permits degraded execution;
- expired/cancelled before execution.

`ALLOW` is the only policy outcome that authorizes ordinary protected execution. `REQUIRE_CONSENT`, `REQUIRE_HUMAN_REVIEW`, and `DEGRADE` are control states—not implicit permission to bypass the missing condition.

Consent is not interchangeable with authorization. Human review is not interchangeable with authentication. A verified local security context is not by itself proof of real-world identity.

## 7. Operation lifecycle

The shared lifecycle is intentionally bounded:

```text
REQUESTED
   ↓
AUTHORIZING
   ├── DENIED
   ├── REQUIRES_CONSENT
   └── REQUIRES_HUMAN_REVIEW
          ↓
      AUTHORIZED
          ↓
      EXECUTING
       ├── SUCCEEDED
       ├── FAILED
       ├── DEGRADED
       ├── CANCELLED
       └── EXPIRED
```

Implementations may omit states that are impossible for a particular capability, but must not invent a second incompatible lifecycle.

## 8. Protected data boundary

The operation contract must never be used as a shortcut around protected data access.

Required order:

```text
Identity Context
      ↓
Authorization Enforcement
      ↓
Policy Decision
      ↓
Protected Data Access
      ↓
Key / payload resolution
      ↓
Execution
      ↓
Result + Provenance + Audit
```

The operation envelope should contain references, classifications, scopes, and digests where useful. It should not duplicate complete protected health records merely to make audit or orchestration convenient.

## 9. Capability boundary

A capability is identified by a stable ID and semantic version. The operation references that capability; it does not embed a private capability definition.

A capability must declare its policy requirements, supported adapters/surfaces, dependencies, maturity, and failure behavior through the existing Capability Kernel/registry boundary.

This contract therefore does **not** create a second capability registry.

## 10. Health and evidence boundary

For health operations:

- Health State remains the canonical personal-health domain representation.
- Health Evidence Graph remains the canonical evidence representation.
- Personal observations are not automatically population evidence.
- Evidence strength and safety classification remain distinct.
- Temporal ordering does not imply causation.
- Provenance, uncertainty, applicability, limitations, and safety information must survive transformations.
- AI-generated interpretations remain interpretations, not newly authoritative observations or evidence claims.

## 11. AI / agent boundary

AI and agentic systems are optional consumers of shared capabilities.

An AI/agent operation may contain:

- the capability requested;
- bounded resource references;
- purpose;
- minimum necessary context references;
- proposed action/parameters;
- model/runtime provenance;
- resulting operation reference.

It must not contain or receive unrestricted protected health data merely because the model requested it. The governed data-access policy determines the minimum necessary context.

Agents cannot:

- self-authorize;
- modify the principal/subject scope after authorization;
- bypass consent or human review;
- turn an inference into an observation;
- convert association into causation;
- treat model output as clinical authority.

## 12. Failure and isolation semantics

Failures must be bounded by capability and adapter boundaries.

- A failed optional adapter must not disable unrelated SOMA capabilities.
- A protected operation must fail closed when authorization, policy, integrity, or protected-data prerequisites fail.
- External provider failure must not silently create a false success result.
- Degraded execution must be explicit and policy-permitted.
- Partial side effects require capability-specific transaction/compensation semantics; the operation envelope records the resulting state rather than pretending atomicity where none exists.

## 13. Idempotency and replay

Side-effecting operations should accept a caller-scoped idempotency key. Implementations must define whether retries are safe and how duplicate execution is prevented or detected.

Idempotency is a safety and integrity mechanism, not merely a performance optimization.

The operation ID, idempotency key, capability version, principal scope, and resource scope together must provide enough information for the executor to distinguish legitimate retry from unauthorized replay.

## 14. Provenance and audit

Every governed operation that reads, transforms, creates, or changes health/evidence information must be traceable to:

```text
Operation
  → capability
  → policy decision
  → input/reference scope
  → execution
  → output/reference
  → provenance
  → audit
```

Audit records must not become a second protected-data store. Use references/digests and minimum-necessary metadata where possible.

## 15. Cross-runtime conformance

Rust remains the canonical/reference domain runtime for security-sensitive and domain-critical behavior. Python/Django, TypeScript/JavaScript, WASM/Dioxus, external providers, AI runtimes, and future protocol adapters must conform to this operation contract.

No runtime may create a parallel authorization or capability lifecycle.

## 16. Explicit non-goals

This contract does not by itself implement:

- a new API gateway;
- a new authorization engine;
- a new capability registry;
- a new audit database;
- a new health-data store;
- autonomous clinical decision-making;
- prescribing or medication modification;
- a general-purpose agent framework;
- blockchain/Web3/Web4/Web5 execution;
- distributed consensus;
- cross-device synchronization.

Those are separate capabilities/adapters that must consume this shared boundary if and when justified.

## 17. Implementation sequence

1. Land the provider-neutral operation schema and contract tests.
2. Reconcile the contract with existing capability registry, policy, identity, protected-data, provenance, and audit primitives.
3. Add a canonical Rust reference representation only after the schema boundary is accepted.
4. Add execution integration at one real protected operation as a vertical slice.
5. Add adapter conformance tests.
6. Only then consider broader orchestration or agent integration.

## 18. Acceptance criteria

The contract is accepted when:

- one versioned operation envelope exists for governed capabilities;
- it composes existing authority boundaries instead of replacing them;
- capability identity is canonical and not duplicated;
- policy decisions cannot be client-forged into authorization;
- protected data remains behind the governed data-access boundary;
- health/evidence semantics remain canonical and provenance-preserving;
- lifecycle and failure semantics are deterministic;
- side-effecting operations have replay/idempotency semantics;
- audit/provenance are traceable without duplicating protected payloads;
- AI/agents cannot self-authorize or widen scope;
- cross-runtime implementations can be tested for conformance;
- no new application or parallel domain model is introduced.

## 19. Architectural outcome

This contract establishes the reusable **governed operation spine** for the SOMA ecosystem.

It allows Web, mobile, messaging, MCP, AI, agents, research services, and future decentralized/protocol surfaces to remain independent while consuming the same canonical capabilities, security authority, health/evidence contracts, and provenance/audit model.
