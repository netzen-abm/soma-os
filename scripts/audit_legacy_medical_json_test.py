import json
import subprocess
import sys
from pathlib import Path


SCRIPT = Path(__file__).with_name("audit_legacy_medical_json.py")


def run_audit(path: Path) -> dict:
    result = subprocess.run(
        [sys.executable, str(SCRIPT), str(path)],
        check=True,
        capture_output=True,
        text=True,
    )
    return json.loads(result.stdout)


def test_concatenated_json_is_detected(tmp_path: Path) -> None:
    source = tmp_path / "legacy.json"
    source.write_text(
        '{"database_version":"1.0"}\n{"database_version":"1.1"}\n',
        encoding="utf-8",
    )

    report = run_audit(source)

    assert report["document_count"] == 2
    assert report["valid_json_sequence"] is True


def test_risk_language_is_flagged(tmp_path: Path) -> None:
    source = tmp_path / "legacy.json"
    source.write_text(
        json.dumps({"claim": "This protocol can cure diabetes"}),
        encoding="utf-8",
    )

    report = run_audit(source)
    hits = report["documents"][0]["risk_language_hits"]
    terms = {hit["term"] for hit in hits}

    assert "cure" in terms


def test_source_fields_are_collected(tmp_path: Path) -> None:
    source = tmp_path / "legacy.json"
    source.write_text(
        json.dumps({"verification_url": "https://example.org/source"}),
        encoding="utf-8",
    )

    report = run_audit(source)
    values = report["documents"][0]["source_values"]

    assert values[0]["value"] == "https://example.org/source"
