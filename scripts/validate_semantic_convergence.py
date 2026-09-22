#!/usr/bin/env python3
"""Mechanical gate for SOMA semantic ownership and authorization convergence."""
from __future__ import annotations
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "schemas/semantic-convergence-manifest-v1.json"

def fail(message: str) -> None:
    raise SystemExit(f"FAIL: {message}")

def read(path: Path):
    if not path.exists():
        fail(f"required canonical artifact missing: {path.relative_to(ROOT)}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"invalid JSON: {path.relative_to(ROOT)}: {exc}")

def main() -> int:
    manifest = read(MANIFEST)
    owners = manifest["canonical_owners"]

    for name, relative in owners.items():
        path = ROOT / relative
        if not path.exists():
            fail(f"canonical owner missing for {name}: {relative}")

    health = read(ROOT / owners["health_state"])
    longitudinal = read(ROOT / owners["longitudinal_observation"])

    expected = set(manifest["relationship_types"])
    forbidden = set(manifest["forbidden_foundational_relationship_types"])

    def collect(schema):
        values = set()
        props = schema.get("properties", {})
        rel = props.get("relationships", {}).get("items", {}).get("properties", {}).get("type", {})
        values.update(rel.get("enum", []))
        return values

    health_relationships = collect(health)
    longitudinal_relationships = collect(longitudinal)

    if forbidden & (health_relationships | longitudinal_relationships):
        fail("forbidden foundational causal relationship vocabulary detected in canonical schemas")

    unknown = (health_relationships | longitudinal_relationships) - expected
    if unknown:
        fail(f"relationship vocabulary drift detected: {sorted(unknown)}")

    if not {"HAS_OBSERVATION", "HAS_INTERVENTION", "HAS_RESPONSE", "HAS_OUTCOME"} <= health_relationships:
        fail("Health State schema does not expose the required longitudinal relationship semantics")

    authorization = read(ROOT / owners["authorization_decision"])
    governed = read(ROOT / owners["governed_operation"])

    auth_required = set(authorization.get("required", []))
    governed_required = set(governed.get("required", []))
    core = set(manifest["authorization_fields"])

    if not {"principal_id", "subject_ref"} <= auth_required:
        fail("canonical authorization decision is missing principal/subject binding")
    if not {"principal_ref", "subject_ref", "capability_id", "capability_version"} <= governed_required:
        fail("governed operation is missing canonical authority identity fields")

    # The protected execution boundary must retain the same authority dimensions.
    rust_root = ROOT / "services/backend-rust/src"
    required_fragments = [
        "principal_ref: String",
        "subject_ref: String",
        "capability_id: String",
        "capability_version: String",
        "resource_type: String",
        "resource_id: String",
        "action: String",
        "tenant_id: String",
        "data_domain: String",
    ]
    rust_text = "\n".join(p.read_text(encoding="utf-8") for p in rust_root.glob("*.rs") if "archive" not in p.parts)
    missing = [fragment for fragment in required_fragments if fragment not in rust_text]
    if missing:
        fail(f"canonical Rust authorization fields missing from runtime: {missing}")

    policy = (ROOT / owners["policy_evaluator"]).read_text(encoding="utf-8")
    protected = (ROOT / owners["protected_data"]).read_text(encoding="utf-8")
    if "PolicyRequest" not in policy or "subject_ref" not in policy:
        fail("Policy Kernel does not expose the canonical subject binding")
    if "class ProtectedDataRequest" not in protected or "subject_ref" not in protected:
        fail("ProtectedDataAccess does not expose the canonical subject binding")

    if len(core) != len(manifest["authorization_fields"]):
        fail("authorization field manifest contains duplicate fields")

    print("PASS: SOMA semantic ownership and authorization convergence")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
