"""Executable unittest contract tests for the SOMA Health Context Framework v1."""

import json
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SCHEMA = ROOT / "schemas" / "health-context-framework-v1.json"


def load_schema():
    return json.loads(SCHEMA.read_text(encoding="utf-8"))


class HealthContextFrameworkContractTests(unittest.TestCase):
    def test_context_schema_is_single_canonical_context_contract(self):
        schema = load_schema()
        self.assertTrue(schema["$id"].endswith("health-context-framework-v1.json"))
        self.assertEqual(schema["properties"]["schema_version"]["const"], "1.0.0")
        self.assertFalse(schema["additionalProperties"])

    def test_initial_contexts_are_governed_and_finite(self):
        contexts = load_schema()["properties"]["context_type"]["enum"]
        self.assertEqual(
            contexts,
            [
                "general_health",
                "athlete_performance",
                "service_veteran",
                "rehabilitation_recovery",
            ],
        )

    def test_context_references_canonical_records(self):
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
            self.assertIn(field, record_refs)
            self.assertEqual(record_refs[field]["type"], "array")

    def test_context_requires_subject_and_provenance(self):
        required = set(load_schema()["required"])
        self.assertTrue(
            {"subject_ref", "context_type", "record_refs", "provenance"} <= required
        )

    def test_context_metadata_is_not_a_second_canonical_domain(self):
        description = load_schema()["properties"]["context_metadata"]["description"]
        self.assertIn("Non-canonical context metadata", description)
        self.assertIn("separately governed versioned schemas", description)

    def test_context_contract_does_not_authorize_clinical_or_employment_decisions(self):
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
            self.assertIn(forbidden, text)


if __name__ == "__main__":
    unittest.main()
