# ADR-004: Optional User Identity

- **Status:** Accepted
- **Date:** 2026-09-07
- **Scope:** End-user identity and account creation

## Context

SOMA is privacy-first health infrastructure. Requiring an account before a person can use ordinary local health capabilities creates unnecessary identity collection and turns authentication into a product prerequisite rather than a security control.

The existing `IdentityContext v1` correctly requires a verified security context before an operation reaches the Policy Kernel. That requirement must not be interpreted as requiring real-world identity or a persistent SOMA account.

## Decision

SOMA adopts **progressive identity**:

- Local/anonymous use is the default where capability risk permits it.
- Persistent login is optional.
- Authentication is required only when the specific capability needs a durable trust, recovery, sharing, organizational, contractual, regulatory, or equivalent security property.
- The reason for escalation must be explicit and capability-specific.
- Login must never be required solely for analytics, advertising, convenience, model-training access, or implementation simplicity.

### Authentication is not the same as real-world identification

An anonymous local principal can have a `VERIFIED` `IdentityContext` when SOMA has verified the integrity/continuity of the local security context. `VERIFIED` means that the security facts in the context were verified; it does **not** mean that the person has proved their civil identity or created a cloud account.

This preserves the current fail-closed enforcement contract while allowing privacy-preserving anonymous operation.

Example:

```text
Anonymous local principal
        |
        | local context verified
        v
IdentityContext (VERIFIED, pseudonymous principal)
        |
        v
Identity Authorization Enforcement
        |
        v
Policy Kernel
        |
        v
Capability permitted for anonymous mode
```

A remote account-backed principal follows the same enforcement chain, but its authentication provenance can represent the selected account authentication mechanism and its assurance level can be higher where justified.

## Strong-reason test for login

A capability may require login only when at least one concrete property is necessary and cannot safely be provided through a local/pseudonymous identity:

1. remote encrypted synchronization or backup;
2. recovery after loss of a device or local credential;
3. durable multi-device continuity;
4. explicit sharing/delegation to another person or organization;
5. organization/workspace membership;
6. higher-assurance identity proofing;
7. regulated, contractual, billing, or legally attributable workflows;
8. durable cross-session consent/audit semantics that require an account identity.

The capability owner must document why anonymous/local identity is insufficient and must request only the minimum identity attributes needed.

## Security implications

- Anonymous mode is not an authorization bypass.
- Anonymous and authenticated principals use the same Policy Kernel and capability controls.
- Sensitive capabilities can require authentication, stronger assurance, consent, or human review independently of whether the user has an account.
- The end-user identity model is completely separate from the PostgreSQL `somaos_persistence` service role.
- A database service role must never be exposed as or confused with a user login.

## Product implication

The default SOMA experience should make useful local functionality available without account creation. When a capability needs a persistent identity, SOMA should explain **why**, what changes, what minimum data is required, and what the user can still do without signing in.

## Consequence

This decision makes privacy a product architecture property rather than merely a privacy-policy statement. It also prevents account creation from becoming a hidden prerequisite for health-data ownership.
