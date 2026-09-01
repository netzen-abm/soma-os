#!/usr/bin/env python3
"""Build a review queue from the read-only legacy medical audit report."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any



def build_queue(report: dict[str, Any]) -> dict[str, Any]:
    """Convert audit findings into explicit, non-publishing review items."""
    items: list[dict[str, Any]] = []

    for document in report.get("documents", []):
        risk_hits = document.get("risk_language_hits", [])
        sources = document.get("source_values", [])
        requires_review = bool(risk_hits or not sources)

        items.append(
            {
                "review_id": f"LEGACY-DOC-{document['document_index']:04d}",
                "document_index": document["document_index"],
                "review_state": "UNREVIEWED",
                "requires_review": requires_review,
                "risk_language_hits": risk_hits,
                "source_values": sources,
                "publication_eligible": False,
                "evidence_status": "UNREVIEWED",
                "management_boundary": "INFORMATION_ONLY",
                "review_actions": [
                    "Verify every source independently.",
                    "Classify the knowledge type.",
                    "Assess evidence strength and limitations.",
                    "Review safety and interactions.",
                    "Rewrite unsupported treatment or cure language.",
                ],
            }
        )

    return {
        "schema_version": "1.0.0",
        "source_report": report.get("file"),
        "publication_policy": "REVIEW_REQUIRED",
        "items": items,
    }


def main() -> int:
    if len(sys.argv) != 3:
        print(
            "Usage: build_medical_review_queue.py "
            "<audit-report.json> <review-queue.json>"
        )
        return 2

    source = Path(sys.argv[1])
    destination = Path(sys.argv[2])
    report = json.loads(source.read_text(encoding="utf-8"))
    queue = build_queue(report)

    destination.write_text(
        json.dumps(queue, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
