#!/usr/bin/env python3
"""Offline fail-closed tests for branch retirement."""
from __future__ import annotations
import unittest
from retire_merged_branch import CANONICAL_BRANCHES, archive_tag_name, candidate_for_event, retire_batch, retire_merged_branch

REPO = "netzen-abm/soma-os"
SHA = "a" * 40


def make_event(branch="feature/example", merged=True, base="main", head_repo=REPO, sha=SHA):
    return {"action": "closed", "pull_request": {"number": 123, "merged": merged,
        "base": {"ref": base, "repo": {"full_name": REPO}},
        "head": {"ref": branch, "sha": sha, "repo": {"full_name": head_repo}}}}


class FakeClient:
    def __init__(self, sha=SHA):
        self.branches = {name: {"sha": SHA, "protected": True} for name in CANONICAL_BRANCHES}
        self.branches["feature/example"] = {"sha": sha, "protected": False}
        self.tags, self.open_pr_heads, self.open_pr_bases, self.operations = {}, set(), set(), []

    def branch_names(self):
        return set(self.branches)

    def open_pr_head_branches(self):
        return set(self.open_pr_heads)

    def branch_state(self, branch):
        self.operations.append("read-branch")
        return self.branches.get(branch)

    def has_other_open_pr(self, branch, pr_number):
        self.operations.append("check-prs")
        return branch in self.open_pr_heads or branch in self.open_pr_bases

    def tag_sha(self, tag):
        self.operations.append("read-tag")
        return self.tags.get(tag)

    def create_tag(self, tag, sha):
        self.operations.append("create-tag")
        self.tags[tag] = sha

    def wait_for_tag(self, tag, expected_sha):
        self.operations.append("wait-for-tag")
        actual = self.tags.get(tag)
        if actual != expected_sha:
            raise RuntimeError(f"recovery tag mismatch: expected {expected_sha}, found {actual}")
        return actual

    def delete_branch(self, branch):
        self.operations.append("delete-branch")
        del self.branches[branch]

    def audit(self, body):
        self.operations.append("audit")


