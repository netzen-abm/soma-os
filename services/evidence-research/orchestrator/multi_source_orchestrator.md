# SOMA Multi-Source Research Orchestrator

## Purpose

Coordinate evidence discovery across multiple independent national, international and global research sources. PubMed is one adapter within this system and is never treated as the sole evidence source.

## Core principle

> Search broadly enough to reduce source bias, preserve every source's provenance, and assess the underlying evidence rather than counting databases.

## Orchestration flow

```text
Protocol / Claim
      ↓
Research Question Builder
      ↓
Source Router
      ├── Biomedical literature
      ├── Systematic reviews
      ├── Clinical trial registries
      ├── Indian research repositories
      ├── Traditional medicine repositories
      ├── Food / nutrition databases
      ├── Safety / regulatory sources
      └── Botanical / identity sources
              ↓
       Parallel adapter search
              ↓
       Result normalization
              ↓
       Deduplication
              ↓
       Relevance screening
              ↓
       Exact/component/related matching
              ↓
       Contradiction detection
              ↓
       Safety evidence
              ↓
       Evidence assessment
              ↓
       Verification package
```

## Source routing

The router selects sources according to the research question.

### General biomedical question

Use multiple relevant literature sources, beginning with PubMed and Europe PMC and expanding to appropriate review/guideline sources.

### India-relevant question

Include relevant Indian sources such as:

- Clinical Trials Registry–India;
- AYUSH Research Portal;
- ICMR repositories/portals;
- CDSCO safety/regulatory sources;

alongside appropriate global sources.

### Food / nutrition question

Include relevant food-composition and nutrition sources such as USDA FoodData Central and FAOSTAT, while separately searching human outcome literature.

### Traditional medicine question

Preserve the originating knowledge system and search relevant traditional-medicine research sources alongside modern research databases.

### Safety question

Use appropriate regulatory and safety sources in addition to biomedical literature.

## Minimum source diversity

The router should normally seek at least two relevant source classes/providers where the question permits.

For high-risk research, target at least three independent source classes/providers where feasible.

This is a search-coverage rule, not an evidence-strength score.

## Independence rule

Multiple databases indexing the same publication count as **one underlying study**, not multiple independent studies.

Deduplication keys should prefer:

1. DOI;
2. PMID;
3. trial registration ID;
4. publisher identifier;
5. normalized title + author + year as fallback.

## Evidence provenance

Every result must retain:

```yaml
provider_id:
provider_record_id:
source_class:
source_url:
verification_url:
retrieved_at:
search_context_id:
```

Never erase provider provenance during normalization.

## Contradiction search

The orchestrator must explicitly issue searches for:

- null findings;
- negative outcomes;
- adverse events;
- systematic reviews that challenge the proposed benefit;
- guideline disagreement;
- methodological criticism where relevant.

A positive search result must not suppress contradictory evidence.

## Trial registry integration

Trial registrations are evidence of study existence/design, not evidence of efficacy.

The orchestrator should link registrations to publications where a matching identifier exists.

Unpublished or ongoing studies should be labelled accordingly.

## Search completion states

```text
COMPLETED_WITH_RESULTS
COMPLETED_NO_RELEVANT_RESULTS
PARTIAL_PROVIDER_FAILURE
PROVIDER_UNAVAILABLE
RATE_LIMITED
```

`COMPLETED_NO_RELEVANT_RESULTS` requires completion of the required source routes.

`PARTIAL_PROVIDER_FAILURE` must be visible in the evidence package and must not be converted into an absence-of-evidence claim.

## No-evidence declaration

Only after required searches complete can the engine produce:

> We did not identify directly relevant supporting research for this exact protocol in the sources searched as of [date].

The declaration must include the search context and should identify any provider failures that prevented complete coverage.

## Evidence synthesis rule

The orchestrator aggregates evidence records; it does not simply vote by source count.

For example:

```text
PubMed → Study A
Europe PMC → Study A
Publisher → Study A

Underlying evidence = 1 study
```

## Regional balance

For India-relevant questions, the router should avoid an India-only or global-only evidence search when both domains have relevant sources.

The objective is complementary coverage, not forced symmetry.

## Traditional knowledge boundary

Traditional sources can document:

- source claims;
- historical use;
- native terminology;
- traditional preparation;
- stated rationale;
- traditional validation context.

They should not be automatically converted into clinical efficacy evidence.

## AI role

AI can assist the orchestrator with:

- query expansion;
- synonym discovery;
- terminology mapping;
- candidate screening;
- study clustering;
- contradiction identification.

AI must not decide that a study supports a protocol without retaining the evidence relationship and source provenance.

## Privacy

The orchestrator operates on source-derived research questions. It must not require personal health information.

Research queries must not contain user names, identifiers, medical records or other personal/sensitive personal data unless a future explicitly governed capability requires otherwise; the default is no such data.

## Cost policy

Initial routing prioritizes free/public sources. Paid databases can be added as optional adapters later, but the core evidence capability must remain functional without them.

## Verification

The final evidence package must provide original verification paths wherever available:

- PubMed;
- DOI;
- publisher;
- government repository;
- clinical-trial registry;
- primary traditional source.

## Shared infrastructure requirement

The orchestrator is a shared SOMA service consumed by every surface:

```text
Web / Android / iOS / AI-RAG / Research UI
                    ↓
          Multi-Source Orchestrator
                    ↓
             Evidence Registry
```

No client surface may implement private source-routing logic.

## Design decision

SOMA does not seek a single "best database." It builds a **traceable evidence network** in which relevant national, international and global sources can contribute independently, while the underlying studies and their limitations remain visible.
