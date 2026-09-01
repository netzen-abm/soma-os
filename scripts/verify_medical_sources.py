#!/usr/bin/env python3
"""Classify source URLs without upgrading medical evidence status."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from urllib.parse import urlparse
from urllib.request import Request, urlopen


TIMEOUT_SECONDS = 10


def classify_url(url: str) -> str:
    """Return a transport-level verification state for one URL."""
    if not url or not url.startswith(("http://", "https://")):
        return "INVALID"

    parsed = urlparse(url)
    if not parsed.netloc or "example." in parsed.netloc.lower():
        return "PLACEHOLDER"

    request = Request(
        url,
        method="HEAD",
        headers={"User-Agent": "SOMA-Evidence-Verifier/1.0"},
    )

    try:
        with urlopen(request, timeout=TIMEOUT_SECONDS) as response:
            status = getattr(response, "status", 200)
            return "VERIFIED" if 200 <= status < 400 else "UNAVAILABLE"
    except Exception:
        return "UNAVAILABLE"


def verify_sources(record: dict) -> dict:
    """Attach URL verification metadata without changing evidence status."""
    sources = []

    for source in record.get("sources", []):
        url = source.get("verification_url", "")
        sources.append(
            {
                **source,
                "verification_state": classify_url(url),
            }
        )

    return {
        **record,
        "sources": sources,
        "source_verification_performed": True,
    }


def main() -> int:
    if len(sys.argv) != 3:
        print(
            "Usage: verify_medical_sources.py "
            "<canonical-draft.json> <verified-draft.json>"
        )
        return 2

    source_path = Path(sys.argv[1])
    destination_path = Path(sys.argv[2])
    document = json.loads(source_path.read_text(encoding="utf-8"))

    verified = {
        **document,
        "verification_mode": "TRANSPORT_ONLY",
        "records": [verify_sources(record) for record in document.get("records", [])],
    }

    destination_path.write_text(
        json.dumps(verified, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