class RetirementTests(unittest.TestCase):
    def test_exact_nine_canonical_names(self):
        self.assertEqual(CANONICAL_BRANCHES, set("main development security/current architecture/current feature/current health/current research/current ai-agent/current integration/current".split()))

    def test_tag_is_stable_and_branch_specific(self):
        self.assertEqual(archive_tag_name("feature/example", SHA), archive_tag_name("feature/example", SHA))
        self.assertNotEqual(archive_tag_name("feature/example", SHA), archive_tag_name("feature/other", SHA))

    def test_merged_same_repo_pr_to_main_is_candidate(self):
        self.assertEqual(candidate_for_event(make_event(), REPO)["branch"], "feature/example")

    def test_canonical_branch_never_candidate(self):
        self.assertTrue(all(candidate_for_event(make_event(branch=b), REPO) is None for b in CANONICAL_BRANCHES))

    def test_unmerged_fork_or_non_main_pr_is_skipped(self):
        self.assertIsNone(candidate_for_event(make_event(merged=False), REPO))
        self.assertIsNone(candidate_for_event(make_event(head_repo="external/fork"), REPO))
        self.assertIsNone(candidate_for_event(make_event(base="development"), REPO))

    def test_invalid_sha_or_ref_fails_closed(self):
        with self.assertRaises(ValueError):
            candidate_for_event(make_event(sha="bad"), REPO)
        with self.assertRaises(ValueError):
            candidate_for_event(make_event(branch="../main"), REPO)

    def test_archive_tag_precedes_branch_deletion(self):
        candidate = candidate_for_event(make_event(), REPO)
        client = FakeClient()
        tag = retire_merged_branch(client, candidate, REPO, "https://example/run")
        self.assertEqual(client.tags[tag], SHA)
        self.assertNotIn("feature/example", client.branches)
        self.assertLess(client.operations.index("create-tag"), client.operations.index("delete-branch"))

    def test_head_mismatch_preserves_branch(self):
        candidate = candidate_for_event(make_event(), REPO)
        client = FakeClient("b" * 40)
        with self.assertRaisesRegex(RuntimeError, "tip mismatch"):
            retire_merged_branch(client, candidate, REPO, "https://example/run")
        self.assertIn("feature/example", client.branches)
        self.assertEqual(client.tags, {})
        self.assertNotIn("delete-branch", client.operations)

    def test_open_pr_dependency_preserves_branch(self):
        candidate = candidate_for_event(make_event(), REPO)
        client = FakeClient()
        client.open_pr_heads.add("feature/example")
        with self.assertRaisesRegex(RuntimeError, "another open PR"):
            retire_merged_branch(client, candidate, REPO, "https://example/run")
        self.assertIn("feature/example", client.branches)
        self.assertNotIn("delete-branch", client.operations)

    def test_open_pr_base_dependency_preserves_branch(self):
        candidate = candidate_for_event(make_event(), REPO)
        client = FakeClient()
        client.open_pr_bases.add("feature/example")
        with self.assertRaisesRegex(RuntimeError, "another open PR"):
            retire_merged_branch(client, candidate, REPO, "https://example/run")
        self.assertIn("feature/example", client.branches)
        self.assertNotIn("delete-branch", client.operations)

    def test_protected_branch_preserves_branch(self):
        candidate = candidate_for_event(make_event(), REPO)
        client = FakeClient()
        client.branches["feature/example"]["protected"] = True
        with self.assertRaisesRegex(RuntimeError, "protection state"):
            retire_merged_branch(client, candidate, REPO, "https://example/run")
        self.assertEqual(client.tags, {})
        self.assertNotIn("delete-branch", client.operations)

    def test_all_recovery_tags_precede_any_batch_deletion(self):
        client = FakeClient()
        second_sha = "b" * 40
        client.branches["feature/second"] = {"sha": second_sha, "protected": False}
        candidates = [
            candidate_for_event(make_event(), REPO),
            {"branch": "feature/second", "head_sha": second_sha, "pr_number": 124},
        ]
        retire_batch(client, candidates, "https://example/run")
        last_tag = max(i for i, op in enumerate(client.operations) if op == "create-tag")
        first_delete = client.operations.index("delete-branch")
        self.assertLess(last_tag, first_delete)
        self.assertNotIn("feature/example", client.branches)
        self.assertNotIn("feature/second", client.branches)

    def test_batch_preflight_failure_deletes_nothing(self):
        client = FakeClient()
        second_sha = "b" * 40
        client.branches["feature/second"] = {"sha": "c" * 40, "protected": False}
        candidates = [
            candidate_for_event(make_event(), REPO),
            {"branch": "feature/second", "head_sha": second_sha, "pr_number": 124},
        ]
        with self.assertRaisesRegex(RuntimeError, "no branch refs deleted"):
            retire_batch(client, candidates, "https://example/run")
        self.assertIn("feature/example", client.branches)
        self.assertIn("feature/second", client.branches)
        self.assertNotIn("delete-branch", client.operations)

    def test_unmanifested_branch_aborts_before_archival_or_deletion(self):
        candidate = candidate_for_event(make_event(), REPO)
        client = FakeClient()
        client.branches["unreviewed/new-work"] = {"sha": "b" * 40, "protected": False}
        with self.assertRaisesRegex(RuntimeError, "unmanifested active branches"):
            retire_batch(client, [candidate], "https://example/run")
        self.assertEqual(client.tags, {})
        self.assertNotIn("delete-branch", client.operations)

    def test_archive_tag_collision_preserves_branch(self):
        candidate = candidate_for_event(make_event(), REPO)
        client = FakeClient()
        client.tags[archive_tag_name("feature/example", SHA)] = "b" * 40
        with self.assertRaisesRegex(RuntimeError, "recovery tag mismatch"):
            retire_merged_branch(client, candidate, REPO, "https://example/run")
        self.assertIn("feature/example", client.branches)
        self.assertNotIn("delete-branch", client.operations)


if __name__ == "__main__":
    unittest.main(verbosity=2)
