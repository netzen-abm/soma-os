#!/usr/bin/env python3
"""Validate canonical SOMA Health State / Evidence Graph v1 JSON Schema contracts and representative instances."""

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


def validate_health_state_contract(schema: dict) -> None:
    required = set(schema["required"])
    expected = {"id", "subject_ref", "entity_type", "schema_version", "status", "recorded_time", "provenance", "classification"}
    if not expected.issubset(required):
        raise ValueError("health-state-v1: canonical envelope requirements changed unexpectedly")
    entity_types = set(schema["properties"]["entity_type"]["enum"])
    if "observation" not in entity_types or "interpretation" not in entity_types:
        raise ValueError("health-state-v1: observation/interpretation boundary missing")
    if schema["properties"]["schema_version"].get("const") != "1.0.0":
        raise ValueError("health-state-v1: unexpected schema version")


def validate_evidence_contract(schema: dict) -> None:
    if schema["properties"]["schema_version"].get("const") != "1.0.0":
        raise ValueError("health-evidence-graph-v1: unexpected schema version")
    evidence_levels = set(schema["properties"]["evidence_level"]["enum"])
    safety_levels = set(schema["properties"]["safety_classification"]["enum"])
    if not evidence_levels or not safety_levels:
        raise ValueError("health-evidence-graph-v1: evidence and safety dimensions must remain explicit")
    if not {"E3_SUPPORTED", "E4_WELL_SUPPORTED"}.issubset(evidence_levels):
        raise ValueError("health-evidence-graph-v1: canonical evidence levels missing")


def validate_link_contract(schema: dict) -> None:
    required = set(schema["required"])
    expected = {"id", "schema_version", "health_state_ref", "evidence_ref", "relationship", "provenance"}
    if not expected.issubset(required):
        raise ValueError("health-state-evidence-link-v1: required directional/provenance fields missing")
    relationships = set(schema["properties"]["relationship"]["enum"])
    if not relationships:
        raise ValueError("health-state-evidence-link-v1: relationship vocabulary is empty")


def main() -> int:
    loaded = {path.name: load(path) for path in SCHEMAS}
    for path in SCHEMAS:
        schema = loaded[path.name]
        if not schema.get("required"):
            raise ValueError(f"{path}: root required list is empty")
        if schema.get("additionalProperties") is not False:
            raise ValueError(f"{path}: root must reject unknown properties")
        print(f"validated schema: {path.relative_to(ROOT)}")

    validate_health_state_contract(loaded["health-state-v1.json"])
    validate_evidence_contract(loaded["health-evidence-graph-v1.json"])
    validate_link_contract(loaded["health-state-evidence-link-v1.json"])
    print("validated semantic contract invariants: health-state / evidence / directional-link")
    print(f"validated {len(SCHEMAS)} SOMA health contracts")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
