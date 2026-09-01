# SOMA Plug-and-Play Capability Adapter Contract

## Purpose

SOMA core must not depend on a specific transport, identity system, ledger,
storage network, AI provider, or decentralized protocol. Future capabilities
must be addable as adapters without rewriting the core domain or existing
surfaces.

## Capability boundary

Every optional capability should expose a stable interface and an adapter:

```text
SOMA Core Contract
       |
       +-- Web adapter
       +-- Mobile adapter
       +-- Telegram adapter
       +-- WhatsApp adapter
       +-- Nostr adapter
       +-- Web3 / wallet adapter
       +-- Decentralized identity adapter
       +-- IPFS / content-addressed storage adapter
       +-- Future protocol adapter
```

A failure of one adapter must not disable unrelated adapters.

## Web3 and decentralized capabilities

Web3 is a future capability, not a mandatory runtime dependency. The core
must remain fully functional without a wallet, blockchain, token, or smart
contract.

Potential future adapters include:

- wallet connection and signing;
- decentralized identity / verifiable credentials;
- blockchain anchoring for integrity proofs;
- content-addressed storage;
- Nostr identity and event transport;
- decentralized governance or attestations.

These adapters must be independently deployable and replaceable.

## No protocol lock-in

The core must never store protocol-specific objects as its only canonical
representation. Protocol-specific payloads belong at the adapter boundary.

Canonical SOMA data should contain stable identifiers, provenance, capability
metadata, and policy decisions. An adapter maps those objects to its protocol.

## Optional means user choice

A decentralized capability must be opt-in where user choice is relevant.
Users must be able to use SOMA without adopting a wallet, blockchain, public
key identity, or decentralized transport.

## Shared capability policy

Adapters consume shared policy contracts for:

- privacy;
- consent;
- safety;
- authorization;
- data minimization;
- provenance;
- retention;
- auditability.

A surface must not invent a weaker policy for a specific transport.

## Failure isolation

```text
Adapter A failure
      X
      |
      +----> Core remains operational
      +----> Adapter B remains operational
      +----> Adapter C remains operational
```

Adapters should fail closed for protected operations and degrade gracefully
for optional discovery or transport features.

## Implementation rule

Do not add a protocol merely because it is available. Add it when there is a
clear product requirement, security model, operational owner, and testable
adapter contract.
