# SOMA Capability Assurance Requirements v1

## Decision

Capability authorization MUST be explicit about the identity and assurance properties required by the capability. `IdentityContext.identity_mode` is an identity fact; it is not itself an authorization grant.

A capability MAY require:

- anonymous/local identity;
- authenticated account;
- a minimum identity assurance level;
- a durable identity relationship;
- explicit consent;
- human review;
- a jurisdiction scope;
- a data-classification boundary.

These requirements are evaluated by the Policy Kernel before a grant can produce `ALLOW`.

## Why this exists

SOMA adopts progressive identity. Most local, low-risk capabilities should not force account creation. Conversely, operations involving durable remote trust, recovery, sharing, delegation, organizational membership, regulated workflows, or higher-assurance identity must have a machine-enforceable authorization requirement.

Without explicit capability metadata, teams can accidentally encode login requirements in UI or adapter code, creating inconsistent security semantics and making the privacy promise unenforceable.

## Canonical capability metadata

Each capability entry declares an `identity_requirements` object:

```json
{
  "identity_requirements": {
    "applies_to_principal_types": ["person"],
    "allowed_identity_modes": ["anonymous_local", "authenticated_account"],
    "minimum_assurance_level": "LOW",
    "requires_durable_identity": false
  }
}
```

`applies_to_principal_types` is critical: SOMA has multiple principal classes (person, agent, service, application, device), while the current `IdentityContext` account/anonymous modes are person-oriented. Agent/device/service authorization MUST NOT be incorrectly modeled as a human login requirement. Their identity and assurance contracts will be defined separately.

### Semantics

`allowed_identity_modes` is the set of identity modes eligible for principals covered by `applies_to_principal_types`. Missing or malformed security metadata MUST fail closed when the requirement applies to the requesting principal type.

`minimum_assurance_level` is an ordered floor: `LOW < SUBSTANTIAL < HIGH`.

`requires_durable_identity=true` means the capability requires an authenticated account with a durable identity relationship. This flag MUST NOT be satisfied by `anonymous_local` regardless of the requested assurance level.

The registry describes capability requirements; the Policy Kernel remains authoritative for the final decision.

## Evaluation order

```text
IdentityContext
  -> capability exists
  -> principal type allowed
  -> applicable identity requirements
  -> identity mode allowed
  -> assurance meets minimum
  -> durable identity requirement satisfied
  -> consent / human review / safety / scope
  -> exact grant
  -> ALLOW / DENY / REQUIRE_CONSENT / REQUIRE_HUMAN_REVIEW / DEGRADE
```

Identity requirements MUST be evaluated before an exact grant can result in `ALLOW`. A valid grant cannot override an unmet capability identity requirement.

For a capability whose requirement does not apply to the requesting principal type, the requirement does not grant or deny authority; the normal principal identity and authorization contracts remain authoritative.

## Privacy invariant

A capability MUST NOT set `requires_durable_identity=true` merely for analytics, advertising, model training, implementation convenience, or product funnel optimization.

When authentication is required, the capability documentation MUST state why anonymous/local identity cannot safely provide the required security, continuity, authorization, recovery, legal, interoperability, or equivalent property.

## Non-goals

This contract does not select an authentication provider or define account storage. It does not define identity semantics for agents, services, applications, or devices. Those require their own explicit principal assurance contracts before production use.
