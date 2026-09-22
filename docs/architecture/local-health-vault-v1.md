# SOMA Local Health Vault v1

**Status:** Architecture contract / implementation gate
**Date:** 2026-09-08
**Scope:** Device-local encrypted persistence for user-controlled personal health information

## 1. Decision

The Local Health Vault is the primary trust boundary for SOMA personal health data. It is a storage and cryptographic boundary, not a second health domain model and not a cloud database abstraction.

The canonical relationship is:

`Canonical Health Entities -> Vault Records -> Local Index/View -> Health Intelligence`

The vault stores canonical entities and protected supporting records; it does not redefine their clinical semantics.

## 2. Existing-system audit findings

The repository already contains health-vault/device-sync related code and export/import paths. These must be treated as legacy implementation surfaces until they satisfy this contract.

In particular, `LocalDeviceHealthVault` is currently an implementation model rather than a sufficient security contract. The presence of local storage or a vault-shaped type does not establish encryption, key isolation, integrity, deletion semantics, or application-level exfiltration resistance.

The PHR schemas correctly establish a local-first product boundary and reference canonical health entities. The next missing foundation is an explicit machine-readable vault envelope and executable invariants before persistent PHR implementation proceeds.

## 3. Security objectives

The vault MUST provide:

- confidentiality of protected health records at rest;
- authenticated integrity for protected records;
- explicit key lifecycle and recovery semantics;
- separation of encryption keys from ordinary record metadata;
- versioned envelopes for migration;
- tamper detection before plaintext is trusted;
- provenance for imported and derived records;
- controlled export and backup;
- local audit events for access, export, sharing and destructive operations;
- secure deletion semantics appropriate to the underlying storage technology;
- fail-closed behavior for malformed, unauthenticated or unsupported records.

The vault MUST NOT imply that device-local storage eliminates risks from malware, OS compromise, unlocked devices, backups, screenshots, exports, or recipient devices.

## 4. Threat boundary

Assume an attacker may obtain:

1. the application database/files without the decryption key;
2. stale or partially copied vault files;
3. malformed or replayed vault records;
4. exported records after user authorization;
5. a compromised application/plugin attempting unauthorized access;
6. metadata without plaintext;
7. recovery material if the user stores it insecurely.

The contract does not claim to defend against a fully compromised operating system. That is a platform security boundary and must be documented separately.

## 5. Canonical vault record envelope

Every protected record MUST have a stable record identifier and explicit envelope metadata. The envelope should contain:

- `record_id`;
- `subject_ref`;
- `entity_type`;
- `schema_version`;
- `classification`;
- `content_type`;
- `ciphertext`;
- `nonce`;
- `key_ref`;
- `encryption_algorithm`;
- `integrity_algorithm` where applicable;
- `provenance_ref`;
- `created_at` / `updated_at`;
- optional deletion/tombstone metadata.

Plaintext health content MUST NOT be required in the envelope. Search indexes should contain only the minimum metadata necessary and must not become an accidental plaintext health-data store.

## 6. Cryptographic contract

Implementation MUST use an authenticated-encryption construction approved by the platform cryptographic policy. The initial implementation should prefer a well-reviewed AEAD construction available in the canonical Rust cryptographic stack rather than custom cryptography.

Keys MUST NOT be derived from user profile fields, predictable identifiers, or application configuration values.

The design MUST distinguish:

`Root/Recovery Material -> Key Derivation/Key Wrapping -> Vault Data Keys -> Record Encryption`

Exact KDF, key hierarchy, platform keystore integration, backup and recovery choices are implementation decisions that require a separate security review. They must not be guessed in the schema.

## 7. Key isolation

Application modules, AI providers, media adapters and research adapters MUST NOT receive raw vault master/recovery keys.

A capability should receive only the minimum decrypted data necessary for its authorized operation.

The intended path is:

`User Intent -> Identity/Policy -> Vault Authorization -> Minimum Record Selection -> Decrypt in trusted boundary -> Capability -> Result -> Audit`

A generic `get_all_health_data()` interface is prohibited.

## 8. Record integrity and provenance

A record is not trusted merely because it decrypts.

