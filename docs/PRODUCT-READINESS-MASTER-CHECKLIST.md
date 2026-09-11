# SOMA — Product Readiness Master Checklist

**Status:** Living master checklist  
**Objective:** Move SOMA from consolidated engineering baseline to a deployable, secure, maintainable product without weakening shared-infrastructure or governance boundaries.

> This checklist is the canonical execution/status tracker. A task is not `DONE` because code exists; it is `DONE` only when the applicable implementation, verification, security/privacy, failure behavior, documentation, and deployment gates have been checked.

## Status vocabulary

- `DONE` — implemented and verified.
- `IN PROGRESS` — actively being implemented.
- `BLOCKED` — cannot safely proceed until a prerequisite is resolved.
- `PLANNED` — accepted work, not started.
- `DEFERRED` — intentionally postponed with a reason.
- `NOT APPLICABLE` — explicitly ruled out.

---

## 0. Current execution gate — 2026-09-12

### P0 — Trustworthy CI

- [x] Canonical GitHub CI workflows use GitHub-hosted `ubuntu-latest` runners.
- [x] Active workflow definitions contain executable steps.
- [x] Actions permissions were reviewed from repository settings evidence.
- [x] Repeated PR #82/#83 jobs were inspected through the GitHub Actions API.
- [x] Repeated pre-execution signature recorded: `runner_id = 0`, empty runner name, `steps = []`.
- [x] Job-log retrieval failure recorded as `BlobNotFound`.
- [x] CI infrastructure issue recorded in GitHub Issue #84.
- [ ] Verify GitHub Actions-specific usage/quota/account execution state.
- [ ] Restore actual runner assignment.
- [ ] Obtain at least one CI run with real executable steps and logs.
- [ ] Establish trustworthy green CI baseline.

**Gate rule:** A pre-execution Actions failure is classified as CI infrastructure failure, not as evidence that repository tests failed. Do not weaken, bypass, or force-merge around this gate.

### P1 — Shared security foundation

- [x] IdentityContext contract exists.
- [x] Identity Authorization Enforcement exists.
- [x] Policy Kernel exists and current implementation version is v0.4.0.
- [x] Governed Capability Operation v1 merged.
- [x] Authorization + Policy Decision Boundary v1 architecture contract created.
- [x] Authorization Decision Boundary JSON Schema created.
- [ ] Validate Authorization Decision Boundary contract tests through executable CI.
- [ ] Reconcile runtime integration across Identity Authorization Enforcement, Policy Kernel, Governed Operation, and ProtectedDataAccess.
- [ ] Adversarial authorization integration tests.
- [ ] Merge PR #83 after genuine validation.

---

## 1. Governance and execution rules

- [x] Main is the canonical integration baseline.
- [x] Archive/verify-before-delete rule established.
- [x] Shared-first capability rule established.
- [x] No force-merge policy established.
- [x] Canonical GitHub CI path established.
- [x] Documentation authority registry established.
- [x] Documentation content/organization audit established.
- [ ] Establish independent-review requirement for security-critical changes.
- [ ] Establish CODEOWNERS/security ownership for shared security infrastructure.
- [ ] Establish release/versioning policy.
- [ ] Establish production change/rollback procedure.
- [ ] Establish incident-response procedure.

## 2. Repository and source-of-truth integrity

- [x] Mainline consolidation work established.
- [x] Legacy CI configuration archived where verified.
- [x] Cleanup process documented.
- [x] Documentation authority hierarchy established.
- [x] Documentation organization reviewed and unnecessary movement frozen.
- [ ] Complete repository tree audit from current `main` after pending documentation PRs are resolved.
- [ ] Classify remaining legacy/archive artifacts.
- [ ] Verify no duplicate capability implementation exists.
- [ ] Verify no undocumented generated artifact is committed.
- [ ] Verify secrets/credentials are absent from repository history and working tree.

## 3. Shared Capability Infrastructure

- [x] Capability contract established.
- [x] Shared capability registry established.
- [x] Registry validation tests established.
- [x] Adapter boundary established.
- [x] Failure semantics documented.
- [x] User-choice rule documented for optional capabilities.
- [ ] Complete hardened Capability Registry review and merge PR #80 after executable CI.
- [ ] Capability dependency model.
- [ ] Capability lifecycle/version compatibility rules.
- [ ] Capability health/readiness state model.

