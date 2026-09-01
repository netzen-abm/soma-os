#!/usr/bin/env python3
"""Extract explicit study fields without inferring missing evidence."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from typing import Any


FIELD_PATTERNS = {
    "sample_size": (
        r"\b(?:n|sample size|participants?|subjects?)\s*[:=]?\s*"
        r"(\d{1,7})\b"
    ),
    "duration_or_follow_up": (
        r"\b(\d+(?:\.\d+)?)\s*"
        r"(day|days|week|weeks|month|months|year|years)\b"
    ),
}

DESIGN_SIGNALS = (
    ("randomized", "RANDOMIZED_SIGNAL"),
    ("randomised", "RANDOMIZED_SIGNAL"),
    ("clinical trial", "CLINICAL_TRIAL_SIGNAL"),
    ("observational", "OBSERVATIONAL_SIGNAL"),
    ("systematic review", "SYSTEMATIC_REVIEW_SIGNAL"),
    ("meta-analysis", "META_ANALYSIS_SIGNAL"),
)


def extract_explicit_fields(source: dict[str, Any]) -> dict[str, Any]:
    """Extract only explicit textual signals supplied in source metadata."""
    text = " ".join(
        str(source.get(field, ""))
        for field in ("title", "source_summary")
    )
    result: dict[str, Any] = {
        "source_id": source.get("source_id"),
        "study_design": None,
        "population": None,
        "sample_size": None,
        "intervention_or_exposure": None,
        "comparator": None,
        "duration_or_follow_up": None,
        "primary_outcomes": [],
        "reported_effects": [],
        "adverse_events_or_safety": [],
        "authors_conclusion": None,
        "limitations": [],
        "extraction_status": "NOT_EXTRACTED",
        "extraction_method": "EXPLICIT_METADATA_ONLY",
        "source_span_required": True,
        "human_review_required": True,
    }

    lowered = text.lower()
    for phrase, signal in DESIGN_SIGNALS:
        if phrase in lowered:
            result["study_design"] = signal
            break

    for field, pattern in FIELD_PATTERNS.items():
        match = re.search(pattern, text, flags=re.IGNORECASE)
        if not match:
            continue

        if field == "sample_size":
            result[field] = int(match.group(1))
        else:
            result[field] = f"{match.group(1)} {match.group(2)}"

    extracted_fields = (
        result["study_design"],
        result["sample_size"],
        result["duration_or_follow_up"],
    )
    if any(value is not None for value in extracted_fields):
        result["extraction_status"] = "PARTIALLY_EXTRACTED"

    return result


def extract_document(document: dict[str, Any]) -> dict[str, Any]:
    records = []
    for record in document.get("records", []):
        records.append(
            {
                **record,
                "study_evidence": [
                    extract_explicit_fields(source)
                    for source in record.get("sources", [])
                ],
            }
        )

    return {
        **document,
        "study_extraction_mode": "EXPLICIT_METADATA_ONLY",
        "records": records,
    }


def main() -> int:
    if len(sys.argv) != 3:
        print(
            "Usage: extract_study_evidence.py "
            "<classified-draft.json> <study-draft.json>"
        )
        return 2

    source_path = Path(sys.argv[1])
    destination_path = Path(sys.argv[2])
    document = json.loads(source_path.read_text(encoding="utf-8"))

    extracted = extract_document(document)
    destination_path.write_text(
        json.dumps(extracted, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
