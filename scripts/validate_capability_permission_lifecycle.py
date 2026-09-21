#!/usr/bin/env python3
"""Validate the canonical SOMA capability-permission lifecycle contract."""
from __future__ import annotations
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PATH = ROOT / "schemas/capability-permission-lifecycle-v1.json"

data = json.loads(PATH.read_text(encoding="utf-8"))
required = set(data.get("required", []))
required_fields = {"permission_id","principal_ref","subject_ref","capability_id","capability_version","purpose","scope","status","granted_at","expires_at","auto_revoke"}
missing = required_fields - required
if missing:
    raise SystemExit(f"FAIL: permission lifecycle missing required fields: {sorted(missing)}")
if data["properties"].get("auto_revoke", {}).get("const") is not True:
    raise SystemExit("FAIL: permission lifecycle must require automatic revocation")
if data["properties"].get("regrant_requires_fresh_consent", {}).get("const") is not True:
    raise SystemExit("FAIL: re-grant must require fresh consent")
statuses = set(data["properties"].get("status", {}).get("enum", []))
if not {"GRANTED","ACTIVE","REVOKED","EXPIRED"} <= statuses:
    raise SystemExit("FAIL: incomplete permission lifecycle states")
print("PASS: SOMA capability permission lifecycle")
