import json
import unittest
from datetime import datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCHEMA_PATH = ROOT / "schemas/governed-capability-operation-v1.json"


def _type_ok(value, expected):
    return {"string": isinstance(value, str), "object": isinstance(value, dict), "array": isinstance(value, list), "boolean": isinstance(value, bool)}.get(expected, True)


def validate(schema, instance, path="root"):
    errors = []
    if "type" in schema and not _type_ok(instance, schema["type"]):
        return [f"{path} must be a {schema['type']}"]
    if "const" in schema and instance != schema["const"]:
        errors.append(f"{path} must equal {schema['const']}")
    if "enum" in schema and instance not in schema["enum"]:
        errors.append(f"{path} has invalid enum value")
    if isinstance(instance, str):
        if "minLength" in schema and len(instance) < schema["minLength"]:
            errors.append(f"{path} must not be empty")
        if schema.get("format") == "date-time":
            try:
                datetime.fromisoformat(instance.replace("Z", "+00:00"))
            except ValueError:
                errors.append(f"{path} must be a date-time")
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
    if isinstance(instance, list) and "items" in schema:
        for index, value in enumerate(instance):
            errors.extend(validate(schema["items"], value, f"{path}[{index}]"))
    for rule in schema.get("allOf", []):
        condition = rule.get("if")
        if condition and isinstance(instance, dict) and all(key in instance for key in condition.get("required", [])):
            errors.extend(validate(rule.get("then", {}), instance, path))
    return errors


def validate_operation_semantics(instance):
    errors = []
    status = instance.get("status")
    policy = instance.get("policy")
    decision = policy.get("decision") if isinstance(policy, dict) else None
    gated = {"DENIED", "REQUIRES_CONSENT", "REQUIRES_HUMAN_REVIEW", "AUTHORIZED", "EXECUTING", "SUCCEEDED", "FAILED", "DEGRADED"}
    if status in gated and not isinstance(policy, dict):
        errors.append("policy decision is required after authorization begins")
    expected = {"DENIED": {"DENY"}, "REQUIRES_CONSENT": {"REQUIRE_CONSENT"}, "REQUIRES_HUMAN_REVIEW": {"REQUIRE_HUMAN_REVIEW"}, "AUTHORIZED": {"ALLOW", "DEGRADE"}, "EXECUTING": {"ALLOW", "DEGRADE"}, "SUCCEEDED": {"ALLOW", "DEGRADE"}, "FAILED": {"ALLOW", "DEGRADE"}, "DEGRADED": {"DEGRADE"}}
    if status in expected and decision not in expected[status]:
        errors.append(f"status {status} is incompatible with policy decision {decision}")
    if status == "REQUIRES_CONSENT" and (not isinstance(instance.get("consent"), dict) or instance["consent"].get("required") is not True or instance["consent"].get("status") != "pending"):
        errors.append("consent gate must be explicitly pending")
    if status == "REQUIRES_HUMAN_REVIEW" and (not isinstance(instance.get("human_review"), dict) or instance["human_review"].get("required") is not True or instance["human_review"].get("status") != "pending"):
        errors.append("human-review gate must be explicitly pending")
    if status in {"AUTHORIZED", "EXECUTING", "SUCCEEDED", "FAILED", "DEGRADED"}:
        if isinstance(instance.get("consent"), dict) and instance["consent"].get("required") is True and instance["consent"].get("status") != "granted":
            errors.append("execution cannot proceed through an ungranted consent gate")
        if isinstance(instance.get("human_review"), dict) and instance["human_review"].get("required") is True and instance["human_review"].get("status") != "approved":
            errors.append("execution cannot proceed through an unapproved human-review gate")
    return errors


class GovernedCapabilityOperationContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))

    def base_operation(self):
        return {"operation_id": "op-001", "schema_version": "1.0.0", "status": "REQUESTED", "principal_ref": "principal-001", "identity_context_ref": "identity-001", "subject_ref": "subject-001", "capability_id": "health.timeline.read", "capability_version": "1.0.0", "action": "read", "resource_type": "health_observation", "purpose": "personal_health_management", "requested_at": "2026-09-11T08:00:00Z"}

    def policy(self, decision="ALLOW"):
        return {"decision": decision, "policy_version": "0.4.0", "decision_ref": "policy-001"}

    def assert_valid(self, instance):
        self.assertEqual(validate(self.schema, instance), [])
        self.assertEqual(validate_operation_semantics(instance), [])

    def test_schema_is_strict_and_versioned(self):
        self.assertFalse(self.schema["additionalProperties"])
        self.assertEqual(self.schema["properties"]["schema_version"]["const"], "1.0.0")

    def test_minimal_requested_operation_is_valid(self):
        self.assert_valid(self.base_operation())

    def test_required_identity_and_capability_fields(self):
        for field in ("identity_context_ref", "capability_id"):
            operation = self.base_operation(); del operation[field]
            self.assertTrue(validate(self.schema, operation))

    def test_nested_policy_is_actually_validated(self):
        operation = self.base_operation(); operation["policy"] = self.policy()
        self.assert_valid(operation)
        operation["policy"]["decision"] = "GRANT_MYSELF_ADMIN"
        self.assertTrue(validate(self.schema, operation))
        operation["policy"]["unexpected"] = True
        self.assertTrue(validate(self.schema, operation))
        operation["policy"] = {"decision": "ALLOW"}
        self.assertTrue(validate(self.schema, operation))

    def test_nested_gates_and_execution_are_strict(self):
        operation = self.base_operation(); operation["status"] = "AUTHORIZED"; operation["policy"] = self.policy()
        operation["consent"] = {"required": True, "status": "granted"}; operation["execution"] = {"runtime_ref": "ai-1"}
        self.assert_valid(operation)
        operation["consent"]["status"] = "bogus"
        self.assertTrue(validate(self.schema, operation))

    def test_lifecycle_policy_and_gate_semantics(self):
        denied = self.base_operation(); denied.update(status="DENIED", policy=self.policy("ALLOW"))
        self.assertTrue(validate_operation_semantics(denied))
        denied["policy"] = self.policy("DENY"); self.assert_valid(denied)
        consent = self.base_operation(); consent.update(status="REQUIRES_CONSENT", policy=self.policy("REQUIRE_CONSENT"), consent={"required": True, "status": "pending"}); self.assert_valid(consent)
        consent["consent"]["status"] = "denied"; self.assertTrue(validate_operation_semantics(consent))

    def test_unknown_root_properties_are_rejected(self):
        operation = self.base_operation(); operation["agent_granted_authorization"] = True
        self.assertTrue(validate(self.schema, operation))

    def test_sensitive_payload_is_not_required_by_envelope(self):
        operation = self.base_operation(); operation["result"] = {"status": "success", "result_ref": "vault-ref-001", "digest": "sha256:example"}
        self.assert_valid(operation)
        self.assertNotIn("health_record", operation)
        self.assertNotIn("raw_payload", operation)


if __name__ == "__main__":
    unittest.main()
