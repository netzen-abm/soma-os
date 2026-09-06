# SOMA — Canonical Domain Runtime & Polyglot Implementation Architecture v1

**Status:** Design baseline  
**Scope:** Web and cross-runtime architecture  
**Decision:** One canonical domain; multiple implementation runtimes where justified.

## 1. Executive decision

SOMA is **language-polyglot but domain-monolithic**.

The architectural authority is the canonical SOMA domain model and its contracts—not any particular programming language, framework, UI toolkit, transport, provider, or deployment surface.

**Rust is SOMA's Canonical Domain Runtime.** It is the canonical/reference implementation for security-sensitive and domain-critical behavior. Other languages may implement adapters, services, tooling, scientific workflows, or user interfaces where they provide a material advantage, but they must consume and conform to canonical SOMA contracts.

### Non-negotiable rule

> **No implementation language may redefine canonical SOMA domain semantics.**

This prevents semantic drift between Rust, Python, TypeScript/JavaScript, and future runtimes.

## 2. Canonical architecture

```text
                         SOMA CANONICAL DOMAIN
                                  │
                  ┌───────────────┴───────────────┐
                  │       Domain Contracts        │
                  │ Identity • Policy • Health    │
                  │ Evidence • Data Access • etc. │
                  └───────────────┬───────────────┘
                                  │
                    CANONICAL DOMAIN RUNTIME
                                  │
                                Rust
                                  │
             ┌────────────────────┼────────────────────┐
             │                    │                    │
            WASM              Server Runtime       Native
             │                    │                    │
          Dioxus              APIs/DB/Jobs          Services
             │                    │
          Web App                │
             │            ┌───────┴────────┐
             │            │                │
         JS/TS interop  Python          Other
                          │
                       Django
                          │
               AI • Research • RAG
               Scientific/Data Services
```

The diagram describes authority, not a requirement that every request pass through Rust synchronously. A service may execute independently, but the domain contract it implements remains canonical.

## 3. Runtime responsibilities

### 3.1 Rust — canonical domain runtime

Rust is the preferred canonical implementation for:

- Identity and authorization enforcement
- Protected data access boundaries
- Policy-sensitive operations
- Health-state and evidence contract enforcement
- Cryptographic and integrity-sensitive operations
- Provenance-preserving transformations
- Deterministic domain logic
- Persistence infrastructure and transaction boundaries
- Concurrency-sensitive infrastructure
- Security-critical validation
- WASM-shared domain capabilities

Rust does **not** automatically become the owner of every application feature. The criterion is domain authority and security sensitivity, not language preference.

### 3.2 WASM — client execution of canonical capabilities

WASM is an execution target, not a second domain runtime.

Where useful, selected Rust domain capabilities may be compiled to WASM for:

- Client-side validation
- Deterministic calculations
- Privacy-sensitive local processing
- Evidence/integrity verification
- Cryptographic verification
- Offline-capable workflows
- Shared browser/server behavior

WASM implementations must preserve the same canonical semantics as the Rust server/reference implementation.

### 3.3 Dioxus — Rust-native web application surface

Dioxus is an approved web UI option for Rust/WASM-native experiences.

It owns presentation and client interaction—not policy authority.

Dioxus components must call canonical capabilities/contracts rather than reimplementing authorization, evidence classification, or protected-data policy.

### 3.4 Python — scientific, AI, and research runtime

Python is a first-class implementation runtime where its ecosystem provides a material advantage, especially for:

- AI/ML
- RAG
- Scientific computing
- Statistics and analysis
- Research workflows
- NLP
- Evidence analysis
- Evaluation pipelines
- Data processing and experimentation

Python services must treat canonical SOMA contracts as external architectural authority.

### 3.5 Django — Python application/service framework

Django may provide:

- Research/back-office applications
- Administrative workflows
- Selected APIs
- Operational interfaces
- Python-native orchestration

Django must not become an independent authorization authority. Security-sensitive authorization remains governed by the canonical security contracts and enforcement boundary.

### 3.6 TypeScript/JavaScript — platform and browser integration

TypeScript/JavaScript may be used for:

- Browser APIs
- Platform integrations
- Third-party SDKs
- Specialized web libraries
- Progressive enhancement
- Accessibility tooling
- Maps and other specialized UI capabilities
- Compatibility with existing web ecosystems

TypeScript/JavaScript must not silently duplicate or reinterpret canonical domain policy.

## 4. Language boundary contract

Every non-canonical runtime must cross an explicit contract boundary:

```text
Implementation Language
        │
        ▼
Adapter / Service Boundary
        │
        ▼
Canonical SOMA Contract
        │
        ▼
Canonical Domain Semantics
```

The prohibited pattern is:

```text
Implementation Language
        │
        ▼
Independent interpretation of domain semantics
```

### Boundary requirements

Each adapter/service should define:

1. Which canonical contracts it consumes.
2. Which canonical capabilities it invokes.
3. Which data it may read/write.
4. Which identity context it accepts.
5. Which authorization decision is authoritative.
6. How provenance and uncertainty are preserved.
7. How failures and degraded modes are represented.
8. Which version of the contract it implements.

## 5. Security authority

There must be **one authorization model**, not one authorization implementation per language.

The current Policy Kernel and Identity → Authorization Enforcement work establish the canonical security boundary. The emerging Protected Data Access boundary must preserve this property when the Rust database adapter is implemented.

A Python, Django, TypeScript, Dioxus, or external adapter may request authorization, but it must not invent a parallel grant model.

Likewise, PostgreSQL RLS is defense-in-depth for protected persistence. It is not a replacement for the canonical authorization contract.

## 6. Data authority

Canonical data contracts define meaning and invariants.

