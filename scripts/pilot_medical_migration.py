#!/usr/bin/env python3
"""Run the evidence pipeline on a small fixture without publishing data."""

from __future__ import annotations

import json
import sys
from pathlib import Path

from audit_legacy_medical_json import audit_document
from build_medical_review_queue import build_queue
from migrate_medical_claims import migrate_queue


def run_pilot(source_path: Path) -> dict:
    text = source_path.read_text(encoding="utf-8")
    document = json.loads(text)
    report = {
        "file": str(source_path),
        "document_count": 1,
        "documents": [audit_document(document, 1)],
    }
    queue = build_queue(report)
    canonical = migrate_queue(queue)

    for record in canonical["records"]:
        if record["evidence_status"] != "UNREVIEWED":
            raise ValueError("Pilot attempted to upgrade evidence status")
        if record["migration_metadata"]["publication_eligible"]:
            raise ValueError("Pilot produced a publishable record")

    return {
        "status": "PILOT_VALIDATED",
        "migration_mode": canonical["migration_mode"],
        "record_count": len(canonical["records"]),
        "publication_eligible_records": 0,
        "records": canonical["records"],
    }


def main() -> int:
    if len(sys.argv) != 2:
        print("Usage: pilot_medical_migration.py <fixture.json>")
        return 2

    result = run_pilot(Path(sys.argv[1]))
    print(json.dumps(result, ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