## 4. Policy and authorization infrastructure

- [x] Resource-scoped Policy Kernel contract merged to main.
- [x] Principal ID/type isolation tested.
- [x] Resource instance isolation established.
- [x] Unknown capability fails closed.
- [x] Malformed policy/capability declarations fail closed.
- [x] Identity assurance requirements implemented in Policy Kernel v0.4.
- [x] IdentityContext + Identity Authorization Enforcement boundary established.
- [x] Authorization + Policy Decision Boundary architecture established.
- [ ] Authorization Decision Boundary schema/test validation.
- [ ] Consent lifecycle semantics.
- [ ] Human-review/approval lifecycle semantics.
- [x] Tenant/data-domain isolation architecture defined; end-to-end data-layer validation remains pending.
- [ ] Jurisdiction/data-residency policy where required.
- [x] Dedicated persistence database service identity boundary established.
- [ ] Credential lifecycle and rotation model.
- [ ] Service-to-service identity beyond persistence DB boundary.
- [ ] Authorization decisions auditable without sensitive-data leakage.

## 5. Health information infrastructure

- [x] Canonical Health State Model.
- [x] Canonical Health Evidence Graph.
- [x] Health State ↔ Evidence linkage contract.
- [x] PHR repository boundary.
- [x] Local Health Vault foundation.
- [x] Longitudinal Observation/Timeline boundary.
- [x] Health context framework.
- [x] Health safety-event architecture baseline.
- [x] Longitudinal evidence-driven health-intelligence architecture baseline.
- [ ] Machine-readable validation for all newly introduced health extensions.
- [ ] First complete longitudinal product workflow.

## 6. Evidence and research infrastructure

- [x] Evidence domain contract.
- [x] Evidence registry schema.
- [x] Provider-neutral research adapter contract.
- [x] PubMed adapter specification/implementation.
- [x] Multi-source orchestrator scaffold.
- [x] Source provenance contract.
- [x] Canonical claim schema.
- [x] Evidence migration hardening.
- [x] Cross-paradigm evidence methodology.
- [x] Epistemic context mapping.
- [x] Behavioral/epistemic evidence architecture baseline.
- [x] Food–Life evidence architecture baseline.
- [ ] Production-grade provider error taxonomy.
- [ ] Evidence freshness/versioning policy.
- [ ] Evidence conflict/reconciliation policy.
- [ ] Research-result audit trail.
- [ ] Evidence access authorization.
- [ ] Evidence retention/deletion policy.

## 7. Health and safety boundary

- [x] Education vs management support vs treatment distinction documented.
- [x] Medical data governance documented.
- [x] Conservative source classification established.
- [x] Publication gates documented.
- [ ] Production safety review.
- [ ] User-facing uncertainty/limitations presentation.
- [ ] Escalation/red-flag handling.
- [ ] Professional-review workflow where required.
- [ ] Safety-event audit model.
- [ ] Health-data access control verification.

## 8. Identity, privacy and security

- [x] Local-first/privacy foundation exists.
- [x] Unsafe vault/device operations fail closed.
- [x] Sensitive Messenger/WhatsApp logging removed.
- [x] Misleading cryptographic claims removed.
- [x] Canonical IdentityContext defined.
- [x] Protected-data persistence service identity boundary established.
- [ ] Authentication flow audit.
- [ ] Authorization end-to-end audit.
- [ ] Session/token lifecycle audit.
- [ ] Secret management policy.
- [ ] Encryption-at-rest assessment.
- [ ] Encryption-in-transit assessment.
- [ ] Data minimization assessment.
- [ ] Consent and revocation model.
- [ ] Data export/deletion workflow.
- [ ] DPDP/privacy compliance review.
- [ ] Threat model and abuse cases.
- [ ] Dependency vulnerability audit.
- [ ] Security incident logging and response.

## 9. Agent and AI infrastructure

- [x] Agent platform architecture documented.
- [x] Agent capability registered in shared registry.
- [x] AI declared optional and user-controlled.
- [ ] Canonical Agent Registry implementation.
- [ ] Agent versioning/lifecycle.
- [ ] Agent approval/publishing workflow.
- [ ] Agent ownership/provenance.
- [ ] Agent-to-capability authorization.
- [ ] Long-running Agent Runtime.
- [ ] Retry/idempotency/cancellation semantics.
- [ ] Secure memory/context contract.
- [ ] Context retention/deletion policy.
- [ ] Agent failure isolation.
- [ ] Model/provider adapter contract.
- [ ] Prompt-injection/tool-poisoning boundary.
- [ ] Sensitive-data redaction policy.
- [ ] Model/output provenance.
- [ ] Human-review gates for high-impact operations.
- [ ] Provider outage/degradation behavior.
- [ ] Model/version auditability.

