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


def validate_operation_semantics(instance):
    """Validate cross-field authority/lifecycle invariants not expressible by the lightweight schema checker."""
    errors = []
    status = instance.get("status")
    policy = instance.get("policy")
    policy_decision = policy.get("decision") if isinstance(policy, dict) else None

    if status in {"DENIED", "REQUIRES_CONSENT", "REQUIRES_HUMAN_REVIEW", "AUTHORIZED", "EXECUTING", "SUCCEEDED", "FAILED", "DEGRADED"}:
        if not isinstance(policy, dict):
            errors.append("policy decision is required after authorization begins")
        elif not all(isinstance(policy.get(key), str) and policy[key].strip() for key in ("decision", "policy_version", "decision_ref")):
            errors.append("policy record must contain decision, policy_version, and decision_ref")

    expected_decisions = {
        "DENIED": {"DENY"},
        "REQUIRES_CONSENT": {"REQUIRE_CONSENT"},
        "REQUIRES_HUMAN_REVIEW": {"REQUIRE_HUMAN_REVIEW"},
        "AUTHORIZED": {"ALLOW", "DEGRADE"},
        "EXECUTING": {"ALLOW", "DEGRADE"},
        "SUCCEEDED": {"ALLOW", "DEGRADE"},
        "FAILED": {"ALLOW", "DEGRADE"},
        "DEGRADED": {"DEGRADE"},
    }
    if status in expected_decisions and policy_decision not in expected_decisions[status]:
        errors.append(f"status {status} is incompatible with policy decision {policy_decision}")

    if status == "REQUIRES_CONSENT":
        consent = instance.get("consent")
        if not isinstance(consent, dict) or consent.get("required") is not True or consent.get("status") != "pending":
            errors.append("consent gate must be explicitly pending when consent is required")

    if status == "REQUIRES_HUMAN_REVIEW":
        review = instance.get("human_review")
        if not isinstance(review, dict) or review.get("required") is not True or review.get("status") != "pending":
            errors.append("human-review gate must be explicitly pending when review is required")

    if status in {"AUTHORIZED", "EXECUTING", "SUCCEEDED", "FAILED", "DEGRADED"}:
        consent = instance.get("consent")
        if isinstance(consent, dict) and consent.get("required") is True and consent.get("status") != "granted":
            errors.append("execution cannot proceed through an ungranted consent gate")
        review = instance.get("human_review")
        if isinstance(review, dict) and review.get("required") is True and review.get("status") != "approved":
            errors.append("execution cannot proceed through an unapproved human-review gate")

    return errors


class GovernedCapabilityOperationContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))

    def assert_valid(self, instance):
        self.assertEqual(validate(self.schema, instance), [])
        self.assertEqual(validate_operation_semantics(instance), [])

    def assert_schema_invalid(self, instance):
        self.assertTrue(validate(self.schema, instance))

    def assert_semantically_invalid(self, instance):
        self.assertTrue(validate_operation_semantics(instance))

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

    def policy(self, decision="ALLOW"):
        return {"decision": decision, "policy_version": "0.4.0", "decision_ref": "policy-001"}

    def test_schema_is_strict_and_versioned(self):
        self.assertEqual(self.schema["$schema"], "https://json-schema.org/draft/2020-12/schema")
        self.assertFalse(self.schema["additionalProperties"])
        self.assertEqual(self.schema["properties"]["schema_version"]["const"], "1.0.0")

    def test_minimal_requested_operation_is_valid(self):
        self.assert_valid(self.base_operation())

    def test_operation_requires_identity_and_capability_authority(self):
        operation = self.base_operation()
        del operation["identity_context_ref"]
        self.assert_schema_invalid(operation)
        operation = self.base_operation()
        del operation["capability_id"]
        self.assert_schema_invalid(operation)

    def test_policy_decision_is_bounded(self):
        operation = self.base_operation()
        operation["policy"] = self.policy()
        self.assert_valid(operation)
        operation["policy"]["decision"] = "GRANT_MYSELF_ADMIN"
        self.assert_schema_invalid(operation)

    def test_lifecycle_is_bounded(self):
        operation = self.base_operation()
        for status in self.schema["properties"]["status"]["enum"]:
            operation["status"] = status
            if status in {"DENIED", "REQUIRES_CONSENT", "REQUIRES_HUMAN_REVIEW", "AUTHORIZED", "EXECUTING", "SUCCEEDED", "FAILED", "DEGRADED"}:
                operation["policy"] = self.policy("DENY" if status == "DENIED" else "REQUIRE_CONSENT" if status == "REQUIRES_CONSENT" else "REQUIRE_HUMAN_REVIEW" if status == "REQUIRES_HUMAN_REVIEW" else "DEGRADE" if status == "DEGRADED" else "ALLOW")
            if status == "REQUIRES_CONSENT":
                operation["consent"] = {"required": True, "status": "pending"}
            if status == "REQUIRES_HUMAN_REVIEW":
                operation["human_review"] = {"required": True, "status": "pending"}
            self.assert_valid(operation)
            operation = self.base_operation()
        operation["status"] = "SELF_AUTHORIZED"
        self.assert_schema_invalid(operation)

    def test_authorized_execution_cannot_exist_without_policy(self):
        operation = self.base_operation()
        operation["status"] = "EXECUTING"
        self.assert_semantically_invalid(operation)
        operation["policy"] = self.policy()
        self.assert_valid(operation)

    def test_policy_must_match_lifecycle_authority(self):
        operation = self.base_operation()
        operation["status"] = "DENIED"
        operation["policy"] = self.policy("ALLOW")
        self.assert_semantically_invalid(operation)
        operation["policy"] = self.policy("DENY")
        self.assert_valid(operation)

    def test_consent_and_review_gates_cannot_be_bypassed(self):
        consent = self.base_operation()
        consent["status"] = "AUTHORIZED"
        consent["policy"] = self.policy()
        consent["consent"] = {"required": True, "status": "pending"}
        self.assert_semantically_invalid(consent)
        consent["consent"]["status"] = "granted"
        self.assert_valid(consent)

        review = self.base_operation()
        review["status"] = "AUTHORIZED"
        review["policy"] = self.policy()
        review["human_review"] = {"required": True, "status": "pending"}
        self.assert_semantically_invalid(review)
        review["human_review"]["status"] = "approved"
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
        self.assert_schema_invalid(operation)

    def test_ai_agent_reference_can_be_a_runtime_but_not_a_new_authority_field(self):
        operation = self.base_operation()
        operation["execution"] = {"runtime_ref": "ai-runtime-001"}
        self.assert_valid(operation)
        operation["agent_granted_authorization"] = True
        self.assert_schema_invalid(operation)


if __name__ == "__main__":
    unittest.main()
