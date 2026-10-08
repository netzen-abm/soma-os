# SOMA — Canonical Capability Registry v1

**Status:** Foundational shared-infrastructure contract / bounded v1
**Scope:** Entire SOMA ecosystem

## Decision

`services/shared/capability_registry.json` is the canonical source of truth for SOMA capability identity and registry metadata.

The registry is shared ecosystem infrastructure. It is not an application registry, surface registry, AI registry, or protocol-specific implementation inventory.

The Rust capability module is a typed runtime projection of this registry. It must not maintain an independent list of capabilities.

## Canonical model

```
Canonical Capability Registry (JSON)
              ↓
     language/runtime projection
              ↓
     governed capability operation
              ↓
   adapters / surfaces / executors
```

The canonical registry owns identity and governance metadata. Runtime projections may optimize parsing, typing, caching, or lookup, but may not redefine canonical identity.

## Architectural boundary

A capability is identified by stable `id`, semantic `version`, `domain`, lifecycle `status`, `maturity`, principal/identity requirements, policy controls, adapters, and surfaces.

The `(id, version)` pair is the stable versioned capability reference used by governed operations.

Adapters and surfaces consume capabilities; they do not own capability identity. Adding another surface must not duplicate capability definitions, policy semantics, health models, evidence models, or provenance logic.

AI remains optional and user-choice governed. Decentralized identity, Web3, content-addressed storage, Nostr, and similar protocols remain adapters/capabilities rather than mandatory core dependencies.

## Validation gate

CI must prove the registry is valid, IDs and `(id, version)` pairs are unique, governance and identity fields are valid, AI remains optional, decentralized capabilities remain adapters, and runtime projections cannot silently invent unknown registry semantics.

## Non-goals

This contract does not create a new execution engine, replace the Policy Kernel, replace Identity Authorization Enforcement, replace the Governed Capability Operation contract, or create separate registries per surface.

> **One capability identity. One versioned registry. Many runtimes. Many adapters. Many surfaces. One governed SOMA ecosystem.**
