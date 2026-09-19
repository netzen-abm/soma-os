#!/usr/bin/env python3
"""Repository authorization invariant gate.

Fails closed if protected authorization request construction regresses into
actor/subject collapse or if protected Python request models lose subject_ref.
This is a static governance gate; runtime authorization remains canonical.
"""
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
errors: list[str] = []

rust_root = ROOT / "services" / "backend-rust" / "src"
for path in rust_root.rglob("*.rs"):
    text = path.read_text(encoding="utf-8")
    for match in re.finditer(r"AuthorizationRequest\s*\{(?P<body>.*?)\}", text, re.S):
        body = match.group("body")
        if "principal_ref:" in body and "subject_ref:" not in body:
            errors.append(f"{path}: AuthorizationRequest literal lacks subject_ref")

vault = rust_root / "canonical_vault_authorizer.rs"
if vault.exists():
    text = vault.read_text(encoding="utf-8")
    if "principal_ref: context.subject_ref" in text:
        errors.append(f"{vault}: principal_ref derived from subject_ref")

policy = ROOT / "services" / "shared" / "policy_kernel.py"
protected = ROOT / "services" / "shared" / "protected_data_access.py"
boundary = ROOT / "services" / "shared" / "authorization_policy_decision_boundary.py"

for path in (policy, protected, boundary):
    text = path.read_text(encoding="utf-8")
    if "subject_ref" not in text:
        errors.append(f"{path}: missing subject_ref")

protected_text = protected.read_text(encoding="utf-8")
if "class ProtectedDataRequest" not in protected_text:
    errors.append(f"{protected}: ProtectedDataRequest definition missing")
elif not re.search(r"class ProtectedDataRequest.*?subject_ref\s*:", protected_text, re.S):
    errors.append(f"{protected}: ProtectedDataRequest is not subject-bound")

if errors:
    print("\n".join(errors))
    sys.exit(1)

print("Authorization invariant gate: PASS")
