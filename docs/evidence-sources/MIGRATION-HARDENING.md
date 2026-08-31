# Evidence Migration Hardening

## Engineering decision

Legacy medical content is migrated into a **non-publishing canonical draft**. Legacy source references are preserved as draft source objects so later verification, classification, provenance, and evidence assessment cannot silently lose provenance.

## Invariants

1. Migration never upgrades `evidence_status`.
2. Migration never makes a record publication-eligible.
3. Legacy source URLs are preserved when valid HTTP(S) references exist.
4. Duplicate source URLs are deduplicated within a migrated record.
5. Risk-language findings remain attached to the migration metadata.
6. Missing information remains unknown; the migration does not infer clinical facts.

## Pipeline

`legacy JSON → read-only audit → review queue → canonical draft → source verification → source classification → study extraction → governed evidence assessment → safety review → publication gate`

The publication gate remains the authoritative boundary: only governed `PUBLISHED` records may become user-facing SOMA knowledge.
