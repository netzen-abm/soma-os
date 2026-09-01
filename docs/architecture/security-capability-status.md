# SOMA Security Capability Status

**Date:** 2026-08-31
**Branch:** `stabilization/baseline-audit`

## Purpose

Record the actual security capability of the current implementation without
using stronger security terminology than the code supports.

## Status

| Capability | Current status | Allowed claim |
| --- | --- | --- |
| Ed25519 local signing | IMPLEMENTED | Digital signature generation is implemented. |
| SHA-256 hashing | IMPLEMENTED | Hashing is implemented. |
| Nostr event ID hashing | IMPLEMENTED | Event ID calculation is implemented. |
| Nostr Schnorr event signing | BLOCKED | Not implemented; do not claim signed Nostr events. |
| Vault authenticated encryption | BLOCKED | Not implemented; vault export remains disabled. |
| Vault integrity verification | BLOCKED | Not implemented; vault import remains disabled. |
| Mnemonic standards-based recovery | BLOCKED | Not implemented; recovery remains disabled. |
| Zero-knowledge proof system | NOT IMPLEMENTED | Do not use ZKP terminology for current crypto modules. |

## Design rule

A security feature is enabled only when its implementation, tests, key
handling, failure behavior, and verification requirements have been reviewed.

A placeholder, mock, empty signature, Base64 encoding, or hash is never to be
represented as encryption, signing, zero-knowledge proof, or authenticated
integrity protection.

## Nostr boundary

The current Nostr module calculates an event ID and derives a public key from
a secp256k1 secret key, but deliberately leaves the event signature empty until
a standards-compliant Schnorr implementation is integrated and tested.

Therefore Nostr event publication must not be described as cryptographically
signed in the current state.

## Vault boundary

Vault import/export is intentionally disabled pending authenticated encryption,
integrity verification, and key lifecycle management.

Do not replace the disabled path with ad-hoc cryptography merely to make the
feature appear complete.

## Recovery boundary

Mnemonic recovery remains disabled pending a standards-compliant mnemonic
scheme, secure secret handling, and recovery tests.

## Main merge requirement

These blocked capabilities prevent a claim of complete secure vault/recovery
functionality. They do not prevent the repository from being developed, but
`main` must not present them as production-ready security capabilities.
