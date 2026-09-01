import unittest
from pathlib import Path


CONTRACT = Path(__file__).resolve().parent / "mcp_adapter_contract.md"


class McpAdapterContractTests(unittest.TestCase):
    def test_contract_exists(self):
        self.assertTrue(CONTRACT.is_file())

    def test_contract_declares_security_boundary(self):
        text = CONTRACT.read_text(encoding="utf-8")
        self.assertIn("not the SOMA authorization model", text)
        self.assertIn("untrusted inputs", text)
        self.assertIn("local MCP server is not implicitly trusted", text)

    def test_contract_declares_failure_isolation(self):
        text = CONTRACT.read_text(encoding="utf-8")
        self.assertIn("MCP server failure must be isolated", text)
        self.assertIn("Retries for side-effecting operations require idempotency", text)

    def test_contract_declares_production_data_controls(self):
        text = CONTRACT.read_text(encoding="utf-8")
        self.assertIn("unrestricted production credentials", text)
        self.assertIn("short-lived authorization", text)


if __name__ == "__main__":
    unittest.main()
