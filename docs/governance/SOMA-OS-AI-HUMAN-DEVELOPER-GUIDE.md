# SOMA-OS AI / Human / Developer Guide v1

**Status:** governing operating guide
**Scope:** the complete SOMA ecosystem and its shared repository

## 1. What SOMA is

SOMA-OS is one health and wellbeing ecosystem built from shared infrastructure. It is not four separate applications. Web, mobile, Telegram, WhatsApp, Messenger, desktop, AI, agentic, MCP, Nostr, Web3/DID/VC, and future surfaces are interfaces, adapters, or capabilities that consume the same governed foundation.

The architecture grows by composition:

```text
Shared capability
  → shared contract
  → shared policy/security
  → shared data/evidence semantics
  → shared execution
  → provenance/audit
  → adapters/surfaces
```

Never build the same health, evidence, privacy, safety, authorization, or provenance logic independently for each surface.

## 2. Canonical system spine

```text
Identity
  → Capability
  → Policy
  → Gateway
  → Execution
  → Evidence
  → Interpretation
  → Audit
```

Security-sensitive protected-data execution additionally follows:

```text
IdentityContext
  → Identity Authorization Enforcement
  → Policy Kernel
  → ProtectedDataAccess
  → Trusted Persistence Identity
  → Trusted Transaction Context
  → PostgreSQL RLS / constraints
  → Protected Data
```

AI and agents may request governed capabilities. They cannot become the authority that grants themselves access.

## 3. Canonical health model

SOMA separates:

```text
Observation
  ≠ Claim
  ≠ Evidence
  ≠ Interpretation
  ≠ Recommendation
```

Personal observations belong to the Health State domain. General research evidence belongs to the Health Evidence Graph. Explicit linkage connects them without making one the source of truth for the other.

Longitudinal reasoning is based on observations over time, provenance, context, uncertainty, contradiction, applicability, evidence quality, and bounded interpretation.

Unknown time remains unknown. Derived signals retain their source observations and processing lineage.

## 4. Evidence and epistemic discipline

SOMA can investigate conventional, traditional, complementary, behavioral, psychological, lifestyle, mechanistic, emerging, and unconventional claims. Inclusion is not endorsement.

The operating rule is:

> Investigate without prejudice. Conclude according to evidence. Preserve uncertainty where evidence is insufficient.

For unusual claims:

```text
Claim
 → decompose
 → identify assumptions
 → generate competing hypotheses
 → retrieve evidence
 → test discriminators
 → compare explanations
 → assess safety
 → preserve uncertainty
```

Lateral thinking expands the hypothesis space; it does not lower the evidence standard.

## 5. Behavioral and psychological information

Behavioral records may capture context, reported thought/belief, affect/intensity, response, intervention/exercise, outcome, source, provenance, uncertainty, consent/purpose, and schema version.

AI may assist with structured journaling, reflection, retrieval, summarization, pattern exploration, and hypothesis generation. It must not silently diagnose, infer hidden causes as facts, prescribe, modify treatment, or promote a model-generated label into canonical health data.

A useful transformation is:

```text
User-reported information
 → observed record
 → AI interpretation
 → hypothesis
```

not:

```text
User text
 → model label
 → canonical health fact
```

## 6. Safety

SOMA informs decisions; authorized humans make clinical decisions.

Evidence status and safety status are independent. Uncertainty is not permission for high-risk experimentation. High-consequence capabilities require stronger authorization, safety controls, provenance, auditability, and appropriate human review.

## 7. Privacy

Local-first is preferred, but local computation is not automatically synonymous with security. Data minimization, explicit transfer, encryption where required, authorization before protected-data access, and auditability remain necessary.

Do not claim “100% privacy” merely because a model runs locally.

## 8. AI / agent boundary

AI is optional and user-controlled where the capability permits it. Core SOMA functionality must not depend on a particular model provider.

Models and agents:

- consume governed context;
- respect policy and authorization;
- preserve provenance;
- expose uncertainty;
- cannot widen scope;
- cannot change principal or subject without authorization;
- cannot bypass human-review gates;
- cannot redefine canonical health/evidence truth.

## 9. Provider and protocol architecture

Providers are replaceable adapters. PubMed is a provider, not the evidence system. Apple Health, Health Connect, wearables, medical records, labs, devices, and manual observations should enter through governed ingestion adapters and normalize into canonical observations.

Transport failures must remain isolated. A Telegram failure must not make Android or Web fail. Nostr/Web3/DID/VC/IPFS and similar protocols remain optional adapter boundaries until justified and implemented securely.

## 10. Research lessons adopted from recent analysis

The SOMA architecture deliberately adopts these useful patterns:

- structured behavioral observations;
- longitudinal context rather than isolated events;
- local-first computation where appropriate;
- provider-neutral ingestion;
- evidence passports;
- claim decomposition;
- competing hypotheses;
- explicit negative/contradictory evidence;
- mechanism extraction without mechanism inflation;
- fine-grained user permissions;
- separation of research evidence from personal health facts.

It deliberately rejects:

- autonomous therapy as a default capability;
- AI diagnosis or prescribing;
- unsupported “informational field” therapeutic claims as facts;
- automatic causal inference;
- a separate alternative-medicine subsystem;
- a separate behavioral-health application;
- provider-specific canonical health models;
- a new database merely for a new feature.

## 11. Implementation gate

Before adding a capability, answer:

1. Does an existing canonical contract already cover it?
2. Is the behavior shared across surfaces?
3. Which schema is authoritative?
4. Which policy and authorization rules apply?
5. What data is required and how is it classified?
6. What provenance must survive?
7. What happens on provider failure?
8. What is the unsafe or ambiguous case?
9. What adversarial tests are required?
10. Does the capability change product readiness?

If these cannot be answered, implementation should stop at architecture/research rather than inventing a parallel subsystem.

## 12. Documentation hierarchy

```text
README.md
  ↓ entry point
Docs index
  ↓ navigation
Architecture contracts
  ↓ system semantics
Schemas + executable tests
  ↓ machine semantics
ADRs / decisions
  ↓ rationale
Implementation documents
  ↓ concrete engineering behavior
Product readiness checklist
  ↓ status
Research / evidence sources
  ↓ supporting knowledge
Archive
  ↓ historical record
```

When two documents overlap, consolidate rather than create another summary. Keep the more authoritative document as the source and make secondary documents point to it.

## 13. Change and merge discipline

Every branch is audited against current `main` before merge. No force merge. No bypass around unavailable or failed validation. If CI infrastructure fails before a job executes, record it as infrastructure failure rather than pretending the code passed.

After merge:

```text
verify main
 → verify CI
 → reconcile documentation
 → reconcile readiness tracker
 → record consequential decision
```

## 14. Repository hygiene

Archive before deletion. Preserve old source material until evidence supports removal. Avoid duplicate directories, duplicate schemas, duplicate health-state models, duplicate evidence authorities, and duplicate documentation.

A clean repository is not the repository with the fewest files; it is the repository where each file has a clear owner, authority, lifecycle, and reason to exist.

## 15. Definition of done

A SOMA change is not complete merely because code exists. Completion requires the applicable combination of:

```text
Architecture
+ Contract
+ Implementation
+ Authorization
+ Security
+ Privacy
+ Failure semantics
+ Evidence/provenance
+ Auditability
+ Tests
+ CI
+ Documentation
+ Deployment implications
```

Only the applicable gates should be required; speculative future architecture must not create unnecessary implementation work.
