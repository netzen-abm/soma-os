#!/usr/bin/env python3
"""Validation tests for the reviewed SHA-pinned branch-retirement manifest."""
import unittest
from retire_merged_branch import CANONICAL_BRANCHES, manifest_candidates

SHA = "a" * 40


def make_manifest(entries=None, canonical=None):
    return {"schema_version": "1.0.0", "inventory_main_sha": SHA,
        "canonical_branches": sorted(canonical or CANONICAL_BRANCHES),
        "entries": entries if entries is not None else [{
            "branch": "feature/old-line", "expected_sha": SHA,
            "disposition": "PRESERVE_THEN_DELETE",
            "reason": "Preserve this exact history before retiring its active ref.",
            "evidence": "docs/governance/branch-retirement-register-v3.md"}]}


class ManifestTests(unittest.TestCase):
    def test_valid_entry(self):
        result = manifest_candidates(make_manifest(), 178)
        self.assertEqual((len(result), result[0]["branch"], result[0]["head_sha"]), (1, "feature/old-line", SHA))

    def test_requires_exact_nine_canonical_branches(self):
        with self.assertRaisesRegex(ValueError, "canonical branch set"):
            manifest_candidates(make_manifest(canonical=["main"]), 178)

    def test_cannot_retire_canonical_branch(self):
        entry = make_manifest()["entries"][0] | {"branch": "main"}
        with self.assertRaisesRegex(ValueError, "canonical branch"):
            manifest_candidates(make_manifest(entries=[entry]), 178)

    def test_rejects_bad_sha_and_disposition(self):
        entry = make_manifest()["entries"][0] | {"expected_sha": "bad"}
        with self.assertRaisesRegex(ValueError, "expected SHA"):
            manifest_candidates(make_manifest(entries=[entry]), 178)
        entry = make_manifest()["entries"][0] | {"disposition": "DELETE_NOW"}
        with self.assertRaisesRegex(ValueError, "disposition"):
            manifest_candidates(make_manifest(entries=[entry]), 178)

    def test_rejects_duplicate_branches(self):
        entry = make_manifest()["entries"][0]
        with self.assertRaisesRegex(ValueError, "duplicate branch"):
            manifest_candidates(make_manifest(entries=[entry, entry]), 178)

    def test_requires_reason_and_evidence(self):
        entry = make_manifest()["entries"][0] | {"reason": "", "evidence": ""}
        with self.assertRaisesRegex(ValueError, "retirement reason"):
            manifest_candidates(make_manifest(entries=[entry]), 178)


if __name__ == "__main__":
    unittest.main(verbosity=2)