Imported documents, device measurements, user entries, clinician-authored records and machine-derived records MUST preserve source/provenance distinctions.

For imported material, retain source reference and transformation history where technically possible. OCR output is an extraction, not automatically a verified clinical fact.

Derived records MUST reference their inputs and processing version.

## 9. Local indexing

The vault may maintain a local index for usability, but indexing is a separate privacy surface.

The index MUST:

- minimize sensitive fields;
- document which fields are plaintext versus encrypted;
- support rebuilding from canonical encrypted records where practical;
- avoid copying full clinical documents into search tables;
- respect record deletion and revocation semantics;
- avoid telemetry leakage.

## 10. Transactions and crash safety

Vault writes MUST be atomic from the application's perspective.

A successful write means the encrypted record, integrity metadata, index update and required audit event have reached a recoverable consistent state, or the operation fails without presenting partial success.

Crash recovery MUST tolerate interrupted writes without silently accepting truncated or mixed-version records.

## 11. Versioning and migration

Vault envelopes and canonical health schemas evolve independently but must remain explicitly versioned.

Migration MUST:

- preserve provenance;
- preserve source record identity where semantics remain unchanged;
- record transformation history;
- fail closed on unsupported versions;
- never silently reinterpret clinical meaning;
- support rollback/recovery at the storage layer where technically possible.

## 12. Deletion

SOMA must distinguish logical deletion, cryptographic deletion and physical storage reclamation.

A delete operation MUST produce an auditable state transition and prevent normal application reads from returning the deleted record.

Physical secure erasure depends on the storage medium, filesystem, database engine and backup strategy; the product must not promise guarantees it cannot technically provide.

## 13. Backup and export

Backup is not automatically equivalent to synchronization.

Any backup/export path MUST explicitly define:

- encryption state;
- key dependency;
- destination;
- scope;
- retention;
- restore behavior;
- audit event;
- user confirmation.

Cloud backup is optional and must never become the default health-data path merely for implementation convenience.

## 14. Sharing boundary

Doctor sharing and emergency access MUST operate from explicit, purpose-bound selection of vault records.

Sharing MUST NOT expose the vault master key or establish standing recipient access to the vault.

The sharing contract remains the authority for scope, purpose, consent, expiry, export and resharing controls.

## 15. AI boundary

Local AI may decrypt only the minimum context authorized for a particular request.

External AI requires an explicit external-processing policy/consent decision followed by minimization/redaction before transfer.

AI output is not automatically written back as a clinical fact. Derived interpretations require provenance and explicit classification.

## 16. Emergency boundary

The Emergency Profile is a separate disclosure artifact. Emergency presentation must never imply general vault access.

Break-glass access is out of scope for v1.

## 17. Non-goals

The v1 vault contract does not define:

- a hospital EHR;
- a universal clinical data model;
- a cloud-hosted health database;
- autonomous clinical authorization;
- cryptographic primitives invented by SOMA;
- automatic emergency break-glass access;
- unrestricted AI access;
- hidden analytics access to health content.

## 18. Implementation gate

Before production PHR persistence is considered ready, SOMA MUST have executable tests for at least:

1. unauthorized record access is denied;
2. plaintext is not persisted where the contract requires ciphertext;
3. tampered ciphertext fails authentication;
4. wrong key fails closed;
5. unsupported schema/envelope versions fail closed;
6. record-to-subject isolation is enforced;
7. AI/capability access is minimum-necessary;
8. export/share is explicit and auditable;
9. deletion prevents ordinary reads;
10. crash/interrupted-write recovery does not produce false success;
11. provenance survives import and transformation;
12. local indexes do not silently become plaintext vault copies;
13. backups are encrypted and separately governed;
14. emergency profile cannot escalate to full vault access;
15. external processing cannot occur without explicit authorization.

## 19. Architectural conclusion

The Health Vault is not simply a database. It is a **local security boundary around canonical health information**.

The implementation sequence is therefore:

`Vault Contract -> Cryptographic Design Review -> Storage Adapter -> Authorization Enforcement -> Adversarial Tests -> PHR Repository -> Intelligence Runtime`

No production health-data persistence should be declared complete until these gates are executable in CI.
