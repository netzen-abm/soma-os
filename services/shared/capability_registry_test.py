import json
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parent
REGISTRY = ROOT / "capability_registry.json"


class CapabilityRegistryTests(unittest.TestCase):
    def test_registry_is_valid_and_unique(self):
        data = json.loads(REGISTRY.read_text(encoding="utf-8"))
        capabilities = data["capabilities"]
        ids = [item["id"] for item in capabilities]

        self.assertTrue(ids)
        self.assertEqual(len(ids), len(set(ids)))

    def test_decentralized_capabilities_are_optional_adapters(self):
        data = json.loads(REGISTRY.read_text(encoding="utf-8"))
        items = {item["id"]: item for item in data["capabilities"]}

        for capability_id in (
            "protocol.web3",
            "identity.decentralized",
            "storage.content-addressed",
        ):
            capability = items[capability_id]
            self.assertIn("adapters", capability)
            self.assertIn("surfaces", capability)
            self.assertNotEqual(capability["status"], "core-dependency")

    def test_ai_is_not_a_core_dependency(self):
        data = json.loads(REGISTRY.read_text(encoding="utf-8"))
        ai = next(
            item for item in data["capabilities"]
            if item["id"] == "intelligence.ai"
        )
        self.assertEqual(ai["status"], "optional")


if __name__ == "__main__":
    unittest.main()
