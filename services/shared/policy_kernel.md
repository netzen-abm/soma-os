# SOMA Policy Kernel v0.4

**Status:** Current implementation contract  
**Version:** 0.4.0  
**Authority:** Shared policy-evaluation implementation contract. The canonical architecture composition boundary is documented separately in `docs/architecture/authorization-policy-decision-boundary-v1.md`.

## Purpose

The Policy Kernel is SOMA's deterministic, side-effect-free policy evaluator for capabilities, agents, tools, resources, adapters, and application surfaces. It evaluates a normalized `PolicyRequest` against the canonical capability registry, identity requirements, policy context, and trusted grants.

It is **not** the authentication system, consent lifecycle, human-review workflow, protected-data executor, audit store, or a second authorization engine.

## Core rule

No agent, model, MCP server, runtime, tool, adapter, or application surface is an authority merely because it can execute code or hold context. Authority is established only by the canonical authorization path and its evaluated policy decision.

## Request model

Every consequential operation is representable as:

```text
principal + capability + resource + action + context
```

Where:

- **principal** identifies the actor (human, agent, service, or system);
- **capability** identifies the governed capability;
- **resource** identifies both the resource class and concrete resource instance;
- **action** identifies the requested operation;
- **context** carries policy-relevant facts but cannot create authorization by itself.

The evaluator requires non-empty string values for the principal, capability, resource, and action dimensions and requires a mapping-like context.

## Capability validation

The kernel indexes capabilities from the shared registry. An unknown capability fails closed.

A capability must declare a non-empty list of valid `principal_types`. A request whose principal type is not declared by the capability is denied.

Identity requirements declared by a capability are validated before the grant decision. Malformed identity requirements fail closed.

## Identity requirements

Where a capability declares identity requirements, the evaluator requires an identity context that matches the request's:

- `principal_id`;
- `principal_type`;
- verified authentication status;
- permitted identity mode;
- minimum assurance level;
- durable-identity requirement where applicable.

The supported assurance ordering is:

```text
LOW < SUBSTANTIAL < HIGH
```

The supported identity modes are `anonymous_local` and `authenticated_account`.

Identity Context validation and upstream authorization enforcement remain separate security responsibilities. The Policy Kernel does not authenticate credentials or invent identity.

## Resource-instance isolation

The grant key is:

```text
principal_id
principal_type
capability_id
resource_type
resource_id
action
```

Authorization therefore applies to the concrete resource instance. A grant for `source-1` does not authorize `source-2` unless a separate grant exists.

Caller context cannot substitute for or expand the resource identity represented by the request.

## Decision model

The kernel returns exactly one explicit decision:

```text
ALLOW
DENY
REQUIRE_CONSENT
REQUIRE_HUMAN_REVIEW
DEGRADE
```

Unknown, malformed, mismatched, or insufficiently authorized requests fail closed.

## Decision evaluation order

The current evaluator applies these checks in order:

```text
request validity
  ↓
capability existence
  ↓
capability principal-type validity
  ↓
identity requirements
  ↓
human-review requirement
  ↓
consent requirement
  ↓
explicit policy deny
  ↓
exact resource-instance grant
  ↓
degrade requirement
  ↓
ALLOW
```

This ordering is implementation behavior and must not be described as a general-purpose policy language beyond what the evaluator actually implements.

## Policy precedence

The conceptual policy precedence remains:

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

A lower layer cannot override a higher safety or authorization constraint.

The current evaluator represents selected contextual gates through explicit context fields. A future richer policy-composition contract must not silently change the v0.4 evaluator semantics.

## Consent and human review

Consent and human review are explicit decision states, not authentication substitutes.

When the request context requires human review, the evaluator returns `REQUIRE_HUMAN_REVIEW`. When consent is required, it returns `REQUIRE_CONSENT`.

The kernel does not manage the lifecycle, evidence, revocation, expiry, or user interface of consent or human approval. Those belong to separate contracts.

## Grants

The evaluator consumes explicit trusted grants supplied to the kernel. Grant storage and administration remain outside the evaluator.

The current grant contract provides exact resource-instance matching. Expiry, revocation, delegation, grant provenance, tenant/data-domain policy composition, and capability-version compatibility remain separate hardening contracts unless implemented explicitly.

## Tenant and data-domain scope

Tenant and data-domain isolation are enforced through the broader SOMA identity/authorization and protected-data path. The Policy Kernel must not be treated as the sole enforcement point for database isolation.

A target outside the validated security scope must be rejected by the canonical authorization path before protected execution.

## Audit boundary

The kernel returns a deterministic `PolicyDecision` containing:

```text
 decision
 policy_version
 reason_code
```

It does not persist audit events. The surrounding authorization/operation runtime is responsible for emitting an audit event from the immutable decision result.

Audit records must not copy secrets, credentials, or sensitive health payloads merely to improve debugging.

## Adapter rule

Adapters consume canonical policy decisions; they do not replace or reinterpret them.

MCP, agent runtimes, Web3, Nostr, DID/VC, content-addressed storage, research providers, messaging transports, and AI providers remain interchangeable adapters around shared capability and policy infrastructure.

## Determinism and side effects

For the same normalized request, registry, identity context, and trusted grant set, the evaluator must return the same decision.

Network calls, model inference, database writes, external tool execution, and audit persistence do not belong inside the core evaluator.

## Versioning

`0.4.0` is the current implementation version reflected by `services/shared/policy_kernel.py`.

The earlier v0.3 resource-scope contract remains preserved by the decision history. It must not be represented as the current implementation contract after v0.4 adoption.

A future breaking change must increment the Policy Kernel version and explicitly document migration or compatibility behavior.

## Relationship to Authorization + Policy Decision Boundary

The canonical security composition is:

```text
IdentityContext
  → Identity Authorization Enforcement
  → Authorization + Policy Decision Boundary
  → Policy Kernel
  → Canonical Authorization Decision
  → Governed Capability Operation / ProtectedDataAccess
```

The Authorization + Policy Decision Boundary is the authoritative composition contract. The Policy Kernel remains the policy evaluator within that boundary.

## Non-goals

The Policy Kernel does not by itself implement:

- authentication provider integration;
- credential lifecycle;
- consent lifecycle;
- human-review workflow;
- grant administration;
- delegation;
- revocation infrastructure;
- database RLS;
- protected-data execution;
- distributed transaction management;
- autonomous clinical decisions;
- model-based self-authorization.

## Implementation truthfulness

This document describes the current evaluator. Architecture documents may define future composition, but they must not imply that a future capability is already implemented merely because the architecture names it.
