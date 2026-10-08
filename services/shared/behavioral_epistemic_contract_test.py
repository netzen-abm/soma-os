import json
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SCHEMAS = {
    "health_state": ROOT / "schemas/health-state-v1.json",
    "behavioral": ROOT / "schemas/behavioral-observation-v1.json",
    "passport": ROOT / "schemas/epistemic-evidence-passport-v1.json",
    "evidence": ROOT / "schemas/health-evidence-graph-v1.json",
}


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
        if isinstance(expected, list):
            if value is None and "null" in expected:
                continue
            expected = next((item for item in expected if item != "null"), None)
        if expected == "string" and not isinstance(value, str):
            errors.append(f"{key} must be a string")
        if expected == "integer" and not isinstance(value, int):
            errors.append(f"{key} must be an integer")
        if expected == "array" and not isinstance(value, list):
            errors.append(f"{key} must be an array")
        if "const" in rule and value != rule["const"]:
            errors.append(f"{key} must equal {rule['const']}")
        if "enum" in rule and value not in rule["enum"]:
            errors.append(f"{key} has invalid enum value")
        if isinstance(value, str) and "minLength" in rule and len(value) < rule["minLength"]:
            errors.append(f"{key} must not be empty")
        if isinstance(value, int) and "minimum" in rule and value < rule["minimum"]:
            errors.append(f"{key} below minimum")
        if isinstance(value, int) and "maximum" in rule and value > rule["maximum"]:
            errors.append(f"{key} above maximum")
    return errors


class BehavioralEpistemicContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.schemas = {
            name: json.loads(path.read_text(encoding="utf-8"))
            for name, path in SCHEMAS.items()
        }

    def assert_valid(self, name, instance):
        self.assertEqual(validate(self.schemas[name], instance), [])

    def assert_invalid(self, name, instance):
        self.assertTrue(validate(self.schemas[name], instance))

    def test_contracts_are_strict_and_versioned(self):
        for schema in self.schemas.values():
            self.assertEqual(schema["$schema"], "https://json-schema.org/draft/2020-12/schema")
            self.assertFalse(schema["additionalProperties"])
            self.assertTrue(schema["required"])

    def test_behavioral_observation_is_provenance_bearing(self):
        observation = {
            "id": "behavior-001",
            "schema_version": "1.0.0",
            "subject_ref": "person-001",
            "observed_at": "2026-09-11T10:00:00Z",
            "context": "work meeting",
            "reported_thought": "I may make a mistake",
            "reported_affect": "anxious",
            "reported_intensity": 60,
            "response_or_behavior": "paused and wrote notes",
            "source": "user_report",
            "provenance_ref": "prov-001",
            "uncertainty": "reported",
            "classification": "sensitive_personal_health",
        }
        self.assert_valid("behavioral", observation)

    def test_behavioral_contract_cannot_be_used_as_a_diagnosis_container(self):
        observation = {
            "id": "behavior-002",
            "schema_version": "1.0.0",
            "subject_ref": "person-001",
            "observed_at": "2026-09-11T10:00:00Z",
            "source": "user_report",
            "provenance_ref": "prov-002",
            "uncertainty": "reported",
            "diagnosis": "example",
        }
        self.assert_invalid("behavioral", observation)

    def test_epistemic_passport_requires_claim_provenance_and_safety(self):
        passport = {
            "id": "passport-001",
            "schema_version": "1.0.0",
            "claim_ref": "claim-001",
            "knowledge_system": "behavioral_research",
            "claim_type": "behavioral",
            "evidence_design": "observational_cohort",
            "epistemic_status": "preliminary",
            "uncertainty": "conflicting",
            "provenance_ref": "prov-003",
            "safety_status": "insufficient_safety_evidence",
        }
        self.assert_valid("passport", passport)

    def test_epistemic_passport_is_not_a_health_state_observation(self):
        observation = {
            "id": "hs-001",
            "subject_ref": "person-001",
            "entity_type": "observation",
            "schema_version": "1.1.0",
            "status": "active",
            "recorded_time": "2026-09-11T10:00:00Z",
            "concept": "sleep_duration",
            "value": 7,
            "unit": "hours",
            "provenance": {"origin": "user", "method": "self_report"},
            "classification": "personal_health",
            "uncertainty": "reported",
        }
        self.assert_valid("health_state", observation)
        self.assert_invalid("passport", observation)
        self.assert_invalid("evidence", observation)

    def test_behavioral_observation_does_not_imply_causation(self):
        observation = {
            "id": "behavior-003",
            "schema_version": "1.0.0",
            "subject_ref": "person-001",
            "observed_at": "2026-09-11T10:00:00Z",
            "source": "user_report",
            "provenance_ref": "prov-004",
            "uncertainty": "reported",
            "response_or_behavior": "felt better after exercise",
        }
        self.assert_valid("behavioral", observation)
        self.assertNotIn("CAUSES", self.schemas["health_state"]["properties"].get("relationships", {}).get("items", {}).get("properties", {}).get("type", {}).get("enum", []))

    def test_unknown_behavioral_fields_are_rejected(self):
        instance = {
            "id": "behavior-004",
            "schema_version": "1.0.0",
            "subject_ref": "person-001",
            "observed_at": "2026-09-11T10:00:00Z",
            "source": "user_report",
            "provenance_ref": "prov-005",
            "uncertainty": "reported",
            "model_confidence": 0.99,
        }
        self.assert_invalid("behavioral", instance)


if __name__ == "__main__":
    unittest.main()
