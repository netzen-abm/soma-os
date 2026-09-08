import json
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
PROFILE_SCHEMA_PATH = ROOT / "schemas/personal-health-profile-v1.json"
EMERGENCY_SCHEMA_PATH = ROOT / "schemas/emergency-health-profile-v1.json"
SHARING_SCHEMA_PATH = ROOT / "schemas/health-sharing-grant-v1.json"
HEALTH_STATE_SCHEMA_PATH = ROOT / "schemas/health-state-v1.json"


class PersonalHealthRecordContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.profile = json.loads(PROFILE_SCHEMA_PATH.read_text(encoding="utf-8"))
        cls.emergency = json.loads(EMERGENCY_SCHEMA_PATH.read_text(encoding="utf-8"))
        cls.sharing = json.loads(SHARING_SCHEMA_PATH.read_text(encoding="utf-8"))
        cls.health_state = json.loads(HEALTH_STATE_SCHEMA_PATH.read_text(encoding="utf-8"))

    def test_schemas_are_strict_json_schema_contracts(self):
        for schema in (self.profile, self.emergency, self.sharing):
            self.assertEqual(schema["$schema"], "https://json-schema.org/draft/2020-12/schema")
            self.assertFalse(schema["additionalProperties"])
            self.assertTrue(schema["required"])

    def test_personal_profile_references_canonical_records(self):
        self.assertEqual(self.profile["properties"]["schema_version"]["const"], "1.0.0")
        record_refs = self.profile["properties"]["record_refs"]["properties"]
        self.assertIn("health_state_refs", record_refs)
        self.assertIn("medication_refs", record_refs)
        self.assertIn("prescription_refs", record_refs)
        self.assertIn("document_refs", record_refs)
        self.assertIn("observation_refs", record_refs)
        self.assertIn("evidence_refs", record_refs)
        self.assertIn("health_state_refs", self.profile["properties"]["record_refs"]["properties"])

    def test_profile_does_not_replace_canonical_health_state(self):
        self.assertEqual(
            self.health_state["$id"],
            "https://soma-os.org/schema/health-state-v1.json",
        )
        self.assertIn("health_state_refs", self.profile["properties"]["record_refs"]["properties"])
        self.assertNotIn("diagnosis", self.profile["properties"])

    def test_emergency_profile_is_explicitly_user_approved_and_minimal(self):
        policy = self.emergency["properties"]["disclosure_policy"]
        self.assertEqual(policy["properties"]["purpose"]["const"], "emergency")
        self.assertTrue(policy["properties"]["user_approved"]["const"])
        self.assertTrue(policy["properties"]["minimum_necessary"]["const"])
        self.assertEqual(
            self.emergency["properties"]["classification"]["const"],
            "sensitive_personal_health",
        )

    def test_emergency_profile_has_field_level_provenance_support(self):
        provenance = self.emergency["properties"]["provenance"]["properties"]
        self.assertIn("field_source_refs", provenance)
        self.assertIn("record_freshness_at", provenance)

    def test_sharing_is_purpose_and_scope_bound(self):
        required = set(self.sharing["required"])
        self.assertTrue({"recipient_ref", "purpose", "scope", "status", "consent"}.issubset(required))
        scope = self.sharing["properties"]["scope"]["properties"]
        self.assertIn("record_refs", scope)
        self.assertIn("minimum_necessary", scope)
        self.assertIn("allow_export", scope)
        self.assertIn("allow_resharing", scope)

    def test_emergency_sharing_cannot_set_minimum_necessary_false(self):
        rule = self.sharing["allOf"][0]
        self.assertEqual(
            rule["if"]["properties"]["purpose"]["const"],
            "emergency",
        )
        self.assertTrue(
            rule["then"]["properties"]["scope"]["properties"]["minimum_necessary"]["const"]
        )

    def test_medication_intelligence_is_not_a_prescribing_contract(self):
        profile_text = json.dumps(self.profile).lower()
        emergency_text = json.dumps(self.emergency).lower()
        for text in (profile_text, emergency_text):
            self.assertNotIn('"prescribe"', text)
            self.assertNotIn('"dosage_change"', text)
            self.assertNotIn('"medication_discontinue"', text)


if __name__ == "__main__":
    unittest.main()
