# SOMA — Canonical Capability Registry v1

**Status:** Foundational shared-infrastructure contract / bounded v1  
**Date:** 2026-09-11  
**Scope:** Entire SOMA ecosystem

## 1. Decision

`services/shared/capability_registry.json` is the canonical source of truth for SOMA capability identity and registry metadata.

The registry is shared ecosystem infrastructure. It is not an application registry, surface registry, AI registry, or protocol-specific implementation inventory.

The Rust capability module is a typed runtime projection of this registry. It must not maintain an independent list of capabilities.

## 2. Why this boundary exists

SOMA previously contained two capability representations:

1. the shared JSON capability registry used by governance and CI;
2. a Rust `CapabilityDescriptor` table containing a separate hard-coded inventory.

That duplication creates drift risk: a capability could exist in one representation, have a different version/status/domain in another, or be consumed by runtime code without passing through the governance registry.

The canonical boundary removes that ambiguity without forcing every runtime to use the same implementation language.

## 3. Canonical model

```text
Canonical Capability Registry (JSON)
              ↓
     language/runtime projection
              ↓
     governed capability operation
              ↓
   adapters / surfaces / executors
```

The canonical registry owns identity and governance metadata. Runtime projections may optimize parsing, typing, caching, or lookup, but may not redefine canonical identity.

## 4. Capability identity

A capability is identified by:

- stable `id`;
- semantic `version`;
- `domain`;
- lifecycle `status`;
- `maturity`;
- principal and identity requirements;
- policy controls;
- supported adapters;
- supported surfaces.

The `(id, version)` pair is the stable versioned capability reference used by governed operations.

## 5. Registry versus implementation

The registry does not claim that implementation exists merely because an entry exists.

`status` describes the capability lifecycle boundary. `maturity` describes implementation maturity. Security-sensitive capabilities require the corresponding evidence and tests before being treated as production-ready.

A runtime projection must reject unknown domain/status values rather than silently inventing a local interpretation.

## 6. Relationship to the Governed Capability Operation

The governed operation contract consumes the canonical `capability_id` and `capability_version`.

The operation contract does not copy the full capability registry record into every operation. It references the canonical identity/version and obtains authorization through the existing identity and Policy Kernel path.

Therefore:

```text
Capability Registry
      ↓ identity/version
Governed Operation
      ↓ authorization
Policy Kernel
      ↓
Protected Data / Evidence
      ↓
Execution + Result + Provenance + Audit
```

A caller cannot create a new capability by submitting an arbitrary operation envelope.

## 7. Relationship to adapters and surfaces

Adapters and surfaces are consumers of capabilities, not owners of capability identity.

Examples include web, Android, iOS, Telegram, WhatsApp, Messenger, MCP, Nostr, Web3/DID/VC, storage providers, AI providers, and future protocols.

Adding another surface must not require duplicating capability definitions, policy semantics, health models, evidence models, or provenance logic.

Optional adapters remain optional. Failure of one adapter must not disable unrelated SOMA capabilities.

## 8. AI and decentralized capability boundary

AI remains an optional capability governed by user choice. A model/provider cannot promote itself to a core dependency through registry metadata.

Decentralized identity, Web3, content-addressed storage, Nostr, and similar protocols remain adapters/capabilities. Their existence in the registry does not make them mandatory for SOMA core operation.

## 9. Migration rule

Existing capability IDs are preserved in v1. This is a canonicalization change, not an arbitrary renaming exercise.

The Rust hard-coded inventory is replaced by a projection of the canonical JSON registry. Existing consumers should continue to resolve capabilities by stable ID.

Future ID renames require an explicit compatibility/migration decision and must not be performed as incidental cleanup.

## 10. Validation requirements

CI must prove:

1. the canonical registry is valid JSON;
2. the registry conforms to the versioned schema boundary;
3. capability IDs are unique;
4. `(id, version)` pairs are unique;
5. required governance fields are present;
6. identity requirements are structurally valid;
7. policy/adapters/surfaces are arrays with valid values;
8. AI remains optional and user-choice governed;
9. decentralized capabilities remain adapters rather than core dependencies;
10. the Rust runtime contains no independent capability inventory;
11. the Rust projection can parse every canonical registry entry;
12. unknown registry values fail rather than silently degrade into an invented runtime meaning.

## 11. Non-goals

This v1 does not:

- create a new capability execution engine;
- replace the Policy Kernel;
- replace Identity Authorization Enforcement;
- replace the Governed Capability Operation contract;
- introduce a plugin marketplace;
- require dynamic remote registry loading;
- create separate registries per application or surface;
- require AI, agents, blockchain, decentralized identity, or any particular protocol;
- redesign capability semantics unrelated to canonicalization.

## 12. Architectural outcome

SOMA now has one authoritative capability vocabulary with language-specific projections:

> **One capability identity. One versioned registry. Many runtimes. Many adapters. Many surfaces. One governed SOMA ecosystem.**
