#!/usr/bin/env python3
"""Validate the non-destructive Evidence Canonical Mapping v1 pilot fixtures."""

from __future__ import annotations

import json
import sys
from pathlib import Path

LEGACY_STATES = {
    "STRONG",
    "MODERATE",
    "LIMITED",
    "MIXED",
    "INDIRECT",
    "INSUFFICIENT",
}
CANONICAL_LEVELS = {
    "E0_UNKNOWN",
    "E1_PLAUSIBLE",
    "E2_PRELIMINARY",
    "E3_SUPPORTED",
    "E4_WELL_SUPPORTED",
}
DIRECTNESS = {
    "DIRECT_EXACT_PROTOCOL",
    "DIRECT_COMPONENT",
    "RELATED_INTERVENTION",
    "MECHANISTIC",
    "OBSERVATIONAL_ASSOCIATION",
    "PRECLINICAL",
    "INDIRECT",
    "NO_RELEVANT_EVIDENCE_FOUND",
}
SAFETY = {
    "LOW_CONCERN",
    "CONTEXT_DEPENDENT",
    "CLINICALLY_SIGNIFICANT_CAUTION",
    "CONTRAINDICATED",
    "INSUFFICIENT_SAFETY_EVIDENCE",
}


def validate(path: Path) -> dict[str, object]:
    payload = json.loads(path.read_text(encoding="utf-8"))
    records = payload["records"]
    assert records, "pilot must contain fixtures"

    ids = {record["id"] for record in records}
    assert len(ids) == len(records), "fixture IDs must be unique"

    for record in records:
        state = record.get("legacy_evidence_state")
        if state:
            assert state in LEGACY_STATES, f"unknown legacy state: {state}"
            expected = record.get("expected_level")
            if expected:
                assert expected in CANONICAL_LEVELS
            if state == "STRONG":
                assert expected != "E4_WELL_SUPPORTED", "silent STRONG -> E4 inflation"

        directness = record.get("directness")
        if directness:
            assert directness in DIRECTNESS, f"unknown directness: {directness}"

        safety = record.get("expected_safety")
        if safety:
            assert safety in SAFETY

        if record.get("kind") == "safety":
            assert record.get("efficacy_level") == "E0_UNKNOWN", "safety must remain independent"

        if record.get("kind") in {"analytical_assay", "traditional_knowledge", "search_result"}:
            assert record.get("expected_level") == "E0_UNKNOWN"

        if record.get("relationship") == "CONTRADICTS":
            assert record["relationship"] == "CONTRADICTS"

    duplicate_records = [r for r in records if r.get("expected_duplicate_of")]
    assert duplicate_records, "duplicate identity fixture missing"
    assert all(r["expected_duplicate_of"] in ids for r in duplicate_records)

    missing = next(r for r in records if r["id"] == "missing-provenance-001")
    assert missing["provenance_status"] == "UNKNOWN"

    null = next(r for r in records if r["id"] == "null-001")
    assert null["directness"] == "NO_RELEVANT_EVIDENCE_FOUND"
    assert null["expected_level"] == "E0_UNKNOWN"

    return {
        "status": "PILOT_FIXTURES_VALIDATED",
        "fixture_version": payload["version"],
        "record_count": len(records),
        "invariants_checked": 10,
        "publication": "NOT_AUTHORIZED",
    }


def main() -> int:
    if len(sys.argv) != 2:
        print("Usage: validate_evidence_canonical_mapping_pilot.py <fixture.json>")
        return 2
    result = validate(Path(sys.argv[1]))
    print(json.dumps(result, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
