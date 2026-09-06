# SOMA — Product Readiness Master Checklist

**Status:** Living master checklist  
**Objective:** Move SOMA from consolidated engineering baseline to a deployable, secure, maintainable product without weakening shared-infrastructure or governance boundaries.

> This checklist is the canonical execution tracker. A task is not `DONE` because code exists; it is `DONE` only when implementation, verification, security/privacy implications, failure behavior, documentation, and deployment implications have been checked.

## Status vocabulary

- `DONE` — implemented and verified.
- `IN PROGRESS` — actively being implemented.
- `BLOCKED` — cannot safely proceed until a prerequisite is resolved.
- `PLANNED` — accepted work, not started.
- `DEFERRED` — intentionally postponed with a reason.
- `NOT APPLICABLE` — explicitly ruled out.

---

## 0. Governance and execution rules

- [x] Main is the canonical integration baseline.
- [x] Archive/verify-before-delete rule established.
- [x] Shared-first capability rule established.
- [x] No force-merge policy established.
- [x] Canonical GitHub CI path established.
- [x] Historical feature/integration branches retired after verification.
- [ ] Establish independent-review requirement for security-critical changes.
- [ ] Establish CODEOWNERS/security ownership for shared security infrastructure.
- [ ] Establish release/versioning policy.
- [ ] Establish production change/rollback procedure.
- [ ] Establish incident-response procedure.

## 1. Repository and source-of-truth integrity

- [x] Mainline consolidated.
- [x] Remote stale branches pruned/retired.
- [x] Legacy CI configuration removed/archived where verified.
- [x] Cleanup manifest documented.
- [x] Backend misplaced fetch engine archived rather than silently deleted.
- [ ] Perform complete repository tree audit from current `main`.
- [ ] Classify every remaining legacy/archive artifact.
- [ ] Verify no duplicate capability implementation exists.
- [ ] Verify no undocumented generated artifact is committed.
- [ ] Verify secrets/credentials are absent from repository history and working tree.
- [ ] Add repository architecture map.

## 2. Shared Capability Kernel

- [x] Capability contract established.
- [x] Shared capability registry established.
- [x] Registry validation tests established.
- [x] Adapter boundary established.
- [x] Failure semantics documented.
- [x] User-choice rule documented for optional capabilities.
- [ ] Complete hardened Policy Kernel integration review.
- [ ] Merge hardened Policy Kernel only after required independent review.
- [ ] Post-merge Policy Kernel audit.
- [ ] Add capability dependency model.
- [ ] Add capability lifecycle/version compatibility rules.
- [ ] Add capability health/readiness state model.

## 3. Policy and authorization infrastructure

- [x] Canonical principal-scoped authorization contract merged to main via Policy Kernel v0.3.
- [x] Principal ID and principal type isolation tested.
- [x] Resource type isolation tested.
- [x] Resource instance isolation merged to main via Policy Kernel v0.3.
- [x] Unknown capability fails closed.
- [x] Malformed policy/capability declaration fails closed.
- [ ] Consent semantics defined.
- [ ] Human-review/approval semantics defined.
- [x] Tenant/data-domain isolation model defined via IdentityContext v1 and Identity → Authorization Enforcement v1; end-to-end data-layer enforcement remains pending.
- [ ] Jurisdiction/data-residency policy model defined where required.
- [ ] Credential lifecycle and rotation model defined.
- [ ] Service-to-service identity model defined.
- [ ] Authorization decisions auditable without leaking sensitive data.

## 4. Agent platform

- [x] Agent platform architecture documented.
- [x] Clarification and feedback loop documented.
- [x] Agent capability registered in shared registry.
- [ ] Canonical Agent Registry implementation.
- [ ] Agent versioning and lifecycle states.
- [ ] Agent approval/publishing workflow.
- [ ] Agent ownership and provenance.
- [ ] Agent-to-capability authorization.
- [ ] Long-running Agent Runtime.
- [ ] Retry/idempotency/cancellation semantics.
- [ ] Durable asynchronous job state.
- [ ] Secure Memory Bank contract.
- [ ] Context retention/deletion policy.
- [ ] Cross-session context access controls.
- [ ] Agent failure isolation.

## 5. MCP infrastructure

