import json
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SCHEMA_PATH = ROOT / "schemas/longitudinal-observation-v1.json"


class LongitudinalObservationContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))

    def test_schema_is_strict_and_versioned(self):
        self.assertEqual(self.schema["$schema"], "https://json-schema.org/draft/2020-12/schema")
        self.assertFalse(self.schema["additionalProperties"])
        self.assertEqual(self.schema["properties"]["schema_version"]["const"], "1.0.0")
        self.assertEqual(self.schema["properties"]["entity_type"]["const"], "observation")

    def test_canonical_observation_fields_exist(self):
        properties = self.schema["properties"]
        for field in ("id", "subject_ref", "concept", "value", "unit", "recorded_at", "provenance", "classification", "uncertainty"):
            self.assertIn(field, properties)
        self.assertIn("observed_at", properties)
        self.assertIn("observed_from", properties)
        self.assertIn("observed_to", properties)

    def test_observation_and_recording_time_are_distinct(self):
        required = set(self.schema["required"])
        self.assertIn("recorded_at", required)
        self.assertNotIn("observed_at", required)

    def test_interval_time_requires_both_bounds(self):
        rules = self.schema["allOf"]
        self.assertEqual(len(rules), 2)
        self.assertIn("observed_to", rules[0]["then"]["required"])
        self.assertIn("observed_from", rules[1]["then"]["required"])

    def test_uncertainty_and_provenance_are_required(self):
        self.assertIn("uncertainty", self.schema["required"])
        self.assertIn("provenance", self.schema["required"])
        self.assertEqual(set(self.schema["properties"]["provenance"]["required"]), {"origin", "method"})

    def test_association_is_not_causation(self):
        relationships = set(self.schema["properties"]["relationships"]["items"]["properties"]["type"]["enum"])
        self.assertIn("ASSOCIATED_WITH", relationships)
        self.assertIn("TEMPORALLY_PRECEDES", relationships)
        self.assertNotIn("CAUSES", relationships)

    def test_provenance_and_classification_are_explicit(self):
        classification = set(self.schema["properties"]["classification"]["enum"])
        self.assertIn("personal_health", classification)
        self.assertIn("derived_health", classification)
        self.assertIn("research_context", classification)


if __name__ == "__main__":
    unittest.main()
