#!/usr/bin/env python3
"""Validate changed programming files against the 179-line maximum."""
from __future__ import annotations

import os
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXTENSIONS = {".py", ".rs", ".js", ".ts", ".tsx", ".jsx", ".sh", ".sql"}


def changed_files() -> list[Path]:
    base = os.environ.get("GITHUB_BASE_SHA") or os.environ.get("GITHUB_EVENT_BEFORE")
    if base and base != "0000000000000000000000000000000000000000":
        result = subprocess.run(
            ["git", "diff", "--name-only", base, "HEAD"],
            cwd=ROOT,
            check=True,
            capture_output=True,
            text=True,
        )
    else:
        result = subprocess.run(
            ["git", "diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD"],
            cwd=ROOT,
            check=True,
            capture_output=True,
            text=True,
        )
    return [ROOT / item for item in result.stdout.splitlines()]


def main() -> int:
    violations = []
    for path in changed_files():
        if path.suffix.lower() not in EXTENSIONS or not path.is_file():
            continue
        lines = path.read_text(encoding="utf-8").splitlines()
        if len(lines) >= 180:
            violations.append((path.relative_to(ROOT), len(lines)))
    if violations:
        for path, lines in violations:
            print(f"FAIL: {path} has {lines} lines; maximum is 179")
        return 1
    print("PASS: changed programming files are under 180 lines")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
