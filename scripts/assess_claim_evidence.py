#!/usr/bin/env python3
"""Assess claim-source alignment without assigning clinical efficacy."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from typing import Any


STRONG_DESIGN_TYPES = {
    "SYSTEMATIC_REVIEW",
    "META_ANALYSIS",
    "CONTROLLED_STUDY",
}

WEAK_DESIGN_TYPES = {
    "MECHANISTIC_STUDY",
    "TRADITIONAL_SOURCE",
    "EXPERT_EDUCATION",
}



def normalize(text: str) -> set[str]:
    """Create a small normalized term set for conservative overlap checks."""
    words = re.findall(r"[a-z0-9]{4,}", text.lower())
    stop = {
        "about", "after", "among", "been", "being", "from", "into",
        "that", "their", "there", "these", "this", "with", "study",
        "health", "clinical", "research", "information",
    }
    return {word for word in words if word not in stop}



def assess_record(record: dict[str, Any]) -> dict[str, Any]:
    claim = record.get("claim") or ""
    claim_terms = normalize(claim)
    assessments: list[dict[str, Any]] = []

    for source in record.get("sources", []):
        source_text = " ".join(
            str(source.get(field, ""))
            for field in ("title", "source_summary", "publisher")
        )
        source_terms = normalize(source_text)
        overlap = sorted(claim_terms & source_terms)
        source_type = source.get(
            "source_type_inferred",
            source.get("source_type", "UNKNOWN"),
        )

        if not claim_terms:
            alignment = "NOT_ASSESSABLE"
        elif not source_terms:
            alignment = "INSUFFICIENT_SOURCE_DETAIL"
        elif len(overlap) >= 2:
            alignment = "POTENTIAL_ALIGNMENT"
        else:
            alignment = "NO_CLEAR_ALIGNMENT"

        assessments.append(
            {
                "source_id": source.get("source_id"),
                "source_type": source_type,
                "claim_source_alignment": alignment,
                "overlap_terms": overlap,
                "design_signal": _design_signal(source_type),
                "requires_manual_review": True,
            }
        )

    return {
        **record,
        "evidence_assessment": {
            "status": "PRELIMINARY",
            "clinical_efficacy_not_determined": True,
            "source_assessments": assessments,
            "manual_review_required": True,
        },
        "evidence_status": "UNREVIEWED",
        "evidence_level": "NOT_ASSESSED",
    }



def _design_signal(source_type: str) -> str:
    if source_type in STRONG_DESIGN_TYPES:
        return "HIGHER_DESIGN_SIGNAL"
    if source_type in WEAK_DESIGN_TYPES:
        return "LIMITED_DESIGN_SIGNAL"
    return "UNKNOWN_DESIGN_SIGNAL"



def main() -> int:
    if len(sys.argv) != 3:
        print(
            "Usage: assess_claim_evidence.py "
            "<classified-draft.json> <assessed-draft.json>"
        )
        return 2

    source_path = Path(sys.argv[1])
    destination_path = Path(sys.argv[2])
    document = json.loads(source_path.read_text(encoding="utf-8"))

    assessed = {
        **document,
        "evidence_assessment_mode": "PRELIMINARY_MANUAL_REVIEW_REQUIRED",
        "records": [
            assess_record(record)
            for record in document.get("records", [])
        ],
    }

    destination_path.write_text(
        json.dumps(assessed, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
