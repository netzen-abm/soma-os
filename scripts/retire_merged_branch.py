#!/usr/bin/env python3
"""Archive exact branch heads before retiring approved noncanonical refs."""

from __future__ import annotations

import hashlib
import json
import os
import re
import sys
from pathlib import Path
from typing import Any
from urllib.error import HTTPError
from urllib.parse import quote
from urllib.request import Request, urlopen

CANONICAL_BRANCHES = frozenset({
    "main", "development", "security/current", "architecture/current",
    "feature/current", "health/current", "research/current",
    "ai-agent/current", "integration/current",
})
MANIFEST_PATH = Path("docs/governance/branch-retirement-manifest-v1.json")
RETIREMENT_ISSUE = 161
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
    if (
        base.get("ref") != "main"
        or (base.get("repo") or {}).get("full_name") != repository
        or (head.get("repo") or {}).get("full_name") != repository
    ):
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
    return {"branch": branch, "head_sha": sha, "pr_number": number, "reason": f"Source branch of merged PR #{number}", "evidence": f"PR #{number}"}


def manifest_candidates(manifest: dict[str, Any], pr_number: int) -> list[dict[str, Any]]:
    if manifest.get("schema_version") != "1.0.0":
        raise ValueError("unsupported branch-retirement manifest schema")
    canonical = manifest.get("canonical_branches")
    if not isinstance(canonical, list) or set(canonical) != CANONICAL_BRANCHES:
        raise ValueError("manifest canonical branch set does not match the protected nine")
    if not isinstance(manifest.get("inventory_main_sha"), str) or not SHA_RE.fullmatch(manifest["inventory_main_sha"]):
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
        result.append({
            "branch": branch, "head_sha": sha, "pr_number": pr_number,
            "disposition": disposition, "reason": reason.strip(), "evidence": evidence.strip(),
        })
    return result


def read_manifest(path: Path, pr_number: int) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    with path.open(encoding="utf-8") as handle:
        data = json.load(handle)
    if not isinstance(data, dict):
        raise ValueError("branch-retirement manifest root must be an object")
    return manifest_candidates(data, pr_number)


def tracked_config_reference(branch: str, root: Path = Path(".")) -> str | None:
    paths: list[Path] = []
    for folder in (".github", ".circleci"):
        directory = root / folder
        if directory.is_dir():
            paths.extend(
                p for p in directory.rglob("*")
                if p.is_file() and p.suffix in {".yml", ".yaml", ".toml", ".json"}
            )
    for name in (".gitlab-ci.yml", "render.yaml", "render.yml", "vercel.json", "netlify.toml", "fly.toml", "railway.json"):
        p = root / name
        if p.is_file():
            paths.append(p)
    token = re.compile(r"(?<![A-Za-z0-9._/-])" + re.escape(branch) + r"(?![A-Za-z0-9._/-])")
    for path in paths:
        try:
            text = path.read_text(encoding="utf-8")
        except (OSError, UnicodeError):
            return str(path)
        if token.search(text):
            return str(path)
    return None


