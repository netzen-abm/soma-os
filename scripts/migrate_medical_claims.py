#!/usr/bin/env python3
"""Map legacy findings into conservative canonical draft records."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any
from urllib.parse import urlparse


DEFAULT_BOUNDARY = "INFORMATION_ONLY"


def build_sources(item: dict[str, Any]) -> list[dict[str, Any]]:
    """Preserve source references without asserting their credibility."""
    sources: list[dict[str, Any]] = []

    for position, value in enumerate(item.get("source_values", []), 1):
        url = str(value.get("value", ""))
        parsed = urlparse(url)
        if parsed.scheme not in {"http", "https"} or not parsed.netloc:
            continue

        sources.append(
            {
                "source_id": f"LEGACY-SOURCE-{item['document_index']:04d}-{position:02d}",
                "source_type": "LEGACY_UNCLASSIFIED",
                "title": "Legacy source reference",
                "publisher": None,
                "publication_date": None,
                "verification_url": url,
                "source_summary": None,
            }
        )

    return sources


def map_item(item: dict[str, Any]) -> dict[str, Any]:
    """Create a conservative canonical draft record."""
    index = item["document_index"]

    return {
        "claim_id": f"SOMA-LEGACY-{index:04d}",
        "claim": None,
        "knowledge_class": None,
        "evidence_status": "UNREVIEWED",
        "evidence_level": "NOT_ASSESSED",
        "population_context": None,
        "intervention_or_exposure": None,
        "outcome": None,
        "limitations": [
            "Legacy content has not been independently evidence-assessed."
        ],
        "uncertainty": [
            "Canonical fields require source and expert review."
        ],
        "sources": build_sources(item),
        "safety": {
            "status": "NOT_ASSESSED",
            "interaction_notes": [],
            "professional_review_required": True,
        },
        "management_boundary": DEFAULT_BOUNDARY,
        "prohibited_claims": [
            "Do not present as diagnosis, prescription, treatment, cure,"
            "reversal, or prevention without appropriate evidence and review."
        ],
        "review": {
            "review_state": "UNREVIEWED",
            "reviewer": None,
            "last_verified": None,
        },
        "migration_metadata": {
            "legacy_document_index": index,
            "risk_language_hits": item.get("risk_language_hits", []),
            "legacy_source_values": item.get("source_values", []),
            "publication_eligible": False,
        },
    }


def migrate_queue(queue: dict[str, Any]) -> dict[str, Any]:
    """Map a review queue without upgrading evidence or publication state."""
    return {
        "schema_version": "1.0.0",
        "migration_mode": "NON_PUBLISHING",
        "source_report": queue.get("source_report"),
        "records": [map_item(item) for item in queue.get("items", [])],
    }


def main() -> int:
    if len(sys.argv) != 3:
        print(
            "Usage: migrate_medical_claims.py "
            "<review-queue.json> <canonical-draft.json>"
        )
        return 2

    source = Path(sys.argv[1])
    destination = Path(sys.argv[2])
    queue = json.loads(source.read_text(encoding="utf-8"))
    canonical = migrate_queue(queue)

    destination.write_text(
        json.dumps(canonical, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
