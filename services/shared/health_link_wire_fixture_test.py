import json
import unittest
from pathlib import Path

from health_contract_test import SCHEMAS, validate


ROOT = Path(__file__).resolve().parents[2]
FIXTURE = ROOT / "tests/fixtures/health-state-evidence-link-v1.json"


class HealthLinkWireFixtureTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.schema = json.loads(SCHEMAS["link"].read_text(encoding="utf-8"))
        cls.fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))

    def test_shared_fixture_matches_root_schema(self):
        self.assertEqual(validate(self.schema, self.fixture), [])
        provenance_schema = self.schema["properties"]["provenance"]
        self.assertEqual(validate(provenance_schema, self.fixture["provenance"]), [])

    def test_absent_optional_fields_remain_absent(self):
        self.assertNotIn("context", self.fixture)
        self.assertNotIn("created_at", self.fixture["provenance"])
        self.assertNotIn("actor_ref", self.fixture["provenance"])
        self.assertNotIn("context", self.schema["required"])
        self.assertNotIn("uncertainty", self.schema["required"])

    def test_null_does_not_substitute_for_optional_object_or_timestamp(self):
        candidate = dict(self.fixture)
        candidate["context"] = None
        self.assertTrue(validate(self.schema, candidate))

        provenance = dict(self.fixture["provenance"])
        provenance["created_at"] = None
        self.assertTrue(validate(self.schema["properties"]["provenance"], provenance))

    def test_unknown_fields_are_rejected_at_both_contract_levels(self):
        candidate = dict(self.fixture)
        candidate["unexpected"] = True
        self.assertTrue(validate(self.schema, candidate))

        provenance = dict(self.fixture["provenance"])
        provenance["unexpected"] = True
        self.assertTrue(validate(self.schema["properties"]["provenance"], provenance))


if __name__ == "__main__":
    unittest.main()
