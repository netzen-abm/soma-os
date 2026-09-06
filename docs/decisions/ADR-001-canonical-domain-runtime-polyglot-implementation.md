# ADR-001 — Canonical Domain Runtime and Polyglot Implementation

**Status:** Accepted design baseline
**Date:** 2026-09-04

## Context

SOMA requires multiple implementation technologies for web delivery, scientific research, AI, browser integration, and security-sensitive infrastructure. Treating each language or framework as an independent domain authority would create policy drift, inconsistent data semantics, duplicated security logic, and difficult interoperability.

## Decision

SOMA will use a **canonical domain with multiple implementation runtimes where justified**.

Rust is designated the **Canonical Domain Runtime** and canonical/reference implementation for security-sensitive and domain-critical behavior.

Approved roles:

- **Rust:** canonical domain/runtime for security-sensitive and deterministic domain infrastructure.
- **WASM:** execution target for selected Rust capabilities in client environments.
- **Dioxus:** Rust/WASM web UI surface where appropriate.
- **Python:** AI, ML, scientific computing, research, evidence analysis, RAG, and data workflows.
- **Django:** Python application, administrative, research, and selected service/API workloads.
- **TypeScript/JavaScript:** browser, platform, ecosystem, and specialized web integration.

## Architectural rule

> **No implementation language may redefine canonical SOMA domain semantics.**

All non-canonical runtimes cross explicit contract/adapter boundaries.

There must be one authorization model and one canonical set of domain invariants, rather than independent policy implementations in Rust, Python, Django, or the browser.

## Consequences

### Positive

- Security and domain semantics remain coherent across surfaces.
- Rust can be reused through WASM where client-side execution is useful.
- Python's scientific and AI ecosystem remains available without displacing the domain authority.
- Dioxus can provide a Rust-native web surface without making the UI the security authority.
- TypeScript/JavaScript remains available for mature browser and ecosystem integrations.
- New runtimes can be admitted without changing canonical domain meaning.

### Constraints

- Every runtime adapter requires explicit contract conformance.
- Cross-runtime serialization/versioning must be maintained.
- Domain logic must not be casually copied for convenience.
- CI must eventually include contract and conformance testing across important runtimes.

## Security implications

The existing Policy Kernel and Identity → Authorization Enforcement boundaries remain authoritative. The Protected Data Access boundary must continue to use the same security contract when implemented in Rust/PostgreSQL.

PostgreSQL RLS, when introduced, is defense-in-depth and not a substitute for canonical authorization.

## Rejected alternatives

### Rust-only implementation language

Rejected because SOMA benefits materially from Python's scientific/AI ecosystem and JavaScript/TypeScript's browser ecosystem. Rust remains canonical without being the only language.

### Framework-specific domain authority

Rejected because it would couple security and domain semantics to Django, Dioxus, browser code, or any future framework.

### Independent policy engines per runtime

Rejected because divergent authorization semantics create unacceptable security and governance risk.

## Review trigger

Revisit this ADR if a new runtime needs to own security-sensitive canonical behavior, or if contract conformance cannot be maintained without duplicating domain authority.
