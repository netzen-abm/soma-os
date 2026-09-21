# SOMA-OS Code Maintainability Policy

## Authority

This policy governs source maintainability. It does **not** impose an arbitrary universal file-length limit.

## Review principle

Line count is a diagnostic signal, not an architectural rule. A file should be refactored when its size reflects multiple responsibilities, obscures security boundaries, reduces testability, or materially harms reviewability.

## Before changing a large file

1. Read the complete file.
2. Measure its actual line count.
3. Identify its responsibilities and public interfaces.
4. Check references, callers, tests, and security boundaries.
5. Determine whether the size reflects legitimate cohesion or accumulated responsibilities.
6. Refactor only when there is a concrete maintainability or architectural reason.
7. Split by responsibility, not arbitrary line ranges.
8. Re-run formatting, compilation, tests, and relevant security gates.

## Security-sensitive code

Authorization, identity, protected-data, cryptographic, health-data, and privacy code receives a higher review standard. Splitting such code must preserve fail-closed behavior, subject binding, provenance, auditability, and canonical ownership.

## Practical guideline

The repository may use **180 lines as a review trigger** for a source file, but crossing that number is not itself a defect. A file above the guideline must be inspected before deciding whether to refactor.

## Architectural rule

Prefer one cohesive module with one clear responsibility over multiple artificially small modules. Do not create fragmentation merely to satisfy a numeric threshold.
