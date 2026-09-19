import json
import unittest
from datetime import datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCHEMA_PATH = ROOT / "schemas/authorization-policy-decision-boundary-v1.json"


def _type_ok(value, expected):
    return {"string": isinstance(value, str), "object": isinstance(value, dict), "array": isinstance(value, list), "boolean": isinstance(value, bool)}.get(expected, True)


def validate(schema, instance, path="root"):
    errors = []
    if "type" in schema and not _type_ok(instance, schema["type"]):
        return [f"{path} must be a {schema['type']}"]
    if "enum" in schema and instance not in schema["enum"]:
        errors.append(f"{path} has invalid enum value")
    if isinstance(instance, str):
        if "minLength" in schema and len(instance) < schema["minLength"]:
            errors.append(f"{path} must not be empty")
        if schema.get("format") == "date-time":
            try:
                parsed = datetime.fromisoformat(instance.replace("Z", "+00:00"))
                if parsed.tzinfo is None:
                    raise ValueError("timezone required")
            except ValueError:
                errors.append(f"{path} must be a timezone-aware date-time")
    if isinstance(instance, dict):
        for key in schema.get("required", []):
            if key not in instance:
                errors.append(f"{path}: missing required property: {key}")
        properties = schema.get("properties", {})
        if schema.get("additionalProperties") is False:
            errors.extend(f"{path}: unknown property: {key}" for key in instance if key not in properties)
        for key, value in instance.items():
            if key in properties:
                errors.extend(validate(properties[key], value, f"{path}.{key}"))
    return errors


class AuthorizationPolicyDecisionBoundaryContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))

    def base_decision(self, decision="ALLOW"):
        return {
            "request_id": "req-001",
            "principal_id": "principal-001",
            "principal_type": "person",
            "subject_ref": "subject-001",
            "capability_id": "health.timeline.read",
            "resource_type": "health_observation",
            "resource_id": "observation-001",
            "action": "read",
            "tenant_scope_binding": "tenant-a",
            "data_domain_scope_binding": "health",
            "decision": decision,
            "policy_version": "0.4.0",
            "reason_code": "policy_allowed",
            "enforcement_version": "1.1.0",
            "decision_time": "2026-09-11T08:00:00Z",
        }

    def test_schema_is_strict_and_versioned(self):
        self.assertFalse(self.schema["additionalProperties"])
        self.assertEqual(self.schema["$id"], "https://soma-os.org/schemas/authorization-policy-decision-boundary-v1.json")

    def test_valid_allow_decision(self):
        self.assertEqual(validate(self.schema, self.base_decision()), [])

    def test_all_canonical_decisions_are_supported(self):
        for decision in ("ALLOW", "DENY", "REQUIRE_CONSENT", "REQUIRE_HUMAN_REVIEW", "DEGRADE"):
            self.assertEqual(validate(self.schema, self.base_decision(decision)), [])

    def test_required_fields_cannot_be_omitted(self):
        for field in self.schema["required"]:
            decision = self.base_decision()
            del decision[field]
            self.assertTrue(validate(self.schema, decision), field)

    def test_unknown_properties_are_rejected(self):
        decision = self.base_decision()
        decision["agent_granted_authorization"] = True
        self.assertTrue(validate(self.schema, decision))

    def test_invalid_decision_is_rejected(self):
        decision = self.base_decision("GRANT_MYSELF_ADMIN")
        self.assertTrue(validate(self.schema, decision))

    def test_empty_security_dimensions_are_rejected(self):
        for field in ("principal_id", "capability_id", "resource_type", "resource_id", "action", "tenant_scope_binding", "data_domain_scope_binding", "policy_version", "reason_code", "enforcement_version"):
            decision = self.base_decision()
            decision[field] = ""
            self.assertTrue(validate(self.schema, decision), field)

    def test_decision_contains_references_not_sensitive_payloads(self):
        decision = self.base_decision()
        self.assertNotIn("health_record", decision)
        self.assertNotIn("raw_payload", decision)
        self.assertNotIn("credential", decision)
        self.assertNotIn("token", decision)

    def test_naive_timestamp_without_timezone_is_rejected(self):
        decision = self.base_decision()
        decision["decision_time"] = "2026-09-11T08:00:00"
        self.assertTrue(validate(self.schema, decision))


if __name__ == "__main__":
    unittest.main()
