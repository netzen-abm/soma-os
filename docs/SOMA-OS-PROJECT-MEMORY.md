# SOMA-OS Project Memory

**Purpose:** Preserve the important architectural, product, security, evidence,
and engineering decisions developed during SOMA-OS work so future contributors
can continue without losing context.

**Last consolidated:** 2026-09-02

## 1. Product identity

SOMA-OS is a health and wellbeing infrastructure project.

Primary scope:

- health and wellbeing;
- integration across medical streams;
- health and biomedical research;
- evidence discovery and synthesis;
- health-related policy research and analysis;
- privacy-first health data infrastructure;
- governed health intelligence and decision support;
- interoperable health capabilities and provider adapters.

SOMA-OS is distinct from Janavani. Janavani is a separate civic-infrastructure
project. Do not combine their product roadmaps, terminology, repositories, or
requirements.

Generic technical patterns may be reused independently when they are genuinely
domain-neutral and do not create cross-project coupling.

## 2. Product philosophy

SOMA-OS should not be positioned as merely:

- an AI chatbot;
- an agent platform;
- an MCP server;
- a medical database;
- a single medical system;
- a replacement for clinicians;
- a cure/remedy engine.

AI, agents, protocols, research connectors, and user surfaces are capabilities
that can consume SOMA infrastructure.

The core product proposition is health-oriented, evidence-informed, privacy-
first infrastructure.

## 3. Core architecture

The preferred shared infrastructure spine is:

```text
Identity
  -> Capability
  -> Policy
  -> Gateway
  -> Execution
  -> Evidence
  -> Audit
```

Applications and interfaces should remain thin surfaces over shared
capabilities.

Preferred capability lifecycle:

```text
Health question/task
  -> capability selection
  -> policy evaluation
  -> evidence/data access
  -> governed execution
  -> traceable result
  -> audit/provenance
```

Reusable capabilities should be implemented once in shared infrastructure and
consumed through stable contracts.

## 4. Medical-stream integration principle

SOMA-OS is intended to integrate knowledge and research across relevant medical
streams.

Integration means interoperability, discovery, provenance, comparison, and
evidence-aware synthesis.

It does **not** mean treating all medical systems, interventions, or claims as
equally validated.

The evidence model must preserve epistemic status, source type, evidence
quality, uncertainty, disagreement, safety context, and provenance.

## 5. Evidence infrastructure

The project should support multiple source classes rather than depend on one
provider.

Useful source classes include:

1. peer-reviewed research and systematic reviews;
2. clinical-trial registries;
3. official government and international sources;
4. institutional research publications;
5. expert/educational sources;
6. commercial/vendor material;
7. social/self-published material.

Lower-tier sources may be useful for discovery or representation of a claim,
but must not be silently promoted to clinical evidence.

Duplicate database records for the same underlying study must not be counted as
independent evidence.

Provider failure is not evidence absence.

Research records should preserve original source URL, provenance, retrieval
context, evidence status, and enough metadata for independent verification.

## 6. Evidence examples and lessons retained

Previous research work established the following patterns:

- Millet and whole-grain evidence has legitimate peer-reviewed support, but
  effects depend on food type, preparation, population, and outcome.
- Slow-paced breathing/HRV research has a legitimate physiological literature,
  but mechanistic findings should not be inflated into broad clinical claims.
- Industry-affiliated HeartMath findings can be represented with explicit
  attribution and study provenance, but should not be generalized as universal
  clinical efficacy.
- Exclusion-zone/fourth-phase-water claims remain scientifically contested;
  SOMA must represent competing explanations and avoid presenting a contested
  mechanism as established medical fact.
- Traditional-medicine knowledge can be integrated as a distinct knowledge
  and research class while maintaining safety, evidence, and provenance
  distinctions.

## 7. Health safety boundary

SOMA may support evidence discovery, education, synthesis, management
information, research, policy analysis, and decision support where appropriate.

It must not silently turn informational evidence into:

- diagnosis;
- individualized prescription;
- guaranteed treatment;
- cure claims;
- instructions to discontinue prescribed treatment.

High-consequence outputs require stronger safety controls and appropriate
professional oversight.

## 8. Privacy principles

Privacy is a default architecture property.

Preferred path:

```text
User device
  -> local storage/computation where possible
  -> explicit external capability only when needed
  -> minimized data
  -> protected transport
```

Personal health data should not be sent to research providers merely to answer
a research question.

Pseudonymization is not automatically anonymization. Hashes and identifiers
must be treated according to their actual re-identification risk and applicable
privacy requirements.

## 9. Policy Kernel

The Policy Kernel is shared infrastructure, not application-specific
authorization.

The hardened grant model should include identity and type/resource context,
with a conceptual key of:

```text
principal_id
principal_type
capability_id
resource_type
action
```

The registry should fail closed for malformed or missing principal types.

The kernel should eventually support, as justified by real use cases:

- resource scope IDs;
- tenant isolation;
- expiry;
- revocation;
- delegation;
- grant provenance;
- policy-decision audit;
- capability version compatibility;
- usage/rate constraints.

Do not add complexity without an operational requirement.

## 10. Security findings retained

Important previously identified issues must remain visible until resolved:

### Cryptography

Base64 encoding is not encryption. Any function described as encryption must
use a real authenticated-encryption design such as an appropriately reviewed
AEAD construction.

Vault export/import must have an explicit, tested format contract. Compression
and decompression must match.

### Key restoration

