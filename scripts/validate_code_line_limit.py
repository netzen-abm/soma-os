#!/usr/bin/env python3
"""Validate SOMA programming-file length against the 179-line maximum."""
from __future__ import annotations

import argparse
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXTENSIONS = {".py", ".rs", ".js", ".ts", ".tsx", ".jsx", ".sh", ".sql"}
EXCLUDED = {"target", ".git", "node_modules"}


def tracked_files():
    result = subprocess.run(
        ["git", "ls-files"], cwd=ROOT, check=True, capture_output=True, text=True
    )
    return [ROOT / item for item in result.stdout.splitlines()]


def candidate_files(paths):
    if not paths:
        return tracked_files()
    return [ROOT / item for item in paths]


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("paths", nargs="*")
    args = parser.parse_args()
    violations = []
    for path in candidate_files(args.paths):
        if not path.exists() or path.suffix.lower() not in EXTENSIONS:
            continue
        if any(part in EXCLUDED for part in path.parts):
            continue
        lines = path.read_text(encoding="utf-8").splitlines()
        if len(lines) >= 180:
            violations.append((path.relative_to(ROOT), len(lines)))
    if violations:
        for path, lines in violations:
            print(f"FAIL: {path} has {lines} lines; maximum is 179")
        return 1
    print("PASS: checked programming files are under 180 lines")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
