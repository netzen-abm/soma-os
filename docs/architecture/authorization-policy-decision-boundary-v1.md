# SOMA Authorization + Policy Decision Boundary v1

**Status:** Architecture contract / implementation gate
**Version:** 1.0.0
**Scope:** Shared SOMA infrastructure

## Purpose

SOMA requires one canonical boundary through which authorization-relevant context is validated, policy is evaluated, and the resulting decision becomes authoritative for a governed operation. This boundary composes existing security primitives; it does not create a second policy engine, authentication system, consent system, or execution engine.

## Canonical path

```text
IdentityContext
  → Identity Authorization Enforcement
  → Authorization + Policy Decision Boundary
  → Policy Kernel
  → Canonical Authorization Decision
  → Governed Capability Operation / ProtectedDataAccess
  → Execution / Protected Persistence
  → Provenance + Audit
```

The **Policy Kernel remains the sole policy evaluator**. The new boundary is the composition and authority boundary around that evaluator.

## Responsibilities

The boundary MUST:

1. accept authoritative IdentityContext;
2. use the existing Identity → Authorization Enforcement contract;
3. bind the request to principal, capability, concrete resource, action, tenant and data-domain scope;
4. invoke the canonical Policy Kernel rather than implementing another policy algorithm;
5. preserve decision, policy version and reason code;
6. return an immutable canonical authorization result;
7. prevent caller-controlled metadata from widening trusted identity or scope;
8. provide stable semantics to operations, protected-data access, agents and adapters.

It MUST NOT authenticate credentials, invent identity, maintain a parallel grant store, execute protected operations, persist audit events inside the policy evaluator, or convert a non-ALLOW result into ALLOW.

## Request and scope

The consequential request dimensions remain the Policy Kernel contract:

```text
principal_id
principal_type
capability_id
resource_type
resource_id
action
```

Principal identity must exactly match the validated IdentityContext. Authorization is bound to the concrete resource instance; resource type alone is insufficient. Tenant and data-domain scope are enforced by the identity/enforcement boundary and must be satisfied before protected execution.

The boundary MUST NOT silently add tenant/data-domain dimensions to the Policy Kernel grant key. Those are composable scope controls unless a future Policy Kernel version explicitly changes its canonical grant contract.

## Decision semantics

The canonical decision vocabulary is:

```text
ALLOW
DENY
REQUIRE_CONSENT
REQUIRE_HUMAN_REVIEW
DEGRADE
```

The boundary MUST preserve these states. `ALLOW` means current identity/scope enforcement and policy evaluation succeeded; it does not prove downstream persistence, cryptographic, transaction, safety, or execution success. `DEGRADE` is not equivalent to `ALLOW`.

## Canonical decision envelope

The implementation should expose a minimal immutable result containing, as applicable:

```text
request_id
principal_id
principal_type
capability_id
resource_type
resource_id
action
tenant_scope_binding
data_domain_scope_binding
decision
policy_version
reason_code
enforcement_version
decision_time
```

Credentials, tokens, secrets, sensitive health payloads and unrestricted caller metadata MUST NOT be embedded.

## Policy composition

The boundary delegates policy semantics to the Policy Kernel. Existing precedence remains authoritative:

```text
system safety
  ↓
organization policy
  ↓
department policy
  ↓
resource policy
  ↓
user consent / preference
  ↓
agent task context
```

Adapters and operations MUST NOT duplicate or override this precedence.

## Governed Operation

Authorization is a prerequisite to execution, not the operation lifecycle itself:

```text
REQUESTED → AUTHORIZING → canonical decision
                              ├→ DENIED
                              ├→ REQUIRE_CONSENT
                              ├→ REQUIRE_HUMAN_REVIEW
                              ├→ DEGRADE
                              └→ AUTHORIZED → EXECUTING → SUCCEEDED / FAILED
```

A Governed Capability Operation MUST consume the canonical decision and MUST NOT reinterpret it to manufacture authority.

## Protected Data Access

ProtectedDataAccess remains the provider-neutral persistence gate:

```text
Authorization Decision Boundary
  → ProtectedDataAccess
  → trusted persistence identity / transaction context
  → PostgreSQL / vault / approved adapter
```

Database constraints and RLS are defense-in-depth, not replacements for canonical authorization.

## AI, agents and adapters

AI models, agent runtimes, MCP, Web/mobile clients, messaging transports, Nostr, Web3, DID/VC, research providers and future protocols are adapters/callers around shared SOMA authorization infrastructure. They may request authorization but MUST NOT self-authorize, widen identity/scope, replace the canonical decision, or bypass identity enforcement/policy evaluation.

## Failure semantics

The boundary MUST fail closed for missing/unverified/expired/invalid identity, principal mismatch, target scope mismatch, malformed request, unknown capability, invalid capability identity requirements, insufficient assurance, missing grant, Policy Kernel DENY, REQUIRE_CONSENT, REQUIRE_HUMAN_REVIEW, and unexpected authorization errors.

## Security invariants

1. No trusted identity from caller metadata.
2. Identity mode alone never grants authorization.
3. Consent alone never grants authorization.
4. Human-review intent alone never grants authorization.
5. AI/model output never grants authorization.
6. Resource type alone never grants authorization.
7. No cross-tenant execution.
8. No cross-data-domain execution.
9. No protected access after non-ALLOW.
10. No second policy evaluator in adapters/surfaces.
11. No sensitive payloads in decision/audit metadata.
12. No silent fail-open behavior.

## Implementation gate

The first implementation must be the smallest shared composition layer over the existing Identity → Authorization Enforcement and Policy Kernel primitives. Tests must cover valid ALLOW, wrong principal, wrong tenant/domain/resource, unknown capability, assurance failure, expired context, caller scope override, all non-ALLOW decision states, malformed input, protected-adapter non-invocation after denial, sensitive-payload exclusion, and inability of downstream operations to replace the canonical decision.

## Non-goals

This contract does not implement authentication providers, credential lifecycle, accounts, consent lifecycle, human-review workflow, grant administration, delegation, revocation infrastructure, database RLS, distributed transactions, autonomous agents, or clinical decision authority.

## Architectural decision

SOMA adopts this boundary as the canonical composition boundary between verified identity/security context, policy evaluation and governed execution. Existing Identity → Authorization Enforcement remains the identity/scope layer; the Policy Kernel remains the sole policy evaluator; Governed Capability Operation and ProtectedDataAccess consume the authoritative result. This is shared SOMA infrastructure and is reusable by every surface and capability without creating separate applications or authorization systems.
