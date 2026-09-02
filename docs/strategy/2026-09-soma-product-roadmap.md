# SOMA-OS Product Roadmap — Health Intelligence

**Date:** 2026-09-02  
**Status:** Working roadmap

## Strategic objective

Turn SOMA-OS from a strong shared-infrastructure architecture into one validated, useful health-intelligence product without allowing feature expansion to outrun evidence, safety, privacy or engineering maturity.

## S0 — Repository stabilization

**Status:** Substantially complete.

Focus:

- canonical `main`;
- repository memory;
- architecture decisions;
- safe branch governance;
- CI baseline;
- security audit findings;
- archive-before-delete discipline.

Remaining work is continuous hardening rather than a reason to expand scope.

## S1 — Shared capability architecture

**Status:** Substantially complete; hardening continues.

Core:

- identity;
- capability registry;
- Policy Kernel;
- gateway boundaries;
- provider adapters;
- evidence/provenance primitives;
- privacy and safety policies;
- auditability.

Acceptance principle:

> A new surface should consume shared capabilities rather than duplicate them.

## S2 — Policy Kernel

**Status:** Current hardening phase.

Immediate priorities:

1. make principal identity/type/resource semantics unambiguous;
2. fail closed on malformed authorization inputs;
3. add adversarial and regression tests;
4. resolve PR/base mergeability normally;
5. keep authorization separate from AI/model judgment.

Later hardening may include scope IDs, expiry, revocation, delegation, provenance, version compatibility, rate limits and tenant isolation where justified.

## S3 — Evidence Intelligence Layer

**Status:** Next major build.

Build:

- canonical evidence schema;
- source adapters;
- provenance records;
- evidence classification;
- uncertainty representation;
- duplicate-study detection;
- verification workflow;
- evidence search and synthesis;
- Evidence Passport™ generation.

Reference flow:

```text
Question
  -> source discovery
  -> evidence extraction
  -> classification
  -> verification
  -> synthesis
  -> provenance
  -> auditable answer
```

## S4 — Health State Model

Represent a person's health context without pretending that a model is a diagnosis.

Potential dimensions:

- goals;
- symptoms;
- behaviors;
- nutrition context;
- activity;
- sleep;
- stress;
- body composition;
- relevant measurements;
- medications/treatments where explicitly supplied;
- environmental/social context;
- constraints and preferences.

Privacy requirement: local-first and data minimization remain defaults.

## S5 — Nutrition Intelligence

Focus first on evidence-aware nutrition reasoning rather than a massive food recommendation catalogue.

Capabilities:

- dietary pattern analysis;
- culturally relevant food context;
- nutrient/food evidence lookup;
- intervention hypotheses;
- safety and contraindication checks;
- bounded experiments;
- outcome tracking.

Nutrition recommendations should expose uncertainty and avoid universal claims.

## S6 — Lifestyle Intelligence

Integrate:

- physical activity;
- sleep;
- stress management;
- social connection;
- risky-substance avoidance;
- nutrition.

The system should convert these into practical, measurable behavior experiments where appropriate.

## S7 — Intervention / Experiment Engine

Core object:

```text
Hypothesis
  -> Intervention
  -> Duration
  -> Measures
  -> Result
  -> Interpretation
  -> Next action
```

This creates a continuous learning loop rather than one-time recommendations.

Safety requirements:

- bounded scope;
- explicit uncertainty;
- stop conditions where appropriate;
- escalation to qualified professionals for high-risk situations;
- no instructions to discontinue prescribed treatment.

## S8 — SOMA Practitioner

Provide qualified professionals with:

- longitudinal health context;
- evidence explorer;
- evidence passports;
- intervention builder;
- safety checks;
- follow-up;
- outcome measurement;
- multidisciplinary information exchange.

The system supports professional judgment; it does not replace it.

## S9 — SOMA Research

Build governed research infrastructure for:

- evidence maps;
- intervention registries;
- hypothesis registries;
- real-world evidence;
- longitudinal outcomes;
- systematic-review discovery;
- research provenance.

Research data governance, consent, privacy, ethics and appropriate de-identification/pseudonymization are prerequisites.

## S10 — External integrations

Add adapters for:

- laboratories;
- wearables;
- health records where legally and technically appropriate;
- research databases;
- food/nutrition databases;
- institutional systems;
- AI providers;
- agent runtimes;
- MCP;
- other protocols.

No provider should become the architectural authority.

## S11 — Ecosystem expansion

Only after the core reference flow is reliable should SOMA consider broad expansion into additional applications, channels, decentralized protocols or agent surfaces.

## First product milestone

The most important milestone is not the launch of many features.

It is a verified end-to-end workflow:

```text
User health question
      -> context
      -> capability
      -> policy
      -> evidence
      -> safety check
      -> bounded action/experiment
      -> measurement
      -> result
      -> adaptation
      -> audit/provenance
```

If SOMA cannot execute this flow reliably, expanding the number of interfaces does not improve the product.

## Product success criteria

Early success should be measured by:

- evidence traceability;
- safety correctness;
- user comprehension;
- useful actionability;
- measurable outcomes;
- privacy preservation;
- professional usefulness where applicable;
- reproducibility;
- low provider lock-in;
- low duplication across surfaces.

Feature count is not a primary success metric.

## Non-goals for the first release

- universal health assistant;
- autonomous diagnosis;
- autonomous prescribing;
- cure claims;
- supplement commerce;
- unlimited testing;
- unverified biohacking protocols;
- automatic equivalence between medical systems;
- autonomous agent authority over health decisions.