Hashing a mnemonic directly into a secp256k1 key is not equivalent to a
standards-compliant wallet derivation scheme. If key restoration remains in
scope, use an appropriate standard derivation design or fail closed.

### Nostr

Nostr broadcast must remain disabled/fail-closed until real cryptographic
signing and verification are implemented. A digest or empty signature is not a
valid Nostr signature.

### Meta webhooks

WhatsApp/Messenger inbound webhooks must verify the provider signature over raw
request bytes before accepting the payload. Sensitive message content must not
be written to logs.

### Secrets

A Messenger page access token was previously found hardcoded in source during
audit. It must be considered compromised: rotate it and store the replacement
outside source control using the appropriate secret mechanism.

### UniFFI

The build architecture previously used UniFFI scaffolding generation against a
Rust source file. This needs to be reconciled with the actual UniFFI UDL/proc-
macro architecture before the binding layer is considered production-ready.

## 11. CI and workflow principles

CI must protect `main` without becoming unnecessarily expensive or noisy.

Preferred model:

- pull request -> main: complete validation gates;
- push to main: post-merge verification;
- controlled/manual operations: migrations and consequential research/data
  operations;
- avoid unnecessary full operational pipelines on every feature push.

Formatting should be deterministic and checked in CI. Avoid auto-commit
formatting workflows that grant write access merely to normalize source.

## 12. Branch and repository governance

`main` is the canonical integration branch.

Never force a merge solely because a branch has useful changes.

Before integrating:

1. audit the branch against current `main`;
2. inspect changed files and architecture impact;
3. validate all relevant CI;
4. resolve conflicts deliberately;
5. merge normally only when the receiving `main` is capable of accepting the
   change.

Archive first. Delete only after evidence.

For obsolete branches, preserve branch tips or equivalent archive evidence
before deletion when historical value exists.

## 13. PR #12 history

PR #12 (`security: harden shared Policy Kernel and CI gate`) was created from
`main` to harden principal identity/type/resource handling and add a shared
Policy/Registry CI gate.

At the last verified inspection on 2026-09-02 it was still open and not merged,
with GitHub reporting `mergeable: false`. The branch contained four commits and
four changed files. Individual CI check-runs had previously completed
successfully, but a repository status/mergeability discrepancy remained.

Therefore: do not force merge. Re-check the PR and its checks after the
receiving branch is stable, resolve any mergeability issue, and merge normally
only after verification.

## 14. Repository cleanup history

The project has been undergoing consolidation of duplicate and multi-
generation material.

Historical material should be archived before removal.

A stale nested working copy was previously identified locally; the canonical
Git repository should remain a single clean working tree.

GitLab material is historical unless deliberately reintroduced. GitHub Actions
is the canonical CI location unless a documented reason says otherwise.

## 15. Evidence migration infrastructure

The repository contains an evidence migration/research pipeline with scripts
for legacy auditing, research queue generation, canonical migration,
publication verification, orchestration, schema validation, and tests.

The migration pilot passed validation, but an important design lesson remains:
legacy source URLs must not disappear during canonicalization. If a source is
not yet fully classified/verified, preserve it as provisional provenance rather
than dropping it or pretending verification exists.

## 16. Reference architecture lesson from external agent platforms

OpenClaw was reviewed as an architectural reference, not as a product
requirement or dependency.

The useful lesson is a gateway/control-plane approach in which tools, skills,
plugins, channels, and sessions attach to a common runtime.

For SOMA, the equivalent lesson is:

- protocol adapters are surfaces;
- capabilities are shared contracts;
- policy remains centralized;
- evidence/provenance remains shared;
- external platforms must not become the authority over SOMA health data.

## 17. Practical SOMA application roadmap

### Phase A — Stabilize the foundation

- reconcile `main` and PR state;
- complete branch cleanup;
- minimize workflows;
- resolve known security/build blockers;
- strengthen policy and capability tests.

### Phase B — Shared health infrastructure

- capability registry;
- identity/access model;
- policy kernel;
- health evidence model;
- provenance/audit model;
- privacy/data policy;
- safety policy;
- provider adapter contracts.

### Phase C — First end-to-end product

Build one real health research/evidence workflow from question to traceable
answer, with policy, provenance, safety, and audit.

### Phase D — Health intelligence

Add longitudinal knowledge, professional decision support, health-policy
intelligence, and governed agents only where validated by actual requirements.

### Phase E — Interoperability

Add additional research/health-data providers and protocol adapters without
creating parallel business logic.

### Phase F — Ecosystem expansion

Only after the core is reliable, evaluate MCP, decentralized protocols,
additional client surfaces, and other integrations.

## 18. Working rules for future contributors

1. Do not mix SOMA-OS with Janavani.
2. Do not treat AI as the SOMA product identity.
3. Do not duplicate shared capability logic inside applications.
4. Do not call base64 encryption.
5. Do not enable insecure/mock cryptographic broadcast.
6. Do not log sensitive health/message payloads.
7. Do not silently promote weak evidence to strong evidence.
8. Do not equate medical-stream integration with evidentiary equivalence.
9. Do not force merges.
10. Archive before deletion.
11. Verify before claiming completion.
12. Record important decisions in the repository.

## 19. Definition of a healthy SOMA ecosystem

A healthy SOMA ecosystem is one in which a new health application can be built
primarily by composing existing capabilities rather than recreating policy,
privacy, evidence, safety, provenance, identity, and audit logic.

The measure of architectural success is therefore not the number of modules,
protocols, or AI integrations. It is how safely and efficiently the shared
infrastructure can support real health use cases.
