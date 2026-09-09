import json
import pathlib
import unittest


ROOT = pathlib.Path(__file__).resolve().parents[2]
SCHEMA_PATH = ROOT / "schemas" / "local-health-vault-storage-index-v1.json"


class LocalHealthVaultStorageIndexContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        with SCHEMA_PATH.open(encoding="utf-8") as handle:
            cls.schema = json.load(handle)

    def test_schema_is_strict_and_versioned(self):
        self.assertEqual(self.schema["$schema"], "https://json-schema.org/draft/2020-12/schema")
        self.assertEqual(self.schema["type"], "object")
        self.assertTrue(self.schema["additionalProperties"] is False)
        self.assertIn("schema_version", self.schema["required"])

    def test_protected_classification_is_fixed(self):
        self.assertEqual(self.schema["properties"]["classification"]["const"], "sensitive_personal_health")

    def test_record_state_is_fail_closed(self):
        self.assertEqual(set(self.schema["properties"]["record_state"]["enum"]), {"ACTIVE", "TOMBSTONED"})

    def test_identity_and_key_reference_are_mandatory(self):
        required = set(self.schema["required"])
        self.assertIn("subject_ref", required)
        self.assertIn("key_ref", required)
        self.assertIn("provenance_ref", required)

    def test_index_has_no_plaintext_health_payload_field(self):
        forbidden = {"plaintext", "health_data", "payload", "ciphertext"}
        self.assertTrue(forbidden.isdisjoint(self.schema["properties"]))

    def test_index_contract_is_provider_neutral(self):
        serialized = json.dumps(self.schema).lower()
        for provider in ("postgres", "postgresql", "sqlite", "cloud", "s3"):
            self.assertNotIn(provider, serialized)


if __name__ == "__main__":
    unittest.main()
