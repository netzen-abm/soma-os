# SOMA Governed Capability Operation v1

**Status:** Foundational shared-infrastructure contract / bounded v1  
**Date:** 2026-09-11

## 1. Decision

SOMA requires one provider-neutral operation envelope for governed capability execution.

The operation contract composes existing SOMA authorities; it does not replace them:

```text
Identity
   ↓
Capability
   ↓
Policy
   ↓
Protected Data / Evidence
   ↓
Execution
   ↓
Result
   ↓
Provenance
   ↓
Audit
```

This is ecosystem infrastructure, not a new application. Web, mobile, messaging, MCP, AI, agents, research, and future protocol surfaces consume the same governed operation semantics through adapters.

## 2. Existing authorities remain authoritative

The operation contract is an orchestration boundary, not a new security authority.

- **IdentityContext** remains the canonical identity/security-context representation.
- **Identity → Authorization Enforcement** remains responsible for authorization enforcement.
- **Policy Kernel** remains responsible for policy evaluation and its fail-closed decisions.
- **Capability Kernel / registry** remains responsible for capability identity, lifecycle, policy declarations, adapters, surfaces, and maturity.
- **Protected Data Access** remains the protected-data gate and must execute only after authorization.
- **Health State / Health Evidence contracts** remain canonical for their respective domains.
- **Provenance and audit systems** remain authoritative for their records.
- **Database RLS** remains defense-in-depth rather than a replacement authorization model.

The operation records references to these decisions and artifacts rather than copying their sensitive payloads.

## 3. Operation envelope

The minimum operation identity is:

- `operation_id` — stable identifier for this governed operation;
- `correlation_id` — optional trace/correlation identifier;
- `idempotency_key` — optional replay/idempotency control;
- `schema_version` — operation contract version;
- `status` — governed lifecycle state;
- `requested_at` — request time.

Authority context is explicit:

- `principal_ref`;
- `identity_context_ref`;
- optional `subject_ref` when the operation is subject-scoped;
- `capability_id` and `capability_version`;
- `action`;
- `resource_type` and optional `resource_ref`;
- `purpose`;
- optional `requested_data_scope`.

Sensitive health payloads, evidence bodies, credentials, keys, tokens, or complete records do **not** belong in the operation envelope.

## 4. Policy binding

Once policy evaluation occurs, the operation may retain:

- policy decision;
- policy version;
- decision reference.

Supported policy outcomes are bounded to the existing Policy Kernel vocabulary:

```text
ALLOW
DENY
REQUIRE_CONSENT
REQUIRE_HUMAN_REVIEW
DEGRADE
```

The operation must never accept a caller-supplied policy decision as authoritative. The runtime must obtain the decision from the canonical policy enforcement path and record a reference to it.

## 5. Consent and human review

Consent and human review are explicit gates where required.

An operation cannot silently convert `REQUIRE_CONSENT` or `REQUIRE_HUMAN_REVIEW` into `ALLOW`.

The operation may record:

- whether consent/review is required;
- current gate status;
- reference to the authoritative consent/review record.

The operation contract does not define a new consent authority.

## 6. Execution boundary

After authorization requirements are satisfied, execution may occur through a provider/runtime adapter.

Execution metadata may identify:

- executor;
- runtime;
- start/completion times;
- bounded failure code/class.

An AI model, agent, external provider, transport, or UI cannot modify the principal, subject, capability, policy authority, or authorization result merely because it performs execution.

## 7. Result boundary

Results are represented by references and integrity metadata rather than embedding potentially sensitive payloads.

A result may record:

- success/failure/partial/degraded status;
- result reference;
- integrity digest where available.

The result reference must resolve through the applicable governed data-access boundary. A result envelope is not permission to read the referenced resource.

## 8. Provenance and audit

A completed or materially failed governed operation should be traceable through:

```text
operation → policy decision → execution/result → provenance → audit
```

The operation may hold `provenance_ref` and `audit_ref`. These are references, not replacements for the authoritative provenance/audit records.

Audit records must not require storing full health payloads merely to explain that an operation occurred.

## 9. Lifecycle

The bounded v1 lifecycle supports:

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
          └── DEGRADED
