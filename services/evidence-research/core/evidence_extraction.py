"""Bounded structured evidence extraction boundary for SOMA.

The boundary applies explicitly supplied, source-grounded study facts to a
screened evidence candidate. It does not invent facts, infer missing values,
or perform evidence assessment.
"""
from __future__ import annotations

from dataclasses import dataclass
from typing import Mapping

from evidence_research_core import EvidenceCandidate
from evidence_screening import ScreenedEvidenceCandidate


@dataclass(frozen=True)
class EvidenceExtraction:
    """Explicitly supplied study facts extracted from a source."""

    title: str | None = None
    authors: tuple[str, ...] = ()
    year: int | None = None
    journal: str | None = None
    study_type: str | None = None
    registration_id: str | None = None
    doi: str | None = None
    pubmed_id: str | None = None
    population: str | None = None
    sample_size: int | None = None
    intervention: str | None = None
    comparator: str | None = None
    duration: str | None = None
    outcomes: tuple[str, ...] = ()
    effect_estimates: tuple[str, ...] = ()
    main_findings: str | None = None
    limitations: tuple[str, ...] = ()
    risk_of_bias: str | None = None
    funding: str | None = None
    conflicts_of_interest: str | None = None
    safety_findings: tuple[str, ...] = ()


@dataclass(frozen=True)
class StructuredEvidenceRecord:
    """Canonical candidate plus explicit source-grounded extraction."""

    candidate: EvidenceCandidate
    extraction: EvidenceExtraction


class StructuredEvidenceExtractionBoundary:
    """Apply an explicit extraction without interpreting source content."""

    @staticmethod
    def _text(value: object, field: str) -> str | None:
        if value is None:
            return None
        if not isinstance(value, str):
            raise ValueError(f"{field} must be a string or null")
        value = value.strip()
        return value or None

    @classmethod
    def from_mapping(
        cls,
        screened: ScreenedEvidenceCandidate,
        fields: Mapping[str, object],
    ) -> StructuredEvidenceRecord:
        """Create a record from explicitly supplied source-grounded fields.

        This method deliberately performs shape validation only. It never
        extracts facts from prose, fills missing values, or assigns meaning.
        """
        if screened.candidate.screening_status != "INCLUDED":
            raise ValueError("structured extraction requires an INCLUDED candidate")

        allowed = {
            "title", "authors", "year", "journal", "study_type",
            "registration_id", "doi", "pubmed_id", "population", "sample_size",
            "intervention", "comparator", "duration", "outcomes",
            "effect_estimates", "main_findings", "limitations", "risk_of_bias",
            "funding", "conflicts_of_interest", "safety_findings",
        }
        unknown = set(fields) - allowed
        if unknown:
            raise ValueError(f"unknown extraction fields: {sorted(unknown)}")

        def texts(name: str) -> tuple[str, ...]:
            value = fields.get(name, ())
            if value is None:
                return ()
            if isinstance(value, str) or not isinstance(value, (tuple, list)):
                raise ValueError(f"{name} must be a sequence of strings")
            result = tuple(item.strip() for item in value)
            if any(not item for item in result):
                raise ValueError(f"{name} cannot contain empty values")
            return result

        year = fields.get("year")
        if year is not None and (not isinstance(year, int) or isinstance(year, bool)):
            raise ValueError("year must be an integer or null")
        sample_size = fields.get("sample_size")
        if sample_size is not None and (
            not isinstance(sample_size, int) or isinstance(sample_size, bool) or sample_size < 0
        ):
            raise ValueError("sample_size must be a non-negative integer or null")

        extraction = EvidenceExtraction(
            title=cls._text(fields.get("title"), "title"),
            authors=texts("authors"),
            year=year,
            journal=cls._text(fields.get("journal"), "journal"),
            study_type=cls._text(fields.get("study_type"), "study_type"),
            registration_id=cls._text(fields.get("registration_id"), "registration_id"),
            doi=cls._text(fields.get("doi"), "doi"),
            pubmed_id=cls._text(fields.get("pubmed_id"), "pubmed_id"),
            population=cls._text(fields.get("population"), "population"),
            sample_size=sample_size,
            intervention=cls._text(fields.get("intervention"), "intervention"),
            comparator=cls._text(fields.get("comparator"), "comparator"),
            duration=cls._text(fields.get("duration"), "duration"),
            outcomes=texts("outcomes"),
            effect_estimates=texts("effect_estimates"),
            main_findings=cls._text(fields.get("main_findings"), "main_findings"),
            limitations=texts("limitations"),
            risk_of_bias=cls._text(fields.get("risk_of_bias"), "risk_of_bias"),
            funding=cls._text(fields.get("funding"), "funding"),
            conflicts_of_interest=cls._text(fields.get("conflicts_of_interest"), "conflicts_of_interest"),
            safety_findings=texts("safety_findings"),
        )
        return StructuredEvidenceRecord(candidate=screened.candidate, extraction=extraction)
