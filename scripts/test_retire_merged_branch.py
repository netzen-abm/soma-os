#!/usr/bin/env python3
"""Offline safety tests for PR-driven branch retirement."""

from __future__ import annotations

import unittest

from retire_merged_branch import (
    CANONICAL_BRANCHES,
    archive_tag_name,
    candidate_for_event,
    retire_merged_branch,
)


REPO = "netzen-abm/soma-os"
HEAD_SHA = "a" * 40


def event(
    *,
    branch: str = "feature/example",
    merged: bool = True,
    base_ref: str = "main",
    head_repo: str = REPO,
    head_sha: str = HEAD_SHA,
) -> dict:
    return {
        "action": "closed",
        "pull_request": {
            "number": 123,
            "merged": merged,
            "base": {"ref": base_ref, "repo": {"full_name": REPO}},
            "head": {
                "ref": branch,
                "sha": head_sha,
                "repo": {"full_name": head_repo},
            },
        },
    }


class FakeClient:
    def __init__(self, branch: str = "feature/example", sha: str = HEAD_SHA) -> None:
        self.branches = {branch: sha}
        self.tags: dict[str, str] = {}
        self.open_pr_branches: set[str] = set()
        self.operations: list[str] = []

    def current_branch_sha(self, branch: str) -> str | None:
        self.operations.append("read-branch")
        return self.branches.get(branch)

    def has_other_open_pr(self, branch: str, pr_number: int) -> bool:
        self.operations.append("check-open-prs")
        return branch in self.open_pr_branches

    def tag_sha(self, tag: str) -> str | None:
        self.operations.append("read-tag")
        return self.tags.get(tag)

    def create_tag(self, tag: str, head_sha: str) -> None:
        self.operations.append("create-tag")
        if tag in self.tags:
            raise RuntimeError("tag already exists")
        self.tags[tag] = head_sha

    def delete_branch(self, branch: str) -> None:
        self.operations.append("delete-branch")
        del self.branches[branch]

    def add_audit_comment(self, body: str) -> None:
        self.operations.append("audit-comment")


class RetirementPolicyTests(unittest.TestCase):
    def test_exactly_nine_canonical_branches_are_protected(self) -> None:
        self.assertEqual(
            CANONICAL_BRANCHES,
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
            },
        )

    def test_tag_is_stable_for_a_specific_pr_head(self) -> None:
        self.assertEqual(
            archive_tag_name(123, HEAD_SHA),
            f"soma-branch-archive-pr-123-{HEAD_SHA[:12]}",
        )

    def test_merged_same_repo_noncanonical_pr_to_main_is_candidate(self) -> None:
        result = candidate_for_event(event(), REPO)
        self.assertEqual(
            result,
            {"branch": "feature/example", "head_sha": HEAD_SHA, "pr_number": 123},
        )

    def test_canonical_branch_is_never_candidate(self) -> None:
        for branch in CANONICAL_BRANCHES:
            with self.subTest(branch=branch):
                self.assertIsNone(candidate_for_event(event(branch=branch), REPO))

    def test_unmerged_pr_is_never_candidate(self) -> None:
        self.assertIsNone(candidate_for_event(event(merged=False), REPO))

    def test_pr_targeting_non_main_is_never_candidate(self) -> None:
        self.assertIsNone(candidate_for_event(event(base_ref="development"), REPO))

    def test_fork_pr_is_never_candidate(self) -> None:
        self.assertIsNone(
            candidate_for_event(event(head_repo="external/fork"), REPO)
        )

    def test_invalid_sha_fails_closed(self) -> None:
        with self.assertRaises(ValueError):
            candidate_for_event(event(head_sha="not-a-sha"), REPO)

    def test_invalid_branch_name_fails_closed(self) -> None:
        with self.assertRaises(ValueError):
            candidate_for_event(event(branch="../main"), REPO)

    def test_archive_tag_precedes_branch_deletion(self) -> None:
        candidate = candidate_for_event(event(), REPO)
        assert candidate is not None
        client = FakeClient()
        tag = retire_merged_branch(client, candidate, REPO, "https://example/run")
        self.assertEqual(client.tags[tag], HEAD_SHA)
        self.assertNotIn("feature/example", client.branches)
        self.assertLess(
            client.operations.index("create-tag"),
            client.operations.index("delete-branch"),
        )

    def test_head_mismatch_preserves_branch_without_archive_or_delete(self) -> None:
        candidate = candidate_for_event(event(), REPO)
        assert candidate is not None
        client = FakeClient(sha="b" * 40)
        with self.assertRaisesRegex(RuntimeError, "advanced after PR merge"):
            retire_merged_branch(client, candidate, REPO, "https://example/run")
        self.assertIn("feature/example", client.branches)
        self.assertEqual(client.tags, {})
        self.assertNotIn("delete-branch", client.operations)

    def test_open_pr_dependency_preserves_branch(self) -> None:
        candidate = candidate_for_event(event(), REPO)
        assert candidate is not None
        client = FakeClient()
        client.open_pr_branches.add("feature/example")
        with self.assertRaisesRegex(RuntimeError, "another open PR"):
            retire_merged_branch(client, candidate, REPO, "https://example/run")
        self.assertIn("feature/example", client.branches)
        self.assertEqual(client.tags, {})
        self.assertNotIn("delete-branch", client.operations)

    def test_conflicting_archive_tag_preserves_branch(self) -> None:
        candidate = candidate_for_event(event(), REPO)
        assert candidate is not None
        client = FakeClient()
        tag = archive_tag_name(123, HEAD_SHA)
        client.tags[tag] = "b" * 40
        with self.assertRaisesRegex(RuntimeError, "archive tag mismatch"):
            retire_merged_branch(client, candidate, REPO, "https://example/run")
        self.assertIn("feature/example", client.branches)
        self.assertNotIn("delete-branch", client.operations)


if __name__ == "__main__":
    unittest.main(verbosity=2)
