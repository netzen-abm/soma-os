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

The PHR schemas establish a local-first product boundary and reference canonical health entities. The repository already contains a provider-neutral encrypted record primitive, an encrypted AES-256-GCM index, authenticated record/key-reference binding, and a file-backed storage adapter. The remaining work is to reconcile this contract with that implementation and prove each production guarantee through targeted adversarial tests. Do not add a parallel envelope abstraction unless a demonstrated requirement is not represented by the existing record/index contract.

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

### 18.1 Initial implementation-to-evidence matrix

This is a conservative source review, not a claim that the production system has passed all gates. “Proven” means a narrowly scoped behavior has a named implementation/test; it does not certify the entire security objective. Workflow names and test paths must be revalidated against the exact PR head before promotion.

| Gate | Initial status | Existing evidence | Remaining proof / smallest useful next test |
|---|---|---|---|
| 1. Unauthorized record access denied | **Partial** | `local_health_vault_storage_store.rs` authorizes `put/get/list/tombstone/verify`; storage tests and `local_health_vault_storage_list_authorization_tests.rs` cover authorization ordering. | Add table-driven denial tests for every action and assert no key resolution, disk mutation, or plaintext return after denial. |
| 2. Plaintext not persisted where ciphertext is required | **Partial** | `local_health_vault.rs` provides encrypted record primitives; `local_health_vault_storage_crypto.rs` encrypts the index. | Write a representative sensitive payload, inspect every persisted bundle byte/string, and assert sensitive plaintext is absent while permitted metadata is explicitly enumerated. |
| 3. Tampered ciphertext fails authentication | **Proven narrowly** | `local_health_vault_storage_tests.rs::ciphertext_tampering_fails_integrity`; record and index crypto validators return integrity errors. | Extend tampering coverage to record ciphertext, index ciphertext, nonce, tag, and authenticated record/key references independently. |
| 4. Wrong key fails closed | **Partial** | Record/index decryption errors are mapped to integrity failures; key resolution failures are explicit. | Add tests using a validly shaped bundle and wrong key for both record and index; assert no plaintext and no mutation. |
| 5. Unsupported schema/envelope versions fail closed | **Partial** | `decrypt_index` rejects an index schema version other than `INDEX_SCHEMA_VERSION`. | Test unsupported record schema/envelope versions separately, including read, list, and migration paths. |
| 6. Record-to-subject isolation | **Proven narrowly** | `get` loads beneath the authorized subject path; `put` binds context subject to record subject; `validate_binding` checks subject; list checks decrypted subject. | Add adversarial path/reference substitution tests across put/get/list/tombstone and assert no cross-subject disclosure. |
| 7. AI/capability access is minimum-necessary | **Missing in this storage-path review** | The reviewed vault store enforces vault actions but does not itself demonstrate external-AI minimization. | Test a governed AI request with excess fields and prove authorization plus redaction/minimization happens before any provider call. |
| 8. Export/share is explicit and auditable | **Missing in this storage-path review** | No export/share implementation or audit path was established by the inspected files. | Add an end-to-end denial-by-default test; require purpose, selected records, recipient, expiry, consent, and audit before producing export bytes. |
| 9. Deletion prevents ordinary reads | **Proven narrowly** | `tombstone` updates record and encrypted index state; `get` rejects tombstoned records; repository tests cover tombstone visibility. | Test restart/reopen, list, repository lookup, and malformed/tampered tombstone states. Do not equate tombstoning with physical erasure. |
| 10. Interrupted writes do not produce false success | **Partial** | `atomic_write` writes a temporary file, calls `sync_all`, and renames; it returns errors on failed write/sync/rename. | Add fault-injection tests around each write/sync/rename boundary and restart recovery; document filesystem/directory durability assumptions. |
| 11. Provenance survives import/transformation | **Partial** | The record/index contract carries `provenance_ref`; `validate_binding` checks it against the record. | Test import and transformation pipelines retain source, transformation lineage, timestamps, and provenance references end-to-end. |
| 12. Index does not become a plaintext health-data copy | **Partial** | Index payload is encrypted with AES-256-GCM and AAD binds record/key references. The serialized bundle still contains some record/index envelope metadata. | Enumerate all persisted fields and test that sensitive clinical values are absent from plaintext metadata, filenames, logs, and index records. |
| 13. Backups are encrypted and separately governed | **Missing in this storage-path review** | No backup/restore guarantee is established by the inspected vault store and crypto files. | Add backup/restore integration tests with separate key-access policy, integrity checks, and denial when required keys are unavailable. |
| 14. Emergency profile cannot escalate to full vault access | **Missing in this storage-path review** | No emergency-specific authorization proof was established by the inspected files. | Add adversarial tests showing emergency scope is record/purpose-limited and cannot read/list unrelated records or resolve unrelated keys. |
| 15. External processing requires explicit authorization | **Missing in this storage-path review** | The reviewed storage adapter does not establish the complete external-provider boundary. | Use a fake provider and prove no network/provider invocation occurs without consent/policy approval; assert redaction precedes invocation. |

**Promotion rule:** do not label a gate “proven” solely because a function exists or a general CI workflow passes. Require a specific test asserting the relevant invariant, run on the exact PR head. Where the test or production boundary is absent, keep the status **missing** and implement only the smallest testable slice.

## 19. Architectural conclusion

The Health Vault is not simply a database. It is a **local security boundary around canonical health information**.

The implementation sequence is therefore:

`Vault Contract -> Cryptographic Design Review -> Storage Adapter -> Authorization Enforcement -> Adversarial Tests -> PHR Repository -> Intelligence Runtime`

No production health-data persistence should be declared complete until these gates are executable in CI.
