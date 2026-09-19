# SOMA-OS Programming-File Audit — 2026-09-19

Status: active audit baseline
Baseline: `main` at `0f9443eef12a54ea98b30c14603545fe7b42c35a`

## Purpose

This audit covers executable/programming surfaces visible in the repository: Rust backend code, shared Python infrastructure, evidence-research adapters/core/orchestration, client code, network/bot adapters, migration/legacy code, database access/migrations, scripts and tests.

The audit follows the repository's archive-first rule. It does not delete or rewrite code merely because it is old.

## Executive findings

### P0 — resolve before expanding the authorization surface

The Python authorization path is semantically behind the canonical Rust authorization model.

Canonical Rust requests distinguish:

`principal_ref` = actor

`subject_ref` = health/data subject

The Python path currently has:

- `PolicyRequest.principal_id`
- `PolicyRequest.principal_type`
- capability/resource/action
- tenant/data-domain target
- no canonical `subject_ref`

`ProtectedDataRequest` likewise carries authorization context, capability/resource/action and target tenant/data domain, but no explicit target subject.

This matters for delegated access: clinician, caregiver, researcher, service, agent, or another authorized actor must not be collapsed into the person whose health data is being accessed.

Current Rust implementation already enforces the actor/subject distinction. The Python implementation must converge rather than become a second semantic model.

**Disposition:** CONSOLIDATE into the canonical principal → subject → tenant/data-domain → capability → resource → action model. Do not create another authorization engine.

### P1 — evidence architecture is converging, but provider contracts need one final runtime audit

Current research code contains:

- PubMed adapter
- OpenAlex adapter
- Unpaywall access resolver
- other public-data adapters
- canonical `ResearchRecord`
- `ResearchAccessLocation`
- multi-source orchestration
- Evidence Research Core
- explicit downstream screening/extraction/directness/quality/safety/causality/synthesis boundaries

The architecture is directionally coherent. Before adding Semantic Scholar or DOAJ, verify every adapter uses the same normalized identity, provenance, failure and authorization semantics.

**Disposition:** KEEP / CONVERGE. No second evidence engine.

### P1 — protected-data access has multiple implementation layers, but their roles are documented as distinct

Relevant active boundaries include:

- `services/shared/authorization_policy_decision_boundary.py`
- `services/shared/identity_authorization_enforcement.py`
- `services/shared/policy_kernel.py`
- `services/shared/protected_data_access.py`
- `services/backend-rust/src/canonical_authorization.rs`
- `services/backend-rust/src/protected_db_context.rs`
- `services/backend-rust/src/canonical_vault_authorizer.rs`

The repository documentation explicitly treats the Python boundary as the canonical composition boundary and the Rust contract as an adapter/decision consumer rather than a second policy engine.

The audit found no evidence in the searched active paths of a third independent policy engine. The remaining concern is semantic convergence, especially subject binding.

**Disposition:** KEEP the layers; CONSOLIDATE semantics.

### P1 — direct protected database access is tightly concentrated

The protected-data audit identifies the principal Rust SQL paths:

- `services/backend-rust/src/db_layer.rs`
- `services/backend-rust/src/protected_db_context.rs`
- legacy promotion paths
- PostgreSQL security integration tests

The repository also has `scripts/audit_protected_data_access.py` and CI wiring for that audit.

**Disposition:** KEEP and continue exact-head protected-data gates.

### P1 — legacy/migration executable surface requires explicit lifecycle classification

Executable legacy/migration code includes:

- `legacy_data_classification.py`
- `legacy_data_promotion.py`
- `legacy_promotion.rs`
- `legacy_promotion_preflight.rs`
- medical migration scripts
- review-queue builders
- source-audit/verification utilities

These must not be casually deleted. Their correct classification is:

1. active production path;
2. migration-only path;
3. audit/verification tooling;
4. compatibility boundary;
5. archive candidate.

**Disposition:** CLASSIFY first; archive only with evidence.

## Programming surface inventory

### Rust backend

High-value canonical domains found:

- canonical authorization
- protected DB context
- local health vault
- vault authorization adapter
- health state repository
- intervention
- measurement
- outcome
- evidence linkage
- longitudinal observation repository
- longitudinal context
- personal health record repository
- crypto/key lifecycle
- database layer
- Nostr/network adapters
- bot/webhook adapters
- legacy promotion

