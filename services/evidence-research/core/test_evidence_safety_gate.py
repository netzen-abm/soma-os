from __future__ import annotations

import sys
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).parent))

from evidence_safety_gate import EvidenceSafetyAssessment, EvidenceSafetyGate  # noqa: E402


def _assessment(ids: tuple[str, ...], status: str = "REQUIRES_HUMAN_REVIEW") -> EvidenceSafetyAssessment:
    return EvidenceSafetyAssessment(
        status=status,
        evidence_ids=ids,
        safety_findings=("Safety information requires explicit review.",),
        rationale="Explicit bounded safety assessment.",
        assessment_method="human_review",
        assessed_at="2026-09-15T00:00:00Z",
    )


def test_assessment_rejects_unknown_status() -> None:
    with pytest.raises(ValueError, match="invalid safety status"):
        _assessment(("pubmed:1",), "CAUSAL")


def test_assessment_requires_explicit_evidence_and_method() -> None:
    with pytest.raises(ValueError, match="at least one evidence record"):
        _assessment(())


def test_gate_requires_matching_synthesized_evidence() -> None:
    with pytest.raises(ValueError, match="requires synthesized evidence"):
        EvidenceSafetyGate.apply(object(), _assessment(("pubmed:1",)))


def test_gate_requires_exact_evidence_identity() -> None:
    from evidence_synthesis import SynthesizedEvidence

    synthesized = SynthesizedEvidence(evidence=(), assessment=_assessment(("pubmed:1",)))
    with pytest.raises(ValueError, match="must match synthesized evidence"):
        EvidenceSafetyGate.apply(synthesized, _assessment(("pubmed:2",)))
