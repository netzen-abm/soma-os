# SOMA-OS Consolidated Decision Log

**Period:** 2026-09-01 through 2026-09-08
**Purpose:** Preserve the major product, architecture, evidence, security, privacy, and engineering decisions developed during the last 7–8 days so they remain institutional memory rather than conversation-only knowledge.

> This log consolidates verified project decisions and work captured during the period. It is a decision record, not a verbatim transcript.

## 1. Product identity and architecture spine

SOMA-OS is health intelligence infrastructure, not merely an app, chatbot, video platform, agent platform, MCP server, medical database, or replacement for clinicians.

Working positioning:

**SOMA-OS — The Health Intelligence Infrastructure**

Working category:

**Evidence-aware health intelligence**

A later product-facing expression is:

**SOMA — Your Personal Health Intelligence System**

Core product loop:

```text
Research -> Understand -> Personalize -> Monitor -> Learn -> Share -> Improve
```

Core infrastructure loop:

```text
Identity -> Capability -> Policy -> Gateway -> Execution -> Evidence -> Audit
```

Canonical Domain Runtime: Rust.
Application/integration languages remain polyglot where justified.

## 2. Medical-paradigm neutrality

The project explicitly rejected the simplistic claim that modern biomedicine/allopathy never considers the body as an integrated system. Modern biomedical fields include systems biology, physiology, immunology, endocrinology, multimorbidity, and other integrative approaches.

The stronger architectural concern is that dominant clinical/research workflows can operationalize disease, intervention, and outcome through reductionist units and therefore miss systemic, relational, longitudinal, or individualized phenomena.

SOMA adopts:

> **No medical paradigm receives automatic epistemic privilege. No medical paradigm receives automatic exemption from scrutiny.**

Operational principle:

> **Investigate without prejudice. Conclude according to evidence.**

SOMA should use pluralistic methods with universal integrity requirements. It must distinguish evidence quality from conformity to a particular medical paradigm.

Mandatory conceptual distinctions:

```text
Evidence != Evidence Method
Evidence Quality != Paradigm Conformity
Claim != Evidence != Interpretation != Recommendation
```

The evidence model therefore represents knowledge system, epistemology, diagnostic model, intervention model, outcome model, research method, evidence assessment, safety, applicability, contradiction, negative evidence, uncertainty, and provenance separately.

## 3. Health Evidence Graph

The canonical Health Evidence Graph was extended to v1.1.0 to support cross-paradigm evidence representation.

Canonical entities include Claim, Source, Study, Population, Intervention/Exposure, Outcome, Evidence Assessment, Safety Assessment, and Applicability.

Evidence levels remain:

- E0_UNKNOWN
- E1_PLAUSIBLE
- E2_PRELIMINARY
- E3_SUPPORTED
- E4_WELL_SUPPORTED

Safety is independent from efficacy/evidence strength.

Claim types:

- phenomenological;
- empirical relationship;
- causal;
- ontological/theoretical.

Research method is separate from evidence level. Contradictory evidence and negative evidence are first-class. Provenance and transformation history are mandatory.

The model can represent traditional, complementary, integrative, emerging, and other knowledge systems without silently promoting their claims.

Potential future refinement: strengthen machine-readable conditional validation so causal claims cannot carry an incompatible causal-inference status.

## 4. Cross-paradigm evidence capability

`evidence.cross_paradigm` was established as a governed capability.

Pipeline:

```text
Claim
 -> Knowledge System Identification
 -> Paradigm/Epistemology
 -> Claim Decomposition
 -> Research Method Identification
 -> Evidence Retrieval
 -> Integrity Analysis
 -> Bias/Confounding
 -> Cross-Paradigm Comparison
 -> Contradiction
 -> Replication
 -> Safety
 -> Uncertainty
 -> Evidence Passport
```

Padaav/VCPCRF was used as a benchmark/case study for evaluating evidence without either dismissing it merely because it is traditional/Ayurvedic or promoting it merely because it is unconventional. The retained lesson is that observational signals, attrition, confounding, safety, replication, and causal limitations must remain explicit.

