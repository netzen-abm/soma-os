#!/usr/bin/env python3
"""Fail-closed static audit for protected PostgreSQL access paths.

This is intentionally conservative: it inventories direct SQL/database primitives
outside the canonical protected persistence boundary and reports findings for review.
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
    pathlib.Path("services/backend-rust/src/db_postgres_integration_test.rs"),
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


def main() -> int:
    findings: list[str] = []
    for path in files():
        relative = path.relative_to(ROOT)
        text = path.read_text(encoding="utf-8", errors="replace")
        for line_number, line in enumerate(text.splitlines(), 1):
            matched = [name for name, pattern in PATTERNS.items() if pattern.search(line)]
            if not matched:
                continue
            if relative in ALLOWED_FILES or relative.parts[:3] == ("database", "migrations",):
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
    print("This audit is static inventory only; runtime authorization and DB enforcement remain separate gates.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
