# Local Health Vault Storage & Index v1

**Status:** Architecture contract / implementation gate  
**Branch:** `architecture/local-health-vault-storage-index-v1`

## Purpose

Define the smallest provider-neutral persistence boundary required to turn the existing SOMA Local Health Vault encryption primitive into a durable local record store and encrypted index.

This is shared SOMA infrastructure. It is not a PHR-only database and must serve General Health, Athlete Performance, Service Veteran, Rehabilitation/Recovery, research contribution, and future governed contexts through the same vault boundary.

## Canonical boundary

`Canonical Health Record → Identity/Authorization → Policy Kernel → Vault Access Boundary → Key Lifecycle → Encrypted Local Record Store → Encrypted Index → Health Vault → PHR/Context Views`

The PHR remains a governed view/index over canonical records; this contract must not create a second canonical health model.

## Storage invariants

1. Protected health content is stored only as encrypted vault records.
2. The storage adapter never receives or persists raw key material outside the trusted key boundary.
3. `key_ref` is an opaque selector and is not authentication or authorization.
4. Every index entry is bound to exactly one `subject_ref`, classification, key reference and provenance reference.
5. Index metadata contains no plaintext health payload and is not a substitute for encrypted content.
6. Authorization is evaluated before record decryption.
7. Reads must not disclose existence of records outside the authorized subject/scope.
8. Writes and index publication must be atomic: a visible index entry must never point to an uncommitted or unverifiable ciphertext record.
9. Tombstoning is explicit and auditable; deletion semantics are separate from cryptographic erasure and recovery policy.
10. Provider implementations are replaceable. The domain contract must not depend on PostgreSQL, SQLite, mobile storage, cloud storage or a particular operating system.

## Required provider-neutral operations

The implementation contract should expose the minimum operations:

- `put(record, metadata)` — atomically persist encrypted record and index entry.
- `get(record_id, authorized_context)` — authorize first, then retrieve and authenticate ciphertext.
- `list(scope, authorized_context)` — return only authorized index metadata.
- `tombstone(record_id, authorized_context)` — atomically mark the record unavailable for normal reads.
- `verify(record_id, authorized_context)` — verify envelope/index integrity without exposing plaintext.

The concrete adapter may use transactions, journaling or an equivalent atomicity mechanism, but those mechanisms remain implementation details.

## Failure semantics

- Missing record: fail closed without revealing another subject's record existence.
- Invalid authorization: deny before decryption.
- Invalid key reference: fail closed.
- Authentication/tag failure: treat the record as integrity failure; never return plaintext.
- Index/ciphertext mismatch: fail closed and surface an auditable integrity error.
- Interrupted write: neither half of the record/index pair becomes a valid committed record.
- Tombstoned record: normal retrieval is denied.
- Provider failure: no fallback to plaintext, weaker protection, or an alternate ungoverned store.

## PHR integration rule

The personal health profile consumes authorized vault-backed record references. It may maintain derived views/indexes, but it must not become an independent canonical store for health facts. Contexts likewise reference canonical records rather than duplicating them.

## Explicit non-goals for v1

- No cloud sync implementation.
- No recovery UX or recovery-key protocol.
- No unrestricted administrative bypass.
- No direct AI access to vault keys or raw vault storage.
- No production claim for device/platform keystore integration.
- No separate vault/database per health context.
- No migration of legacy PostgreSQL vitals into the vault without a separately verified classification and migration plan.

## Verification gate

Before merge, executable tests must cover schema validity, provider neutrality, subject isolation, authorization-before-decryption, atomic visibility, tombstone semantics, tamper detection, index/ciphertext mismatch, and interrupted-write behavior. Exact-head CI must be green before merge.