### Shared Python

High-value shared domains:

- policy kernel
- identity authorization enforcement
- authorization/policy decision boundary
- protected-data access
- legacy classification/promotion
- MCP adapter contracts
- capability registry
- agent platform contracts

### Evidence Research

Current executable layers:

- external knowledge source contract
- research source contract
- PubMed
- OpenAlex
- Unpaywall
- OWID
- UN Data
- data.gov.in
- multi-source orchestration
- Evidence Research Core
- screening
- extraction
- directness
- quality/bias
- safety
- consistency
- causality
- synthesis

### Client and transport surfaces

Rust/Dioxus and transport/network code includes:

- client main/UI modules
- file/dialogue handling
- multimodal UI
- fermentation tracker
- Nostr client/listener/WASM client
- WhatsApp/Messenger webhook/outbound adapters
- bot menus
- localization

These should remain adapters/surfaces and must not redefine canonical authorization, health semantics or evidence semantics.

### Scripts/tests

There is a substantial verification surface covering:

- architecture governance
- protected-data access
- health contracts
- identity/authorization
- capability registry
- evidence canonical mapping
- medical-source verification
- migration
- research adapters
- evidence-core stages
- vault/key lifecycle
- longitudinal observations

This is a strength, but the audit should continue checking that test-only compatibility models are not mistaken for production authority.

## Security patterns checked

Searches across active code found no matches for:

- `eval(`
- `exec(`
- `pickle`
- `yaml.load`
- `requests.get`

Network access is concentrated in explicit research adapters and transport/network modules.

The Rust backend uses `sqlx` for database access. Protected SQL paths are covered by the repository's protected-data audit and PostgreSQL integration gates.

## Data-flow concern requiring continued attention

Local health-vault code correctly separates:

`principal → subject → scope → authorization → decryption/access`

The PHR/local repository code uses subject-bound vault paths and protected index metadata.

The audit should continue to ensure every new protected repository follows the same sequence and cannot construct an authorization context from arbitrary caller strings.

## AI/agent authority boundary

The searched architecture and code continue to support the rule that AI/agents are consumers/executors of governed capabilities, not authorization authorities.

No new AI authorization engine was found during this audit.

The existing research and governed-operation contracts also keep provider/AI output separate from evidence authority.

## KEEP / CONSOLIDATE / MIGRATE / ARCHIVE / DELETE

### KEEP

- canonical Rust authorization contract
- Python Policy Kernel
- Python authorization composition boundary
- protected-data enforcement boundary
- canonical health-state repositories
- Evidence Research Core
- provider adapters using canonical research contracts
- migration/preflight code that is still required
- verification/audit scripts
- contract tests

### CONSOLIDATE

- Python authorization semantics with Rust principal/subject model
- provider identity/provenance semantics
- research adapter failure semantics
- access-location enrichment semantics
- any remaining duplicate health/evidence authorization context representations

### MIGRATE

- compatibility vault authorization interfaces when a real production provider can replace them safely
- older migration flows when their active replacement is verified
- any remaining provider-specific normalized research record models

### ARCHIVE CANDIDATES

Only after dependency/build/runtime evidence confirms they are not active:

- `services/backend-rust/archive/*`
- superseded migration implementations
- obsolete provider-specific prototypes
- historical CI implementations

### DELETE

No programming-file deletion is justified by this audit yet.

Deletion remains evidence-gated.

## Immediate priority

The highest-value programming gap is **not another research provider**.

It is convergence of the Python protected-data authorization path with the already-canonical Rust actor/subject model.

Target:

`Principal → Identity → Authorized Subject Scope → Tenant/Data Domain → Capability → Resource → Action`

The implementation should preserve generic research operations while requiring explicit subject binding wherever the capability operates on protected health/data subject records.

## Audit conclusion

SOMA-OS currently has a strong shared-infrastructure shape, but the programming surface is large enough that feature expansion can hide semantic duplication.

The correct next engineering sequence is:

1. finish programming/security convergence;
2. make authorization subject-aware across all active runtimes;
3. finish evidence adapter convergence;
4. classify legacy/migration code;
5. run complete build/test/security gates;
6. only then add further research providers or product capabilities.

No broad cleanup deletion is justified yet.