## 10. MCP infrastructure

- [x] MCP architecture decision documented.
- [x] MCP adapter boundary documented.
- [x] MCP capability registered.
- [x] MCP security contract tests established.
- [ ] Canonical MCP client/server implementation where required.
- [ ] Tool discovery authorization.
- [ ] Tool invocation authorization.
- [ ] Input/output validation.
- [ ] Tool provenance/trust metadata.
- [ ] Prompt-injection/tool-poisoning defenses.
- [ ] MCP audit events.
- [ ] MCP failure isolation.
- [ ] MCP conformance/integration tests.

## 11. Data and storage

- [x] Evidence registry migration exists.
- [x] Protected-data service identity migration boundary established.
- [ ] Production persistence credential cutover.
- [ ] PostgreSQL RLS end-to-end verification.
- [ ] Final protected-row NOT NULL/constraint gate after legacy verification.
- [ ] Clean-database migration execution test.
- [ ] Migration rollback/recovery strategy.
- [ ] Database backup/restore verification.
- [ ] Schema versioning policy.
- [ ] Connection/security configuration audit.
- [ ] Data retention/deletion semantics.
- [ ] Content-addressed storage adapter contract.

## 12. Decentralized capability layer — future plug-and-play

- [x] Nostr capability boundary registered.
- [x] Web3 capability boundary registered.
- [x] Decentralized identity boundary registered.
- [x] Content-addressed storage boundary registered.
- [ ] Protocol-neutral adapter contracts reviewed together.
- [ ] Signing/security claims verified before activation.
- [ ] Network/provider failure isolation.
- [ ] User consent and key ownership model.
- [ ] Data sovereignty implications.

These remain optional adapters, not core dependencies.

## 13. Product surfaces

- [ ] Define first production user journey.
- [ ] Define MVP acceptance criteria.
- [ ] Connect first product surface to shared capabilities.
- [ ] Authentication UX.
- [ ] Capability discovery UX.
- [ ] Consent/permission UX.
- [ ] Evidence/provenance UX.
- [ ] Error/degraded-mode UX.
- [ ] Accessibility audit.
- [ ] Localization/i18n audit.
- [ ] Mobile integration plan.
- [ ] Telegram adapter integration plan.
- [ ] WhatsApp adapter integration plan.
- [ ] Messenger adapter integration plan.
- [ ] Verify no surface-specific shared-policy duplication.

## 14. Observability and operations

- [ ] OpenTelemetry-compatible telemetry design.
- [ ] Structured audit event model.
- [ ] Correlation/request IDs.
- [ ] Capability invocation telemetry.
- [ ] Policy decision telemetry.
- [ ] Agent execution telemetry.
- [ ] MCP invocation telemetry.
- [ ] Evidence pipeline telemetry.
- [ ] Sensitive-data redaction.
- [ ] Metrics/health dashboards.
- [ ] Alert thresholds.
- [ ] Incident investigation workflow.
- [ ] Log retention policy.

## 15. CI/CD and supply-chain security

- [x] Canonical GitHub CI path.
- [x] Evidence pipeline workflow.
- [x] Dependabot configuration.
- [ ] Restore trustworthy GitHub-hosted runner execution.
- [ ] Required status checks documented.
- [ ] Branch protection/ruleset verified.
- [ ] Security-critical CODEOWNERS.
- [ ] Dependency lockfile policy.
- [ ] SBOM generation.
- [ ] Dependency vulnerability scanning.
- [ ] Secret scanning.
- [ ] Container image scanning.
- [ ] Controlled/reproducible build assessment.
- [ ] Release artifact signing strategy.
- [ ] Deployment approval gate.
- [ ] Rollback automation.

## 16. Documentation and knowledge architecture

