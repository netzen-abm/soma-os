import json
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parent
REGISTRY = ROOT / "capability_registry.json"
SCHEMA = ROOT.parents[1] / "schemas" / "capability-registry-v1.json"


class CapabilityRegistryTests(unittest.TestCase):
    def load_registry(self):
        return json.loads(REGISTRY.read_text(encoding="utf-8"))

    def load_schema(self):
        return json.loads(SCHEMA.read_text(encoding="utf-8"))

    def test_registry_is_valid_and_unique(self):
        data = self.load_registry()
        capabilities = data["capabilities"]
        ids = [item["id"] for item in capabilities]

        self.assertTrue(ids)
        self.assertEqual(len(ids), len(set(ids)))

    def test_registry_uses_canonical_schema_version(self):
        data = self.load_registry()
        schema = self.load_schema()
        self.assertEqual(data["schema_version"], schema["properties"]["schema_version"]["const"])

    def test_registry_entries_satisfy_schema_required_fields(self):
        data = self.load_registry()
        schema = self.load_schema()
        capability_schema = schema["$defs"]["capability"]

        for capability in data["capabilities"]:
            for field in capability_schema["required"]:
                self.assertIn(field, capability, msg=f"missing {field}: {capability.get('id')}")
            self.assertIsInstance(capability["principal_types"], list)
            self.assertTrue(capability["principal_types"])
            self.assertEqual(len(capability["principal_types"]), len(set(capability["principal_types"])))

            requirements = capability["identity_requirements"]
            for field in schema["$defs"]["identity_requirements"]["required"]:
                self.assertIn(field, requirements, msg=f"missing identity requirement {field}: {capability.get('id')}")
            self.assertTrue(requirements["applies_to_principal_types"])
            self.assertEqual(set(requirements["applies_to_principal_types"]), set(capability["principal_types"]))
            self.assertTrue(set(requirements["applies_to_principal_types"]).issubset(
                {"person", "agent", "service", "application", "device"}
            ))

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

    def test_sensitive_capabilities_must_declare_permission_lifecycle_when_enabled(self):
        data = self.load_registry()
        for capability in data["capabilities"]:
            if capability.get("requires_permission_lifecycle") is True:
                self.assertIn("surfaces", capability)
                self.assertIn("adapters", capability)
                self.assertTrue(capability["adapters"] or capability["surfaces"])


if __name__ == "__main__":
    unittest.main()
