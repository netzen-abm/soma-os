# SOMA — Evidence Research Engine

## Status

Canonical shared-infrastructure architecture for protocol evidence discovery, research synthesis, contradiction detection, safety discovery, epistemic context and user-verifiable references.

## 1. Purpose

The Evidence Research Engine converts a source-derived claim or disease-management protocol into a transparent, reproducible evidence assessment.

It does **not** prescribe treatment, diagnose disease, validate a remedy, or replace professional clinical judgment.

## 2. Canonical flow

```text
Source / Claim / Protocol
          ↓
Protocol Decomposition
          ↓
Concept + Ingredient Normalization
          ↓
Epistemic Context Mapping
          ↓
Research Question Generation
          ↓
Evidence Search
    ┌─────┼──────────┐
    ↓     ↓          ↓
Support  Null/     Safety
Evidence Contradictory Evidence
    └─────┼──────────┘
          ↓
Deduplication + Screening
          ↓
Structured Extraction
          ↓
Directness Assessment
          ↓
Quality / Bias / Consistency
          ↓
Evidence Synthesis
          ↓
User-Verifiable Evidence Package
```

## 3. Research question generation

The engine should decompose a protocol into searchable questions rather than issue one broad AI query.

For each protocol:

```text
Protocol
├── disease-management context
├── food/plant/ingredient
├── preparation
├── dose/concentration
├── duration
├── administration
├── claimed outcome
└── safety context
```

Research queries should separately investigate:

1. exact protocol;
2. major components;
3. related interventions;
4. relevant outcomes;
5. adverse effects/interactions;
6. contradictory/null evidence;
7. relevant guidelines/reviews.

## 4. Evidence search strategy

The engine should prefer structured scholarly and authoritative sources where available.

Suggested source classes:

- PubMed/MEDLINE;
- peer-reviewed publishers;
- systematic-review databases;
- clinical-guideline repositories;
- government health agencies;
- WHO resources;
- authoritative food/nutrition databases;
- botanical/taxonomic sources;
- regulatory safety sources.

Search provenance must be retained.

The engine should follow a reproducible review pattern: formulate the question, define inclusion/exclusion criteria, search broadly, screen, extract data, assess bias/quality and synthesize results. This aligns with established systematic-review methodology. 

## 5. Search layers

### Layer A — Exact protocol

Search the complete intervention wording, variants, source terminology and preparation details.

### Layer B — Components

Search each material/practice and clinically relevant outcome separately.

### Layer C — Related intervention

Search comparable dietary, botanical, lifestyle or preparation interventions.

### Layer D — Mechanism

Search biochemical/physiological mechanisms only as mechanistic evidence.

### Layer E — Safety

Search interactions, contraindications, adverse effects, contamination and dose/exposure risks.

### Layer F — Contradiction

Explicitly search null, negative, conflicting and critical evidence.

## 6. Screening

Each retrieved record receives:

```yaml
screening_status:
relevance:
protocol_match:
population_match:
outcome_match:
safety_relevance:
duplicate_of:
exclusion_reason:
```

No study should be included merely because its title contains the disease or ingredient.

## 7. Structured extraction

For each included study:

```yaml
study_id:
title:
authors:
year:
journal:
study_type:
registration_id:
doi:
pubmed_id:
population:
sample_size:
intervention:
comparator:
duration:
outcomes:
effect_estimates:
main_findings:
limitations:
risk_of_bias:
funding:
conflicts_of_interest:
safety_findings:
source_url:
verification_url:
```

The extraction layer must distinguish reported results from model-generated interpretation.

## 8. Evidence directness

Every protocol-to-study relationship must receive one directness value:

- `DIRECT_EXACT_PROTOCOL`
- `DIRECT_COMPONENT`
- `RELATED_INTERVENTION`
- `MECHANISTIC`
- `OBSERVATIONAL_ASSOCIATION`
- `PRECLINICAL`
- `INDIRECT`
- `NO_RELEVANT_EVIDENCE_FOUND`

The engine must never silently upgrade an indirect study into exact-protocol evidence.

## 9. Quality and bias

Quality assessment should consider study design and relevant methodological limitations.

At minimum:

```text
Study design
Risk of bias
Sample size / precision
Comparator quality
Outcome validity
Follow-up duration
Attrition
Selective reporting
Confounding
External validity
```

The engine should preserve the underlying assessment rather than outputting an unexplained single AI score.

## 10. Evidence synthesis

Synthesis must distinguish:

- supportive findings;
- null findings;
- contradictory findings;
- safety findings;
- indirect evidence;
- unresolved questions.

If meta-analysis is available, preserve the published effect estimates and uncertainty rather than recomputing them without a validated statistical pipeline.

## 11. No-evidence state

If the search identifies no sufficiently relevant evidence:

