# SOMA Policy Kernel v0.1

**Status:** Foundation contract
**Version:** 0.1.0

## Purpose

The Policy Kernel is the shared authorization and governance decision boundary
for SOMA capabilities, agents, tools, resources, adapters, and application
surfaces. It must remain protocol-neutral and deterministic.

## Core rule

No agent, model, MCP server, OpenClaw runtime, tool, adapter, or application
surface is an authority merely because it can execute code or hold context.
Authority is granted only by an evaluated policy decision.

## Request model

Every consequential operation should be representable as:

```text
principal + capability + resource + action + context
```

Where:

- **principal** identifies the actor (human, agent, service, or system);
- **capability** identifies the governed capability;
- **resource** identifies the target and its data classification;
- **action** identifies the requested operation;
- **context** carries purpose, surface, consent, jurisdiction, risk, and other
  policy-relevant facts.

## Decision model

The kernel returns one explicit decision:

```text
ALLOW
DENY
REQUIRE_CONSENT
REQUIRE_HUMAN_REVIEW
DEGRADE
```

Unknown, malformed, or insufficiently authorized requests fail closed.

## Policy precedence

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
User preferences can shape permitted behavior but cannot grant authority that
system or organization policy forbids.

## Authorization grants

The v0.1 evaluator consumes explicit grants supplied by a trusted policy
source. A request's own context must never be able to create its authorization
grant. The evaluator therefore denies a request unless its principal type,
capability, and action match an explicit positive grant.

The grant store is intentionally outside the evaluator. A later implementation
may load grants from a database, policy service, signed policy bundle, or other
approved source without changing the decision contract.

## Delegation

Delegation must be explicit, scoped, time-bounded, and auditable. An agent
cannot delegate authority greater than the authority granted to it.

## Production data

Production access must use least privilege and scoped authorization. Stored
conversation memory, prior consent, or agent assertions are not sufficient by
themselves to authorize a new consequential operation.

## Consent and human review

Consent is a policy input, not an unconditional permission token. The kernel
must evaluate whether consent is applicable to the requested capability,
resource, purpose, scope, and current context.

High-risk actions may require human review even when a technical credential
would permit execution. Review requirements must be explicit and auditable.

## Audit event contract

A consequential decision should eventually produce an audit event containing,
at minimum:

```text
request_id
principal_id
capability_id
resource_type
resource_id
action
decision
policy_version
reason_codes
timestamp
```

The v0.1 evaluator does not persist audit events itself. Audit emission belongs
to the surrounding gateway/runtime so that policy evaluation remains
side-effect free. Sensitive payloads and secrets must not be copied into audit
records merely to make an event easier to debug.

## Adapter rule

Adapters consume Policy Kernel decisions; they do not replace them. MCP,
OpenClaw, Web3, Nostr, DID, IPFS, research providers, messaging transports,
and AI providers remain interchangeable adapters around shared policy and
capability contracts.

## Determinism

The v0.1 evaluator must be deterministic for the same normalized request,
policy version, registry, and grant set. Network calls and model inference do
not belong inside the core decision function.

## Versioning

Policy schemas and decision semantics are versioned. Breaking changes require
a new contract version and compatibility review before adoption by `main`.

## Non-goals for v0.1

- autonomous policy generation;
- model-based authorization decisions;
- unrestricted agent delegation;
- automatic permission escalation;
- production cryptography activation;
- protocol-specific authorization logic inside the kernel;
- persistent audit storage inside the evaluator.
