#!/usr/bin/env python3
"""Audit legacy medical JSON without modifying source data."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from typing import Any


RISK_TERMS = (
    "cure",
    "treat",
    "reverse",
    "prevent",
    "halt",
    "replace",
    "stagger",
    "diagnos",
    "prescrib",
)

SOURCE_FIELDS = (
    "source",
    "source_url",
    "verification_url",
    "reference",
    "reference_url",
    "open_data_repository_url",
)


def extract_json_documents(text: str) -> list[Any]:
    """Decode concatenated JSON documents from legacy text."""
    decoder = json.JSONDecoder()
    documents: list[Any] = []
    position = 0

    while position < len(text):
        while position < len(text) and text[position].isspace():
            position += 1
        if position >= len(text):
            break

        document, end = decoder.raw_decode(text, position)
        documents.append(document)
        position = end

    return documents


def walk(value: Any, path: str = "$") -> list[tuple[str, str]]:
    """Return string fields with their JSON paths."""
    found: list[tuple[str, str]] = []

    if isinstance(value, dict):
        for key, item in value.items():
            child_path = f"{path}.{key}"
            found.extend(walk(item, child_path))
    elif isinstance(value, list):
        for index, item in enumerate(value):
            found.extend(walk(item, f"{path}[{index}]"))
    elif isinstance(value, str):
        found.append((path, value))

    return found


def audit_document(document: Any, index: int) -> dict[str, Any]:
    strings = walk(document)
    risk_hits: list[dict[str, str]] = []
    source_values: list[dict[str, str]] = []

    for path, value in strings:
        lowered = value.lower()
        for term in RISK_TERMS:
            if term in lowered:
                risk_hits.append({"path": path, "term": term})

        if any(field in path.lower() for field in SOURCE_FIELDS):
            source_values.append({"path": path, "value": value})

    return {
        "document_index": index,
        "root_type": type(document).__name__,
        "risk_language_hits": risk_hits,
        "source_values": source_values,
    }


def main() -> int:
    if len(sys.argv) != 2:
        print("Usage: audit_legacy_medical_json.py <path>")
        return 2

    path = Path(sys.argv[1])
    text = path.read_text(encoding="utf-8")

    documents = extract_json_documents(text)
    report = {
        "file": str(path),
        "document_count": len(documents),
        "valid_json_sequence": bool(documents),
        "documents": [
            audit_document(document, index)
            for index, document in enumerate(documents, start=1)
        ],
        "notes": [
            "Audit only: source file is never modified.",
            "Risk-language hits require human/evidence review.",
            "A source URL is not treated as verified merely because it exists.",
        ],
    }

    print(json.dumps(report, ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
