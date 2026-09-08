"""Executable contract tests for the SOMA Health Context Framework v1."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SCHEMA = ROOT / "schemas" / "health-context-framework-v1.json"


def load_schema():
    return json.loads(SCHEMA.read_text(encoding="utf-8"))


def test_context_schema_is_single_canonical_context_contract():
    schema = load_schema()
    assert schema["$id"].endswith("health-context-framework-v1.json")
    assert schema["properties"]["schema_version"]["const"] == "1.0.0"
    assert schema["additionalProperties"] is False


def test_initial_contexts_are_governed_and_finite():
    contexts = load_schema()["properties"]["context_type"]["enum"]
    assert contexts == [
        "general_health",
        "athlete_performance",
        "service_veteran",
        "rehabilitation_recovery",
    ]


def test_context_references_canonical_records():
    record_refs = load_schema()["properties"]["record_refs"]["properties"]
    for field in (
        "health_state_refs",
        "observation_refs",
        "intervention_refs",
        "outcome_refs",
        "document_refs",
        "medication_refs",
        "prescription_refs",
        "evidence_refs",
    ):
        assert field in record_refs
        assert record_refs[field]["type"] == "array"


def test_context_requires_subject_and_provenance():
    required = set(load_schema()["required"])
    assert {"subject_ref", "context_type", "record_refs", "provenance"} <= required


def test_context_metadata_is_not_a_second_canonical_domain():
    description = load_schema()["properties"]["context_metadata"]["description"]
    assert "Non-canonical context metadata" in description
    assert "separately governed versioned schemas" in description


def test_context_contract_does_not_authorize_clinical_or_employment_decisions():
    text = (ROOT / "docs" / "architecture" / "health-context-framework-v1.md").read_text(
        encoding="utf-8"
    )
    for forbidden in (
        "autonomous clinical diagnosis",
        "autonomous prescribing",
        "automated return-to-play clearance",
        "insurance/employment decisions",
        "military fitness determinations",
    ):
        assert forbidden in text


if __name__ == "__main__":
    for name, fn in sorted(globals().items()):
        if name.startswith("test_"):
            fn()
    print("health context framework contract tests: PASS")
