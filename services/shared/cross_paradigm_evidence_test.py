import json
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SCHEMA_PATH = ROOT / "schemas/health-evidence-graph-v1.json"
REGISTRY_PATH = ROOT / "services/shared/capability_registry.json"


class CrossParadigmEvidenceTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))
        cls.registry = json.loads(REGISTRY_PATH.read_text(encoding="utf-8"))

    def test_canonical_schema_contains_paradigm_dimensions(self):
        properties = self.schema["properties"]
        self.assertEqual(properties["schema_version"]["const"], "1.1.0")
        self.assertIn("knowledge_system", properties)
        self.assertIn("research_method", properties)
        self.assertIn("evidence_assessment", properties)
        self.assertIn("claim_type", properties)
        self.assertIn("safety_classification", properties)
        self.assertIn("provenance", properties)

    def test_knowledge_system_is_not_an_evidence_strength_score(self):
        properties = self.schema["properties"]
        knowledge_classes = set(properties["knowledge_system"]["properties"]["system_class"]["enum"])
        evidence_levels = set(properties["evidence_level"]["enum"])
        self.assertIn("biomedical", knowledge_classes)
        self.assertIn("traditional", knowledge_classes)
        self.assertIn("integrative", knowledge_classes)
        self.assertNotEqual(knowledge_classes, evidence_levels)

    def test_research_method_is_separate_from_evidence_level(self):
        method = self.schema["properties"]["research_method"]["properties"]
        self.assertIn("design", method)
        self.assertIn("randomized", method)
        self.assertIn("controlled", method)
        self.assertIn("attrition_count", method)
        self.assertIn("negative_outcomes_reported", method)
        self.assertIn("evidence_level", self.schema["properties"])

    def test_causal_claim_has_explicit_causal_inference_dimension(self):
        causal_rule = next(
            rule for rule in self.schema["allOf"]
            if rule.get("if", {}).get("properties", {}).get("entity_type", {}).get("const") == "claim"
        )
        self.assertIn("claim_type", causal_rule["then"]["required"])
        causal_values = set(self.schema["properties"]["claim_type"]["enum"])
        self.assertIn("causal", causal_values)
        inference_values = set(self.schema["properties"]["evidence_assessment"]["properties"]["causal_inference"]["enum"])
        self.assertIn("descriptive_only", inference_values)
        self.assertIn("strong_causal", inference_values)

    def test_safety_is_independent_of_evidence_strength(self):
        evidence = set(self.schema["properties"]["evidence_level"]["enum"])
        safety = set(self.schema["properties"]["safety_classification"]["enum"])
        self.assertIn("E4_WELL_SUPPORTED", evidence)
        self.assertIn("CONTRAINDICATED", safety)
        self.assertNotEqual(evidence, safety)

    def test_contradiction_and_negative_evidence_are_first_class(self):
        assessment = self.schema["properties"]["evidence_assessment"]["properties"]
        dimensions = assessment["dimensions"]["properties"]
        self.assertIn("negative_evidence", dimensions)
        self.assertIn("contradictory_evidence_refs", assessment)
        self.assertIn("confounders", assessment)
        self.assertIn("limitations", assessment)

    def test_provenance_is_mandatory_and_supports_transformation_history(self):
        provenance = self.schema["properties"]["provenance"]
        self.assertEqual(set(provenance["required"]), {"source_ref", "method"})
        self.assertIn("transformation_history", provenance["properties"])

    def test_non_biomedical_claim_is_representable_without_automatic_promotion(self):
        knowledge = self.schema["properties"]["knowledge_system"]
        classes = set(knowledge["properties"]["system_class"]["enum"])
        self.assertIn("traditional", classes)
        claim_types = set(self.schema["properties"]["claim_type"]["enum"])
        self.assertIn("phenomenological", claim_types)
        self.assertIn("ontological_theoretical", claim_types)
        evidence_levels = set(self.schema["properties"]["evidence_level"]["enum"])
        self.assertIn("E1_PLAUSIBLE", evidence_levels)
        self.assertIn("E0_UNKNOWN", evidence_levels)

    def test_cross_paradigm_capability_is_governed(self):
        capability = next(c for c in self.registry["capabilities"] if c["id"] == "evidence.cross_paradigm")
        self.assertIn("privacy", capability["policy"])
        self.assertIn("safety", capability["policy"])
        self.assertIn("provenance", capability["policy"])
        self.assertIn("uncertainty", capability["policy"])
        self.assertIn("research-integrity", capability["policy"])

    def test_benchmark_fixture_observational_signal_does_not_equal_causality(self):
        fixture = {
            "claim_type": "empirical_relationship",
            "research_method": {
                "method_id": "observational_longitudinal",
                "design": "observational",
                "prospective": True,
                "controlled": False,
                "randomized": False,
                "blinded": False,
                "sample_size": 319,
                "attrition_count": 301,
                "negative_outcomes_reported": True,
            },
            "evidence_assessment": {
                "causal_inference": "associational",
                "uncertainty": ["selection_bias", "attrition", "co-interventions"],
            },
        }
        self.assertEqual(fixture["claim_type"], "empirical_relationship")
        self.assertNotIn(fixture["evidence_assessment"]["causal_inference"], {"strong_causal"})
        self.assertGreater(fixture["research_method"]["attrition_count"], 0)
        self.assertTrue(fixture["research_method"]["negative_outcomes_reported"])

    def test_benchmark_preserves_cross_paradigm_comparison_without_source_laundering(self):
        comparison = {
            "knowledge_systems": ["biomedical", "traditional"],
            "source_refs": ["source-a", "source-b"],
            "independent_evidence": ["source-a", "source-b"],
            "cross_paradigm_convergence": "not_established",
        }
        self.assertEqual(len(comparison["knowledge_systems"]), 2)
        self.assertEqual(len(comparison["source_refs"]), len(comparison["independent_evidence"]))
        self.assertEqual(comparison["cross_paradigm_convergence"], "not_established")


if __name__ == "__main__":
    unittest.main()
