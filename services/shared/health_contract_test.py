import copy
import json
import unittest
from pathlib import Path

from jsonschema import Draft202012Validator


ROOT = Path(__file__).resolve().parents[2]
SCHEMAS = {
    "health_state": ROOT / "schemas/health-state-v1.json",
    "evidence": ROOT / "schemas/health-evidence-graph-v1.json",
    "link": ROOT / "schemas/health-state-evidence-link-v1.json",
}


class HealthContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.schemas = {
            name: json.loads(path.read_text(encoding="utf-8"))
            for name, path in SCHEMAS.items()
        }
        for schema in cls.schemas.values():
            Draft202012Validator.check_schema(schema)

    def assert_valid(self, schema_name, instance):
        Draft202012Validator(self.schemas[schema_name]).validate(instance)

    def assert_invalid(self, schema_name, instance):
        errors = list(Draft202012Validator(self.schemas[schema_name]).iter_errors(instance))
        self.assertTrue(errors, f"expected {schema_name} instance to be invalid")

    def assert_schema_shape(self, schema):
        self.assertEqual(schema["$schema"], "https://json-schema.org/draft/2020-12/schema")
        self.assertFalse(schema["additionalProperties"])
        self.assertTrue(schema["required"])

    def test_health_state_schema_envelope_and_boundaries(self):
        schema = self.schemas["health_state"]
        self.assert_schema_shape(schema)
        self.assertEqual(schema["properties"]["schema_version"]["const"], "1.0.0")
        self.assertIn("observation", schema["properties"]["entity_type"]["enum"])
        self.assertIn("interpretation", schema["properties"]["entity_type"]["enum"])
        self.assertEqual(schema["properties"]["classification"]["enum"], [
            "personal_health", "sensitive_personal_health", "derived_health", "research_context"
        ])

    def test_observation_requires_concept_and_uncertainty(self):
        rules = self.schemas["health_state"]["allOf"]
        observation_rule = next(r for r in rules if r["if"]["properties"]["entity_type"]["const"] == "observation")
        self.assertEqual(set(observation_rule["then"]["required"]), {"concept", "uncertainty"})

    def test_interpretation_requires_relationships_and_uncertainty(self):
        rules = self.schemas["health_state"]["allOf"]
        interpretation_rule = next(r for r in rules if r["if"]["properties"]["entity_type"]["const"] == "interpretation")
        self.assertEqual(set(interpretation_rule["then"]["required"]), {"relationships", "uncertainty"})

    def test_evidence_keeps_strength_and_safety_as_separate_dimensions(self):
        schema = self.schemas["evidence"]
        self.assert_schema_shape(schema)
        evidence = set(schema["properties"]["evidence_level"]["enum"])
        safety = set(schema["properties"]["safety_classification"]["enum"])
        self.assertIn("E3_SUPPORTED", evidence)
        self.assertIn("E4_WELL_SUPPORTED", evidence)
        self.assertIn("CONTRAINDICATED", safety)
        self.assertNotEqual(evidence, safety)

    def test_link_is_directional_and_provenance_bearing(self):
        schema = self.schemas["link"]
        self.assert_schema_shape(schema)
        self.assertEqual(set(schema["required"]), {
            "id", "schema_version", "health_state_ref", "evidence_ref", "relationship", "provenance"
        })
        relationships = set(schema["properties"]["relationship"]["enum"])
        self.assertIn("EVIDENCE_INFORMS_INTERPRETATION", relationships)
        self.assertIn("PERSONAL_RESPONSE_OBSERVED_AFTER_INTERVENTION", relationships)
        self.assertIn("method", schema["properties"]["provenance"]["required"])

    def test_positive_health_observation_instance(self):
        self.assert_valid("health_state", {
            "id": "hs-obs-001",
            "subject_ref": "person-001",
            "entity_type": "observation",
            "schema_version": "1.0.0",
            "status": "active",
            "recorded_time": "2026-09-03T12:00:00Z",
            "concept": "sleep_duration",
            "value": 7.5,
            "unit": "hours",
            "provenance": {"origin": "user", "method": "self_report"},
            "classification": "personal_health",
            "uncertainty": "reported",
        })

    def test_negative_observation_missing_uncertainty(self):
        instance = {
            "id": "hs-obs-002",
            "subject_ref": "person-001",
            "entity_type": "observation",
            "schema_version": "1.0.0",
            "status": "active",
            "recorded_time": "2026-09-03T12:00:00Z",
            "concept": "sleep_duration",
            "provenance": {"origin": "user", "method": "self_report"},
            "classification": "personal_health",
        }
        self.assert_invalid("health_state", instance)

    def test_negative_health_state_unknown_root_property(self):
        instance = {
            "id": "hs-obs-003",
            "subject_ref": "person-001",
            "entity_type": "observation",
            "schema_version": "1.0.0",
            "status": "active",
            "recorded_time": "2026-09-03T12:00:00Z",
            "concept": "sleep_duration",
            "provenance": {"origin": "user", "method": "self_report"},
            "classification": "personal_health",
            "uncertainty": "reported",
            "unexpected_field": True,
        }
        self.assert_invalid("health_state", instance)

    def test_positive_interpretation_instance(self):
        self.assert_valid("health_state", {
            "id": "hs-int-001",
            "subject_ref": "person-001",
            "entity_type": "interpretation",
            "schema_version": "1.0.0",
            "status": "active",
            "recorded_time": "2026-09-03T12:05:00Z",
            "provenance": {"origin": "system", "method": "rule_based"},
            "classification": "derived_health",
            "uncertainty": "inferred",
            "relationships": [{"type": "DERIVED_FROM", "target_ref": "hs-obs-001"}],
        })

    def test_negative_interpretation_missing_relationships(self):
        instance = {
            "id": "hs-int-002",
            "subject_ref": "person-001",
            "entity_type": "interpretation",
            "schema_version": "1.0.0",
            "status": "active",
            "recorded_time": "2026-09-03T12:05:00Z",
            "provenance": {"origin": "system", "method": "rule_based"},
            "classification": "derived_health",
            "uncertainty": "inferred",
        }
        self.assert_invalid("health_state", instance)

    def test_positive_evidence_claim_instance(self):
        self.assert_valid("evidence", {
            "id": "ev-claim-001",
            "entity_type": "claim",
            "schema_version": "1.0.0",
            "status": "EVIDENCE_ASSESSED",
            "statement": "Example research claim",
            "claim_type": "association",
            "evidence_level": "E3_SUPPORTED",
            "uncertainty": "reported",
            "provenance": {"source_ref": "source-001", "method": "literature_review"},
        })

    def test_negative_evidence_claim_missing_evidence_level(self):
        instance = {
            "id": "ev-claim-002",
            "entity_type": "claim",
            "schema_version": "1.0.0",
            "status": "UNREVIEWED",
            "statement": "Example research claim",
            "claim_type": "association",
            "uncertainty": "reported",
            "provenance": {"source_ref": "source-001", "method": "literature_review"},
        }
        self.assert_invalid("evidence", instance)

    def test_evidence_strength_and_safety_can_coexist_without_collapsing(self):
        instance = {
            "id": "ev-claim-003",
            "entity_type": "claim",
            "schema_version": "1.0.0",
            "status": "SAFETY_REVIEWED",
            "statement": "Example claim requiring safety qualification",
            "claim_type": "intervention_effect",
            "evidence_level": "E3_SUPPORTED",
            "safety_classification": "CONTRAINDICATED",
            "uncertainty": "reported",
            "provenance": {"source_ref": "source-002", "method": "evidence_assessment"},
        }
        self.assert_valid("evidence", instance)

    def test_positive_directional_link_instance(self):
        self.assert_valid("link", {
            "id": "link-001",
            "schema_version": "1.0.0",
            "health_state_ref": "hs-int-001",
            "evidence_ref": "ev-claim-001",
            "relationship": "EVIDENCE_INFORMS_INTERPRETATION",
            "provenance": {"method": "policy_governed_reference"},
        })

    def test_negative_directional_link_invalid_relationship(self):
        instance = {
            "id": "link-002",
            "schema_version": "1.0.0",
            "health_state_ref": "hs-int-001",
            "evidence_ref": "ev-claim-001",
            "relationship": "SUPPORTS",
            "provenance": {"method": "policy_governed_reference"},
        }
        self.assert_invalid("link", instance)

    def test_personal_observation_is_not_accepted_as_evidence_claim(self):
        observation = {
            "id": "hs-obs-004",
            "subject_ref": "person-001",
            "entity_type": "observation",
            "schema_version": "1.0.0",
            "status": "active",
            "recorded_time": "2026-09-03T12:00:00Z",
            "concept": "sleep_duration",
            "value": 7.5,
            "unit": "hours",
            "provenance": {"origin": "user", "method": "self_report"},
            "classification": "personal_health",
            "uncertainty": "reported",
        }
        self.assert_valid("health_state", observation)
        self.assert_invalid("evidence", observation)

    def test_unknown_root_properties_are_rejected_by_contract(self):
        for schema in self.schemas.values():
            self.assertFalse(schema["additionalProperties"])


if __name__ == "__main__":
    unittest.main()
