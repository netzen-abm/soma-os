#!/usr/bin/env python3
"""Unit tests for archive-tag visibility in the GitHub REST adapter."""
from __future__ import annotations

import unittest
from unittest.mock import Mock, patch

from github_branch_retirement import GitHubApi

REPO = "netzen-abm/soma-os"
SHA = "a" * 40


class GitHubArchiveTagVisibilityTests(unittest.TestCase):
    def client(self) -> GitHubApi:
        return GitHubApi(REPO, "test-token", "https://api.github.com")

    def test_delayed_visibility_retries_until_exact_sha_appears(self) -> None:
        client = self.client()
        client.tag_sha = Mock(side_effect=[None, None, SHA])
        with patch("github_branch_retirement.time.sleep") as sleep:
            self.assertEqual(client.wait_for_tag("archive-tag", SHA), SHA)
        self.assertEqual(client.tag_sha.call_count, 3)
        self.assertEqual([call.args[0] for call in sleep.call_args_list], [0.2, 0.4])

    def test_visible_wrong_sha_fails_without_retry(self) -> None:
        client = self.client()
        client.tag_sha = Mock(return_value="b" * 40)
        with patch("github_branch_retirement.time.sleep") as sleep:
            with self.assertRaisesRegex(RuntimeError, "recovery tag mismatch"):
                client.wait_for_tag("archive-tag", SHA)
        client.tag_sha.assert_called_once_with("archive-tag")
        sleep.assert_not_called()

    def test_never_visible_tag_fails_closed_after_bounded_retries(self) -> None:
        client = self.client()
        client.tag_sha = Mock(return_value=None)
        with patch("github_branch_retirement.time.sleep") as sleep:
            with self.assertRaisesRegex(RuntimeError, "did not become visible after 6 reads"):
                client.wait_for_tag("archive-tag", SHA)
        self.assertEqual(client.tag_sha.call_count, 6)
        self.assertEqual(sleep.call_count, 5)


if __name__ == "__main__":
    unittest.main(verbosity=2)
