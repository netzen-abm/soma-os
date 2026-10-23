#!/usr/bin/env python3
"""Archive and retire the source branch of a merged same-repository PR.

Safety order: validate the merged PR event -> confirm exact branch tip ->
check open PR references -> create/verify immutable recovery tag -> recheck
tip and dependencies -> delete branch ref -> verify deletion -> audit.
This script never retires one of SOMA's nine canonical active branches.
"""

from __future__ import annotations

import json
import os
import re
import sys
from typing import Any
from urllib.error import HTTPError
from urllib.parse import quote
from urllib.request import Request, urlopen

CANONICAL_BRANCHES = frozenset(
    {
        "main",
        "development",
        "security/current",
        "architecture/current",
        "feature/current",
        "health/current",
        "research/current",
        "ai-agent/current",
        "integration/current",
    }
)
BRANCH_RETIREMENT_ISSUE = 161
SHA_RE = re.compile(r"^[0-9a-f]{40}$")
BRANCH_CHAR_RE = re.compile(r"^[A-Za-z0-9._/-]+$")


def is_valid_branch_ref(branch: object) -> bool:
    if not isinstance(branch, str) or not branch or not BRANCH_CHAR_RE.fullmatch(branch):
        return False
    if branch.startswith("/") or branch.endswith(("/", ".")) or "//" in branch:
        return False
    if ".." in branch or "@{" in branch or "\\" in branch:
        return False
    components = branch.split("/")
    return all(
        component
        and not component.startswith(".")
        and not component.endswith(".lock")
        for component in components
    )


def archive_tag_name(pr_number: int, head_sha: str) -> str:
    if not isinstance(pr_number, int) or pr_number < 1:
        raise ValueError("invalid PR number")
    if not SHA_RE.fullmatch(head_sha):
        raise ValueError("invalid merged head SHA")
    return f"soma-branch-archive-pr-{pr_number}-{head_sha[:12]}"


def candidate_for_event(event: dict[str, Any], repository: str) -> dict[str, Any] | None:
    if event.get("action") != "closed":
        return None
    pr = event.get("pull_request")
    if not isinstance(pr, dict) or pr.get("merged") is not True:
        return None

    base = pr.get("base") or {}
    head = pr.get("head") or {}
    base_repo = (base.get("repo") or {}).get("full_name")
    head_repo = (head.get("repo") or {}).get("full_name")

    # Only same-repository PRs targeting canonical main trigger source-branch
    # cleanup. Forks and other base branches are intentionally left untouched.
    if base.get("ref") != "main" or base_repo != repository or head_repo != repository:
        return None

    branch = head.get("ref")
    head_sha = head.get("sha")
    pr_number = pr.get("number")
    if not is_valid_branch_ref(branch):
        raise ValueError("merged PR has an invalid source branch name")
    if branch in CANONICAL_BRANCHES:
        return None
    if not isinstance(head_sha, str) or not SHA_RE.fullmatch(head_sha):
        raise ValueError("merged PR has an invalid source head SHA")
    if not isinstance(pr_number, int) or pr_number < 1:
        raise ValueError("merged PR has an invalid number")

    return {"branch": branch, "head_sha": head_sha, "pr_number": pr_number}


class GitHubApi:
    def __init__(self, repository: str, token: str, api_url: str) -> None:
        if "/" not in repository or not token:
            raise ValueError("repository and GitHub token are required")
        self.repository = repository
        self.token = token
        self.api_url = api_url.rstrip("/")

    def request(
        self,
        method: str,
        path: str,
        payload: dict[str, Any] | None = None,
        *,
        allow_404: bool = False,
    ) -> Any:
        body = json.dumps(payload).encode("utf-8") if payload is not None else None
        request = Request(
            f"{self.api_url}/{path.lstrip('/')}",
            data=body,
            method=method,
            headers={
                "Accept": "application/vnd.github+json",
                "Authorization": f"Bearer {self.token}",
                "X-GitHub-Api-Version": "2022-11-28",
                "Content-Type": "application/json",
                "User-Agent": "soma-branch-retirement",
            },
        )
        try:
            with urlopen(request, timeout=30) as response:
                raw = response.read()
        except HTTPError as exc:
            detail = exc.read().decode("utf-8", errors="replace")
            if allow_404 and exc.code == 404:
                return None
            raise RuntimeError(
                f"GitHub API {method} {path} failed ({exc.code}): {detail[:500]}"
            ) from exc
        if not raw:
            return None
        return json.loads(raw.decode("utf-8"))

    def list_items(self, path: str) -> list[dict[str, Any]]:
        items: list[dict[str, Any]] = []
        page = 1
        while True:
            separator = "&" if "?" in path else "?"
            page_items = self.request(
                "GET", f"{path}{separator}per_page=100&page={page}"
            )
            if not isinstance(page_items, list):
                raise RuntimeError(f"Expected list from GitHub API: {path}")
            items.extend(page_items)
            if len(page_items) < 100:
                return items
            page += 1

    def current_branch_sha(self, branch: str) -> str | None:
        for item in self.list_items(f"repos/{self.repository}/branches"):
            if item.get("name") == branch:
                commit = item.get("commit") or {}
                sha = commit.get("sha")
                return sha if isinstance(sha, str) else None
        return None

    def has_other_open_pr(self, branch: str, pr_number: int) -> bool:
        prs = self.list_items(f"repos/{self.repository}/pulls?state=open")
        return any(
            item.get("number") != pr_number
            and (item.get("head") or {}).get("ref") == branch
            and ((item.get("head") or {}).get("repo") or {}).get("full_name")
            == self.repository
            for item in prs
        )

    def tag_sha(self, tag: str) -> str | None:
        result = self.request(
            "GET",
            f"repos/{self.repository}/git/ref/tags/{quote(tag, safe='')}",
            allow_404=True,
        )
        if result is None:
            return None
        obj = result.get("object") or {}
        if obj.get("type") != "commit":
            raise RuntimeError(f"Archive tag {tag} is not a lightweight commit tag")
        return obj.get("sha")

    def create_tag(self, tag: str, head_sha: str) -> None:
        self.request(
            "POST",
            f"repos/{self.repository}/git/refs",
            {"ref": f"refs/tags/{tag}", "sha": head_sha},
        )

    def delete_branch(self, branch: str) -> None:
        # Encode the complete ref so slashes in branch names stay within the
        # GitHub API's single {ref} path parameter.
        ref = quote(f"heads/{branch}", safe="")
        self.request("DELETE", f"repos/{self.repository}/git/refs/{ref}")

    def add_audit_comment(self, body: str) -> None:
        self.request(
            "POST",
            f"repos/{self.repository}/issues/{BRANCH_RETIREMENT_ISSUE}/comments",
            {"body": body},
        )


