#!/usr/bin/env python3
"""Fail-closed static audit for protected PostgreSQL access paths.

This is intentionally conservative: it inventories direct SQL/database primitives
outside the canonical protected persistence boundary and reports findings for review.
It also protects the migration/control-plane lifecycle by rejecting accidental
wiring of privileged legacy-promotion executors into the application runtime.
It does not attempt to prove runtime safety and never auto-approves an exception.
"""

from __future__ import annotations

import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
EXCLUDED_PARTS = {".git", "target", "__pycache__", ".pytest_cache"}
ALLOWED_FILES = {
    pathlib.Path("services/backend-rust/src/db_layer.rs"),
    pathlib.Path("services/backend-rust/src/protected_db_context.rs"),
    pathlib.Path("services/backend-rust/src/legacy_promotion.rs"),
    pathlib.Path("services/backend-rust/src/legacy_promotion_preflight.rs"),
    pathlib.Path("services/backend-rust/src/db_postgres_integration_test.rs"),
    pathlib.Path("scripts/audit_protected_data_access.py"),
}
MIGRATION_CONTROL_PLANE = {
    pathlib.Path("services/backend-rust/src/legacy_promotion.rs"),
}
CONTROL_PLANE_CONSTRUCTORS = {
    "LegacyPromotionExecutor::new(",
    "LegacyPromotionPreflightExecutor::new(",
}
PATTERNS = {
    "sqlx_query": re.compile(r"\bsqlx::query(?:_as)?\s*\("),
    "sqlx_raw_sql": re.compile(r"\bsqlx::raw_sql\s*\("),
    "pool_query": re.compile(r"\b(?:PgPool|Pool<Postgres>)\b"),
    "protected_table": re.compile(r"\banonymized_user_vitals\b"),
}


def files() -> list[pathlib.Path]:
    result = []
    for path in ROOT.rglob("*"):
        if not path.is_file() or any(part in EXCLUDED_PARTS for part in path.parts):
            continue
        if path.suffix not in {".rs", ".py", ".sql"}:
            continue
        result.append(path)
    return result


def is_test_or_archive(relative: pathlib.Path) -> bool:
    return "archive" in relative.parts or relative.name.endswith("_test.rs")


def audit_control_plane_wiring(all_files: list[pathlib.Path]) -> list[str]:
    findings: list[str] = []

    # Legacy promotion is deliberately migration/control-plane infrastructure.
    # It must not become part of the application module graph accidentally.
    main_rs = ROOT / "services/backend-rust/src/main.rs"
    if main_rs.exists():
        main_text = main_rs.read_text(encoding="utf-8", errors="replace")
        if re.search(r"\b(?:pub\s+)?mod\s+legacy_promotion\s*;", main_text):
            findings.append(
                "services/backend-rust/src/main.rs: legacy_promotion module is wired into the application runtime"
            )

    # Privileged control-plane executors must not be instantiated by unrelated
    # production code. Their own implementation and dedicated tests remain valid.
    for path in all_files:
        relative = path.relative_to(ROOT)
        if path.suffix != ".rs" or is_test_or_archive(relative):
            continue
        if relative in MIGRATION_CONTROL_PLANE:
            continue
        text = path.read_text(encoding="utf-8", errors="replace")
        for constructor in CONTROL_PLANE_CONSTRUCTORS:
            if constructor in text:
                findings.append(
                    f"{relative}: direct privileged control-plane constructor usage: {constructor}"
                )

    return findings


def main() -> int:
    all_files = files()
    findings = audit_control_plane_wiring(all_files)

    for path in all_files:
        relative = path.relative_to(ROOT)
        text = path.read_text(encoding="utf-8", errors="replace")
        for line_number, line in enumerate(text.splitlines(), 1):
            matched = [name for name, pattern in PATTERNS.items() if pattern.search(line)]
            if not matched:
                continue
            is_migration = relative.parts[:2] == ("database", "migrations")
            if relative in ALLOWED_FILES or is_migration:
                continue
            findings.append(f"{relative}:{line_number}: {','.join(matched)}")

    print("Protected-data static audit")
    print(f"Repository root: {ROOT}")
    if findings:
        print("FINDINGS (manual review required):")
        for finding in findings:
            print(f"- {finding}")
        return 1

    print("PASS: no unallowlisted SQL/database primitives detected.")
    print("PASS: privileged legacy-promotion control plane is not wired into the application runtime.")
    print("This audit is static inventory only; canonical authorization and DB enforcement remain separate gates.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
