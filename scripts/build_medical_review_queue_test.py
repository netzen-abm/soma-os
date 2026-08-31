import json
import subprocess
import sys
from pathlib import Path


SCRIPT = Path(__file__).with_name("build_medical_review_queue.py")


def test_queue_keeps_every_item_unreviewed(tmp_path: Path) -> None:
    report = {
        "file": "legacy.json",
        "documents": [
            {
                "document_index": 1,
                "risk_language_hits": [],
                "source_values": [
                    {
                        "path": "$.source",
                        "value": "https://example.org/source",
                    }
                ],
            }
        ],
    }
    source = tmp_path / "report.json"
    destination = tmp_path / "queue.json"
    source.write_text(json.dumps(report), encoding="utf-8")

    subprocess.run(
        [sys.executable, str(SCRIPT), str(source), str(destination)],
        check=True,
    )

    queue = json.loads(destination.read_text(encoding="utf-8"))
    item = queue["items"][0]

    assert item["review_state"] == "UNREVIEWED"
    assert item["evidence_status"] == "UNREVIEWED"
    assert item["publication_eligible"] is False
    assert item["management_boundary"] == "INFORMATION_ONLY"


def test_risk_findings_require_review(tmp_path: Path) -> None:
    report = {
        "file": "legacy.json",
        "documents": [
            {
                "document_index": 2,
                "risk_language_hits": [{"term": "cure"}],
                "source_values": [],
            }
        ],
    }
    source = tmp_path / "report.json"
    destination = tmp_path / "queue.json"
    source.write_text(json.dumps(report), encoding="utf-8")

    subprocess.run(
        [sys.executable, str(SCRIPT), str(source), str(destination)],
        check=True,
    )

    queue = json.loads(destination.read_text(encoding="utf-8"))
    item = queue["items"][0]

    assert item["requires_review"] is True
    assert "Verify every source independently." in item["review_actions"]
