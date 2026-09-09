# SOMA Hybrid Web Platform v1

**Status:** Design baseline
**Scope:** Web application architecture and runtime boundaries
**Depends on:** Canonical Domain Runtime / Polyglot Implementation Boundary v1

## 1. Decision

SOMA Web will use a hybrid execution model:

- **Rust** is the canonical domain runtime and canonical/reference implementation for domain-critical and security-sensitive behavior.
- **WebAssembly (WASM)** is an execution target for reusable Rust domain capabilities in the browser. WASM is not a second domain authority.
- **Dioxus** is the preferred Rust/WASM presentation and interactive application framework where its model provides clear value. Dioxus owns presentation and client interaction, not authorization or canonical policy.
- **TypeScript/JavaScript** is permitted for browser-native capabilities, platform integrations, ecosystem libraries, progressive enhancement, and UI components where justified. It is an adapter/integration runtime, not a competing domain runtime.
- **Python** is the specialized runtime for AI/ML, RAG, scientific computation, research workflows, and other workloads where its ecosystem materially improves capability.
- **Django** may be used for bounded Python application/admin/API surfaces where justified; it does not redefine canonical SOMA domain semantics.
- **PostgreSQL** remains persistence infrastructure and defense-in-depth. Database controls do not replace application authorization.

The architecture therefore follows:

> **One canonical domain. One contract system. One canonical domain runtime. Multiple bounded implementation runtimes. Independent surfaces.**

## 2. Why Hybrid

A single-language Web stack would create unnecessary constraints. A completely polyglot domain would create semantic drift. SOMA needs the middle path: runtime specialization without domain fragmentation.

Hybrid Web execution provides:

1. Rust-level correctness and reuse for sensitive domain logic.
2. WASM delivery of selected Rust capabilities to the browser.
3. Dioxus for coherent Rust/WASM application surfaces.
4. TypeScript access to browser and Web platform capabilities without forcing those concerns into Rust.
5. Python access to the scientific and AI ecosystem without making Python a second source of truth for security or domain policy.
6. Progressive adoption: existing and future Web surfaces can migrate capability-by-capability rather than through a risky rewrite.

## 3. Runtime Ownership

| Runtime | Owns | Must not own |
|---|---|---|
| Rust | canonical domain logic, security-sensitive logic, policy enforcement, evidence semantics, deterministic rules, persistence infrastructure, concurrency | browser-specific presentation decisions when a bounded adapter is sufficient |
| Rust/WASM | selected reusable domain capabilities executed in-browser | independent authorization model or divergent domain semantics |
| Dioxus | presentation, stateful UI interaction, client orchestration | policy authority, grant evaluation, scope inference |
| TypeScript/JS | browser APIs, Web APIs, third-party browser libraries, integration adapters, progressive enhancement | canonical health/evidence/security semantics |
| Python | AI/ML, RAG, scientific/research computation, evidence-analysis tooling | authoritative authorization, identity scope, safety policy semantics |
| Django | bounded Python application/admin/API concerns | redefining Rust canonical domain contracts |
| PostgreSQL | persistence, constraints, RLS defense-in-depth, transactional integrity | becoming the primary authorization engine |

## 4. Canonical Request Flow

```text
Browser
  |
  +-- Dioxus/WASM ------------------+
  |                                  |
  +-- TypeScript/Web APIs -----------+--> Web adapter boundary
                                     |
                                     v
                             SOMA API / Gateway
                                     |
                                     v
                           Identity + Authorization
                                     |
                                     v
                              Rust Domain Runtime
                                     |
                    +----------------+----------------+
                    |                                 |
                    v                                 v
               PostgreSQL                     Python capability
             persistence layer                AI / ML / RAG
                    |                                 |
                    +---------------+-----------------+
                                    v
                              Evidence / Audit
```

The diagram describes responsibility, not a requirement that every request traverse every runtime.

## 5. Security Boundary

The browser is an untrusted execution environment. WASM and TypeScript code must therefore be treated as clients/adapters unless and until a capability is independently verified at the server/domain boundary.

Rules:

- Never trust browser-supplied tenant, data-domain, principal, authorization, safety, or evidence classification fields.
- Never infer authorization from UI state, route parameters, JavaScript metadata, WASM memory, device metadata, or model output.
- Authorization must terminate at the canonical authorization boundary before protected persistence or protected operations.
- Protected PostgreSQL context must be established only from trusted authorization output.
- Client-side checks are UX and early-failure controls, not security controls.
- Secrets and credentials must not be embedded in WASM bundles or browser JavaScript.
- AI-generated claims cannot directly become canonical health state or safety decisions without the applicable governance/evidence gates.

## 6. Contract Boundary

All runtimes communicate through canonical, versioned contracts. A language binding may translate representation, but may not alter semantics.

At minimum, the boundary must preserve:

- identity and authentication provenance;
- tenant and data-domain scope;
- authorization decision semantics;
- health-state entity types;
- evidence provenance;
- uncertainty;
- safety classification;
- auditability;
- schema/version information.

If a runtime cannot represent a canonical field safely, the capability must fail closed or remain outside that runtime; it must not silently discard the field.