- [x] Documentation authority registry.
- [x] AI/Human/Developer operating guide.
- [x] Architecture index.
- [x] Governance indexes.
- [x] Documentation content/organization audit.
- [x] Citation hygiene.
- [x] Food–Life document moved under architecture authority.
- [x] Cleanup execution plan moved under governance authority.
- [x] Policy Kernel documentation reconciled to implementation v0.4.
- [x] Whole-system architecture baseline reconciled with current shared-first security architecture.
- [ ] Synchronize final documentation changes after pending PR merges.
- [ ] Periodic authority/duplication audit after material architecture changes.

## 17. End-to-end product acceptance

- [ ] Clean checkout builds successfully.
- [ ] Clean environment starts successfully.
- [ ] Database initializes/migrates successfully.
- [ ] Authentication succeeds.
- [ ] Authorization succeeds for allowed operation.
- [ ] Authorization denies wrong principal.
- [ ] Authorization denies wrong capability/resource/action.
- [ ] Core user workflow succeeds.
- [ ] Evidence/provenance preserved.
- [ ] External adapter failure does not break unrelated capabilities.
- [ ] Optional AI failure does not break core product.
- [ ] Optional decentralized capability failure does not break core product.
- [ ] Audit trail produced without sensitive leakage.
- [ ] Recovery from restart verified.
- [ ] Backup restore verified.
- [ ] Security smoke tests pass.
- [ ] Performance baseline established.
- [ ] Accessibility baseline established.
- [ ] Privacy/compliance sign-off.
- [ ] Security sign-off.
- [ ] Product acceptance sign-off.

## 18. Release gate

- [ ] Version/tag selected.
- [ ] Release notes generated.
- [ ] All required CI checks green.
- [ ] No unresolved high-severity security issue.
- [ ] No unresolved data-loss risk.
- [ ] No known broken critical user journey.
- [ ] Production configuration reviewed.
- [ ] Rollback procedure tested.
- [ ] Monitoring active.
- [ ] Support/incident process active.
- [ ] Final go/no-go decision recorded.

---

## Product-ready definition

SOMA is **Product Ready** only when its primary user journey executes in a clean production-like environment while the applicable shared capability, policy, evidence, privacy, security, observability, recovery, and deployment gates are verified.

A registry entry is not implementation. An implementation is not verification. Unit tests are not system readiness. The final gate is an integrated system-level demonstration.

## Execution order

```text
1. Restore trustworthy CI
        ↓
2. Validate capability/documentation/authorization PRs
        ↓
3. Complete authorization runtime integration
        ↓
4. Verify protected-data/RLS boundary
        ↓
5. Establish core runtime/data path
        ↓
6. Build first complete SOMA health/evidence journey
        ↓
7. Add AI/agent/MCP infrastructure only where required by that journey
        ↓
8. Observability + operations
        ↓
9. Security/privacy/compliance audit
        ↓
10. Deployment + recovery drill
        ↓
11. End-to-end acceptance
        ↓
12. Product release
```

## Architectural non-negotiables

1. **Shared-first:** reusable capability belongs in shared infrastructure.
2. **One SOMA ecosystem:** surfaces are adapters/interfaces, not duplicated applications.
3. **Adapter isolation:** providers/transports/protocols cannot redefine core policy.
4. **Fail closed:** security-sensitive operations deny on uncertainty.
5. **Fail independently:** optional capability failure must not disable unrelated capabilities.
6. **User choice:** AI and decentralized capabilities remain optional where appropriate.
7. **Evidence integrity:** provenance and uncertainty travel with evidence-derived output.
8. **No capability inflation:** implementation must not claim security/evidence properties it does not actually provide.
9. **Archive before deletion:** historical material is preserved until deletion is evidenced.
10. **No forced merges:** mergeability, review, CI and security gates are prerequisites.
11. **Main is canonical:** after verified integration, `main` is the source of truth.
12. **Documentation is part of the product:** architecture and important execution decisions are recorded in-repository.
13. **Future decentralization is plug-and-play:** Web3, Nostr, DID/VC and content-addressed storage remain adapters, not hard dependencies.
14. **One canonical domain:** implementation languages and frameworks are replaceable; canonical contracts and domain invariants are not.
15. **Canonical domain runtime:** Rust is the canonical/reference runtime for security-sensitive and domain-critical behavior; other runtimes conform through explicit contracts.
16. **AI is never the security authority or source of canonical health/evidence truth.**
17. **Association is not causation:** causal claims require appropriate evidence and explicit inference methodology.
