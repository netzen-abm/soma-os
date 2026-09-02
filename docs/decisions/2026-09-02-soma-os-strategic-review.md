# SOMA-OS Strategic Review and Working Record

**Date:** 2026-09-02  
**Status:** Working baseline

## Executive conclusion

SOMA-OS has a credible shared-infrastructure foundation but is not yet a
finished production product. The correct strategy is to make `main` safer and
more capable through small, verified increments rather than force-merging
branches.

The project should now narrow its operational focus to health and wellbeing,
health research, integration across medical streams, and health-related policy.

## Current engineering assessment

Recent Policy Kernel hardening introduced explicit principal identity and
principal/resource scope, registry validation, regression coverage, and a
shared-infrastructure CI gate.

The relevant validation run passed:

- shared Policy/Registry tests;
- Rust formatting;
- Rust compilation;
- Clippy;
- cryptographic/integrity tests;
- evidence pipeline tests;
- controlled migration pilot.

A repository-level status endpoint nevertheless remained pending. Therefore,
merge safety must be treated separately from individual test success. No
force merge should be used merely to make the branch appear complete.

## Branch strategy

`main` is the integration target.

Feature branches are temporary implementation vehicles.

Before deleting a feature branch, verify:

1. whether it contains unique commits;
2. whether those commits are already represented in `main` or an accepted PR;
3. whether any useful documentation or code exists only on that branch;
4. whether rollback evidence is retained.

Archive before deletion where historical material has value.

## Workflow strategy

Keep the minimum set of workflows that provide meaningful protection.

Current active workflow architecture should be reviewed around:

- primary CI for code quality and tests;
- evidence pipeline validation;
- protected/manual operational workflows where a migration or health-data
  operation could have consequential effects.

Avoid running expensive operational or migration actions on every feature
branch push when pull-request validation provides the necessary safety gate.

## Code-quality standard

Prefer:

- small modules;
- small functions;
- clear names;
- explicit control flow;
- short readable lines;
- isolated security-sensitive logic;
- adapter boundaries around external providers;
- tests for important failure states;
- automated formatting and linting.

Line length is a maintainability aid, not a reason to obscure logic.

## Security assessment

The Policy Kernel is a strong foundation but should not yet be described as a
complete production authorization system.

Future hardening should consider:

- resource-level scope;
- tenant isolation where needed;
- grant expiry and revocation;
- delegation;
- provenance of authorization grants;
- policy-decision audit records;
- capability versioning;
- adversarial authorization tests;
- property-based tests and fuzzing;
- dependency and supply-chain review;
- independent security review before high-consequence deployment.

Do not add every mechanism prematurely. First make the current narrow kernel
correct, observable, and difficult to misuse.

## Evidence architecture

SOMA should not depend on a single research database.

Research integration should preserve source provenance and distinguish
multiple records for the same underlying study from independent evidence.
Provider failure must never be interpreted as absence of evidence.

The evidence model should distinguish evidence quality and epistemic status,
including established, emerging, observational/mechanistic, traditional or
historical, conflicting, and unsupported claims where applicable.

## Medical-stream integration

Integration across medical streams should mean interoperability, discovery,
comparison, provenance, and evidence-aware synthesis.

It must not mean declaring all systems of medicine equally effective or
scientifically validated.

SOMA should preserve the ability to represent uncertainty, evidence quality,
conflict, contraindication, safety concerns, and source provenance.

## Practical application map

### 1. Evidence and research intelligence

A governed system can accept a health research question, discover relevant
sources, extract evidence, preserve provenance, compare findings, and produce
a traceable synthesis.

### 2. Health and wellbeing intelligence

SOMA can support personal or institutional knowledge systems for health and
wellbeing without automatically turning informational output into diagnosis
or treatment instructions.

### 3. Longitudinal health knowledge

With explicit consent and appropriate safeguards, health information can be
organized over time while keeping local-first storage and minimized external
data transfer as architectural defaults.

### 4. Professional decision support

SOMA may eventually support qualified professionals with evidence retrieval,
provenance, comparison, policy context, and structured decision support.

### 5. Health-policy intelligence

The same infrastructure can support research and analysis of health policies,
regulations, outcomes, implementation evidence, and policy alternatives.

### 6. Health agents

Agents can become governed consumers of SOMA capabilities. The agent should
not become the authority: identity, capability, policy, evidence, execution,
and audit remain the authority chain.

## Core architectural principle

The ecosystem should be organized around reusable capabilities rather than
application-specific duplication.

Preferred model:

```text
Health surface
     |
     v
Shared SOMA infrastructure
     |
     +-- identity
     +-- capability registry
     +-- policy kernel
     +-- evidence
     +-- privacy
     +-- safety
     +-- provenance
     +-- verification
     +-- audit
     |
     v
Provider adapters / research sources
```

## Reference implementation priority

Before expanding into many protocols or applications, build one complete
health-oriented reference flow:

```text
Health question / task
        -> capability
        -> policy
        -> evidence/data access
        -> governed execution
        -> traceable result
        -> audit/provenance
```

This is the primary proof that the architecture works as an operational
system rather than only as documentation.

## Future expansion

Once the reference flow is reliable, SOMA can add adapters for:

- AI models;
- agents;
- MCP;
- research providers;
- health-data systems;
- institutional systems;
- decentralized protocols where justified;
- additional user-facing surfaces.

These remain replaceable adapters around the SOMA core rather than defining
the product itself.

## Strategic risks

The largest strategic risks are:

1. scope explosion;
2. architecture becoming the product instead of enabling a product;
3. premature abstraction;
4. AI becoming the identity of SOMA;
5. unsafe translation of evidence into medical advice;
6. false equivalence among medical streams;
7. confusing passing tests with complete security assurance.

## Working decision

For the next development cycle, prioritize:

1. repository governance and safe branch/workflow cleanup;
2. completion of the shared identity-capability-policy-gateway spine;
3. evidence/provenance integrity;
4. one end-to-end health reference implementation;
5. only then broader application and protocol expansion.

## Project memory rule

Important architecture decisions, audits, critiques, recommendations, and
scope changes must be recorded in this repository. The repository is not only
source code; it is the institutional memory of SOMA-OS.
