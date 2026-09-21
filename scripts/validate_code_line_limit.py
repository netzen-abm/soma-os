#!/usr/bin/env python3
"""Validate SOMA programming-file length against the 179-line maximum."""
from __future__ import annotations

import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXTENSIONS = {".py", ".rs", ".js", ".ts", ".tsx", ".jsx", ".sh", ".sql"}
EXCLUDED = {"target", ".git", "node_modules"}

def tracked_files():
    result = subprocess.run(["git", "ls-files"], cwd=ROOT, check=True, capture_output=True, text=True)
    return [ROOT / item for item in result.stdout.splitlines()]

def main() -> int:
    violations = []
    for path in tracked_files():
        if path.suffix.lower() not in EXTENSIONS or any(part in EXCLUDED for part in path.parts):
            continue
        lines = path.read_text(encoding="utf-8").splitlines()
        if len(lines) >= 180:
            violations.append((path.relative_to(ROOT), len(lines)))
    if violations:
        for path, lines in violations:
            print(f"FAIL: {path} has {lines} lines; maximum is 179")
        return 1
    print("PASS: all tracked programming files are under 180 lines")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
