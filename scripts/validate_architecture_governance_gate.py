#!/usr/bin/env python3
"""Validate core SOMA architecture/governance invariants.

This is intentionally conservative: it fails on structural violations that can
be proven mechanically and reports warnings for items that require review.
"""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REGISTRY = ROOT / "services/shared/capability_registry.json"
MAPPING = ROOT / "schemas/evidence-canonical-mapping-v1.json"
FIXTURES = ROOT / "scripts/fixtures/evidence_canonical_mapping_pilot.json"

CANONICAL_REGISTRY_SCHEMA_VERSION = "1.0.0"
VALID_PRINCIPAL_TYPES = {"person", "agent", "service", "application", "device"}

REQUIRED_CAPABILITY_FIELDS = {
    "id", "version", "status", "maturity", "principal_types", "policy", "adapters", "surfaces"
}
FORBIDDEN_KINDS = {"traditional_knowledge", "analytical_assay", "search_result"}


def fail(message: str) -> None:
    raise SystemExit(f"FAIL: {message}")


def main() -> int:
    warnings: list[str] = []

    if not REGISTRY.exists():
        fail(f"missing capability registry: {REGISTRY}")
    registry = json.loads(REGISTRY.read_text(encoding="utf-8"))
    if registry.get("schema_version") != CANONICAL_REGISTRY_SCHEMA_VERSION:
        fail("capability registry must use canonical schema version 1.0.0")
    capabilities = registry.get("capabilities")
    if not isinstance(capabilities, list) or not capabilities:
        fail("capability registry must contain a non-empty capabilities list")

    ids: set[str] = set()
    for capability in capabilities:
        if not REQUIRED_CAPABILITY_FIELDS <= capability.keys():
            fail(f"capability is missing required fields: {capability}")
        cid = capability["id"]
        if cid in ids:
            fail(f"duplicate capability id: {cid}")
        ids.add(cid)
        if not isinstance(capability["principal_types"], list) or not capability["principal_types"] or not set(capability["principal_types"]) <= VALID_PRINCIPAL_TYPES:
            fail(f"capability has invalid principal_types: {cid}")
        requirements = capability.get("identity_requirements")
        if not isinstance(requirements, dict):
            fail(f"capability has invalid identity_requirements: {cid}")
        if requirements.get("applies_to_principal_types") != capability["principal_types"]:
            fail(f"capability identity requirements must cover exactly its principal types: {cid}")
        if not capability["principal_types"]:
            fail(f"capability has empty principal_types: {cid}")
        if not isinstance(capability["policy"], list) or not capability["policy"]:
            fail(f"capability has no policy controls: {cid}")
        if not isinstance(capability["adapters"], list) or not isinstance(capability["surfaces"], list):
            fail(f"adapters/surfaces must be arrays: {cid}")

    if not MAPPING.exists():
        fail(f"missing canonical mapping schema: {MAPPING}")
    mapping = json.loads(MAPPING.read_text(encoding="utf-8"))
    vocabulary = mapping.get("vocabulary_mappings", [])
    for item in vocabulary:
        if item.get("inflation_allowed") is True:
            fail("evidence vocabulary mapping permits strength inflation")

    if not FIXTURES.exists():
        fail(f"missing evidence mapping fixtures: {FIXTURES}")
    fixture_document = json.loads(FIXTURES.read_text(encoding="utf-8"))
    fixtures = fixture_document.get("records")
    if not isinstance(fixtures, list):
        fail("evidence mapping fixtures must contain a records list")

    for fixture in fixtures:
        name = str(fixture.get("id", "unknown"))
        kind = str(fixture.get("kind", "")).lower()
        expected_level = fixture.get("expected_level")
        legacy_state = fixture.get("legacy_evidence_state")
        if expected_level == "E4_WELL_SUPPORTED" and legacy_state == "STRONG":
            fail(f"legacy STRONG must not silently promote to E4: {name}")
        if kind in FORBIDDEN_KINDS and expected_level not in {None, "E0_UNKNOWN"}:
            fail(f"{kind} fixture cannot be promoted without explicit evidence assessment: {name}")
        if kind == "safety" and fixture.get("efficacy_level") not in {None, "E0_UNKNOWN"}:
            fail(f"safety fixture must keep efficacy independent: {name}")
        if kind == "search_result" and fixture.get("directness") != "NO_RELEVANT_EVIDENCE_FOUND":
            warnings.append(f"search-result fixture should use no-evidence directness: {name}")

    if "intelligence.ai" in ids:
        ai = next(c for c in capabilities if c["id"] == "intelligence.ai")
        if ai.get("status") != "optional" or "user-choice" not in ai.get("policy", []):
            fail("AI capability must remain optional and governed by user-choice")

    if "protocol.nostr" in ids:
        nostr = next(c for c in capabilities if c["id"] == "protocol.nostr")
        if nostr.get("status") not in {"adapter-boundary", "planned"}:
            warnings.append("Nostr status should remain non-operational until real signing is verified")

    print("PASS: architecture and evidence governance invariants")
    for warning in warnings:
        print(f"WARN: {warning}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
