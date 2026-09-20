#!/usr/bin/env python3
"""Repository-level canonical authorization invariant gate."""
from __future__ import annotations
import ast, json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PYTHON_ROOTS = (ROOT / "services", ROOT / "scripts")
RUST_ROOT = ROOT / "services/backend-rust/src"

def fail(message: str) -> None:
    raise SystemExit(f"FAIL: {message}")

def iter_python_files():
    for root in PYTHON_ROOTS:
        if not root.exists():
            continue
        for path in root.rglob("*.py"):
            if "archive" in path.parts or "__pycache__" in path.parts:
                continue
            yield path

def has_subject_keyword(call: ast.Call) -> bool:
    return any(keyword.arg == "subject_ref" for keyword in call.keywords)

def audit_python() -> None:
    policy_calls = protected_calls = 0
    for path in iter_python_files():
        try:
            tree = ast.parse(path.read_text(encoding="utf-8"), filename=str(path))
        except SyntaxError as exc:
            fail(f"Python parse error: {path}: {exc}")
        for node in ast.walk(tree):
            if not isinstance(node, ast.Call):
                continue
            name = node.func.id if isinstance(node.func, ast.Name) else None
            if name == "PolicyRequest":
                policy_calls += 1
                if not has_subject_keyword(node):
                    fail(f"PolicyRequest without explicit subject_ref: {path}:{node.lineno}")
            elif name == "ProtectedDataRequest":
                protected_calls += 1
                if not has_subject_keyword(node):
                    fail(f"ProtectedDataRequest without explicit subject_ref: {path}:{node.lineno}")
    if policy_calls == 0:
        fail("no PolicyRequest construction sites found")
    if protected_calls == 0:
        fail("no ProtectedDataRequest construction sites found")
    if "subject_ref: str | None" not in (ROOT / "services/shared/policy_kernel.py").read_text(encoding="utf-8"):
        fail("PolicyRequest must expose subject_ref")
    if "subject_ref: str" not in (ROOT / "services/shared/protected_data_access.py").read_text(encoding="utf-8"):
        fail("ProtectedDataRequest must require subject_ref")

def audit_rust() -> None:
    canonical = (RUST_ROOT / "canonical_authorization.rs").read_text(encoding="utf-8")
    if "pub principal_ref: String" not in canonical or "pub subject_ref: String" not in canonical:
        fail("Rust AuthorizationRequest must contain principal_ref and subject_ref")
    for path in RUST_ROOT.rglob("*.rs"):
        if "archive" in path.parts:
            continue
        if "principal_ref: context.subject_ref" in path.read_text(encoding="utf-8"):
            fail(f"Rust actor/subject collapse detected: {path}")
    vault = (RUST_ROOT / "canonical_vault_authorizer.rs").read_text(encoding="utf-8")
    if "principal_ref: context.principal_ref.clone()" not in vault:
        fail("vault authorizer must preserve principal_ref")
    if "subject_ref: context.subject_ref.clone()" not in vault:
        fail("vault authorizer must preserve subject_ref")

def audit_governed_operation_schema() -> None:
    path = ROOT / "schemas/governed-capability-operation-v1.json"
    required = set(json.loads(path.read_text(encoding="utf-8")).get("required", []))
    if not {"principal_ref", "subject_ref"} <= required:
        fail("governed capability operation must require principal_ref and subject_ref")

def main() -> int:
    audit_python()
    audit_rust()
    audit_governed_operation_schema()
    print("PASS: canonical authorization invariants")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
