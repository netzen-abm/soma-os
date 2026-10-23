#!/usr/bin/env python3
"""Fail-closed orchestration: archive every approved head, recheck, then delete."""
from __future__ import annotations

import os
import json
import sys
from pathlib import Path
from typing import Any

from branch_retirement_policy import (
    CANONICAL_BRANCHES, MANIFEST_PATH, SHA_RE, archive_tag_name,
    candidate_for_event, manifest_candidates, read_manifest, tracked_config_reference, valid_branch,
)
from github_branch_retirement import GitHubApi


def _prepare(client: Any, item: dict[str, Any], root: Path) -> dict[str, Any]:
    branch, expected = item["branch"], item["head_sha"]
    tag = archive_tag_name(branch, expected)
    if branch in CANONICAL_BRANCHES:
        raise RuntimeError("canonical active branch is protected")
    if not valid_branch(branch) or not SHA_RE.fullmatch(expected):
        raise RuntimeError("invalid branch reference or expected SHA")
    state = client.branch_state(branch)
    if state is None:
        if client.tag_sha(tag) == expected:
            return {**item, "tag": tag, "already_retired": True}
        raise RuntimeError("branch is absent without its exact recovery tag")
    if state.get("sha") != expected:
        raise RuntimeError(f"branch tip mismatch: expected {expected}, found {state.get('sha')}")
    if state.get("protected") is not False:
        raise RuntimeError("branch protection state is not explicitly unprotected")
    if client.has_other_open_pr(branch, item["pr_number"]):
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
    return {**item, "tag": tag, "already_retired": False}


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


def retire_batch(client: Any, candidates: list[dict[str, Any]], run_url: str,
                 root: Path = Path(".")) -> list[str]:
    names = [item.get("branch") for item in candidates]
    if len(names) != len(set(names)):
        raise RuntimeError("duplicate branch in retirement batch")
    prepared, errors = [], []
    # Archive and preflight every item before deleting any active branch ref.
    for item in candidates:
        try:
            prepared.append(_prepare(client, item, root))
        except Exception as exc:
            errors.append(f"{item.get('branch')}: {exc}")
    if errors:
        raise RuntimeError("retirement preflight failed; no branch refs deleted:\n- " + "\n- ".join(errors))
    lines = [f"- {x['branch']} @ {x['head_sha']} -> {x['tag']} ({x.get('disposition', 'merged-PR-source')})" for x in prepared]
    if prepared:
        client.audit("Retirement preflight passed; exact recovery tags exist for all entries.\n\n"
                     + "\n".join(lines) + f"\n\nWorkflow: {run_url}")
    errors = []
    for item in prepared:
        try:
            _recheck(client, item, root)
        except Exception as exc:
            errors.append(f"{item['branch']}: {exc}")
    if errors:
        raise RuntimeError("final batch check failed; no branch refs deleted:\n- " + "\n- ".join(errors))
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
        client.audit(f"Retirement batch complete: deleted {len(deleted)} refs; "
                     f"{len(prepared)-len(deleted)} already had verified recovery tags.\n\n"
                     + "\n".join(lines) + f"\n\nWorkflow: {run_url}")
    return [item["tag"] for item in prepared]


def retire_merged_branch(client: Any, candidate: dict[str, Any], repository: str, run_url: str) -> str:
    # Small compatibility surface for focused tests and single-branch callers.
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
