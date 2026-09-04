# SOMA Policy Kernel v0.3

**Status:** Resource-scoped foundation contract
**Version:** 0.3.0

## Purpose

The Policy Kernel is the shared authorization and governance decision boundary
for SOMA capabilities, agents, tools, resources, adapters, and application
surfaces. It remains protocol-neutral, deterministic, side-effect free, and
fail-closed.

## Core rule

No agent, model, MCP server, runtime, tool, adapter, or application surface is
an authority merely because it can execute code or hold context. Authority is
granted only by an evaluated policy decision.

## Request model

Every consequential operation should be representable as:

```text
principal + capability + resource + action + context
```

Where:

- **principal** identifies the actor (human, agent, service, or system);
- **capability** identifies the governed capability;
- **resource** identifies both the resource class and the concrete resource
  instance;
- **action** identifies the requested operation;
- **context** carries policy-relevant facts but cannot create authorization.

## Resource-instance isolation

A grant is scoped to the concrete `resource_id`, not merely the resource type.
The v0.3 grant key is:

```text
principal_id
principal_type
capability_id
resource_type
resource_id
action
```

Therefore, a grant for `source-1` must not authorize `source-2` unless a
separate grant explicitly exists. A request's context cannot substitute,
expand, or override the resource scope used for authorization.

This is the minimum resource isolation boundary. Tenant/data-domain isolation,
expiry, revocation, delegation, and richer grant provenance remain separate
hardening stages and must not be implied by this contract.

## Decision model

The kernel returns one explicit decision:

```text
ALLOW
DENY
REQUIRE_CONSENT
REQUIRE_HUMAN_REVIEW
DEGRADE
```

Unknown, malformed, mismatched, or insufficiently authorized requests fail
closed.

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

## Context boundary

The request context is policy input, not a grant store. Context may cause a
`REQUIRE_CONSENT`, `REQUIRE_HUMAN_REVIEW`, explicit `DENY`, or `DEGRADE`
decision, but it cannot manufacture an `ALLOW` or expand the authorized
resource scope.

In production, context fields must be derived from trusted policy inputs or
validated upstream claims rather than blindly trusting caller-supplied values.

## Consent and human review

Consent is a policy input, not an unconditional permission token. The kernel
must eventually evaluate whether consent is applicable to the requested
capability, resource, purpose, scope, and current context.

High-risk actions may require human review even when a technical credential
would permit execution. Review requirements must be explicit and auditable.
The current evaluator represents these requirements as decision states; a
canonical consent/review lifecycle is a subsequent contract.

## Authorization grants

The evaluator consumes explicit grants supplied by a trusted policy source.
The grant store is intentionally outside the evaluator. A later implementation
may load grants from a database, policy service, signed policy bundle, or other
approved source without changing the decision contract.

Grant management must eventually define expiry, revocation, provenance,
delegation, tenant/data-domain scope, and capability-version compatibility.
Those mechanisms are not claimed by v0.3 merely because the kernel supports
resource-instance matching.

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

The evaluator does not persist audit events itself. Audit emission belongs to
the surrounding gateway/runtime so policy evaluation remains side-effect free.
Sensitive payloads and secrets must not be copied into audit records merely to
make an event easier to debug.

## Adapter rule

Adapters consume Policy Kernel decisions; they do not replace them. MCP,
agent runtimes, Web3, Nostr, DID, IPFS, research providers, messaging
transports, and AI providers remain interchangeable adapters around shared
policy and capability contracts.

## Determinism

The evaluator must be deterministic for the same normalized request, policy
version, registry, and grant set. Network calls and model inference do not
belong inside the core decision function.

## Versioning

The resource-scope change is a breaking grant-contract change and therefore
uses Policy Kernel version `0.3.0`. Existing v0.2 grant stores must be migrated
or explicitly adapted before adoption. A v0.2 grant cannot be silently treated
as a v0.3 resource-scoped grant.

## Non-goals for v0.3

- tenant isolation implementation;
- grant expiry/revocation implementation;
- unrestricted delegation;
- automatic permission escalation;
- autonomous policy generation;
- model-based authorization decisions;
- protocol-specific authorization logic inside the kernel;
- persistent audit storage inside the evaluator;
- a claim of complete production authorization security.
