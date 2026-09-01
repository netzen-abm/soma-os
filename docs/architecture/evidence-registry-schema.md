# SOMA Evidence Registry Schema

## Purpose

Canonical shared schema for source-derived knowledge, claims, disease-management protocols, research studies, evidence assessments, safety findings, and user-verifiable references.

## Entity graph

```text
EvidenceSource
   ↓
Claim
   ↓
Protocol
   ├── Ingredient / Practice
   ├── SafetyRecord
   └── ResearchEvidence
           ↓
     EvidenceAssessment
           ↓
     VerificationReference
```

## 1. EvidenceSource

```yaml
source_id: string
source_type: enum
canonical_title: string
publisher: string
author: string|null
language: string|null
publication_date: date|null
version: string|null
canonical_url: string
accessed_at: datetime
license_status: enum
provenance_level: enum
notes: string|null
```

`source_type` examples:

- `GOVERNMENT_REPOSITORY`
- `WHO_RESOURCE`
- `REGULATORY_SOURCE`
- `PEER_REVIEWED_PUBLICATION`
- `CLINICAL_GUIDELINE`
- `SYSTEMATIC_REVIEW`
- `BOOK`
- `PRACTITIONER_PROTOCOL`
- `TRADITIONAL_TEXT`
- `PUBLISHER_WEBSITE`
- `DATASET`

## 2. Claim

```yaml
claim_id: string
source_id: string
source_location: string|null
subject_id: string
predicate: string
object_id: string|null
object_text: string|null
claim_type: enum
verbatim_excerpt: string|null
paraphrase: string
reported_by: string|null
extracted_at: datetime
review_status: enum
```

`claim_type` examples:

- `TRADITIONAL_USE_CLAIM`
- `NUTRITIONAL_COMPOSITION_CLAIM`
- `PREPARATION_CLAIM`
- `LIFESTYLE_CLAIM`
- `MECHANISTIC_CLAIM`
- `DISEASE_ASSOCIATION_CLAIM`
- `THERAPEUTIC_EFFICACY_CLAIM`
- `SAFETY_CLAIM`
- `TESTIMONIAL`

A claim records what a source says. It does not establish truth by itself.

## 3. Protocol

```yaml
protocol_id: string
source_id: string
name: string
reported_indication: string
protocol_class: enum
purpose_statement: string
ingredients: []
practices: []
preparation: string|null
administration: string|null
schedule: string|null
duration: string|null
exclusions: []
source_claim_refs: []
safety_refs: []
research_refs: []
recommendation_status: enum
review_status: enum
```

`protocol_class` should include:

- `DISEASE_MANAGEMENT`
- `SUPPORTIVE_LIFESTYLE`
- `NUTRITIONAL_MANAGEMENT`
- `TRADITIONAL_PRACTICE`
- `RESEARCH_PROTOCOL`

`recommendation_status` must support:

- `NOT_A_REMEDY`
- `NOT_A_CURE`
- `INFORMATIONAL_MANAGEMENT_ONLY`
- `PROFESSIONAL_REVIEW_REQUIRED`
- `BLOCKED_HIGH_RISK`

The default is `INFORMATIONAL_MANAGEMENT_ONLY` plus `PROFESSIONAL_REVIEW_REQUIRED` where applicable.

## 4. ResearchEvidence

```yaml
research_id: string
source_id: string
title: string
authors: []
year: integer|null
journal: string|null
doi: string|null
pubmed_id: string|null
source_url: string
verification_url: string
study_type: enum
population: string|null
sample_size: integer|null
intervention: string|null
comparator: string|null
duration: string|null
outcomes: []
main_findings: string
limitations: string|null
safety_findings: string|null
funding: string|null
conflicts_of_interest: string|null
search_context_id: string|null
```

## 5. EvidenceAssessment

```yaml
assessment_id: string
protocol_id: string
research_id: string|null
directness: enum
study_quality: enum
evidence_consistency: enum
precision: enum
external_validity: enum
safety_certainty: enum
overall_status: enum
supporting_or_contradictory: enum
assessor: string|null
assessed_at: datetime
rationale: string
```

`directness`:

- `DIRECT_EXACT_PROTOCOL`
- `DIRECT_COMPONENT`
- `RELATED_INTERVENTION`
- `MECHANISTIC`
- `OBSERVATIONAL_ASSOCIATION`
- `PRECLINICAL`
- `INDIRECT`
- `NO_RELEVANT_EVIDENCE_FOUND`

`supporting_or_contradictory`:

- `SUPPORTING`
- `NULL_OR_MIXED`
- `CONTRADICTORY`
- `CONTEXT_ONLY`

## 6. SafetyRecord

```yaml
safety_id: string
subject_id: string
protocol_id: string|null
population: string|null
contraindications: []
interactions: []
adverse_events: []
dose_context: string|null
contamination_risk: string|null
identity_risk: string|null
regulatory_status: string|null
source_refs: []
confidence: enum
reviewed_at: datetime
```

## 7. VerificationReference

```yaml
verification_id: string
research_id: string|null
source_id: string
label: string
url: string
link_type: enum
accessed_at: datetime
```

`link_type`:

- `PUBMED`
- `DOI`
- `PUBLISHER`
- `GOVERNMENT`
- `REPOSITORY`
- `PRIMARY_SOURCE`

## 8. SearchContext

```yaml
search_context_id: string
searched_at: datetime
systems: []
query_terms: []
date_range: string|null
inclusion_criteria: []
exclusion_criteria: []
result_count: integer
notes: string|null
```

This is required to support the no-evidence declaration and reproducibility.

## 9. No-evidence declaration

If no directly relevant study is found, the system creates an assessment with:

```yaml
directness: NO_RELEVANT_EVIDENCE_FOUND
overall_status: INSUFFICIENT
```

and renders:

> We did not identify directly relevant supporting research for this exact protocol in the sources searched as of [date]. This does not establish that the protocol is ineffective; it means that SOMA has not identified sufficient directly relevant evidence to establish its effectiveness.

## 10. Presentation rule

A user-facing protocol response must include:

1. source-derived protocol;
2. evidence status;
3. directness;
4. supporting/contradictory evidence where available;
5. faithful research summary;
6. limitations;
7. safety information;
8. original verification link;
9. professional consultation notice.

## 11. Hard boundaries

The schema deliberately prevents these transformations:

```text
source claim → medical fact
related study → exact protocol validation
positive association → causal treatment claim
ML result → diagnosis
protocol → remedy
protocol → cure
```

## 12. Privacy

The evidence registry is designed primarily for public/scientific source data and must not require personal user data.

User-specific health information, if a future feature explicitly requires it, must remain outside the public evidence registry and follow SOMA's privacy-by-design architecture. Core evidence retrieval must work without collecting personal or sensitive personal data.
