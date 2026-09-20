#!/usr/bin/env python3
"""Fail-closed inventory check for literal production capability identifiers.

This gate does not attempt to prove runtime reachability. It ensures that any
literal capability_id used by active Rust/Python production code is registered
in the canonical capability registry. Dynamic capability construction remains
an explicit review item.
"""

from __future__ import annotations

import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
REGISTRY = ROOT / "services/shared/capability_registry.json"
SEARCH_ROOTS = (
    ROOT / "services/backend-rust/src",
    ROOT / "services/shared",
)

PATTERNS = (
    re.compile(r"\bcapability_id\s*[:=]\s*["']([^"']+)["']"),
    re.compile(r"\bcapability_id\s*:\s*String::from\(["']([^"']+)["']\)"),
)


def production_files():
    for root in SEARCH_ROOTS:
        if not root.exists():
            continue
        for path in root.rglob("*"):
            if not path.is_file():
                continue
            if path.suffix not in {".rs", ".py"}:
                continue
            if path.name.endswith("_test.rs") or path.name.endswith("_test.py"):
                continue
            if path.name.startswith("test_"):
                continue
            yield path


def main() -> int:
    registry = json.loads(REGISTRY.read_text(encoding="utf-8"))
    registered = {
        capability["id"]
        for capability in registry.get("capabilities", [])
        if isinstance(capability, dict) and isinstance(capability.get("id"), str)
    }

    findings: list[str] = []
    discovered: dict[str, list[str]] = {}

    for path in production_files():
        text = path.read_text(encoding="utf-8", errors="replace")
        for pattern in PATTERNS:
            for match in pattern.finditer(text):
                capability_id = match.group(1)
                discovered.setdefault(capability_id, []).append(str(path.relative_to(ROOT)))
                if capability_id not in registered:
                    findings.append(
                        f"{path.relative_to(ROOT)}: unregistered capability_id literal: {capability_id}"
                    )

    if findings:
        print("FAIL: production capability identifiers are missing from the canonical registry")
        for finding in sorted(set(findings)):
            print(f"- {finding}")
        return 1

    print("PASS: all literal production capability identifiers are registered.")
    for capability_id in sorted(discovered):
        print(f"- {capability_id}: {len(discovered[capability_id])} production source file(s)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
