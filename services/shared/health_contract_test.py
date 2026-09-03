import json
import unittest
from pathlib import Path


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

    def test_unknown_root_properties_are_rejected_by_contract(self):
        for schema in self.schemas.values():
            self.assertFalse(schema["additionalProperties"])


if __name__ == "__main__":
    unittest.main()
