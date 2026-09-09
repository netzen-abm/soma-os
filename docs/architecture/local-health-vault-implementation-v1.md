# SOMA Local Health Vault Implementation v1

**Status:** security implementation baseline
**Date:** 2026-09-08

## Purpose

This implementation establishes the first executable cryptographic primitive for the shared SOMA Local Health Vault. The vault is shared SOMA infrastructure for every health context and surface; it is not a PHR-only database and not a product-specific storage silo.

The repository architecture requires personal health data to remain on the user's device whenever possible and requires sensitive security operations to be isolated, tested, and fail closed.

## Implemented boundary

`services/backend-rust/src/local_health_vault.rs` provides a provider-neutral encrypted record envelope and authenticated encryption/decryption primitive.

The implementation uses:

- AES-256-GCM authenticated encryption through `ring`;
- cryptographically random 96-bit nonces;
- authenticated associated data (AAD) containing record metadata;
- explicit 32-byte key input;
- versioned and classified record envelopes;
- base64 transport encoding for ciphertext and nonce;
- typed fail-closed errors;
- tamper and wrong-key tests.

The plaintext health payload is never placed in the persisted envelope.

## Integrity model

The following metadata is authenticated as AAD:

- record identifier;
- subject reference;
- entity type;
- schema version;
- classification;
- content type;
- key reference;
- provenance reference;
- creation/update timestamps;
- tombstone state.

Therefore, changing either ciphertext or authenticated metadata without the correct key causes decryption to fail.

## Key-management boundary

This v1 primitive deliberately does **not** claim to solve platform key storage, user authentication, recovery, backup-key escrow, hardware keystores, secure enclave integration, or compromised-OS defense.

Keys are supplied to the primitive and are not persisted by this module. Production key lifecycle must be implemented separately with platform-appropriate secure storage and reviewed recovery semantics.

No SOMA module, AI engine, media adapter, research adapter, or client surface should receive a vault master key. Decrypted data must be minimum-necessary and policy-authorized.

## Relationship to legacy vault code

Existing legacy vault import/export and device-sync surfaces remain disabled until their security contracts are migrated to this authenticated vault boundary. Existing encrypted-archive code is not treated as proof that the new vault security objectives are satisfied.

## Required next security stages

1. Cryptographic design review and key lifecycle contract.
2. Platform keystore/key wrapping adapter contract.
3. Atomic local storage adapter and encrypted index.
4. Subject isolation and authorization enforcement.
5. Crash recovery and deletion semantics.
6. Explicit encrypted backup/export contract.
7. Adversarial integration tests for all vault failure states.
8. Controlled integration with the Personal Health Record and Health Context Framework.

## Security non-goals for v1

This implementation does not claim:

- complete device compromise protection;
- guaranteed physical erasure from every storage medium;
- secure cloud synchronization;
- secure recovery from lost credentials;
- automatic clinical verification of stored records;
- authorization merely because a record can be decrypted.

## Acceptance evidence

The executable tests cover:

- successful encrypt/decrypt round trip;
- plaintext not appearing in ciphertext;
- wrong-key rejection;
- ciphertext tamper detection;
- metadata tamper detection;
- invalid key-length rejection;
- nonce randomization.

CI must execute the complete repository Rust test and security gates before any security-sensitive merge.
