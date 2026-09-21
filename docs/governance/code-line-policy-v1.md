# SOMA-OS Code Line Policy v1

## Rule

New or modified programming source files must remain under 180 lines. The maximum permitted file length is therefore 179 lines.

This is a maintainability and reviewability constraint, not a style preference.

## Scope

The rule applies to Python, Rust, JavaScript, TypeScript, shell scripts, SQL migrations, and executable configuration where practical.

It does not apply to prose documentation, licenses, generated lockfiles, or machine-readable schemas unless a separate policy explicitly says otherwise.

## Refactoring rule

When a source file exceeds the limit:

1. Do not compress formatting merely to reduce the count.
2. Preserve behavior and public contracts.
3. Split by responsibility, not arbitrary line ranges.
4. Keep security-sensitive boundaries explicit.
5. Move tests with the responsibility they exercise where practical.
6. Update imports and references.
7. Run formatting, tests, and relevant CI gates.
8. Record unavoidable transitional exceptions explicitly.

## Security priority

Security-sensitive code receives the same line limit but a higher review standard. Splitting a security boundary must never weaken authorization, subject binding, fail-closed behavior, auditability, or provenance.

## Enforcement

The repository validator reports every programming file at or above 180 lines. CI should fail on new violations. Existing violations are technical debt and must be reduced in bounded refactoring passes rather than hidden by exclusions.

## Review invariant

No new feature is complete if it introduces a programming file at 180 lines or more.
