"""GitHub REST adapter for safe SOMA branch retirement."""
from __future__ import annotations

import json
from typing import Any
from urllib.error import HTTPError
from urllib.parse import quote
from urllib.request import Request, urlopen

from branch_retirement_policy import SHA_RE

RETIREMENT_ISSUE = 161


class GitHubApi:
    """Provider adapter only: policy decisions remain in branch-retirement modules."""

    def __init__(self, repository: str, token: str, api_url: str) -> None:
        if "/" not in repository or not token:
            raise ValueError("repository and GitHub token are required")
        self.repository, self.token, self.api_url = repository, token, api_url.rstrip("/")

    def request(self, method: str, path: str, payload: dict[str, Any] | None = None,
                allow_404: bool = False) -> Any:
        body = json.dumps(payload).encode("utf-8") if payload is not None else None
        req = Request(
            f"{self.api_url}/{path.lstrip('/')}", data=body, method=method,
            headers={"Accept": "application/vnd.github+json", "Authorization": f"Bearer {self.token}",
                     "X-GitHub-Api-Version": "2022-11-28", "Content-Type": "application/json",
                     "User-Agent": "soma-branch-retirement"},
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
        items: list[dict[str, Any]] = []
        page = 1
        while True:
            separator = "&" if "?" in path else "?"
            batch = self.request("GET", f"{path}{separator}per_page=100&page={page}")
            if not isinstance(batch, list):
                raise RuntimeError(f"Expected list from GitHub API: {path}")
            items.extend(batch)
            if len(batch) < 100:
                return items
            page += 1

    def merged_pr_for_commit(self, sha: str) -> dict[str, Any] | None:
        if not SHA_RE.fullmatch(sha):
            raise ValueError("push event has no valid commit SHA")
        for pr in self.list_items(f"repos/{self.repository}/commits/{sha}/pulls"):
            base, head = pr.get("base") or {}, pr.get("head") or {}
            if (pr.get("merge_commit_sha") == sha and pr.get("merged_at")
                and base.get("ref") == "main"
                and (base.get("repo") or {}).get("full_name") == self.repository
                and (head.get("repo") or {}).get("full_name") == self.repository):
                return {"action": "closed", "pull_request": {
                    "number": pr.get("number"), "merged": True, "base": base, "head": head}}
        return None

    def branch_state(self, branch: str) -> dict[str, Any] | None:
        for item in self.list_items(f"repos/{self.repository}/branches"):
            if item.get("name") == branch:
                return {"sha": (item.get("commit") or {}).get("sha"), "protected": item.get("protected")}
        return None

    def has_other_open_pr(self, branch: str, pr_number: int) -> bool:
        for pr in self.list_items(f"repos/{self.repository}/pulls?state=open"):
            head = pr.get("head") or {}
            if (pr.get("number") != pr_number and head.get("ref") == branch
                and (head.get("repo") or {}).get("full_name") == self.repository):
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
        self.request("POST", f"repos/{self.repository}/git/refs",
                     {"ref": f"refs/tags/{tag}", "sha": sha})

    def delete_branch(self, branch: str) -> None:
        ref = quote("heads/" + branch, safe="/")
        self.request("DELETE", f"repos/{self.repository}/git/refs/{ref}")

    def audit(self, body: str) -> None:
        self.request("POST", f"repos/{self.repository}/issues/{RETIREMENT_ISSUE}/comments", {"body": body})
