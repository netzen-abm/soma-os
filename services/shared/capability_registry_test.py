import json
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parent
REGISTRY = ROOT / "capability_registry.json"


class CapabilityRegistryTests(unittest.TestCase):
    def load_registry(self):
        return json.loads(REGISTRY.read_text(encoding="utf-8"))

    def test_registry_is_valid_and_unique(self):
        data = self.load_registry()
        capabilities = data["capabilities"]
        ids = [item["id"] for item in capabilities]

        self.assertTrue(ids)
        self.assertEqual(len(ids), len(set(ids)))

    def test_decentralized_capabilities_are_optional_adapters(self):
        data = self.load_registry()
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
        data = self.load_registry()
        ai = next(
            item for item in data["capabilities"]
            if item["id"] == "intelligence.ai"
        )
        self.assertEqual(ai["status"], "optional")

    def test_agent_and_mcp_are_registered(self):
        data = self.load_registry()
        ids = {item["id"] for item in data["capabilities"]}

        self.assertIn("agent.platform", ids)
        self.assertIn("protocol.mcp", ids)


if __name__ == "__main__":
    unittest.main()
