#!/usr/bin/env python3
"""Classify source records without asserting evidence quality."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from urllib.parse import urlparse


DOMAIN_TYPES = {
    "who.int": "GOVERNMENT",
    "ayush.gov.in": "GOVERNMENT",
    "mohfw.gov.in": "GOVERNMENT",
    "fssai.gov.in": "REGULATORY",
    "icmr.gov.in": "GOVERNMENT",
    "pubmed.ncbi.nlm.nih.gov": "CONTROLLED_STUDY",
    "cochranelibrary.com": "SYSTEMATIC_REVIEW",
}

TEXT_PATTERNS = (
    ("systematic review", "SYSTEMATIC_REVIEW"),
    ("meta-analysis", "META_ANALYSIS"),
    ("randomized", "CONTROLLED_STUDY"),
    ("randomised", "CONTROLLED_STUDY"),
    ("clinical trial", "CONTROLLED_STUDY"),
    ("observational", "OBSERVATIONAL_STUDY"),
    ("traditional", "TRADITIONAL_SOURCE"),
    ("guideline", "CLINICAL_GUIDANCE"),
)


def normalize_host(host: str) -> str:
    host = host.lower().split(":", 1)[0]
    return host.removeprefix("www.")


def classify_source(source: dict) -> str:
    """Return a conservative source class, never an evidence grade."""
    url = source.get("verification_url", "")
    host = normalize_host(urlparse(url).netloc)

    for domain, source_type in DOMAIN_TYPES.items():
        if host == domain or host.endswith("." + domain):
            return source_type

    searchable = " ".join(
        str(source.get(field, ""))
        for field in ("title", "source_summary", "publisher")
    ).lower()

    for pattern, source_type in TEXT_PATTERNS:
        if re.search(re.escape(pattern), searchable):
            return source_type

    return "EXPERT_EDUCATION"


def classify_record(record: dict) -> dict:
    """Attach source classes without changing evidence status."""
    classified_sources = []

    for source in record.get("sources", []):
        classified_sources.append(
            {
                **source,
                "source_type_inferred": classify_source(source),
                "source_type_confidence": "PROVISIONAL",
            }
        )

    return {**record, "sources": classified_sources}


def main() -> int:
    if len(sys.argv) != 3:
        print(
            "Usage: classify_medical_sources.py "
            "<canonical-draft.json> <classified-draft.json>"
        )
        return 2

    source_path = Path(sys.argv[1])
    destination_path = Path(sys.argv[2])
    document = json.loads(source_path.read_text(encoding="utf-8"))

    classified = {
        **document,
        "source_classification_mode": "PROVISIONAL",
        "records": [
            classify_record(record)
            for record in document.get("records", [])
        ],
    }

    destination_path.write_text(
        json.dumps(classified, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