- [x] MCP architecture decision documented.
- [x] MCP adapter boundary documented.
- [x] MCP capability registered.
- [x] MCP security contract tests established.
- [ ] Canonical MCP client implementation.
- [ ] Canonical MCP server implementation where SOMA exposes tools.
- [ ] Tool discovery authorization.
- [ ] Tool invocation authorization.
- [ ] Input/output validation.
- [ ] Tool provenance and trust metadata.
- [ ] Prompt-injection/tool-poisoning defenses.
- [ ] MCP audit events.
- [ ] MCP failure isolation.
- [ ] MCP conformance/integration test suite.

## 6. Model/AI safety boundary

- [x] AI declared optional and user-controlled.
- [ ] Model/provider adapter contract finalized.
- [ ] Prompt-injection defense boundary.
- [ ] Tool-use policy enforcement.
- [ ] Sensitive-data redaction policy.
- [ ] Output provenance policy.
- [ ] Hallucination/uncertainty handling.
- [ ] Human-review gates for high-impact operations.
- [ ] Provider outage/degradation behavior.
- [ ] Model/version auditability.
- [ ] AI data-retention policy.

## 7. Evidence and research infrastructure

- [x] Evidence domain contract.
- [x] Evidence registry schema.
- [x] Provider-neutral research adapter contract.
- [x] PubMed adapter specification/implementation.
- [x] Multi-source orchestrator scaffold.
- [x] Source provenance contract.
- [x] Canonical claim schema.
- [x] Non-publishing canonical draft schema.
- [x] Medical migration hardening.
- [x] Evidence pipeline tests.
- [ ] Production-grade provider error taxonomy.
- [ ] Evidence freshness/versioning policy.
- [ ] Evidence conflict/reconciliation policy.
- [ ] Research-result audit trail.
- [ ] Evidence access authorization.
- [ ] Evidence retention/deletion policy.

## 8. Health and safety boundary

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

## 9. Identity, privacy and security

- [x] Local-first/privacy foundation exists.
- [x] Unsafe vault/device operations fail closed.
- [x] Sensitive Messenger/WhatsApp logging removed.
- [x] Misleading cryptographic claims removed.
- [x] Canonical identity model defined by IdentityContext v1.
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

## 10. Backend/runtime

- [x] Rust backend stabilization work consolidated.
- [x] Webhook routes consolidated/configured through environment.
- [x] Typed error hardening performed in sensitive paths.
- [ ] Complete current backend build audit.
- [ ] Complete integration test audit.
- [ ] API contract inventory.
- [ ] API authentication/authorization verification.
- [ ] Request validation.
- [ ] Rate limiting/abuse protection.
- [ ] Idempotency for externally retried operations.
- [ ] Background job lifecycle.
- [ ] Graceful shutdown.
- [ ] Health/readiness endpoints.
- [ ] Configuration validation at startup.
- [ ] Production logging policy.

## 11. Data and storage

- [x] Evidence registry migration exists.
- [x] Shared evidence registry architecture documented.
- [ ] Migration execution tested on clean database.
- [ ] Migration rollback/recovery strategy.
- [ ] Database backup/restore verification.
- [ ] Schema versioning policy.
- [ ] Connection/security configuration audit.
- [ ] Data retention policy.
- [ ] Data deletion semantics.
- [ ] Content-addressed storage adapter contract.
- [ ] Future IPFS/content-addressed adapter remains optional.
- [ ] Data residency strategy.

## 12. Decentralized capability layer — future plug-and-play

- [x] Nostr capability boundary registered.
- [x] Web3 capability boundary registered.
- [x] Decentralized identity boundary registered.
- [x] Content-addressed storage boundary registered.
- [ ] Protocol-neutral adapter contracts reviewed together.
- [ ] Wallet remains optional.
- [ ] Blockchain remains optional.
- [ ] DID/VC remains optional.
- [ ] Nostr remains optional.
- [ ] IPFS/content-addressed storage remains optional.
- [ ] Signing/security claims verified before activation.
- [ ] Network/provider failure isolation tested.
- [ ] User consent and key ownership model.
- [ ] Data sovereignty implications documented.

## 13. Product surfaces

