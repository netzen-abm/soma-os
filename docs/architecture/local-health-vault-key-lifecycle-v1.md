# SOMA Local Health Vault Key Lifecycle Boundary v1

**Status:** architecture/security contract
**Date:** 2026-09-09

## Purpose

Define the security boundary for keys used by the shared SOMA Local Health Vault. This contract follows the merged Local Health Vault v1 cryptographic primitive without promoting raw key handling into application, AI, media, research, device, or UI layers.

The vault is shared SOMA infrastructure for the entire SOMA ecosystem and all governed health contexts. It is not a PHR-specific key store.

## Security objective

A vault encryption key must be:

- generated using an approved cryptographic random source;
- scoped to a defined vault/key domain;
- referenced by a non-secret `key_ref` rather than exposed as application metadata;
- held only by a trusted cryptographic/storage boundary for the minimum required operation;
- unavailable to AI models, media providers, research adapters, ordinary UI code, analytics, logs, telemetry, URLs, or untrusted integrations;
- rotated/replaced without requiring plaintext health data to be exposed outside the trusted vault boundary;
- revocable when compromise is suspected;
- recoverable only through an explicitly governed recovery mechanism;
- auditable without recording key material.

## Canonical key hierarchy

The implementation should support a hierarchy rather than distributing one long-lived vault master key:

`Platform/User Root Protection → SOMA Vault Key Encryption Key (KEK) → Versioned Vault Data Encryption Key (DEK) → Encrypted Record`

The exact platform mechanism is adapter-specific. The hierarchy is the canonical security model.

### Root protection

The root protection layer is owned by the platform/user security boundary, such as an operating-system keystore, hardware-backed keystore, secure enclave/TEE, or an explicitly governed equivalent.

SOMA must not invent a software-only substitute and call it equivalent hardware protection.

### KEK

A KEK protects versioned vault DEKs. A KEK is not used directly as a record encryption key unless a separately reviewed contract explicitly permits that behavior.

### DEK

A DEK is used by the authenticated vault encryption primitive. The current primitive requires a 32-byte AES-256 key. DEKs must be generated cryptographically and must not be deterministically derived from health data, user identifiers, passwords, timestamps, device identifiers, or other predictable values.

## Key references

`key_ref` is an opaque, non-secret identifier for key selection/versioning. It must never contain key material, passwords, recovery secrets, or encoded secrets.

Record metadata may expose `key_ref`; this does not authorize access to the corresponding key.

## Key states

A key record should use an explicit lifecycle:

`PROVISIONED → ACTIVE → RETIRING → RETIRED`

with exceptional states:

`SUSPENDED` and `COMPROMISED`.

Rules:

- `ACTIVE` keys may encrypt new records and decrypt records explicitly authorized for that key version.
- `RETIRING` keys may decrypt existing records but must not encrypt new records.
- `RETIRED` keys must not be used for normal encryption/decryption operations; retention depends on recovery/rewrap requirements.
- `SUSPENDED` keys fail closed pending review.
- `COMPROMISED` keys fail closed for new encryption and require a governed rewrap/rotation response.

## Rotation

Rotation must be forward-safe and must not require plaintext export.

Preferred sequence:

1. provision new key version;
2. authenticate access to the old and new key versions inside the trusted vault boundary;
3. rewrap the DEK or migrate encrypted records according to the selected storage design;
4. verify integrity and provenance;
5. atomically update the key reference/index;
6. retire the old version only when required recovery/retention obligations are satisfied;
7. record an audit event without key material.

Rotation must not silently rewrite provenance or health-record semantics.

## Recovery

Recovery is a separate security product decision, not an accidental property of encryption.

Permitted future models may include platform recovery, user-controlled recovery material, or explicitly governed multi-party recovery. No model is approved by this v1 contract.

Recovery mechanisms must be evaluated for:

- offline compromise;
- phishing/social engineering;
- device theft;
- account takeover;
- recovery-factor loss;
- insider access;
- coercion/privacy risks;
- backup copies;
- revocation after recovery compromise.

A recovery mechanism must never require sending the user's plaintext health vault to a third-party service.

## Platform adapter boundary

The future provider-neutral contract should conceptually expose operations such as:

- provision key version;
- unwrap/open a permitted key reference;
- create a new DEK;
- wrap a DEK under a permitted KEK;
- rotate/rewrap;
- suspend/revoke/compromise;
- inspect non-secret key metadata;
- destroy key material when policy and platform semantics permit.

Platform implementations may strengthen security but may not weaken the canonical lifecycle semantics.

## Memory and process boundary

Raw key bytes must remain inside the smallest practical trusted scope. They must not be serialized into logs, errors, metrics, audit events, database rows, crash reports, URLs, or ordinary API responses.

The current `LocalHealthVaultCrypto` API accepts key bytes because it is a low-level cryptographic primitive. This is **not** approval for application code to manage long-lived keys directly.

A future storage/key adapter must own key acquisition and minimize key lifetime in memory.

## Separation of concerns

The following boundaries remain distinct:

`Identity → Authorization → Vault Key Access → Encryption → Storage`

Possessing a valid authenticated user identity does not itself reveal a vault key.

Authorization to read a health record does not itself imply authorization to export its encryption key.

AI authorization does not imply key access.

Device authorization does not imply unrestricted vault-key access.

## Backup and synchronization

Backup copies must contain only encrypted vault material and the minimum metadata necessary for governed restoration. Backup storage must not receive plaintext health payloads merely because it is easier to implement.

Cloud synchronization, if introduced, is an encrypted transport/storage concern and must not become a decryption authority.

## Threats explicitly addressed

- application-layer key leakage;
- logging/telemetry leakage;
- key reuse across unrelated domains;
- predictable key generation;
- stale-key use after rotation;
- use of compromised/suspended keys;
- unauthorized key export;
- recovery-channel compromise;
- accidental exposure of key material to AI or integrations.

## Threats not solved by this contract

- compromised operating system/kernel;
- fully compromised trusted execution environment;
- malicious device firmware;
- user screenshots or manual copying of decrypted data;
- physical side-channel attacks;
- every platform-specific secure deletion guarantee;
- legal/organizational recovery obligations.

These require platform and operational threat models.

## Required implementation gates

Before production key handling is introduced:

1. define a provider-neutral Rust key lifecycle trait/contract;
2. add a platform adapter boundary without embedding OS-specific behavior in the domain model;
3. add negative tests for wrong key version, suspended/compromised key, unauthorized subject, key export, and stale key use;
4. add memory/logging tests where feasible;
5. define encrypted local index semantics;
6. define crash-safe rotation/rewrap transactions;
7. define backup/recovery contract separately;
8. integrate with Identity Authorization and Policy Kernel without making either a key store;
9. run complete CI and security/integrity gates;
10. update product-readiness status only from executable evidence.

## Non-negotiable invariants

- No plaintext health payload is sent to a key-management provider.
- No AI/model/tool output can create or broaden key authority.
- No caller-controlled `key_ref` can bypass authorization.
- No key material is stored in source control, ordinary database fields, logs, telemetry, or user-visible metadata.
- No deterministic derivation from health data or identity attributes.
- No silent downgrade from hardware/platform protection to software-only protection.
- No production claim until lifecycle, storage, authorization, recovery, rotation, and adversarial behavior are tested together.
