# SOMA — Canonical Governed Capability Operation v1

**Status:** Foundational shared-infrastructure contract / bounded v1  
**Date:** 2026-09-11  
**Scope:** Shared SOMA ecosystem infrastructure

## 1. Decision

SOMA requires one provider-neutral operation envelope for governed capability execution.

The operation contract composes existing SOMA authorities; it does not replace them:

```text
Identity → Capability → Policy → Protected Data / Evidence → Execution → Result → Provenance → Audit
```

This is ecosystem infrastructure, not a new application. Web, mobile, messaging, MCP, AI, agents, research, and future protocol surfaces consume the same governed operation semantics through adapters.

## 2. Existing authorities remain authoritative

- **IdentityContext** remains the canonical identity/security-context representation.
- **Identity → Authorization Enforcement** remains responsible for authorization enforcement.
- **Policy Kernel** remains responsible for policy evaluation and fail-closed decisions.
- **Capability Kernel / registry** remains responsible for capability identity, lifecycle, policy declarations, adapters, surfaces, and maturity.
- **Protected Data Access** remains the protected-data gate and must execute only after authorization.
- **Health State / Health Evidence contracts** remain canonical for their respective domains.
- **Provenance and audit systems** remain authoritative for their records.
- **Database RLS** remains defense-in-depth rather than a replacement authorization model.

The operation records references to these decisions and artifacts rather than copying sensitive payloads.

## 3. Architectural rule

SOMA follows:

> **Shared capability → shared contract → shared policy/security → shared data/evidence contract → shared execution → provenance/audit → adapters/surfaces.**

A surface, AI runtime, agent, or protocol adapter must not create parallel authorization, capability, health-data, evidence, or provenance semantics.

## 4. Operation envelope

The operation envelope contains references and control metadata, not unrestricted health payloads.

Core fields represent:

- operation identity: `operation_id`, optional `correlation_id`, optional `idempotency_key`;
- lifecycle: version and bounded status;
- authority context: principal, verified identity context, optional subject, capability/version, action, resource, purpose, requested data scope;
- policy: decision, policy version, decision reference;
- consent/human review: explicit gate state where required;
- execution: executor/runtime and bounded outcome/failure metadata;
- result: status, reference and optional integrity digest;
- provenance/audit: references to authoritative records.

Sensitive health payloads, credentials, private keys, access tokens, or complete records do not belong in this envelope.

## 5. Authority and gate rules

The operation envelope is an orchestration boundary, not a second authorization engine.

`ALLOW` is the only policy outcome that authorizes ordinary protected execution. `REQUIRE_CONSENT`, `REQUIRE_HUMAN_REVIEW`, and `DEGRADE` are control states and cannot be silently converted into authorization.

A caller-supplied policy object is descriptive input until the canonical authorization path has produced the authoritative policy decision. The operation must not trust a client-provided `ALLOW` as proof of authorization.

Consent is not interchangeable with authorization. Human review is not interchangeable with authentication. A verified local security context is not itself proof of real-world identity.

## 6. Protected-data boundary

The required conceptual order is:

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

The operation envelope never becomes a shortcut around protected data access. Result references do not grant permission to read the referenced resource.

## 7. Lifecycle

The bounded v1 lifecycle is:

```text
REQUESTED → AUTHORIZING
                 ├→ DENIED
                 ├→ REQUIRES_CONSENT
                 └→ REQUIRES_HUMAN_REVIEW
                         ↓
                    AUTHORIZED → EXECUTING
                                  ├→ SUCCEEDED
                                  ├→ FAILED
                                  └→ DEGRADED
```

`CANCELLED` and `EXPIRED` are terminal states where applicable.

Status text is never proof that a gate was satisfied. Gate transitions require authoritative checks.

## 8. Health and evidence boundary

For health operations:

- Health State remains the canonical personal-health representation.
- Health Evidence Graph remains the canonical evidence representation.
- Personal observations are not automatically population evidence.
- Evidence strength and safety classification remain distinct.
- Temporal ordering does not imply causation.
- Provenance, uncertainty, applicability, limitations, and safety information survive transformations.
- AI-generated interpretations remain interpretations, not authoritative observations or evidence claims.

## 9. AI and agent boundary

AI and agentic systems are optional consumers of governed capabilities. They may request operations, provide bounded inputs, and receive authorized results.

They may not self-authorize, widen data scope, change principal or subject ownership after authorization, bypass consent/human review, turn inference into observation, convert association into causation, or become clinical authority.

## 10. Adapter/protocol boundary

Web, Android, iOS, Telegram, WhatsApp, Messenger, MCP, Nostr, Web3/DID/VC, storage adapters, and future protocols remain surfaces/adapters. They consume the shared operation semantics and must not redefine them.

Optional adapter failure must not disable unrelated SOMA capabilities. Protected operations fail closed; optional transport/discovery may degrade only when policy permits.

## 11. Idempotency and replay

Side-effecting operations should use a caller-scoped idempotency key where replay is possible. Idempotency is not authorization. Replays must still satisfy current authorization, policy, consent, resource-state, and expiry requirements.

The v1 contract does not prescribe a distributed replay database, queue, or transaction system.

## 12. Failure semantics

Implementations must preserve the distinction between authorization failure, consent/review gate, policy-directed degradation, execution failure, protected-data failure, provider/adapter failure, and integrity/provenance failure.

A provider outage must not be represented as successful execution merely because an adapter returned a transport-level response. Partial side effects require capability-specific transaction or compensation semantics; the envelope records the resulting state rather than pretending atomicity.

## 13. Conformance

A governed-operation implementation must demonstrate that it:

1. uses the canonical capability identifier/version;
2. uses canonical identity/authorization context;
3. obtains policy decisions from the canonical policy path;
4. cannot elevate authorization through caller-supplied fields;
5. accesses protected data only after authorization;
6. cannot widen subject scope through an adapter;
7. cannot bypass consent/human-review gates;
8. keeps sensitive results behind the governed data boundary;
9. preserves provenance/audit traceability;
10. represents failure/degraded states explicitly;
11. prevents AI/agent execution from becoming a security authority;
12. conforms to the same versioned contract across runtimes.

## 14. Non-goals

This v1 does not create a second Policy Kernel, capability registry, identity provider, consent system, audit database, health-data store, evidence model, distributed transaction system, autonomous clinical decision system, agent framework, blockchain/Web3/Web4/Web5 runtime, or universal workflow engine.

## 15. Implementation sequence

1. Land the provider-neutral schema and contract tests.
2. Reconcile the contract with existing capability, identity, policy, protected-data, provenance, and audit primitives.
3. Add a minimal Rust reference representation only after the schema boundary is accepted.
4. Integrate one real protected operation as a vertical slice.
5. Add adapter conformance tests.
6. Only then consider broader orchestration or agent integration.

## 16. Architectural outcome

This contract establishes the reusable **governed operation spine** for the SOMA ecosystem: one shared capability/security/data/evidence/provenance architecture with multiple independent surfaces and implementation adapters.
