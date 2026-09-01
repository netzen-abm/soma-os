# SOMA Shared Infrastructure

This directory is the home for reusable capability contracts, registries, and
policy-facing primitives.

## Rule

A capability that can serve multiple application surfaces belongs here before
surface-specific integration.

## Current foundation

- `capability_kernel.md` — shared capability lifecycle and integration rules.
- `capability_registry.json` — canonical capability inventory and maturity.

## Planned shared layers

- policy kernel;
- provenance kernel;
- consent and authorization;
- data-boundary contracts;
- transport-neutral messaging;
- provider-adapter contracts;
- capability discovery and negotiation.

Protocol implementations such as Web3, Nostr, decentralized identity, and
content-addressed storage remain adapters behind these contracts.
