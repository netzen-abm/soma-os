# SOMA Optional User Identity v1

## Decision

SOMA users MUST NOT be required to create a login account merely to use the product.

Authentication and authorization are separate concerns. A person may use SOMA with an anonymous, locally scoped identity when the requested capability does not require a persistent account.

A login/account is an explicit capability escalation, not the default entry condition.

## Identity modes

### Anonymous local identity

An anonymous user receives a non-human-readable, non-secret pseudonymous `principal_id` scoped to the local installation/session according to the product's privacy policy. It is not an account identifier and must not be treated as proof of real-world identity.

Anonymous identity is suitable for low-risk capabilities such as local health-data organization, local calculations, evidence discovery, education, and other capabilities explicitly classified as safe for anonymous use.

Anonymous mode MUST NOT silently upload personal health data or create a cloud account.

### Authenticated account

A login-backed identity is appropriate only when a capability genuinely requires durable identity or a remote trust relationship.

Examples of strong reasons include:

- encrypted cloud synchronization or remote backup;
- recovery of a user-controlled vault after device loss;
- continuity across multiple devices where local transfer is insufficient;
- explicit sharing with a practitioner, caregiver, family member, or another authorized party;
- collaborative workspaces or organization membership;
- delegated access or other durable authorization relationships;
- higher-assurance operations that require verified identity;
- regulated, contractual, billing, or organizational workflows where identity is necessary;
- recovery, consent, or audit semantics that cannot be safely implemented with an anonymous local identity.

A login MUST NOT be requested merely for analytics, advertising, convenience to SOMA, model training, or because an implementation happens to be easier with accounts.

## Escalation rule

Before requiring login, the product capability MUST answer:

1. What concrete security, continuity, authorization, recovery, legal, or interoperability property requires durable identity?
2. Why can that property not be provided safely with an anonymous/local identity?
3. What minimum identity attributes are actually required?
4. What data leaves the user's device as a consequence?
5. Can the user postpone or decline account creation without losing unrelated local capabilities?

If there is no strong answer, the capability remains available without login.

## Privacy rule

Login is a user-controlled choice. SOMA should prefer progressive identity: anonymous/local use first, explicit authentication only at the boundary where durable identity becomes necessary.

Account creation MUST NOT be a precondition for storing or processing health information locally on the user's device.

The system must not infer that an anonymous person is authenticated simply because a device, session, email address, phone number, or browser is known.

## Authorization rule

`IdentityContext` describes security facts; it does not grant authorization. Anonymous and authenticated identities still pass through the same authorization chain:

```text
IdentityContext
  -> Identity Authorization Enforcement
  -> Policy Kernel
  -> Capability
  -> Consent / Safety / Scope checks
  -> Execution
  -> Evidence / Audit
```

The Policy Kernel and capability registry determine which capabilities are available to each identity mode.

The database persistence role is a service identity and is unrelated to whether an end user has a SOMA login.

## Non-goals

This decision does not define a specific authentication provider, passwordless mechanism, social login provider, identity proofing vendor, or account database schema. Those are implementation choices to be evaluated against this policy.