class GitHubApi:
    def __init__(self, repository: str, token: str, api_url: str) -> None:
        if "/" not in repository or not token:
            raise ValueError("repository and GitHub token are required")
        self.repository, self.token, self.api_url = repository, token, api_url.rstrip("/")

    def request(self, method: str, path: str, payload: dict[str, Any] | None = None, allow_404: bool = False) -> Any:
        body = json.dumps(payload).encode("utf-8") if payload is not None else None
        req = Request(
            f"{self.api_url}/{path.lstrip('/')}", data=body, method=method,
            headers={
                "Accept": "application/vnd.github+json", "Authorization": f"Bearer {self.token}",
                "X-GitHub-Api-Version": "2022-11-28", "Content-Type": "application/json",
                "User-Agent": "soma-branch-retirement",
            },
        )
        try:
            with urlopen(req, timeout=30) as response:
                raw = response.read()
        except HTTPError as exc:
            detail = exc.read().decode("utf-8", errors="replace")
            if allow_404 and exc.code == 404:
                return None
            raise RuntimeError(f"GitHub API {method} {path} failed ({exc.code}): {detail[:400]}") from exc
        return json.loads(raw.decode("utf-8")) if raw else None

    def list_items(self, path: str) -> list[dict[str, Any]]:
        values: list[dict[str, Any]] = []
        page = 1
        while True:
            sep = "&" if "?" in path else "?"
            batch = self.request("GET", f"{path}{sep}per_page=100&page={page}")
            if not isinstance(batch, list):
                raise RuntimeError(f"Expected list from GitHub API: {path}")
            values.extend(batch)
            if len(batch) < 100:
                return values
            page += 1

    def merged_pr_for_commit(self, sha: str) -> dict[str, Any] | None:
        if not SHA_RE.fullmatch(sha):
            raise ValueError("push event has no valid commit SHA")
        for pr in self.list_items(f"repos/{self.repository}/commits/{sha}/pulls"):
            base, head = pr.get("base") or {}, pr.get("head") or {}
            if (
                pr.get("merge_commit_sha") == sha and pr.get("merged_at")
                and base.get("ref") == "main"
                and (base.get("repo") or {}).get("full_name") == self.repository
                and (head.get("repo") or {}).get("full_name") == self.repository
            ):
                return {
                    "action": "closed",
                    "pull_request": {"number": pr.get("number"), "merged": True, "base": base, "head": head},
                }
        return None

    def branch_state(self, branch: str) -> dict[str, Any] | None:
        for item in self.list_items(f"repos/{self.repository}/branches"):
            if item.get("name") == branch:
                return {"sha": (item.get("commit") or {}).get("sha"), "protected": item.get("protected")}
        return None

    def has_other_open_pr(self, branch: str, pr_number: int) -> bool:
        for pr in self.list_items(f"repos/{self.repository}/pulls?state=open"):
            head = pr.get("head") or {}
            if pr.get("number") != pr_number and head.get("ref") == branch and (head.get("repo") or {}).get("full_name") == self.repository:
                return True
        return False

    def tag_sha(self, tag: str) -> str | None:
        result = self.request("GET", f"repos/{self.repository}/git/ref/tags/{quote(tag, safe='')}", allow_404=True)
        if result is None:
            return None
        obj = result.get("object") or {}
        if obj.get("type") != "commit":
            raise RuntimeError(f"archive tag {tag} is not a lightweight commit tag")
        return obj.get("sha")

    def create_tag(self, tag: str, sha: str) -> None:
        self.request("POST", f"repos/{self.repository}/git/refs", {"ref": f"refs/tags/{tag}", "sha": sha})

    def delete_branch(self, branch: str) -> None:
        self.request("DELETE", f"repos/{self.repository}/git/refs/{quote('heads/' + branch, safe='')}")

    def audit(self, body: str) -> None:
        self.request("POST", f"repos/{self.repository}/issues/{RETIREMENT_ISSUE}/comments", {"body": body})


def _prepare(client: Any, candidate: dict[str, Any], root: Path) -> dict[str, Any]:
    branch, expected = candidate["branch"], candidate["head_sha"]
    tag = archive_tag_name(branch, expected)
    if branch in CANONICAL_BRANCHES:
        raise RuntimeError("canonical active branch is protected")
    if not valid_branch(branch) or not SHA_RE.fullmatch(expected):
        raise RuntimeError("invalid branch reference or expected SHA")
    state = client.branch_state(branch)
    if state is None:
        if client.tag_sha(tag) == expected:
            return {**candidate, "tag": tag, "already_retired": True}
        raise RuntimeError("branch is absent without its exact recovery tag")
    if state.get("sha") != expected:
        raise RuntimeError(f"branch tip mismatch: expected {expected}, found {state.get('sha')}")
    if state.get("protected") is not False:
        raise RuntimeError("branch protection state is not explicitly unprotected")
    if client.has_other_open_pr(branch, candidate["pr_number"]):
        raise RuntimeError("another open PR references the branch")
    config_ref = tracked_config_reference(branch, root)
    if config_ref:
        raise RuntimeError(f"tracked workflow/deployment config references this branch: {config_ref}")
    archived = client.tag_sha(tag)
    if archived is None:
        client.create_tag(tag, expected)
        archived = client.tag_sha(tag)
    if archived != expected:
        raise RuntimeError(f"recovery tag mismatch: expected {expected}, found {archived}")
    return {**candidate, "tag": tag, "already_retired": False}


