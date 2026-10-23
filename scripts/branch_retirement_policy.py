"""Pure policy and manifest validation for SOMA branch retirement."""
from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path
from typing import Any

CANONICAL_BRANCHES = frozenset({
    "main", "development", "security/current", "architecture/current",
    "feature/current", "health/current", "research/current",
    "ai-agent/current", "integration/current",
})
MANIFEST_PATH = Path("docs/governance/branch-retirement-manifest-v1.json")
SHA_RE = re.compile(r"^[0-9a-f]{40}$")
BRANCH_RE = re.compile(r"^[A-Za-z0-9._/-]+$")


def valid_branch(branch: object) -> bool:
    if not isinstance(branch, str) or not branch or not BRANCH_RE.fullmatch(branch):
        return False
    if branch.startswith("/") or branch.endswith(("/", ".")) or "//" in branch:
        return False
    if ".." in branch or "@{" in branch or "\\" in branch:
        return False
    return all(p and not p.startswith(".") and not p.endswith(".lock") for p in branch.split("/"))


def archive_tag_name(branch: str, sha: str) -> str:
    if not valid_branch(branch) or not SHA_RE.fullmatch(sha):
        raise ValueError("invalid branch or SHA for archive tag")
    key = hashlib.sha256(branch.encode("utf-8")).hexdigest()[:10]
    return f"soma-branch-archive-{sha[:12]}-{key}"


def candidate_for_event(event: dict[str, Any], repository: str) -> dict[str, Any] | None:
    if event.get("action") != "closed":
        return None
    pr = event.get("pull_request")
    if not isinstance(pr, dict) or pr.get("merged") is not True:
        return None
    base, head = pr.get("base") or {}, pr.get("head") or {}
    if base.get("ref") != "main" or (base.get("repo") or {}).get("full_name") != repository:
        return None
    if (head.get("repo") or {}).get("full_name") != repository:
        return None
    branch, sha, number = head.get("ref"), head.get("sha"), pr.get("number")
    if not valid_branch(branch):
        raise ValueError("merged PR has invalid source branch name")
    if branch in CANONICAL_BRANCHES:
        return None
    if not isinstance(sha, str) or not SHA_RE.fullmatch(sha):
        raise ValueError("merged PR has invalid source SHA")
    if not isinstance(number, int) or number < 1:
        raise ValueError("merged PR has invalid number")
    return {"branch": branch, "head_sha": sha, "pr_number": number,
            "reason": f"Source branch of merged PR #{number}", "evidence": f"PR #{number}"}


def manifest_candidates(manifest: dict[str, Any], pr_number: int) -> list[dict[str, Any]]:
    if manifest.get("schema_version") != "1.0.0":
        raise ValueError("unsupported branch-retirement manifest schema")
    canonical = manifest.get("canonical_branches")
    if not isinstance(canonical, list) or set(canonical) != CANONICAL_BRANCHES:
        raise ValueError("manifest canonical branch set does not match the protected nine")
    inventory_sha = manifest.get("inventory_main_sha")
    if not isinstance(inventory_sha, str) or not SHA_RE.fullmatch(inventory_sha):
        raise ValueError("manifest inventory_main_sha must be a commit SHA")
    entries = manifest.get("entries")
    if not isinstance(entries, list):
        raise ValueError("manifest entries must be a list")
    result: list[dict[str, Any]] = []
    seen: set[str] = set()
    for entry in entries:
        if not isinstance(entry, dict):
            raise ValueError("manifest entry must be an object")
        branch, sha = entry.get("branch"), entry.get("expected_sha")
        disposition, reason, evidence = entry.get("disposition"), entry.get("reason"), entry.get("evidence")
        if not valid_branch(branch) or branch in CANONICAL_BRANCHES:
            raise ValueError(f"manifest contains an invalid or canonical branch: {branch!r}")
        if not isinstance(sha, str) or not SHA_RE.fullmatch(sha):
            raise ValueError(f"invalid expected SHA for {branch}")
        if disposition not in {"REDUNDANT_DELETE", "PRESERVE_THEN_DELETE"}:
            raise ValueError(f"unsupported retirement disposition for {branch}")
        if not isinstance(reason, str) or not reason.strip():
            raise ValueError(f"retirement reason is required for {branch}")
        if not isinstance(evidence, str) or not evidence.strip():
            raise ValueError(f"evidence reference is required for {branch}")
        if branch in seen:
            raise ValueError(f"duplicate branch in manifest: {branch}")
        seen.add(branch)
        result.append({"branch": branch, "head_sha": sha, "pr_number": pr_number,
                       "disposition": disposition, "reason": reason.strip(), "evidence": evidence.strip()})
    return result


def read_manifest(path: Path, pr_number: int) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    with path.open(encoding="utf-8") as handle:
        manifest = json.load(handle)
    if not isinstance(manifest, dict):
        raise ValueError("branch-retirement manifest root must be an object")
    return manifest_candidates(manifest, pr_number)


def tracked_config_reference(branch: str, root: Path = Path(".")) -> str | None:
    paths: list[Path] = []
    for folder in (".github", ".circleci"):
        directory = root / folder
        if directory.is_dir():
            paths.extend(p for p in directory.rglob("*")
                         if p.is_file() and p.suffix in {".yml", ".yaml", ".toml", ".json"})
    for name in (".gitlab-ci.yml", "render.yaml", "render.yml", "vercel.json",
                 "netlify.toml", "fly.toml", "railway.json"):
        path = root / name
        if path.is_file():
            paths.append(path)
    # Token boundaries distinguish actual ref literals from similar filenames.
    token = re.compile(r"(?<![A-Za-z0-9._/-])" + re.escape(branch) + r"(?![A-Za-z0-9._/-])")
    for path in paths:
        try:
            text = path.read_text(encoding="utf-8")
        except (OSError, UnicodeError):
            return str(path)
        if token.search(text):
            return str(path)
    return None
