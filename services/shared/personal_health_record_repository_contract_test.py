import json
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
REPOSITORY_SCHEMA_PATH = ROOT / "schemas/personal-health-record-repository-v1.json"
PROFILE_SCHEMA_PATH = ROOT / "schemas/personal-health-profile-v1.json"
HEALTH_STATE_SCHEMA_PATH = ROOT / "schemas/health-state-v1.json"


class PersonalHealthRecordRepositoryContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.repository = json.loads(REPOSITORY_SCHEMA_PATH.read_text(encoding="utf-8"))
        cls.profile = json.loads(PROFILE_SCHEMA_PATH.read_text(encoding="utf-8"))
        cls.health_state = json.loads(HEALTH_STATE_SCHEMA_PATH.read_text(encoding="utf-8"))

    def test_repository_schema_is_strict(self):
        self.assertEqual(
            self.repository["$schema"],
            "https://json-schema.org/draft/2020-12/schema",
        )
        self.assertFalse(self.repository["additionalProperties"])
        self.assertEqual(
            self.repository["properties"]["schema_version"]["const"],
            "1.0.0",
        )

    def test_repository_is_a_local_vault_index(self):
        self.assertEqual(
            self.repository["properties"]["repository_kind"]["const"],
            "local_vault_index",
        )
        self.assertIn("vault_record_refs", self.repository["properties"])
        self.assertNotIn("health_payload", self.repository["properties"])
        self.assertNotIn("plaintext_record", self.repository["properties"])

    def test_profile_and_repository_share_reference_based_boundary(self):
        profile_refs = self.profile["properties"]["record_refs"]["properties"]
        self.assertIn("health_state_refs", profile_refs)
        self.assertIn("medication_refs", profile_refs)
        self.assertIn("prescription_refs", profile_refs)
        self.assertIn("observation_refs", profile_refs)
        self.assertIn("evidence_refs", profile_refs)
        self.assertIn("vault_record_refs", self.repository["properties"])

    def test_health_state_remains_canonical_domain_entity(self):
        self.assertEqual(
            self.health_state["$id"],
            "https://soma-os.org/schema/health-state-v1.json",
        )
        self.assertEqual(
            self.health_state["properties"]["schema_version"]["const"],
            "1.1.0",
        )
        self.assertIn("subject_ref", self.health_state["required"])
        self.assertIn("provenance", self.health_state["required"])

    def test_repository_requires_subject_and_provenance(self):
        required = set(self.repository["required"])
        self.assertTrue(
            {"id", "subject_ref", "schema_version", "repository_kind", "vault_record_refs", "provenance"}.issubset(required)
        )
        self.assertTrue(
            {"origin", "method"}.issubset(
                self.repository["properties"]["provenance"]["required"]
            )
        )

    def test_no_parallel_health_domain_model(self):
        properties = self.repository["properties"]
        forbidden = {
            "diagnosis",
            "medications",
            "prescriptions",
            "observations",
            "conditions",
            "interventions",
            "outcomes",
        }
        self.assertTrue(forbidden.isdisjoint(properties))


if __name__ == "__main__":
    unittest.main()
