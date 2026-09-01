import json
import subprocess
import sys
from pathlib import Path


SCRIPT = Path(__file__).with_name("migrate_medical_claims.py")


def run_migration(tmp_path: Path, queue: dict) -> dict:
    source = tmp_path / "queue.json"
    destination = tmp_path / "canonical.json"
    source.write_text(json.dumps(queue), encoding="utf-8")

    subprocess.run(
        [sys.executable, str(SCRIPT), str(source), str(destination)],
        check=True,
    )

    return json.loads(destination.read_text(encoding="utf-8"))


def test_migration_never_publishes(tmp_path: Path) -> None:
    queue = {
        "source_report": "legacy.json",
        "items": [
            {
                "document_index": 1,
                "risk_language_hits": [],
                "source_values": [],
            }
        ],
    }

    result = run_migration(tmp_path, queue)
    record = result["records"][0]

    assert result["migration_mode"] == "NON_PUBLISHING"
    assert record["evidence_status"] == "UNREVIEWED"
    assert record["evidence_level"] == "NOT_ASSESSED"
    assert record["review"]["review_state"] == "UNREVIEWED"
    assert record["review"]["last_verified"] is None
    assert record["sources"] == []
    assert record["migration_metadata"]["publication_eligible"] is False


def test_migration_preserves_risk_and_legacy_sources(tmp_path: Path) -> None:
    queue = {
        "source_report": "legacy.json",
        "items": [
            {
                "document_index": 7,
                "risk_language_hits": [{"term": "cure"}],
                "source_values": [
                    {
                        "path": "$.source",
                        "value": "https://example.org/source",
                    }
                ],
            }
        ],
    }

    result = run_migration(tmp_path, queue)
    record = result["records"][0]

    assert record["migration_metadata"]["risk_language_hits"]
    assert record["migration_metadata"]["legacy_source_values"]
    assert record["sources"][0]["verification_url"] == (
        "https://example.org/source"
    )
    assert record["sources"][0]["source_type"] == "LEGACY_UNCLASSIFIED"
    assert record["management_boundary"] == "INFORMATION_ONLY"


def test_invalid_source_reference_is_not_promoted(tmp_path: Path) -> None:
    queue = {
        "source_report": "legacy.json",
        "items": [
            {
                "document_index": 8,
                "risk_language_hits": [],
                "source_values": [
                    {"path": "$.source", "value": "not-a-url"},
                ],
            }
        ],
    }

    result = run_migration(tmp_path, queue)
    record = result["records"][0]

    assert record["sources"] == []
    assert record["evidence_status"] == "UNREVIEWED"
    assert record["migration_metadata"]["publication_eligible"] is False


def test_migration_output_is_json_serializable(tmp_path: Path) -> None:
    queue = {
        "source_report": "legacy.json",
        "items": [
            {
                "document_index": 9,
                "risk_language_hits": [{"term": "cure"}],
                "source_values": [],
            }
        ],
    }

    result = run_migration(tmp_path, queue)
    json.dumps(result)