## 5. Research contribution and attribution

SOMA should give users a meaningful path to voluntarily contribute to research.

Contribution requirements:

- voluntary;
- purpose-specific;
- explicit consent;
- minimum necessary;
- withdrawable where technically/legal conditions permit;
- provenance-preserving;
- transparent attribution and publication/use constraints.

A user does not silently become a research participant merely by using SOMA.

Attribution should be institutionalized at source/claim/contribution level. Formal acknowledgement-letter capability is a future post-launch product capability, not something to execute during architecture work.

Capability `research.contribution` was added.

## 6. Health Media Intelligence

SOMA should have access to global health, wellness, nutrition, medical education, research, traditional knowledge, and lived-experience media from multiple sources.

However:

**SOMA must not become a video platform.**

Media is an acquisition/discovery/input layer supporting the stronger moat of evidence, provenance, safety, personal context, measurement, and learning.

The governed media pipeline is:

```text
Source
 -> Adapter
 -> Integrity/Access/License Metadata
 -> Media Normalization
 -> Health Relevance
 -> Transcript/Extraction where permitted
 -> Claim Extraction
 -> Knowledge System
 -> Research Method
 -> Evidence Retrieval
 -> Contradiction/Negative Evidence
 -> Safety
 -> Uncertainty/Applicability
 -> Health Evidence Graph
 -> Evidence Passport
```

A video, podcast, interview, testimonial, or lecture is not automatically evidence.

Publicly viewable does not mean public domain, downloadable, embeddable, or licensed for reuse.

FreeTube is treated as a privacy playback/client adapter, not a source of truth. Provider integrations remain adapters; no media provider becomes a health-data trust boundary.

Capabilities added:

- `media.health_discovery`
- `media.health_stream`
- `media.health_transcript`
- `media.health_claim_extraction`
- `media.health_evidence_linking`

## 7. Personal Health Record

SOMA evolved from a research/evidence environment toward a stronger product loop that also lets a person maintain a private digital health memory.

The Personal Health Record architecture includes:

- optional local personal health profile;
- health history;
- prescriptions;
- medication details and longitudinal use;
- measurements/observations;
- reports/documents;
- goals and context;
- longitudinal timeline/progress;
- evidence references;
- controlled doctor/care-team sharing;
- emergency profile.

The PHR is an index/view over canonical health entities, not a second canonical health model.

Medication intelligence is limited to reconciliation, duplication/combination awareness, safety signals, missing information, timeline review, and clinician/pharmacist question generation. It does not prescribe or authorize medication changes.

Test/monitoring suggestions are candidate prompts for professional confirmation, not diagnoses or medical orders.

AI-generated summaries must distinguish recorded facts, imported records, user reports, evidence interpretation, and uncertainty.

## 8. Emergency Health Profile

Emergency disclosure is a separate governed artifact, not a backdoor into the full health vault.

Potential minimum-necessary fields include critical allergies, essential medications, important conditions, implanted devices, emergency contacts, clinician contacts, preferred facility, and emergency notes.

Emergency access must remain scoped, user-approved, auditable, and minimum-necessary.

Break-glass access is not authorized merely by this architecture.

## 9. Optional identity

User login/account creation is optional by default.

Anonymous/local use remains available for safe capabilities. Login becomes valuable when durable identity is genuinely required for recovery, synchronization, sharing/delegation, organization membership, higher-assurance workflows, or similar requirements.

`authentication_status=VERIFIED` means the security context is verified; it does not by itself establish real-world identity.

Anonymous local identity is low assurance and cannot claim substantial/high assurance.

Authentication and authorization remain distinct. The Policy Kernel remains authoritative.

## 10. Local Health Vault

The Local Health Vault is the primary health-data trust boundary.

The vault is not simply a local database abstraction. The security contract requires:

- confidentiality;
- authenticated integrity;
- key lifecycle/recovery;
- key separation;
- versioned envelopes;
- tamper detection;
- provenance;
- controlled export/backup;
- local audit;
- deletion semantics;
- fail-closed behavior.