- [ ] Define first production user journey.
- [ ] Define MVP acceptance criteria.
- [ ] Web product surface connected to shared capabilities.
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
- [ ] No surface-specific duplication of shared policy logic.

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
- [ ] Metrics and health dashboards.
- [ ] Alert thresholds.
- [ ] Incident investigation workflow.
- [ ] Log retention policy.

## 15. CI/CD and supply-chain security

- [x] Canonical GitHub CI path.
- [x] Evidence pipeline workflow.
- [x] Dependabot configuration.
- [ ] Required status checks documented.
- [ ] Branch protection/ruleset verified.
- [ ] Security-critical CODEOWNERS.
- [ ] Dependency lockfile policy.
- [ ] SBOM generation.
- [ ] Dependency vulnerability scanning.
- [ ] Secret scanning.
- [ ] Container image scanning.
- [ ] Reproducible/controlled build assessment.
- [ ] Release artifact signing strategy.
- [ ] Deployment approval gate.
- [ ] Rollback automation.

## 16. Docker and deployment

- [x] Docker runtime configuration was simplified/hardened during stabilization.
- [ ] Clean-machine container build.
- [ ] Production environment configuration documented.
- [ ] Secrets supplied externally, never committed.
- [ ] Database migration during deployment verified.
- [ ] Health/readiness checks verified.
- [ ] Horizontal/vertical scaling assumptions documented.
- [ ] Persistent storage requirements documented.
- [ ] Network ingress/egress policy.
- [ ] TLS termination strategy.
- [ ] Backup/restore drill.
- [ ] Disaster recovery target defined.
- [ ] Rollback drill.
- [ ] Production smoke test.

## 17. End-to-end product acceptance

- [ ] Clean checkout builds successfully.
- [ ] Clean environment starts successfully.
- [ ] Database initializes/migrates successfully.
- [ ] Authentication succeeds.
- [ ] Authorization succeeds for allowed operation.
- [ ] Authorization denies wrong principal.
- [ ] Authorization denies wrong capability/resource/action.
- [ ] Core user workflow succeeds.
- [ ] Evidence/provenance is preserved.
- [ ] External adapter failure does not break unrelated capabilities.
- [ ] Optional AI failure does not break core product.
- [ ] Optional decentralized capability failure does not break core product.
- [ ] Audit trail is produced without sensitive leakage.
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

SOMA is **Product Ready** only when the complete system can execute its primary user journey in a clean production-like environment while the shared capability, policy, evidence, privacy, security, observability, recovery, and deployment gates above are verified.

A capability being present in the registry is **not** sufficient. A capability being implemented is **not** sufficient. Passing unit tests is **not** sufficient.

The final gate is an integrated system-level demonstration.

## Execution order

```text
1. Policy Kernel hardening
        ↓
2. Mainline verification
        ↓
3. Shared Infrastructure CI Gate
        ↓
4. Identity + authorization
        ↓
5. Core runtime/data
        ↓
6. Primary product journey
        ↓
7. Agent + MCP runtime infrastructure
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
2. **Adapter isolation:** providers/transports/protocols cannot redefine core policy.
3. **Fail closed:** security-sensitive operations deny on uncertainty.
4. **Fail independently:** optional capability failure must not disable unrelated capabilities.
5. **User choice:** AI and decentralized capabilities remain optional where appropriate.
6. **Evidence integrity:** provenance and uncertainty travel with evidence-derived output.
7. **No capability inflation:** implementation must not claim security/evidence properties it does not actually provide.
8. **Archive before deletion:** historical material is preserved until deletion is evidenced.
9. **No forced merges:** mergeability, review, CI and security gates are prerequisites.
10. **Main is canonical:** after verified integration, `main` is the source of truth.
11. **Documentation is part of the product:** architecture and important execution decisions are recorded in-repository.
12. **Future decentralization is plug-and-play:** Web3, Nostr, DID/VC and content-addressed storage must remain adapters, not hard dependencies of the core.
13. **One canonical domain:** implementation languages and frameworks are replaceable; canonical contracts and domain invariants are not.
14. **Canonical domain runtime:** Rust is the canonical/reference runtime for security-sensitive and domain-critical behavior; other runtimes must conform through explicit contracts.