## 7. WASM Strategy

WASM is used selectively.

Good candidates:

- deterministic health-state transformations;
- client-side validation that mirrors canonical schemas;
- privacy-preserving local computation;
- cryptographic verification where appropriate;
- offline-capable domain calculations;
- performance-sensitive deterministic algorithms.

Poor candidates:

- authoritative authorization;
- secrets requiring server-side protection;
- direct privileged database access;
- trust decisions based solely on browser execution;
- duplicated business-policy engines.

When a WASM implementation mirrors Rust domain behavior, the Rust implementation remains canonical and conformance tests must prevent semantic drift.

## 8. TypeScript Strategy

TypeScript is a first-class integration language, not a second domain language.

Permitted responsibilities include:

- Web APIs;
- service workers and browser lifecycle integration;
- WebAuthn/passkey browser integration;
- push notifications;
- file and device APIs;
- third-party SDKs;
- accessibility/browser-specific interaction;
- analytics and telemetry adapters subject to privacy policy;
- UI components where ecosystem value outweighs cross-compilation value.

TypeScript may orchestrate calls but must not independently decide whether an operation is authorized.

## 9. Python Strategy

Python remains the preferred specialized runtime for:

- AI/ML;
- retrieval and RAG;
- scientific computation;
- evidence analysis;
- research pipelines;
- experimentation.

Python services consume and emit canonical contracts. They do not create a parallel identity, authorization, safety, or health-state authority.

## 10. Deployment Model

The hybrid architecture should permit independent deployment of:

- static/browser assets;
- WASM bundles;
- Rust API/domain services;
- Python AI/research services;
- PostgreSQL;
- optional integration services.

A failure in one optional runtime must degrade only the dependent capability where possible. For example, AI unavailability must not make basic account, health-state, evidence provenance, or core deterministic functions unavailable unless that capability explicitly requires AI.

## 11. Observability

Every cross-runtime boundary should be observable without leaking sensitive health data.

Required properties:

- request/correlation identifiers;
- capability identifiers;
- schema/contract versions;
- runtime/adapter identity;
- success/failure/degradation outcome;
- authorization outcome where appropriate;
- audit event linkage for protected operations.

Logs must not become an alternate health-data store.

## 12. Performance Strategy

Performance decisions are capability-specific.

- Use WASM for computation that benefits from browser-local execution.
- Use server-side Rust for authoritative or latency-sensitive domain operations.
- Use TypeScript for browser-native operations rather than crossing the WASM boundary unnecessarily.
- Use Python asynchronously for heavyweight AI/scientific workloads.
- Avoid sending sensitive data to a remote capability merely to obtain a computation that can safely run locally.

Do not optimize by moving authority into the client.

## 13. Progressive Architecture

SOMA does not require a full Web rewrite before delivering value.

Migration order:

1. Preserve canonical contracts.
2. Establish Rust domain boundaries.
3. Introduce WASM only for selected deterministic capabilities.
4. Introduce Dioxus for application surfaces where it provides net architectural value.
5. Retain TypeScript for browser/integration boundaries.
6. Connect Python AI/research capabilities through explicit service contracts.
7. Remove duplicate domain implementations only after conformance evidence exists.

Archive legacy implementations before removal.

## 14. Non-Negotiable Invariants

1. No client runtime is a security authority.
2. No second authorization engine may emerge in TypeScript, Python, Dioxus, WASM, or another surface.
3. No runtime may broaden identity scope.
4. No runtime may silently discard provenance, uncertainty, or safety metadata.
5. PostgreSQL RLS is defense-in-depth, not the sole application authorization mechanism.
6. AI is optional and user-controlled where applicable; AI failure must not silently change policy semantics.
7. WASM is an execution target, not a second domain runtime.
8. Language choice never changes domain semantics.
9. Independent surfaces may fail independently.
10. Canonical contracts are versioned and tested across runtime boundaries.

## 15. Acceptance Criteria

This architecture is considered implemented only when the following are demonstrated:

- [ ] Web runtime boundaries are represented in repository architecture documentation.
- [ ] Rust remains the canonical domain/reference implementation.
- [ ] WASM conformance tests demonstrate parity for every duplicated browser-executed domain capability.
- [ ] TypeScript adapters cannot bypass authorization boundaries.
- [ ] Python services consume canonical contracts and cannot redefine security semantics.
- [ ] Cross-runtime contract/version tests exist.
- [ ] Browser security assumptions are documented and tested.
- [ ] Failure/degradation behavior is verified for unavailable Python/AI and optional integrations.
- [ ] Protected data operations cannot bypass identity/authorization enforcement.
- [ ] Deployment topology and observability are documented.

## 16. Decision Summary

SOMA should adopt a **hybrid Web platform**, not a single-language Web platform and not an unconstrained polyglot architecture.

The governing rule is:

> **Rust defines the canonical domain. WASM carries selected Rust capabilities into the browser. Dioxus presents those capabilities. TypeScript integrates with the Web. Python supplies specialized intelligence. Canonical contracts keep them one SOMA.**
