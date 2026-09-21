#!/usr/bin/env python3
"""Validate the local Git view against SOMA's canonical nine-branch policy.

This is a non-destructive audit tool. It never deletes, force-moves, or merges
branches. Run it from a clone that has fetched the remote refs.
"""

from __future__ import annotations

import subprocess
import sys

CANONICAL = {
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


def remote_branches() -> set[str]:
    raw = subprocess.check_output(
        ["git", "for-each-ref", "--format=%(refname:strip=3)", "refs/remotes/origin"],
        text=True,
    )
    return {
        line.strip()
        for line in raw.splitlines()
        if line.strip() and line.strip() != "HEAD" and not line.strip().endswith("/HEAD")
    }


def main() -> int:
    branches = remote_branches()
    missing = sorted(CANONICAL - branches)
    extra = sorted(branches - CANONICAL)

    print(f"canonical={len(CANONICAL)} remote={len(branches)}")
    if missing:
        print("MISSING:")
        for name in missing:
            print(f"  {name}")
    if extra:
        print("NON_CANONICAL:")
        for name in extra:
            print(f"  {name}")

    if missing or extra:
        return 1

    print("PASS: repository exposes exactly the nine canonical active branches.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