```yaml
status: INSUFFICIENT
reason: NO_RELEVANT_EVIDENCE_FOUND
search_context_id: ...
```

User-facing declaration:

> We did not identify directly relevant supporting research for this exact protocol in the sources searched as of [date]. This does not establish that the protocol is ineffective; it means that SOMA has not identified sufficient directly relevant evidence to establish its effectiveness.

Adjacent evidence may be displayed separately, explicitly labelled as indirect or related.

## 12. Contradictory evidence

The engine must actively retain evidence that conflicts with the source claim.

A protocol with both supportive and null/negative studies should be marked:

`MIXED`

unless a documented evidence synthesis justifies a more specific conclusion.

## 13. Safety gate

Before publishing a protocol evidence package, run safety discovery.

```text
Identity verification
       ↓
Dose / concentration
       ↓
Interactions
       ↓
Contraindications
       ↓
Population-specific risk
       ↓
Contamination/adulteration
       ↓
Regulatory status
       ↓
Professional-review requirement
```

Positive efficacy evidence never bypasses the safety gate.

## 14. Epistemic context

If the source belongs to a traditional or other distinct knowledge system, attach the Epistemic Context Mapping record before interpreting the claim.

The engine must preserve:

- original terminology;
- source tradition;
- native concept definition;
- stated validation method;
- translation status;
- external/modern research;
- correspondence assessment.

The engine must not silently translate traditional concepts into biomedical diagnoses or mechanisms.

## 15. Evidence package

The final engine output should be a machine-readable package:

```yaml
protocol:
source:
source_claims: []
epistemic_context: {}
research_question:
search_context: {}
studies: []
assessments: []
supporting_evidence: []
contradictory_evidence: []
safety: []
overall_status:
limitations: []
verification_links: []
professional_review_required: true
```

## 16. User presentation contract

The client layer receives structured data and renders:

### Disease-management protocol

What the source describes.

### Evidence status

Strong / moderate / limited / mixed / indirect / insufficient.

### Research found

Study-by-study summaries.

### How closely it applies

Exact protocol / component / related / mechanistic / observational / preclinical / indirect.

### What the research does not establish

Explicit boundaries.

### Safety

Relevant known concerns and uncertainty.

### Verify the evidence

Original PubMed, DOI, publisher, government or primary-source link.

### Professional consultation

Clear instruction to consult an appropriately qualified healthcare professional, especially for diagnosis, medication decisions, complex disease, pregnancy or high-risk symptoms.

## 17. AI boundaries

AI may:

- formulate search queries;
- classify source types;
- summarize retrieved studies;
- identify relationships;
- identify uncertainty;
- organize evidence.

AI may not:

- invent studies;
- invent citations;
- upgrade evidence directness;
- conceal contradictory findings;
- turn a source claim into a medical fact;
- diagnose;
- prescribe;
- tell users to stop or replace prescribed treatment;
- declare a remedy/cure without independently established evidence and appropriate clinical governance.

Every generated research summary must remain linked to the underlying structured study record.

## 18. Reproducibility

Each evidence run stores:

- timestamp;
- engine version;
- source databases;
- queries;
- filters;
- date range;
- inclusion/exclusion rules;
- retrieved records;
- screened records;
- included records;
- assessment version.

This makes a result auditable and allows future re-runs when literature changes.

## 19. Research freshness

Evidence packages must display when the literature search was performed.

Evidence should be re-searchable rather than treated as permanently current.

## 20. Human review

The engine is a research-assistance system, not an autonomous clinical authority.

Human expert review should be required for:

- high-risk conditions;
- high-risk botanicals or preparations;
- medication interactions;
- protocols involving medication replacement;
- claims of disease reversal/cure;
- ambiguous identity/dose;
- publication of clinical-facing recommendations.

## 21. Shared infrastructure

The Evidence Research Engine is a core SOMA capability.

All application surfaces consume it through shared interfaces:

```text
Web
Android
iOS
AI/RAG
Research UI
Practitioner UI
Future surfaces
      ↓
Shared Evidence Research Engine
      ↓
Shared Evidence Registry
```

No client-specific evidence engine is permitted.

## 22. External alignment

WHO's Global Traditional Medicine Strategy 2025–2034 emphasizes strengthening evidence, ensuring safety and regulation, appropriate integration, and respect for local heritage and rights. WHO's Research and Evidence unit also identifies appropriate research methods and evidence gaps as important work areas. citeturn0search0turn0search2

This architecture is therefore intentionally evidence-first while preserving the contextual integrity of traditional knowledge.

## 23. Design decision

SOMA's evidence engine should optimize for **traceability, directness, uncertainty and user verification**, not for producing a positive recommendation.

The desired outcome is not:

> "SOMA says this works."

The desired outcome is:

> "Here is what the source says, here is what research has actually tested, here is how closely it relates, here are the limitations and safety considerations, and here are the original sources so you can verify them yourself."
