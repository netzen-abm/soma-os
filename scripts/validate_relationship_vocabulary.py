#!/usr/bin/env python3
"""Validate the canonical SOMA relationship vocabulary against domain schemas."""
from __future__ import annotations
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
VOCAB = ROOT / "schemas/relationship-vocabulary-v1.json"

def fail(message: str) -> None:
    raise SystemExit(f"FAIL: {message}")

def relationship_values(schema_path: Path) -> set[str]:
    data = json.loads(schema_path.read_text(encoding="utf-8"))
    try:
        return set(data["properties"]["relationships"]["items"]["properties"]["type"]["enum"])
    except KeyError as exc:
        fail(f"{schema_path}: relationship enum missing: {exc}")

def main() -> int:
    vocab = json.loads(VOCAB.read_text(encoding="utf-8"))
    declared = set(vocab.get("relationship_types", {}))
    if not declared:
        fail("canonical relationship vocabulary is empty")
    if "CAUSES" in declared:
        fail("CAUSES must remain outside the foundational relationship vocabulary")
    for path in (
        ROOT / "schemas/health-state-v1.json",
        ROOT / "schemas/longitudinal-observation-v1.json",
    ):
        values = relationship_values(path)
        unknown = values - declared
        if unknown:
            fail(f"{path}: relationship values not in canonical vocabulary: {sorted(unknown)}")
    for name, metadata in vocab["relationship_types"].items():
        if metadata.get("causal") is not False:
            fail(f"{name}: foundational relationship must declare causal=false")
    print("PASS: canonical relationship vocabulary")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