A runtime may maintain its own internal representation for performance or integration purposes, but translation must be explicit and loss-aware.

For health and evidence data, translations must not silently discard:

- Provenance
- Uncertainty
- Classification
- Source identity
- Applicability
- Safety information
- Temporal meaning
- Authorization scope

Unknown information must remain unknown rather than being inferred merely because a target language or framework has a narrower model.

## 7. Web architecture decision

The preferred Web architecture is therefore:

**Rust + WASM + Dioxus + Python + Django + TypeScript/JavaScript**, with role separation.

This is intentionally a **polyglot web architecture**, not a polyglot domain architecture.

### Recommended deployment shape

```text
Browser
 ├── Dioxus/WASM — canonical Rust-derived client capabilities
 └── TypeScript/JS — browser/platform integrations
          │
          ▼
      Web/API Gateway
          │
    ┌─────┴─────────────┐
    │                   │
 Rust services      Django/Python services
    │                   │
    │              AI / RAG / Research
    │                   │
    └─────────┬─────────┘
              ▼
       Shared data/access
          contracts
              │
              ▼
       PostgreSQL/storage
```

This shape is illustrative. It does not mandate a single deployment topology.

## 8. Independent surfaces

The web surface must remain an adapter/surface, consistent with SOMA's broader ecosystem architecture.

Future surfaces may include:

- Web
- Android
- iOS
- Telegram
- WhatsApp
- Messenger
- Agent interfaces
- MCP
- Nostr
- Web3/DID/VC

A surface failure must not become a domain failure.

Conversely, a surface must not bypass shared security, evidence, or data-access boundaries merely because its implementation language differs.

## 9. Versioning and compatibility

Canonical contracts are versioned independently of implementation languages.

For example:

```text
health-state-v1
identity-context-v1
authorization-enforcement-context-v1
protected-data-access-v1
```

A runtime may upgrade independently only when it remains compatible with the relevant contract.

Breaking contract changes require an explicit migration/version decision. They must not be hidden inside a framework or language upgrade.

## 10. Testing strategy

Testing must operate at three levels:

### Contract tests

Verify language-neutral semantics and schema invariants.

### Reference-runtime tests

Verify the canonical Rust implementation.

### Adapter/conformance tests

Verify Python, Django, TypeScript/JavaScript, WASM, Dioxus, and future implementations conform to the canonical contracts.

Security-critical conformance must include negative/adversarial cases, not only successful examples.

## 11. Decision criteria for introducing another runtime

A new language/framework is justified only when at least one of the following is materially true:

- It provides a capability that would be disproportionately expensive in Rust.
- It provides a substantially stronger ecosystem for a specialized workload.
- It is required for a platform integration.
- It materially improves developer or user experience without weakening domain invariants.
- It provides a bounded experimental/research capability with a clear adapter boundary.

Convenience alone is insufficient justification for duplicating domain logic.

## 12. Anti-patterns explicitly prohibited

- Multiple independent Policy Kernels.
- Language-specific interpretations of authorization semantics.
- Client-side authorization treated as server authority.
- Django becoming the canonical identity/authorization authority.
- AI models becoming policy authorities.
- Database RLS being treated as the complete authorization model.
- Copying domain rules into TypeScript for convenience.
- Copying security rules into Python because an API is Python-based.
- Silent loss of evidence provenance or uncertainty during translation.
- Making an optional runtime or protocol a hard dependency of the core domain.

## 13. Relationship to current SOMA work

This architecture is intentionally compatible with the current consolidation sequence:

1. Policy Kernel remains the shared authorization contract.
2. IdentityContext remains the canonical identity/scope representation.
3. Identity → Authorization Enforcement remains the authorization enforcement boundary.
4. Protected Data Access becomes the provider-neutral data-access gate.
5. Rust becomes the canonical/reference runtime for the security-sensitive enforcement path.
6. Python/Django remains available for AI, research, and application workloads without creating a second security authority.
7. WASM/Dioxus can reuse selected Rust capabilities for the web.

The Protected Data Access implementation must therefore continue toward a Rust PostgreSQL adapter and trusted transaction-local database context rather than moving authorization logic into a new web framework.

## 14. Architectural vocabulary

Use:

> **SOMA is language-polyglot but domain-monolithic.**

> **One canonical domain. One contract system. One set of invariants. One canonical domain runtime. Multiple implementation runtimes and independent surfaces.**

> **Rust is SOMA's Canonical Domain Runtime and canonical/reference implementation for security-sensitive domain behavior.**

Avoid:

> "Rust is SOMA's only programming language."

> "Django owns SOMA authorization."

> "The frontend enforces security."

> "Each service may implement its own policy."

## 15. Acceptance criteria

This architecture is accepted when:

- The canonical domain/runtime terminology is documented.
- Rust's authority is explicitly bounded to canonical/reference domain behavior.
- Python/Django responsibilities are explicit.
- WASM/Dioxus responsibilities are explicit.
- TypeScript/JavaScript responsibilities are explicit.
- Cross-runtime contract boundaries are explicit.
- No implementation language is permitted to redefine canonical semantics.
- Security authority remains singular.
- Evidence/provenance/uncertainty preservation is explicit.
- Optional surfaces and runtimes remain independently deployable where practical.
- Future runtime additions have a defined admission criterion.

## 16. Status

**Architecture decision:** Adopted as the design baseline for SOMA Web and cross-runtime implementation.

**Implementation status:** Documentation only. This document does not by itself authorize changes to security or data-access semantics.

**Next implementation dependency:** Continue Protected Data Access Enforcement v1, with the Rust PostgreSQL adapter and trusted transaction-local database context as the next engineering boundary.
