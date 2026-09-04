# SOMA Identity-to-Authorization Enforcement Boundary v1

**Status:** Design baseline
**Version:** 0.1.0
**Issue:** #24
**Scope:** Shared infrastructure only

## 1. Purpose

IdentityContext v1 establishes canonical verified identity and tenant/data-domain scope. This document defines the next boundary: how those verified security facts become inputs to authorization and protected execution without allowing a transport, adapter, model, or caller-controlled field to broaden authority.

This is an enforcement contract, not an authentication-provider specification and not a claim that production authorization is complete.

## 2. Canonical trust chain

```text
trusted authentication / verification
        |
        v
canonical IdentityContext v1
        |
        v
verified tenant + data-domain scope
        |
        v
authorization request construction
        |
        v
Policy Kernel v0.3
        |
        v
ALLOW / DENY / governed non-allow decision
        |
        v
protected runtime / data operation
```

The protected operation must not execute merely because an IdentityContext is valid. Identity establishes who/what is acting and the verified scope; Policy Kernel authorization establishes whether the requested capability, concrete resource, and action are allowed.

## 3. Trust-boundary rule

Only a trusted verifier/authentication boundary may establish security-sensitive fields in IdentityContext. A caller, UI field, HTTP header, bot metadata, model output, MCP content, tool result, or adapter-local claim is not authoritative merely because it is syntactically valid.

Adapters may translate verified native claims into IdentityContext. They may not:

- replace a verified principal with caller-supplied identity;
- add a tenant not present in verified scope;
- add or broaden a data domain;
- upgrade authentication status or assurance level;
- extend authoritative expiry;
- convert an unverified context into a verified context;
- bypass Policy Kernel evaluation.

## 4. Authorization request envelope

The shared authorization request should contain the Policy Kernel v0.3 grant dimensions plus verified security context required for the boundary:

```text
principal_id
principal_type
capability_id
resource_type
resource_id
action
identity_context_id
tenant_scope
 data_domain_scope
request_id
```

`principal_id`, `principal_type`, `capability_id`, `resource_type`, `resource_id`, and `action` remain the canonical Policy Kernel v0.3 authorization dimensions. Identity context fields provide verified upstream facts and target-scope checks; they do not silently become new v0.3 grant dimensions.

The exact representation of the envelope is an implementation gate and must be machine-readable and tested before production use.

## 5. Target binding

A protected target must declare or resolve its authoritative tenant/data-domain scope. Authorization requires an explicit relationship between:

1. verified identity tenant scope and target tenant;
2. verified identity data-domain scope and target data domain;
3. Policy Kernel grant and the exact resource instance/action.

No target scope may be inferred from resource type alone. No broader scope may be inferred from a parent resource, transport, model output, or historical request.

For multi-scope identities, access is permitted only when the target is explicitly within the verified scope. Ambiguous target scope fails closed.

## 6. Fail-closed conditions

The enforcement boundary must deny protected access when:

- IdentityContext is missing or malformed;
- authentication is not verified where verification is required;
- authentication provenance is absent or untrusted;
- tenant scope is missing, ambiguous, expired, or unverified;
- target tenant is outside verified tenant scope;
- target data domain is outside verified data-domain scope;
- identity context is expired or revoked when authoritative;
- principal identity is inconsistent across trusted inputs;
- the authorization request omits a required v0.3 grant dimension;
- a caller-controlled field attempts to override a verified field;
- the Policy Kernel cannot produce an authoritative allow decision;
- the target's authoritative scope cannot be resolved;
- downstream data enforcement cannot prove the same scope boundary.

Failure, timeout, provider outage, parser ambiguity, or model uncertainty must never turn into ALLOW.

## 7. Policy Kernel compatibility

Policy Kernel v0.3 remains unchanged by this design. Its grant identity is:

```text
principal_id + principal_type + capability_id + resource_type + resource_id + action
```

This design therefore uses a composition model rather than silently changing the existing grant schema:

```text
IdentityContext trust + scope validation
              +
Policy Kernel v0.3 decision
              =
protected operation may proceed
```

Any future decision to bind tenant/data-domain scope directly into Policy Kernel grants requires a separate versioned contract, migration strategy, compatibility analysis, and security review.

## 8. Service-to-service identity

Service principals must not be treated as end users. A service-to-service request requires its own authenticated principal and trusted provenance. If a service acts on behalf of a user, delegation/acting-on-behalf-of semantics must be represented by a separate future contract rather than overloading `principal_id` or caller metadata.

