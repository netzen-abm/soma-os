# SOMA Personal Health Record Repository v1

**Status:** Architecture contract / implementation baseline  
**Version:** 1.0.0  
**Date:** 2026-09-10

## Purpose

Define the provider-neutral repository boundary for a user's Personal Health Record (PHR) after the Local Health Vault Storage & Encrypted Index layer.

The PHR repository is a **view and repository interface over canonical health records in the Local Health Vault**. It must not become a second source of truth or introduce a parallel health domain model.

## Architectural position

```text
User / Application Surface
          |
          v
    PHR Repository API
          |
   Authorization / Policy
          |
   Local Vault Storage API
          |
   Encrypted Health Records
```

The repository owns retrieval, indexing, querying, and lifecycle coordination. The Local Health Vault owns protected record storage and cryptographic confidentiality/integrity.

## Canonical data rule

`Health State` and other canonical health entities remain the domain model. The existing Personal Health Profile remains a user-controlled index/view that references canonical records. The repository must therefore store references and repository metadata rather than copying clinical payloads into an independent PHR model.

The Health State contract is a canonical longitudinal health-state entity, not a diagnosis or declaration of clinical truth. The Personal Health Profile is a local-first index/view that references canonical SOMA entities rather than replacing the canonical model.

## Required repository operations

The implementation contract should provide, at minimum:

- `put_reference` — register a canonical vault record reference after authorization and integrity validation;
- `get` — retrieve an authorized canonical record through the vault boundary;
- `list` — enumerate only records visible to the requesting subject/principal;
- `query` — filter/index records without exposing plaintext to unauthorized callers;
- `timeline` — return chronologically ordered authorized references/records;
- `tombstone` — coordinate logical deletion through the vault lifecycle;
- `verify` — verify repository/index references against the vault record and provenance;
- `rebuild_index` — reconstruct derived repository indexes from authoritative vault records without creating a second source of truth.

## Mandatory security ordering

For protected health records:

```text
Request
  -> Authentication/security context (if present)
  -> Authorization
  -> Subject/scope isolation
  -> Vault record lookup
  -> Integrity verification
  -> Key resolution
  -> Decryption
  -> Canonical record validation
  -> Repository projection/query result
```

Authorization must occur **before** key resolution or decryption. Repository indexing must not become a side channel that exposes protected health content.

## Data minimization

The repository may retain identifiers, classification, entity type, timestamps, provenance references, and other metadata required for indexing. It must not duplicate encrypted health payloads as a plaintext cache.

Search indexes containing sensitive metadata must themselves be protected according to their classification.

## Subject isolation

A repository instance must never permit one subject to enumerate, query, read, tombstone, or verify another subject's records unless an explicit policy grant authorizes that access. Emergency and delegated access remain separate policy-governed paths.

## Failure semantics

The implementation must fail closed for:

- unauthorized subject or scope;
- missing vault record;
- invalid/mismatched reference;
- vault integrity failure;
- unavailable key material;
- malformed canonical record;
- stale or invalid index binding;
- tombstoned record access.

No failure may be silently converted into an empty result when that would conceal an authorization, integrity, or storage failure.

## Provider neutrality

The first implementation may use the existing Local Health Vault filesystem adapter. PostgreSQL, SQLite, mobile storage, cloud synchronization, and other providers must implement this contract rather than redefine it.

The repository contract therefore remains independent of storage technology.

## PHR relationship

The PHR is a user-facing projection over this repository. Athlete, Service Veteran, Rehabilitation/Recovery, and General Health contexts must continue to reference the same canonical records and repository; they must not create parallel health databases.

## Out of scope for v1

- cloud synchronization;
- autonomous clinical decision-making;
- prescribing or medication changes;
- provider-specific database schema as the canonical model;
- automatic import trust;
- emergency disclosure bypasses;
- cross-subject access without an explicit policy grant;
- autonomous deletion of underlying vault records;
- external AI access to vault contents without policy authorization and minimum-necessary data minimization.

## Acceptance gates

Before implementation merge:

1. Contract tests validate schema and architectural invariants.
2. Unit/integration tests cover authorization ordering and subject isolation.
3. Tests cover index corruption, stale references, missing records, tombstones, and rebuild behavior.
4. Tests verify no plaintext protected payload is persisted in the repository index.
5. Rust formatting, compile, lint, and existing SOMA CI gates pass.
6. Evidence/security workflows pass at the exact PR head.

Security-sensitive implementation changes must not be force-merged.
