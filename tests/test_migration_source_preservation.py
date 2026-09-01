import json
import subprocess
import sys
from pathlib import Path

SCRIPT = Path(__file__).parents[1] / "scripts" / "migrate_medical_claims.py"


def run_migration(tmp_path: Path, queue: dict) -> dict:
    source = tmp_path / "queue.json"
    destination = tmp_path / "canonical.json"
    source.write_text(json.dumps(queue), encoding="utf-8")
    subprocess.run([sys.executable, str(SCRIPT), str(source), str(destination)], check=True)
    return json.loads(destination.read_text(encoding="utf-8"))


def test_migration_never_publishes(tmp_path: Path) -> None:
    result = run_migration(tmp_path, {"source_report": "legacy.json", "items": [{"document_index": 1, "risk_language_hits": [], "source_values": []}]})
    record = result["records"][0]
    assert result["migration_mode"] == "NON_PUBLISHING"
    assert record["evidence_status"] == "UNREVIEWED"
    assert record["evidence_level"] == "NOT_ASSESSED"
    assert record["review"]["review_state"] == "UNREVIEWED"
    assert record["migration_metadata"]["publication_eligible"] is False


def test_migration_preserves_legacy_source_as_draft_source(tmp_path: Path) -> None:
    url = "https://example.org/source"
    result = run_migration(tmp_path, {"source_report": "legacy.json", "items": [{"document_index": 7, "risk_language_hits": [{"term": "cure"}], "source_values": [{"path": "$.source", "value": url}]}]})
    record = result["records"][0]
    assert len(record["sources"]) == 1
    assert record["sources"][0]["verification_url"] == url
    assert record["migration_metadata"]["legacy_source_values"]
    assert record["evidence_status"] == "UNREVIEWED"
    assert record["migration_metadata"]["publication_eligible"] is False


def test_duplicate_source_urls_are_deduplicated(tmp_path: Path) -> None:
    url = "https://example.org/source"
    result = run_migration(tmp_path, {"source_report": "legacy.json", "items": [{"document_index": 2, "source_values": [{"value": url}, {"value": url}]}]})
    assert len(result["records"][0]["sources"]) == 1
