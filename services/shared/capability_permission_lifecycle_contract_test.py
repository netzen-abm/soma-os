import json
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCHEMA = ROOT / "schemas/capability-permission-lifecycle-v1.json"

class CapabilityPermissionLifecycleContractTests(unittest.TestCase):
    def test_schema_is_valid_json_and_requires_safe_lifecycle_fields(self):
        data = json.loads(SCHEMA.read_text(encoding="utf-8"))
        required = set(data["required"])
        self.assertTrue({"principal_ref","subject_ref","purpose","scope","status","expires_at","auto_revoke"} <= required)
        self.assertEqual(data["properties"]["auto_revoke"]["const"], True)
        self.assertEqual(data["properties"]["regrant_requires_fresh_consent"]["const"], True)
        self.assertIn("REVOKED", data["properties"]["status"]["enum"])
        self.assertIn("EXPIRED", data["properties"]["status"]["enum"])

if __name__ == "__main__":
    unittest.main()