A service cannot manufacture user identity or user scope merely because it possesses an internal credential.

## 9. Session/token lifecycle assumptions

This boundary depends on an upstream authentication system providing authoritative claims and lifecycle semantics. Before production enforcement, the implementation must establish:

- issuer trust and verifier configuration;
- credential/session/token verification rules;
- expiration handling;
- revocation handling where applicable;
- key rotation and trust-store rotation;
- replay considerations;
- clock-skew policy;
- context revalidation for long-lived/asynchronous work;
- provenance sufficient to audit how identity was established.

Raw credentials, tokens, secrets, or session material must not be stored inside IdentityContext.

## 10. Data-layer enforcement

Application-level authorization is insufficient for protected data if a downstream data layer can be accessed without equivalent isolation. The implementation must establish a defense-in-depth relationship between Policy Kernel decisions and data access controls.

For each protected data operation, the data layer must receive only an authorized target or an equivalent verified scope constraint. Database row-level security (RLS), repository-level predicates, or another enforceable mechanism may be used depending on the storage architecture.

A test that passes only at the API layer does not prove cross-tenant/data-domain isolation.

## 11. Adapter boundary

Every product surface and protocol adapter follows the same pattern:

```text
native authentication
      -> trusted verifier
      -> IdentityContext
      -> shared enforcement boundary
      -> Policy Kernel
      -> adapter/runtime operation
```

Adapters may add stricter local checks. They may not weaken or replace the shared checks.

The same authorization semantics must apply across web, mobile, bots, agents, MCP, and future adapters.

## 12. Asynchronous and long-running execution

Authorization must be evaluated against current trusted identity/scope, not only the state captured when a job was created. Before a protected operation resumes after a queue, retry, scheduled continuation, or long-running model execution, the system must revalidate relevant identity, expiry, revocation, scope, and policy conditions.

Persisted job state must not become an authorization token.

## 13. Audit and privacy

Each protected decision should be correlatable through non-sensitive identifiers such as request ID, context ID, principal ID, resource ID, capability ID, action, policy version, decision, and timestamp, subject to the project's privacy classification.

Audit events must not copy health/evidence payloads merely to explain an authorization decision. Sensitive data must remain outside the identity/authorization envelope unless explicitly required and governed.

## 14. Verification requirements

Before implementation is declared complete, tests must demonstrate at minimum:

### Positive
- verified identity + correct tenant + correct data domain + Policy Kernel ALLOW permits the intended operation;
- exact resource instance authorization remains enforced;
- service principal can access only explicitly authorized targets.

### Negative
- wrong tenant is denied;
- wrong data domain is denied;
- missing tenant scope is denied;
- missing data-domain scope is denied;
- unverified authentication is denied;
- expired/revoked context is denied;
- caller-supplied tenant cannot broaden verified scope;
- caller-supplied data domain cannot broaden verified scope;
- model/tool/MCP metadata cannot broaden scope;
- adapter-local authorization cannot bypass Policy Kernel;
- missing resource ID cannot become a wildcard authorization;
- downstream data-layer mismatch is denied.

### Lifecycle
- token/session expiry invalidates protected access when authoritative;
- key/trust configuration changes do not permit stale trust;
- long-running execution revalidates authorization;
- service-to-service identity remains distinct from user identity.

## 15. Implementation phases

1. **Contract:** machine-readable authorization envelope and target-scope contract.
2. **Trusted boundary:** verifier-facing constructor/validator that can produce an authoritative IdentityContext.
3. **Shared enforcement:** common enforcement function composing scope validation with Policy Kernel v0.3.
4. **Data enforcement:** connect the same boundary to repository/database access controls.
5. **Adapter integration:** web, bot, agent, MCP and other surfaces consume the shared enforcement path.
6. **Lifecycle hardening:** session/token expiry, revocation, rotation and asynchronous revalidation.
7. **Audit:** privacy-preserving decision events and correlation.
8. **Verification:** full negative isolation matrix, integration tests and production-like end-to-end tests.

## 16. Explicit non-goals

This contract does not implement:

- an authentication provider;
- a universal identity provider;
- consent/revocation UX;
- delegation semantics;
- jurisdiction/data-residency policy;
- a replacement Policy Kernel;
- a claim that database/RLS isolation is already complete;
- production security sign-off.

## 17. Completion gate

This boundary is complete only when a protected operation cannot cross tenant or data-domain boundaries through any supported execution path, including direct API calls, adapters, agents, MCP tools, asynchronous jobs, and data-layer access, and the evidence for that claim exists in automated tests and production-like verification.
