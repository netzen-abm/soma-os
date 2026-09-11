import json
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parent
REPO_ROOT = ROOT.parents[1]
REGISTRY = ROOT / "capability_registry.json"
SCHEMA = REPO_ROOT / "schemas" / "capability-registry-v1.json"

REQUIRED_FIELDS = {
    "id",
    "version",
    "domain",
    "status",
    "maturity",
    "principal_types",
    "identity_requirements",
    "policy",
    "adapters",
    "surfaces",
}


class CapabilityRegistryTests(unittest.TestCase):
    def load_registry(self):
        return json.loads(REGISTRY.read_text(encoding="utf-8"))

    def test_registry_matches_canonical_schema_boundary(self):
        data = self.load_registry()
        schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
        self.assertEqual(data["schema_version"], schema["properties"]["schema_version"]["const"])
        self.assertIsInstance(data["capabilities"], list)
        self.assertTrue(data["capabilities"])
        for capability in data["capabilities"]:
            self.assertEqual(set(capability), REQUIRED_FIELDS)
            self.assertTrue(capability["id"])
            self.assertTrue(capability["version"])
            self.assertTrue(capability["principal_types"])
            self.assertTrue(capability["policy"])
            self.assertIsInstance(capability["adapters"], list)
            self.assertIsInstance(capability["surfaces"], list)

    def test_registry_is_valid_and_unique(self):
        data = self.load_registry()
        capabilities = data["capabilities"]
        ids = [item["id"] for item in capabilities]
        self.assertEqual(len(ids), len(set(ids)))
        versions = [(item["id"], item["version"]) for item in capabilities]
        self.assertEqual(len(versions), len(set(versions)))

    def test_registry_is_canonical_not_a_runtime_inventory(self):
        rust = (REPO_ROOT / "services" / "backend-rust" / "src" / "shared_infrastructure.rs").read_text(encoding="utf-8")
        self.assertNotIn("pub const CAPABILITIES", rust)
        self.assertIn("include_str!(\"../../shared/capability_registry.json\")", rust)

    def test_decentralized_capabilities_are_adapters_not_core_dependencies(self):
        data = self.load_registry()
        items = {item["id"]: item for item in data["capabilities"]}
        for capability_id in (
            "protocol.web3",
            "identity.decentralized",
            "storage.content-addressed",
        ):
            capability = items[capability_id]
            self.assertTrue(capability["adapters"])
            self.assertTrue(capability["surfaces"])
            self.assertNotEqual(capability["status"], "core-dependency")

    def test_ai_is_not_a_core_dependency(self):
        data = self.load_registry()
        ai = next(item for item in data["capabilities"] if item["id"] == "intelligence.ai")
        self.assertEqual(ai["status"], "optional")
        self.assertIn("user-choice", ai["policy"])

    def test_agent_and_mcp_are_registered(self):
        data = self.load_registry()
        ids = {item["id"] for item in data["capabilities"]}
        self.assertIn("agent.platform", ids)
        self.assertIn("protocol.mcp", ids)


if __name__ == "__main__":
    unittest.main()
