# SOMA — Ephemeral Device Permission Contract v1

**Status:** Foundational shared-infrastructure contract / bounded v1
**Scope:** Health, wellness, nutrition, research, and related SOMA capabilities

## 1. Decision

When a SOMA capability needs access to a sensitive device or user resource such as microphone/audio, camera, location, contacts/address book, files, sensors, or similar protected input, access is **purpose-bound and ephemeral**.

SOMA must request authorization for the specific capability, resource, subject/scope, and purpose before activation. When that purpose is complete, SOMA must immediately release its own capability lease and stop using the resource.

A later use is a **new activation**. It must not silently reuse a previous SOMA lease or treat historical consent as continuing authorization.

## 2. Important platform boundary

SOMA cannot universally switch off the operating-system permission itself. Android, iOS, browsers, desktop operating systems, and connected devices retain final control over their platform permission state.

Therefore the SOMA contract is stronger and more precise at the application boundary:

```text
Request purpose
    ↓
Canonical identity + authorized subject/scope
    ↓
Policy / consent gates
    ↓
Ephemeral capability lease
    ↓
Use only for declared purpose
    ↓
Purpose complete / timeout / cancellation
    ↓
Immediate SOMA release
    ↓
Next use requires fresh activation
```

If the host platform also permits revocation or one-time access, adapters should use those mechanisms where supported.

## 3. No ambient access

Sensitive capabilities must not remain continuously active merely because the user granted a historical platform permission.

The following are examples of capabilities that should follow this contract where technically applicable:

- microphone / audio capture;
- camera capture;
- precise or background location;
- contacts / address book;
- photos/files/document access;
- health-device sensors;
- wearable/device telemetry;
- Bluetooth or nearby-device access;
- other sensitive user or device resources.

A platform permission being technically granted is not itself authorization for a SOMA operation.

## 4. Purpose limitation

Every activation must carry an explicit machine-readable purpose and capability identity.

Examples:

- capture one health measurement;
- record a user voice note for a requested health journal entry;
- scan a nutrition label;
- import contacts only for an explicitly requested sharing workflow;
- obtain location for a requested local health-service lookup.

SOMA must not use an active lease for an unrelated purpose.

## 5. Lifetime and release

A lease must have a bounded lifetime and must be released on the earliest applicable event:

1. declared purpose completed;
2. user cancellation;
3. operation failure;
4. authorization/consent withdrawal;
5. capability/session termination;
6. lease expiry;
7. security or policy failure.

Release must fail closed. A released or expired lease cannot be used again.

## 6. Fresh activation

A subsequent operation must obtain a new authorization decision and a new lease. Reuse of an old lease is prohibited.

This deliberately makes the lifecycle:

```text
INACTIVE → AUTHORIZING → ACTIVE → RELEASED
                         └──────→ EXPIRED
```

There is no implicit `REACTIVATE` transition. A new activation is created only through the canonical authorization path.

## 7. Security and privacy rules

- Principal and authorized subject must remain distinct.
- Tenant and data-domain scope remain explicit.
- A client, AI model, agent, MCP server, adapter, or provider cannot self-grant access.
- Consent does not replace authorization.
- Authorization does not imply unrestricted collection.
- A capability lease does not grant access to unrelated resources.
- Protected health data remains behind the canonical protected-data boundary.
- Raw device input must carry provenance and purpose context where it enters SOMA.
- Sensitive input must not be copied into logs merely for debugging.
- Fail closed when purpose, scope, authorization, or lease state is ambiguous.

## 8. Ecosystem rule

This is one shared SOMA infrastructure contract, not separate permission systems for separate apps.

Web, Android, iOS, messaging, AI, agent, MCP, research, wearable, and future protocol adapters should implement the same semantics through adapter-specific platform permission mechanisms.

An adapter failure must not broaden permission or grant another surface access.

## 9. Non-goals

This contract does not:

- replace Android/iOS/browser/desktop OS permission systems;
- create a second authorization or policy engine;
- create a new consent database;
- imply continuous sensor ingestion;
- authorize collection beyond the declared purpose;
- permit AI or agents to become permission authorities.

## 10. Required conformance tests

A conforming implementation must demonstrate at minimum:

1. no active lease before authorization;
2. activation requires explicit capability, purpose, scope, and bounded expiry;
3. active lease works only within its lifetime;
4. purpose completion immediately releases the lease;
5. cancellation releases the lease;
6. expiry prevents further use;
7. released leases cannot be reused;
8. a subsequent use requires a new activation;
9. authorization denial cannot mint a lease;
10. changing purpose or scope cannot widen an existing lease;
11. platform adapters do not treat historical platform permission as current SOMA authorization;
12. audit/provenance records can identify capability, purpose, subject/scope, activation, and release without storing unnecessary sensitive payloads.