```

`CANCELLED` and `EXPIRED` are terminal states for applicable operations.

Implementations must not treat status text alone as proof that a previous gate was satisfied. Gate transitions require authoritative checks.

## 10. Subject scope

For subject-scoped health operations, subject scope must be bound to the verified authorization context and canonical enforcement path.

A caller must not be able to change subject scope by manipulating a resource reference, query parameter, adapter payload, or AI prompt.

The operation envelope's optional `subject_ref` is therefore contextual metadata, not an authorization grant.

## 11. AI and agent boundary

AI and agent capabilities are consumers of governed operations.

They may:

- request an operation;
- provide bounded inputs;
- receive authorized results;
- produce derived outputs subject to applicable contracts.

They may not:

- grant themselves authorization;
- rewrite policy decisions;
- widen data scope;
- change principal or subject ownership;
- bypass consent/human-review gates;
- treat model output as canonical health or evidence truth.

This preserves SOMA's shared-infrastructure-first architecture while allowing future AI/agent implementations to evolve independently.

## 12. Adapter and protocol boundary

Web, Android, iOS, Telegram, WhatsApp, Messenger, MCP, Nostr, Web3/DID/VC, storage adapters, and future protocols remain surfaces/adapters.

They consume the operation contract. They do not create parallel operation semantics or security policies.

Failure of an optional adapter must not disable unrelated SOMA capabilities. Protected operations fail closed; optional transport/discovery may degrade only when policy permits.

## 13. Idempotency and replay

`operation_id` identifies the governed operation. Where replay is possible, an idempotency key should be used at the execution boundary.

An idempotency key is not itself authorization. Replayed operations must still respect current authorization, policy, consent, resource state, and expiry requirements.

The v1 contract deliberately does not prescribe a distributed replay database or queue. Those are implementation concerns to be introduced only when operationally required.

## 14. Failure semantics

Failures must preserve the distinction between:

- authorization failure;
- consent/review gate;
- policy-directed degradation;
- execution failure;
- protected-data failure;
- provider/adapter failure;
- integrity/provenance failure.

A provider outage must not be represented as successful execution merely because an adapter returned a transport-level response.

Protected-data and authorization failures remain fail-closed.

## 15. Conformance requirements

Any runtime implementing governed operations must prove:

1. It consumes the canonical capability identifier/version.
2. It uses canonical identity/authorization context.
3. It obtains policy decisions from the canonical policy path.
4. It cannot elevate a policy decision through caller-supplied fields.
5. Protected data is accessed only after authorization.
6. Subject scope cannot be widened by the adapter.
7. Consent and human-review requirements cannot be bypassed.
8. Results remain reference-based where payload sensitivity requires it.
9. Provenance and audit references remain traceable.
10. Failure and degraded states are explicit.
11. AI/agent execution cannot become a security authority.
12. The implementation conforms to the same versioned contract across languages.

## 16. Non-goals

This v1 does not implement:

- a second Policy Kernel;
- a second capability registry;
- a new identity provider;
- a new consent system;
- a new audit database;
- a new health-data store;
- a new evidence model;
- a generic distributed transaction system;
- autonomous clinical decision-making;
- prescribing or medication modification;
- mandatory AI or agent execution;
- mandatory blockchain/Web3/Web5 infrastructure;
- a universal workflow engine.

## 17. Implementation sequence

The safe implementation sequence is:

1. Validate this contract against existing shared primitives.
2. Add contract-level positive and adversarial tests.
3. Add a minimal Rust reference type/boundary without embedding sensitive payloads.
4. Integrate it with existing Identity → Authorization → Policy → Protected Data boundaries only after those interfaces are verified.
5. Add adapter conformance tests.
6. Only then consider application/surface integration.

The operation contract is therefore an enabling shared primitive, not a reason to accelerate every downstream SOMA capability.

## 18. Architectural decision

**Decision:** Adopt as the candidate canonical governed-operation envelope for SOMA shared infrastructure, subject to CI and architecture review.

**Authority rule:** The operation contract composes existing authorities; it does not replace them.

**Ecosystem rule:** One SOMA ecosystem, one shared capability/policy/data/provenance architecture, multiple independent surfaces and implementation adapters.
