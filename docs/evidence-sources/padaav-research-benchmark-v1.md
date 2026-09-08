# Padaav Cross-Paradigm Research Benchmark v1

## Purpose

This benchmark tests whether SOMA can represent and assess health research originating from an Ayurvedic clinical/research context without either automatically endorsing it or dismissing it because its research design differs from a preferred biomedical design.

## Scope

Padaav's public research material is used as a benchmark corpus. This file is a research-governance artifact, not a clinical recommendation and not an endorsement of Padaav, Ayurveda, or any specific treatment protocol.

## Benchmark questions

SOMA must be able to answer:

1. What exactly is being claimed?
2. Which knowledge system and clinical model does the claim arise from?
3. What research method was used?
4. What population was studied?
5. What intervention/exposure was actually delivered?
6. What outcomes were measured or reported?
7. What positive and negative outcomes were reported?
8. What attrition and exclusions occurred?
9. What causal conclusions are justified?
10. What safety information exists?
11. What independent replication exists?
12. What contradictory evidence exists?
13. What remains uncertain?
14. What stronger next study would answer the unresolved question?

## Benchmark cases

### Migraine observational research

A published observational/open-label study associated with Padaav reported outcomes among participants completing a defined Ayurvedic protocol. The publication itself describes the design as uncontrolled/open-label and states that the findings do not permit a definite conclusion and that a properly controlled larger study is needed.

SOMA must preserve this distinction:

```text
Observed improvement reported
        !=
Controlled causal efficacy established
```

### Pancreatitis longitudinal research

Padaav-associated published work includes a large clinical cohort with reported treatment outcomes, attrition, non-response, surgery, deaths, and progression. The benchmark must preserve the complete outcome distribution rather than extracting only positive results.

SOMA should represent the study as potentially valuable longitudinal clinical evidence while keeping causal inference limited by its design and potential confounding.

## Required benchmark behaviors

### Do

- preserve source provenance;
- preserve the original medical/knowledge-system context;
- classify the research method accurately;
- preserve attrition and negative outcomes;
- separate observation from causal inference;
- separate efficacy from safety;
- identify confounding and selection concerns;
- search for independent replication and contradiction;
- state uncertainty explicitly;
- propose research gaps without presenting them as proof.

### Do not

- call an intervention proven because a cohort improved;
- call a study worthless solely because it is observational;
- infer endorsement from inclusion in SOMA;
- infer falsity from absence of an RCT;
- convert marketing claims into evidence claims;
- suppress non-response, adverse outcomes, dropout, progression, or mortality;
- fabricate citations or source URLs.

## Source register

Primary source discovery should use the canonical Padaav research archive and the underlying indexed publications where available.

Canonical research archive:
- https://padaav.com/research/

Research archive is a discovery source; published studies should be linked to their persistent identifiers or primary publication records where available.

## Attribution

SOMA will retain source-level attribution and, where practical, claim-level provenance. Inclusion in this benchmark means that a source is being studied as evidence or as a test of SOMA's epistemic handling. It does not imply endorsement or partnership.

## Research value

This benchmark is deliberately difficult because it tests a capability that a conventional single-axis evidence classifier can mishandle:

> represent a different medical knowledge system faithfully while applying rigorous evidence-integrity requirements.

The benchmark should be expanded over time to include conventional biomedicine, Ayurveda, Siddha, Unani, traditional Chinese medicine, lifestyle medicine, functional medicine, nutrition, indigenous knowledge, mechanistic hypotheses, and other research contexts.
