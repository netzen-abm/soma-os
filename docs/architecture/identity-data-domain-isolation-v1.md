# SOMA Identity and Data-Domain Isolation Contract v1

**Status:** Design baseline  
**Version:** 0.1.0  
**Issue:** #22  
**Scope:** Shared infrastructure only

## 1. Purpose

SOMA needs one canonical identity/context boundary that can be reused by web,
mobile, bot, agent, MCP, evidence, health, and future adapters. The boundary
must make authorization decisions about a concrete actor and an explicitly
bounded data domain without allowing a transport or provider to redefine trust.

## 2. Separation of concerns

These concepts are distinct:

- **Authentication:** evidence that a credential or session was verified by a
  trusted authentication mechanism.
- **Identity:** the stable principal represented by that verified assertion.
- **Scope:** the tenant and data-domain boundaries in which the principal may
  act.
- **Authorization:** the Policy Kernel decision about a requested capability,
  resource, and action.
- **Delegation:** a separate future contract describing when one principal may
  act on behalf of another.

Authentication does not imply authorization. Identity does not imply access.
A caller-supplied string is not proof of identity or scope.

## 3. Canonical execution context

The shared boundary is conceptually:

```text
IdentityContext
  principal_id
  principal_type
  authentication_status
  authentication_provenance
  tenant_scope
  data_domain_scope
  jurisdiction_scope (optional, future policy)
  assurance_level
  issued_at
  expires_at (required when supplied by an authentication system)
  context_id
```

The context is a verified input to policy evaluation, not an application-owned
metadata bag.

## 4. Scope semantics

`tenant_scope` identifies the organizational or isolation boundary within which
the request is valid. `data_domain_scope` identifies the classes/domains of data
that the actor is permitted to address.

A protected operation must have an explicit scope intersection between the
verified identity context and the target resource. Missing, malformed, or
unverified scope must fail closed.

A broader scope must never be inferred from a narrower scope, resource type,
transport, model output, or tool metadata.

## 5. Resource relationship

Policy authorization remains responsible for the concrete resource boundary
introduced by Policy Kernel v0.3:

```text
verified identity context
        ↓
verified tenant/data-domain scope
        ↓
Policy Kernel
        ↓
capability + resource_type + resource_id + action
        ↓
ALLOW / DENY / governed non-allow decision
```

The identity layer does not replace the Policy Kernel, and the Policy Kernel
does not authenticate callers.

## 6. Cross-surface rule

Web, mobile, Telegram, WhatsApp, Messenger, agent, MCP, and other adapters may
translate their native authentication mechanisms into this canonical context.
They must not create independent authorization semantics.

An adapter may reject a request earlier, but it may not broaden a verified
identity, tenant, or data-domain scope.

## 7. Fail-closed requirements

The shared boundary must deny protected access when:

- principal identity is missing or malformed;
- authentication status is not verified where verification is required;
- authentication provenance is absent for a security-sensitive operation;
- tenant scope is missing, ambiguous, or unverified;
- requested data domain is outside verified scope;
- context is expired when expiry is authoritative;
- identity context is inconsistent with the requested principal;
- an adapter attempts to substitute caller-controlled scope for verified scope.

Fallback, provider outage, model output, MCP content, or user input must not
expand permissions.

## 8. Privacy boundary

Identity context should contain the minimum information necessary for policy
and audit. Protected health/evidence payloads do not belong in the identity
context merely for convenience.

Audit records should reference stable IDs and decision metadata rather than
copying sensitive payloads.

## 9. Compatibility with Policy Kernel v0.3

Policy Kernel v0.3 remains the canonical authorization evaluator and requires
`principal_id`, `principal_type`, `capability_id`, `resource_type`,
`resource_id`, and `action` for its grant identity.

This contract adds the missing upstream trust boundary for identity and scope.
It does **not** silently retrofit tenant or data-domain fields into v0.3 grants.
A future Policy Kernel revision may bind verified scope directly into its grant
identity after compatibility and migration are designed.

Existing v0.2/v0.3 grants must never be interpreted as containing tenant or
data-domain isolation unless an explicit contract says so.

## 10. Not yet implemented

This design does not claim implementation of:

- authentication providers;
- credential/session/token lifecycle;
- consent or revocation;
- delegation;
- jurisdiction/data-residency policy;
- persistent authorization audit;
- database row-level enforcement;
- complete end-to-end enforcement across all product surfaces.

Those remain separate implementation and verification gates.

## 11. Implementation gate

Before implementation is merged to `main`, the work must include:

1. machine-readable schema for the identity context;
2. dependency-free contract tests;
3. negative tests for cross-tenant and cross-domain access;
4. provenance/trust tests proving caller metadata cannot manufacture scope;
5. Policy Kernel integration tests;
6. adapter-boundary tests;
7. privacy/audit review;
8. full canonical CI and security review.

No application surface should implement a competing identity/scope model while
this shared contract is being established.