def _recheck(client: Any, item: dict[str, Any], root: Path) -> None:
    if item["already_retired"]:
        return
    branch, expected = item["branch"], item["head_sha"]
    state = client.branch_state(branch)
    if state is None or state.get("sha") != expected:
        raise RuntimeError(f"branch changed or disappeared after archival: {branch}")
    if state.get("protected") is not False:
        raise RuntimeError(f"branch is protected or protection is unknown: {branch}")
    if client.has_other_open_pr(branch, item["pr_number"]):
        raise RuntimeError(f"another open PR references the branch: {branch}")
    config_ref = tracked_config_reference(branch, root)
    if config_ref:
        raise RuntimeError(f"automation dependency appeared for {branch}: {config_ref}")
    if client.tag_sha(item["tag"]) != expected:
        raise RuntimeError(f"recovery tag verification failed for {branch}")


def retire_batch(client: Any, candidates: list[dict[str, Any]], run_url: str, root: Path = Path(".")) -> list[str]:
    names = [item.get("branch") for item in candidates]
    if len(names) != len(set(names)):
        raise RuntimeError("duplicate branch in retirement batch")
    prepared, errors = [], []
    for item in candidates:
        try:
            prepared.append(_prepare(client, item, root))
        except Exception as exc:
            errors.append(f"{item.get('branch')}: {exc}")
    if errors:
        raise RuntimeError("retirement preflight failed; no branch refs deleted:\\n- " + "\\n- ".join(errors))

    summary = [
        f"- {x['branch']} @ {x['head_sha']} -> {x['tag']} ({x.get('disposition', 'merged-PR-source')})"
        for x in prepared
    ]
    if prepared:
        client.audit(
            "Retirement preflight passed. Exact recovery tags exist; the workflow is rechecking "
            "tips, protection, open PR references, and tracked config before deletion.\\n\\n"
            + "\\n".join(summary) + f"\\n\\nWorkflow: {run_url}"
        )
    errors = []
    for item in prepared:
        try:
            _recheck(client, item, root)
        except Exception as exc:
            errors.append(f"{item['branch']}: {exc}")
    if errors:
        raise RuntimeError("final batch check failed; no branch refs deleted:\\n- " + "\\n- ".join(errors))

    deleted = []
    for item in prepared:
        if item["already_retired"]:
            continue
        _recheck(client, item, root)
        client.delete_branch(item["branch"])
        if client.branch_state(item["branch"]) is not None:
            raise RuntimeError(f"GitHub did not confirm branch deletion: {item['branch']}")
        deleted.append(item)
    if prepared:
        client.audit(
            f"Retirement batch completed: deleted {len(deleted)} refs; "
            f"{len(prepared)-len(deleted)} were already retired with exact recovery tags. "
            "All original heads remain recoverable.\\n\\n"
            + "\\n".join(summary) + f"\\n\\nWorkflow: {run_url}"
        )
    return [item["tag"] for item in prepared]


def retire_merged_branch(client: Any, candidate: dict[str, Any], repository: str, run_url: str) -> str:
    return retire_batch(client, [candidate], run_url)[0]


def main() -> int:
    event_path = os.environ.get("GITHUB_EVENT_PATH")
    repository, token = os.environ.get("GITHUB_REPOSITORY", ""), os.environ.get("GITHUB_TOKEN", "")
    api_url = os.environ.get("GITHUB_API_URL", "https://api.github.com")
    run_url = os.environ.get("GITHUB_RUN_URL", "(workflow run URL unavailable)")
    if not event_path:
        print("ERROR: GITHUB_EVENT_PATH is required", file=sys.stderr)
        return 2
    try:
        with open(event_path, encoding="utf-8") as handle:
            event = json.load(handle)
        if event.get("ref") != "refs/heads/main":
            print("SKIP: retirement runs only for pushes to main")
            return 0
        sha = event.get("after")
        if not isinstance(sha, str) or not SHA_RE.fullmatch(sha):
            raise ValueError("push event has no valid after SHA")
        client = GitHubApi(repository, token, api_url)
        merged = client.merged_pr_for_commit(sha)
        if merged is None:
            print("SKIP: main push has no associated merged same-repository PR")
            return 0
        pr_number = merged["pull_request"]["number"]
        candidates = []
        source = candidate_for_event(merged, repository)
        if source is not None:
            candidates.append(source)
        candidates.extend(read_manifest(MANIFEST_PATH, pr_number))
        if not candidates:
            print("SKIP: merged PR has no eligible source branch or manifest targets")
            return 0
        retire_batch(client, candidates, run_url)
        return 0
    except Exception as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
