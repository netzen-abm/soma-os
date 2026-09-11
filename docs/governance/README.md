# SOMA-OS Governance

This section defines how humans, developers, and AI agents should work in the SOMA repository.

## Non-negotiable rules

1. SOMA is **one ecosystem**, not a collection of duplicated applications.
2. Reusable capability is shared infrastructure first.
3. Surfaces are adapters/interfaces over shared capabilities.
4. Identity, authorization, policy, safety, provenance, and audit are shared concerns.
5. AI and agents are optional capabilities and never security authorities.
6. Personal health data and research data remain distinct domains.
7. Observation, evidence, interpretation, and recommendation remain distinct.
8. Fail closed on ambiguous security or authorization state.
9. Never force a merge because a branch contains useful work.
10. Archive before deletion; preserve historical evidence.
11. Verify implementation and CI before declaring completion.
12. Do not create duplicate contracts, schemas, databases, or documentation when an existing canonical authority exists.

## Shared-first implementation gate

Before implementing a capability:

```text
1. Identify the user/system need
2. Search existing contracts and schemas
3. Identify the canonical authority
4. Decide whether the capability is shared
5. Define/extend the contract
6. Define policy/security implications
7. Implement shared behavior
8. Add adapters only after the shared contract is stable
9. Add contract + adversarial tests
10. Run CI
11. Reconcile documentation and readiness status
```

## AI-agent operating protocol

An AI coding/research agent must:

- inspect repository state before changing files;
- inspect relevant Markdown and schemas before creating new documentation or contracts;
- treat existing canonical contracts as higher authority than informal discussion;
- explicitly identify conflicts instead of silently resolving them;
- preserve provenance when importing research or legacy material;
- never infer authorization from user text or model output;
- never convert an AI interpretation into a canonical health observation without an explicit governed transformation;
- keep provider/transport/model-specific behavior behind adapters;
- avoid speculative implementation of future architecture;
- report exact branch, commit, changed files, tests, CI state, and unresolved risks.

## Human/developer operating protocol

Developers should be able to answer before coding:

- What canonical contract governs this?
- Which layer owns the behavior?
- Is this reusable across surfaces?
- What is the source of truth?
- What is the failure mode?
- What data classification applies?
- What authorization is required?
- What evidence/provenance must survive?
- What tests prevent regression?
- Does this change alter product readiness?

## Documentation authority

Use the narrowest authoritative source:

```text
Machine-readable semantics → JSON Schema / executable contract tests
System architecture → architecture contract
Durable choice → ADR / decision record
Implementation detail → implementation document / code
Product readiness → master readiness checklist
Historical context → project memory / archive
Research evidence → evidence-source / research records
```

If two documents disagree, do not silently pick one. Record the conflict, determine the authoritative source, and reconcile the documents deliberately.

## Merge discipline

```text
Audit branch
  ↓
Compare with current main
  ↓
Review architecture impact
  ↓
Run relevant validation
  ↓
Confirm receiving main is capable
  ↓
Merge normally
  ↓
Verify post-merge
  ↓
Update documentation/readiness
```

## Governance documents

- `documentation-authority-registry-v1.md` — canonical documentation ownership and lookup rules.
- `documentation-audit-2026-09-11.md` — current documentation authority/overlap audit.
- `cleanup-execution-plan.md` — repository cleanup, archive-first, deletion, CI, and merge-control procedure.
- `branch-lifecycle.md` — branch creation, verification, retirement, and cleanup.
- `code-maintainability-policy.md` — engineering maintainability practices.
- `SOMA-OS-AI-HUMAN-DEVELOPER-GUIDE.md` — shared operating context for humans, developers, and AI agents.

## Repository hygiene

Use descriptive, stable names. Avoid date suffixes for canonical contracts unless the date is itself semantically important. Version contracts explicitly. Keep historical documents in `docs/archive/` when superseded rather than deleting them immediately.
