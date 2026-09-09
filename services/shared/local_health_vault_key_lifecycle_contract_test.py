import json
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SCHEMA_PATH = ROOT / "schemas" / "local-health-vault-key-lifecycle-v1.json"


class LocalHealthVaultKeyLifecycleContractTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))

    def test_schema_is_strict_and_versioned(self):
        self.assertEqual(self.schema["$schema"], "https://json-schema.org/draft/2020-12/schema")
        self.assertEqual(self.schema["properties"]["schema_version"]["const"], "1.0.0")
        self.assertFalse(self.schema["additionalProperties"])

    def test_key_material_is_not_a_schema_field(self):
        self.assertNotIn("key_material", self.schema["properties"])
        self.assertNotIn("raw_key", self.schema["properties"])
        self.assertNotIn("secret", self.schema["properties"])

    def test_vault_scope_and_algorithm_are_fixed(self):
        self.assertEqual(self.schema["properties"]["key_domain"]["const"], "local_health_vault")
        self.assertEqual(self.schema["properties"]["algorithm"]["const"], "AES-256-GCM")
        self.assertEqual(self.schema["properties"]["key_length_bits"]["const"], 256)

    def test_lifecycle_has_fail_closed_security_states(self):
        states = set(self.schema["properties"]["state"]["enum"])
        self.assertTrue({"PROVISIONED", "ACTIVE", "RETIRING", "RETIRED", "SUSPENDED", "COMPROMISED"}.issubset(states))

    def test_protection_class_requires_explicit_boundary(self):
        protection = set(self.schema["properties"]["protection_class"]["enum"])
        self.assertEqual(protection, {"HARDWARE_BACKED", "PLATFORM_KEYSTORE", "SOFTWARE_PROTECTED"})

    def test_subject_scope_is_required(self):
        required = set(self.schema["properties"]["subject_scope"]["required"])
        self.assertEqual(required, {"subject_ref", "data_domain"})


if __name__ == "__main__":
    unittest.main()
