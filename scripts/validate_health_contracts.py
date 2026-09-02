#!/usr/bin/env python3
"""Validate SOMA Health State / Evidence Graph v1 JSON Schema contracts."""

from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_DIR = ROOT / "schemas"

SCHEMAS = (
    SCHEMA_DIR / "health-state-v1.json",
    SCHEMA_DIR / "health-evidence-graph-v1.json",
    SCHEMA_DIR / "health-state-evidence-link-v1.json",
)


def load(path: Path) -> dict:
    with path.open(encoding="utf-8") as handle:
        value = json.load(handle)
    if not isinstance(value, dict):
        raise ValueError(f"{path}: schema root must be an object")
    if value.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
        raise ValueError(f"{path}: wrong JSON Schema dialect")
    if value.get("schema_version") is not None:
        raise ValueError(f"{path}: schema_version belongs in instances, not root")
    return value


def main() -> int:
    for path in SCHEMAS:
        schema = load(path)
        required = schema.get("required", [])
        if not required:
            raise ValueError(f"{path}: root required list is empty")
        if schema.get("additionalProperties") is not False:
            raise ValueError(f"{path}: root must reject unknown properties")
        print(f"validated: {path.relative_to(ROOT)}")
    print(f"validated {len(SCHEMAS)} SOMA health contracts")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