Canonical vault records use an encrypted envelope containing identifiers, entity/schema metadata, classification, ciphertext, nonce, key reference, cryptographic algorithms, provenance reference, timestamps, and optional tombstone.

Health plaintext must not be required to exist in the vault envelope.

AI, media, research, and other modules do not receive master keys or unrestricted vault access. They receive only authorized minimum-necessary decrypted context.

Device-only storage is a preferred architecture, not an absolute security guarantee. Device compromise, malware, backups, screenshots, exports, and recipient devices remain risks.

## 11. Protected-data PostgreSQL security

The protected-data security architecture was hardened through migrations and executable CI.

Important model:

- non-login trusted PostgreSQL roles provide privilege boundaries;
- production application credentials must use a separately provisioned LOGIN role with appropriate membership and explicit role transition rather than attempting to log in directly as a NOLOGIN trusted role;
- RLS is enabled and forced;
- tenant/data-domain scope is enforced;
- final protected-data NOT NULL gate is fail-closed;
- final migration does not infer or repair missing scope.

The final protected-data NOT NULL gate was merged only after fresh exact-head CI evidence.

A CI sequencing defect was identified and corrected: the final-gate test reset the ephemeral CI schema before replaying migrations, preventing prior integration state from invalidating the intended pre-RLS legacy fixture setup.

## 12. Policy Kernel

The Policy Kernel is shared infrastructure.

A security-critical grant must preserve principal identity and type rather than collapsing grants to principal type alone.

Conceptual grant tuple:

```text
principal_id
principal_type
tenant_id
capability_id
capability_version
resource_type
resource_scope
action
jurisdiction
data_classification
expires_at
conditions
```

The Policy Kernel must remain authoritative for authorization. Intelligence never grants itself authority.

Health governance extensions include identity, consent, capability, scope, safety, provenance, and audit.

## 13. Health Context Framework

A single person can occupy multiple health contexts across life. SOMA therefore adopts:

> **One person. One longitudinal health memory. Multiple governed contexts.**

Initial contexts:

1. `general_health`
2. `athlete_performance`
3. `service_veteran`
4. `rehabilitation_recovery`

Contexts reference the same canonical Health State, Evidence Graph, PHR, Vault, Policy, Identity, Safety, Provenance, Audit, Sharing, and Research Contribution infrastructure.

Contexts must not create separate canonical health databases.

## 14. Athlete Performance Context

Professional and competitive athletes are first-class supported users.

Potential tracked domains:

- training sessions/load/intensity/duration/distance/speed/power/strength/repetitions;
- competition performance and personal bests;
- resting heart rate, HRV, sleep, respiratory data, body metrics, blood pressure and other appropriate measurements;
- fatigue, soreness, stress, perceived recovery, readiness, rest;
- injury and rehabilitation history;
- nutrition, hydration, supplements;
- medications, prescriptions, allergies, labs, imaging references, reports, procedures and clearances where user-recorded.

SOMA may surface patterns such as increased training load with reduced sleep, increased soreness, and reduced performance, but must present these as observations/possible relationships rather than causal diagnoses.

Sharing must be role-specific. Coach, physician, physiotherapist, nutrition professional, team medical director, and research staff receive only explicitly authorized scopes.

Athlete health/performance data is sensitive and is not team-owned by default.

No autonomous return-to-play clearance or employment/team decision-making is authorized.

## 15. Service Veteran / Ex-Service Context

Serving, retired, and ex-service personnel are first-class supported users.

The context can preserve a longitudinal service-to-health record including service history, role, deployment/environmental context, documented injuries, surgeries, rehabilitation, documented exposure history, long-term medications, chronic conditions, pain/mobility, sleep/wellbeing, labs, imaging/document references, and current follow-up.

SOMA must not infer that a current condition was caused by service solely from temporal association. It can organize documented facts, exposure histories, research evidence, uncertainty, and questions for appropriate clinical/occupational/legal review.

