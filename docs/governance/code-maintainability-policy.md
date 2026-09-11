# SOMA Code Maintainability Policy

## Authority boundary

This is an engineering governance policy. It defines maintainability expectations for SOMA source code; technical architecture contracts remain authoritative for system structure and behavior.

## Purpose

SOMA source code must remain easy to review, test, and maintain.

## Rules

1. Rust formatting is enforced by `rustfmt`.
2. Rust source uses an 88-character maximum line width.
3. Prefer short functions with one clear responsibility.
4. Prefer named intermediate values over dense expressions.
5. Avoid deeply nested control flow when early returns improve clarity.
6. Keep public APIs explicit and documented where behavior is non-obvious.
7. Keep security-sensitive operations isolated behind small interfaces.
8. Do not use comments to compensate for unnecessarily complex code.
9. Tests should exercise behavior and failure paths, not only happy paths.
10. CI must verify formatting, compilation, linting, and tests.

## Security-sensitive code

Cryptographic, key-management, privacy, and health-data code receives a
higher maintainability standard. Readability takes priority over cleverness.
Security claims must match the implementation actually present in the code.

## Line-length principle

The 88-character limit is a maintainability guardrail, not a requirement to
force every line to exactly that width. Break long expressions, arguments,
and command invocations at natural semantic boundaries.
