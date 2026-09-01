# SOMA Medical Data Governance Standard

**Status:** Foundational
**Date:** 2026-08-31

## Purpose

Medical and health-related knowledge in SOMA must be represented as
traceable evidence objects, not as unqualified treatment claims.

## Mandatory fields for governed health claims

Each substantive health claim should be traceable to:

- claim identifier;
- source type;
- source title or study identifier;
- publisher or institution;
- publication date where available;
- verification URL;
- evidence level;
- population/context;
- intervention or exposure;
- outcome;
- limitations;
- uncertainty;
- safety/interactions;
- review status;
- last verification date.

## Source classes

SOMA should distinguish, rather than collapse:

1. government and regulatory sources;
2. systematic reviews and meta-analyses;
3. randomized or controlled clinical studies;
4. observational studies;
5. mechanistic and laboratory studies;
6. traditional knowledge and classical sources;
7. institutional clinical guidance;
8. open datasets and repositories;
9. expert or educational material.

No single source class is automatically sufficient for every question.

## Management boundary

A health-management protocol in SOMA is an information object describing a
potential management approach and its supporting evidence. It is not a claim
that the approach treats, cures, reverses, or prevents a disease.

Where evidence is absent or inadequate, the protocol may still be represented
as a clearly labelled traditional, exploratory, or informational framework,
but it must state that supporting clinical evidence was not established.

## Study linkage

A protocol may cite multiple studies. Each study should be independently
identifiable and verifiable.

A source link must not be fabricated. If a reliable verification URL cannot
be established, the record should explicitly state that verification is
unavailable rather than inventing a destination.

## Safety

Interaction data must not prescribe dose changes, timing buffers, medication
cessation, or treatment changes unless the source and clinical governance
process explicitly support that instruction.

Safety alerts should direct the user to an appropriate qualified professional.

## Traditional knowledge

Traditional practices may be preserved as knowledge and cultural context.
They must be labelled as traditional or historical knowledge when clinical
evidence has not established the relevant health outcome.

## Data quality gates

A health record fails publication readiness if it contains:

- unsupported cure or treatment claims;
- fabricated or placeholder source URLs;
- contradictory versions concatenated into one JSON document;
- clinical instructions without provenance;
- unexplained evidence strength;
- claims presented as government-endorsed without evidence;
- speculative mechanisms presented as established clinical facts.

## Migration rule

Legacy medical datasets must be migrated incrementally.

Do not overwrite or delete the legacy record until:

1. the complete source has been preserved;
2. the new record has been validated;
3. all fields have been mapped or explicitly marked unknown;
4. consumers have been checked;
5. tests pass;
6. the migration is documented.

## Review states

Use these states:

- `UNREVIEWED`
- `SOURCE_FOUND`
- `SOURCE_VERIFIED`
- `EVIDENCE_ASSESSED`
- `SAFETY_REVIEWED`
- `PUBLISHED`
- `WITHDRAWN`

`UNREVIEWED` content must not be presented as verified clinical evidence.