## 16. Rehabilitation/Recovery

Rehabilitation is deliberately reusable rather than athlete-specific.

It can support sports injury recovery, post-operative recovery, neurological rehabilitation, mobility recovery, chronic functional rehabilitation, post-illness recovery, and other appropriate recovery journeys.

## 17. Research-oriented personal intelligence

The product is intended to connect:

```text
World knowledge
  Research -> Evidence -> Health knowledge

Personal knowledge
  Records -> Observations -> History -> Measurements

Intelligence
  Compare -> Contextualize -> Patterns -> Uncertainty -> Questions

Action
  Decision -> Intervention -> Measurement -> Learning
```

The system should help the user understand what is known, what is observed personally, what is uncertain, what may be related, what evidence supports or challenges a claim, and what questions are appropriate to ask a professional.

## 18. Privacy and external processing

The preferred architecture is:

```text
User device
 -> local storage/computation where possible
 -> explicit external capability only when needed
 -> minimum necessary data
 -> protected transport
```

Personal health data should not be sent to research/media providers merely to answer a research question.

External AI, research, media, or synchronization capabilities require explicit policy/consent and minimization.

## 19. Architecture and repository governance

`main` is the canonical integration branch.

The project uses a strict rule:

**Archive first. Delete only after evidence.**

Never force-merge useful changes. First audit the branch against current main, inspect changed files, validate relevant CI, resolve conflicts deliberately, and merge normally only when main is capable of accepting the change.

Repository documentation is institutional memory. Architecture decisions, security findings, research-model decisions, roadmap changes, and important product pivots must be documented in-repository.

## 20. Verified PR milestones in this period

### PR #56

Final protected-data NOT NULL gate. Merged after fresh exact-head Evidence Pipeline and Continuous Integration success.

### PR #58

Optional user identity architecture. Merged as commit `6d40d7662681ef35fef5387562cc6ea1aec033c7`.

### PR #61

Cross-paradigm evidence and governed research contribution. Fresh exact-head CI passed and PR merged as `09a971a6965c7d4facce82b63eb03adf1b260380`.

### PR #63

Canonical Health Evidence Graph v1.1.0 extension for cross-paradigm research. Fresh exact-head Evidence Pipeline and all required CI gates passed; merged as `d47d1bd3f68d23a7de9bbbf5206f64532b8332d7`.

### PR #64

Governed Health Media Intelligence architecture. At the time of this consolidated log it remained open and unmerged. Do not claim merge without a fresh GitHub state check.

### PR #65

Governed Personal Health Record and Emergency Profile architecture. It was subsequently merged as `f55a90416050602e0df9f646737fcc14a4d84071` after the required CI gates passed.

## 21. Current implementation priority

The architecture should not now jump directly into many specialized features.

Recommended execution order:

```text
1. Local Health Vault cryptographic/security implementation
2. Health Context Framework machine-readable contract
3. General PHR storage/repository implementation
4. Medication + prescription model
5. Longitudinal timeline/observations
6. Local Health Intelligence runtime
7. Controlled sharing + Emergency Profile
8. Athlete Performance Context
9. Service Veteran Context
10. Rehabilitation/Recovery Context
11. Research Contribution integration
12. Deeper media/evidence integrations
```

## 22. Definition of product readiness

A capability is not product-ready because it has a UI or passes a happy-path test.

Required dimensions are:

```text
Architecture
+ Contract
+ Implementation
+ Authorization
+ Security
+ Privacy
+ Failure Semantics
+ Evidence/Provenance
+ Auditability
+ Integration Tests
+ CI
+ Documentation
+ Deployment Implications
```

All must be considered before declaring production readiness.

## 23. Strategic conclusion

SOMA's moat is not AI, media aggregation, or a large feature count.

The durable advantage is the combination of:

**health context + personal longitudinal memory + evidence + safety + provenance + privacy + action + measurement + learning.**

Specialized users such as professional athletes and veterans strengthen the architecture because they prove that one canonical health memory can support materially different real-world journeys without creating product silos.
