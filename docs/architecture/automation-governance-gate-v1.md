# SOMA Automation & Governance Gate v1

## Purpose

Define the first executable governance layer for SOMA-OS. The gate automates repeatable verification while keeping irreversible, clinical, publication, and deletion decisions explicitly review-gated.

## Principles

1. Automate verification, not accountability.
2. Fail closed on mechanically provable safety or integrity violations.
3. Preserve provenance and uncertainty.
4. Never silently upgrade evidence strength.
5. Keep efficacy and safety independent.
6. Reuse shared infrastructure; do not duplicate application-specific gates.
7. Archive before deletion.
8. AI remains optional and requires user-choice policy.
9. Adapter-boundary capabilities must not be represented as operational merely because code exists.
10. Automated checks are evidence for a decision, not a substitute for the decision.

## Gate levels

- **PASS** — invariant verified.
- **WARN** — non-blocking condition requiring attention.
- **REVIEW REQUIRED** — decision cannot safely be automated.
- **FAIL** — invariant violation; CI should block the change.

## Initial automated scope

### Architecture

- capability registry required fields
- unique capability identifiers
- non-empty principal types
- explicit policy controls
- adapter/surface arrays
- optional AI status and user-choice policy
- non-operational boundary for unfinished protocols

### Evidence

- canonical mapping schema exists
- strength inflation remains prohibited
- canonical pilot fixtures exist
- legacy evidence states cannot silently become stronger canonical levels
- traditional/analytical/search retrieval objects remain unknown until assessed
- safety remains separate from efficacy

### Future automated scope

- secret/credential detection
- architecture dependency boundary checks
- duplicate capability detection
- schema drift detection
- migration count/relationship reconciliation
- provenance completeness
- stale branch/archive candidate reports
- documentation consistency
- security regression checks

## Explicitly review-gated

The automation must not autonomously:

- publish medical evidence
- diagnose a person
- prescribe or recommend treatment as a clinician
- promote uncertain evidence to clinical certainty
- delete legacy records
- delete branches/files without an archive/evidence trail
- force merge a pull request
- declare an unfinished adapter operational

## CI integration strategy

The gate should be added to the existing evidence/CI workflow rather than creating a parallel generic CI system. Existing tests remain authoritative for their domains; this gate adds cross-cutting invariants.

## Migration strategy

The gate is additive and non-destructive. It does not modify legacy evidence records. Migration remains a separate controlled pilot until record-level reconciliation, provenance traceability, and rollback behavior are demonstrated.
