import json
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCHEMA_PATH = ROOT / "schemas/governed-capability-operation-v1.json"


def validate(schema, instance):
    errors = []
    if not isinstance(instance, dict):
        return ["instance must be an object"]
    for key in schema.get("required", []):
        if key not in instance:
            errors.append(f"missing required property: {key}")
    properties = schema.get("properties", {})
    if schema.get("additionalProperties") is False:
        errors.extend(f"unknown property: {key}" for key in instance if key not in properties)
    for key, rule in properties.items():
        if key not in instance:
            continue
        value = instance[key]
        expected = rule.get("type")
        if expected == "string" and not isinstance(value, str):
            errors.append(f"{key} must be a string")
        if expected == "object" and not isinstance(value, dict):
            errors.append(f"{key} must be an object")
        if expected == "array" and not isinstance(value, list):
            errors.append(f"{key} must be an array")
        if "const" in rule and value != rule["const"]:
            errors.append(f"{key} must equal {rule['const']}")
        if "enum" in rule and value not in rule["enum"]:
            errors.append(f"{key} has invalid enum value")
        if isinstance(value, str) and "minLength" in rule and len(value) < rule["minLength"]:
            errors.append(f"{key} must not be empty")
    return errors


class GovernedCapabilityOperationContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))

    def assert_valid(self, instance):
        self.assertEqual(validate(self.schema, instance), [])

    def assert_invalid(self, instance):
        self.assertTrue(validate(self.schema, instance))

    def base_operation(self):
        return {
            "operation_id": "op-001",
            "schema_version": "1.0.0",
            "status": "REQUESTED",
            "principal_ref": "principal-001",
            "identity_context_ref": "identity-001",
            "capability_id": "health.timeline.read",
            "capability_version": "1.0.0",
            "action": "read",
            "resource_type": "health_observation",
            "purpose": "personal_health_management",
            "requested_at": "2026-09-11T08:00:00Z",
        }

    def test_schema_is_strict_and_versioned(self):
        self.assertEqual(self.schema["$schema"], "https://json-schema.org/draft/2020-12/schema")
        self.assertFalse(self.schema["additionalProperties"])
        self.assertEqual(self.schema["properties"]["schema_version"]["const"], "1.0.0")

    def test_minimal_governed_operation_is_valid(self):
        self.assert_valid(self.base_operation())

    def test_operation_requires_identity_and_capability_authority(self):
        operation = self.base_operation()
        del operation["identity_context_ref"]
        self.assert_invalid(operation)
        operation = self.base_operation()
        del operation["capability_id"]
        self.assert_invalid(operation)

    def test_policy_decision_is_bounded(self):
        operation = self.base_operation()
        operation["policy"] = {"decision": "ALLOW", "policy_version": "0.3.0", "decision_ref": "policy-001"}
        self.assert_valid(operation)
        operation["policy"]["decision"] = "GRANT_MYSELF_ADMIN"
        self.assert_invalid(operation)

    def test_lifecycle_is_bounded(self):
        operation = self.base_operation()
        for status in self.schema["properties"]["status"]["enum"]:
            operation["status"] = status
            self.assert_valid(operation)
        operation["status"] = "SELF_AUTHORIZED"
        self.assert_invalid(operation)

    def test_authorized_or_terminal_execution_states_require_policy_record(self):
        policy_required_states = {
            "DENIED", "REQUIRES_CONSENT", "REQUIRES_HUMAN_REVIEW",
            "AUTHORIZED", "EXECUTING", "SUCCEEDED", "FAILED", "DEGRADED",
        }
        for status in policy_required_states:
            operation = self.base_operation()
            operation["status"] = status
            self.assertNotIn("policy", operation)
            self.assertNotIn("policy", operation)
            operation["policy"] = {
                "decision": "ALLOW" if status not in {"DENIED", "REQUIRES_CONSENT", "REQUIRES_HUMAN_REVIEW"} else "DENY",
                "policy_version": "0.4.0",
                "decision_ref": "policy-001",
            }
            self.assert_valid(operation)

    def test_policy_decision_is_required_before_execution_semantically(self):
        operation = self.base_operation()
        operation["status"] = "EXECUTING"
        self.assertFalse("policy" in operation)
        operation["policy"] = {"decision": "ALLOW", "policy_version": "0.4.0", "decision_ref": "policy-001"}
        self.assert_valid(operation)

    def test_consent_and_review_gate_states_are_explicit(self):
        consent = self.base_operation()
        consent["status"] = "REQUIRES_CONSENT"
        consent["consent"] = {"required": True, "status": "pending"}
        self.assert_valid(consent)

        review = self.base_operation()
        review["status"] = "REQUIRES_HUMAN_REVIEW"
        review["human_review"] = {"required": True, "status": "pending"}
        self.assert_valid(review)

    def test_sensitive_payload_is_not_required_by_envelope(self):
        operation = self.base_operation()
        operation["result"] = {"status": "success", "result_ref": "vault-ref-001", "digest": "sha256:example"}
        self.assert_valid(operation)
        self.assertNotIn("health_record", operation)
        self.assertNotIn("raw_payload", operation)

    def test_unknown_root_properties_are_rejected(self):
        operation = self.base_operation()
        operation["unexpected_authority"] = True
        self.assert_invalid(operation)

    def test_ai_agent_reference_can_be_a_runtime_but_not_a_new_authority_field(self):
        operation = self.base_operation()
        operation["execution"] = {"runtime_ref": "ai-runtime-001"}
        self.assert_valid(operation)
        operation["agent_granted_authorization"] = True
        self.assert_invalid(operation)


if __name__ == "__main__":
    unittest.main()
