# SOMA Research Contribution & Attribution v1

## Status

Proposed research-governance contract.

## Purpose

SOMA is intended to turn health observation, evidence review, interventions, outcomes, and research questions into a governed learning ecosystem. Participation must create research value only when the user has explicitly chosen to contribute and the contribution can be governed ethically.

## Core principles

1. **Contribution is voluntary.** A person must not be required to contribute research data merely to use ordinary SOMA capabilities unless a capability explicitly requires it and the user is informed before participation.
2. **Consent is purpose-specific.** Consent to use SOMA is not automatically consent to research, publication, training, sharing, or external disclosure.
3. **User choice is reversible where legally and technically possible.** Withdrawal must stop future optional research use; previously lawfully incorporated aggregate research may not always be technically retractable, and this limitation must be disclosed.
4. **Minimum necessary data.** Research contributions should use the least sensitive data necessary for the approved research question.
5. **Provenance is preserved.** Contributions retain source, collection context, transformation history, and consent/purpose metadata.
6. **No hidden monetization of contribution.** Research participation must not silently become advertising, profiling, model training, or commercial disclosure.
7. **No individual health benefit is promised merely because data are contributed.** Research contribution is participation in knowledge creation, not a guarantee of diagnosis or treatment.
8. **Users may receive appropriate contribution records.** Where feasible, SOMA should expose what was contributed, for which purpose, and what research/publication outputs resulted.

## Contribution levels

SOMA should distinguish:

- `NO_RESEARCH_CONTRIBUTION` — data remain outside optional research use;
- `PRIVATE_RESEARCH_USE` — governed research use without public release;
- `DE_IDENTIFIED_RESEARCH_USE` — approved de-identified/pseudonymized research use;
- `AGGREGATE_RESEARCH_USE` — aggregate analysis where individual re-identification is not intended;
- `PUBLIC_RESEARCH_ARTIFACT` — an approved public artifact with explicit governance;
- `USER_AUTHORED_RESEARCH` — user deliberately contributes an observation, hypothesis, protocol, or case narrative as a research artifact.

These are not interchangeable consent states.

## Research contribution record

A contribution record should be capable of preserving:

- contribution ID;
- principal/participant reference;
- purpose;
- research question/project reference;
- data categories;
- collection period;
- provenance;
- transformation/de-identification history;
- consent version;
- withdrawal state;
- access/governance policy;
- publication/use restrictions;
- resulting research artifact references where appropriate.

## User participation model

The long-term SOMA loop may be:

```text
Observe
  -> Understand
  -> Act
  -> Measure
  -> Learn
  -> Optionally Contribute
  -> Research
  -> Improve Evidence
  -> Improve Future Health Intelligence
```

The final contribution step is deliberately optional. SOMA must not design dark patterns that pressure users into research participation.

## Attribution and source credit

SOMA must give credit to the sources that materially support its evidence and research infrastructure.

For each source, retain where available:

- creator/author;
- organization/institution;
- title;
- publication or release date;
- persistent identifier;
- canonical URL;
- source type;
- license/copyright information;
- retrieval date;
- source contribution to the SOMA artifact.

Credit must not imply endorsement.

A source may be cited because it was analyzed, challenged, used as a benchmark, or used to identify a research gap.

## External acknowledgement policy

After launch, SOMA may send formal acknowledgement letters to institutions, researchers, practitioners, publishers, open-source projects, data providers, and other sources whose work materially contributed to SOMA research or capability development.

Such letters should:

- identify the specific work used;
- provide the relevant SOMA artifact or capability;
- distinguish citation from endorsement;
- avoid implying partnership unless one exists;
- request correction if attribution is incomplete;
- respect licensing and intellectual-property terms.

No organization should be represented as a SOMA partner merely because SOMA cited or analyzed its work.

## Research output attribution

When a SOMA research artifact uses multiple sources, attribution should be claim-level where practical rather than a generic bibliography alone.

For example:

```text
Claim A
  -> Source 1
  -> Source 2
  -> Source 3

Limitation B
  -> Source 2

Contradiction C
  -> Source 4
```

This preserves intellectual lineage and makes source auditing possible.

## Community and user credit

SOMA should eventually support contribution acknowledgement for users who explicitly opt into attribution. Possible levels include:

- anonymous contribution;
- pseudonymous contribution;
- named contribution;
- contributor role;
- research collaborator role where formally appropriate.

Health privacy takes precedence over public recognition. A user's health information must never be exposed merely to provide credit.

## Research ethics boundary

A product telemetry event is not automatically research data. Research involving human participants may require additional ethical, legal, institutional, or regulatory review depending on the activity, jurisdiction, population, and intended use.

SOMA must not label an activity as ethically approved merely because a user consented to a product feature.

## Publication gate

Before external publication of SOMA-generated research:

```text
Research question
  -> approved purpose
  -> data/consent review
  -> provenance verification
  -> analysis validation
  -> privacy review
  -> safety review where relevant
  -> authorship/attribution review
  -> legal/licensing review where required
  -> publication decision
```

## Non-goals

This document does not establish an institutional review board, replace applicable research ethics requirements, or grant permission to collect sensitive data. It defines the product-level governance direction that future research capabilities must implement.

## Decision

SOMA should be designed so that every user can contribute to the world's health knowledge **only by meaningful choice**, with clear purpose, proportional data use, provenance, privacy protection, and transparent attribution.