def retire_merged_branch(
    client: Any,
    candidate: dict[str, Any],
    repository: str,
    run_url: str,
) -> str:
    branch = candidate["branch"]
    expected_sha = candidate["head_sha"]
    pr_number = candidate["pr_number"]

    if branch in CANONICAL_BRANCHES:
        raise RuntimeError("refusing to retire a canonical active branch")
    if not is_valid_branch_ref(branch) or not SHA_RE.fullmatch(expected_sha):
        raise RuntimeError("refusing invalid branch reference or expected SHA")

    current_sha = client.current_branch_sha(branch)
    if current_sha is None:
        raise RuntimeError("source branch is already absent; archive proof is required")
    if current_sha != expected_sha:
        raise RuntimeError(
            f"source branch advanced after PR merge: expected {expected_sha}, found {current_sha}"
        )
    if client.has_other_open_pr(branch, pr_number):
        raise RuntimeError("another open PR still references the source branch")

    tag = archive_tag_name(pr_number, expected_sha)
    archived_sha = client.tag_sha(tag)
    if archived_sha is None:
        client.create_tag(tag, expected_sha)
        archived_sha = client.tag_sha(tag)
    if archived_sha != expected_sha:
        raise RuntimeError(
            f"archive tag mismatch for {tag}: expected {expected_sha}, found {archived_sha}"
        )

    client.add_audit_comment(
        f"PR #{pr_number} merged into main. Archive tag `{tag}` was verified at "
        f"`{expected_sha}`. Rechecking source tip and open-PR dependencies before "
        f"deleting `{branch}`. Workflow: {run_url}"
    )

    # Recheck immediately before deletion. This is deliberately fail-closed:
    # a changed tip or new open PR leaves the branch intact and already archived.
    if client.current_branch_sha(branch) != expected_sha:
        raise RuntimeError("source branch changed after archival; leaving it intact")
    if client.has_other_open_pr(branch, pr_number):
        raise RuntimeError("another PR began referencing the source branch; leaving it intact")

    client.delete_branch(branch)
    if client.current_branch_sha(branch) is not None:
        raise RuntimeError("GitHub did not confirm source-branch deletion")

    try:
        client.add_audit_comment(
            f"Retired `{branch}` after merged PR #{pr_number}. The original head "
            f"`{expected_sha}` is preserved by recovery tag `{tag}`. The source "
            f"branch was confirmed absent. Workflow: {run_url}"
        )
    except Exception as exc:  # deletion already verified; leave a warning in job logs
        print(f"WARNING: retirement succeeded but final issue comment failed: {exc}")

    print(
        f"ARCHIVED_AND_RETIRED branch={branch} sha={expected_sha} "
        f"tag={tag} pr={pr_number}"
    )
    return tag


def main() -> int:
    event_path = os.environ.get("GITHUB_EVENT_PATH")
    repository = os.environ.get("GITHUB_REPOSITORY", "")
    token = os.environ.get("GITHUB_TOKEN", "")
    api_url = os.environ.get("GITHUB_API_URL", "https://api.github.com")
    run_url = os.environ.get("GITHUB_RUN_URL", "(workflow run URL unavailable)")
    if not event_path:
        print("ERROR: GITHUB_EVENT_PATH is required", file=sys.stderr)
        return 2

    try:
        with open(event_path, encoding="utf-8") as event_file:
            event = json.load(event_file)
        candidate = candidate_for_event(event, repository)
        if candidate is None:
            print("SKIP: event is not a same-repository merged PR to main with a noncanonical head.")
            return 0
        client = GitHubApi(repository, token, api_url)
        retire_merged_branch(client, candidate, repository, run_url)
        return 0
    except Exception as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
