# SOMA Identity-to-Authorization Enforcement Implementation v1

**Status:** Implementation baseline
**Version:** 1.0.0
**Design:** `docs/architecture/identity-authorization-enforcement-v1.md`

## Decision

The first implementation phase is a shared enforcement boundary composed of:

1. canonical `IdentityContext v1` validation;
2. explicit target tenant/data-domain binding;
3. principal consistency checks;
4. unchanged `Policy Kernel v0.3` evaluation;
5. fail-closed behavior for non-ALLOW decisions and malformed inputs.

The machine-readable envelope at `schemas/authorization-enforcement-context-v1.json` represents the request shape. The Python enforcement function is the shared reference implementation for this phase.

## Important boundary

This implementation does **not** establish an authentication provider, credential verification service, database/RLS enforcement, delegation, consent/revocation workflow, or production authorization completion.

The caller must supply an authoritative IdentityContext. The enforcement function does not accept caller metadata as an authority source and never merges it into trusted identity/scope.

## Policy Kernel composition

The Policy Kernel grant dimensions remain exactly:

`principal_id + principal_type + capability_id + resource_type + resource_id + action`

The enforcement boundary performs identity/scope checks before invoking the existing Policy Kernel. It does not add tenant or data-domain fields to Policy Kernel grants and does not implement a second authorization algorithm.

## Verification

The dependency-free test matrix covers:

- verified matching identity/scope;
- wrong tenant and data domain;
- missing scope and provenance;
- unverified, expired and revoked contexts;
- caller/model/MCP/adapter attempts to override trusted identity/scope;
- principal mismatch;
- missing resource ID and exact resource-instance enforcement;
- token/credential material exclusion;
- explicit service principal behavior;
- long-running execution expiry/revalidation.

## Remaining gates

The following remain separate implementation gates:

- trusted authentication/session/token lifecycle;
- authoritative revocation and key/trust rotation;
- persistent audit event implementation;
- repository/database/RLS enforcement;
- web, mobile, bot, agent and MCP integration;
- asynchronous execution integration with current-context revalidation;
- complete end-to-end cross-tenant/data-domain isolation evidence;
- independent human security review and production sign-off.
