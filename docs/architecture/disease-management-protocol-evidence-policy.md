# SOMA — Disease Management Protocol & Evidence Policy

## Status

**Architecture decision — locked for implementation**

## 1. Core principle

SOMA will **not present any food, traditional practice, supplement, botanical preparation, lifestyle routine, or source-derived protocol as a remedy or cure**.

SOMA may present a source-derived **Disease Management Protocol** when the protocol is framed as a supportive management approach and is accompanied by an evidence assessment and independently verifiable references.

The distinction is fundamental:

```text
REMEDY / CURE
    ❌ NOT SOMA'S CLAIM

DISEASE MANAGEMENT / SUPPORTIVE PRACTICE
    ↓
SOURCE-DERIVED PROTOCOL
    ↓
INDEPENDENT RESEARCH SEARCH
    ↓
EVIDENCE ASSESSMENT
    ↓
SAFETY / INTERACTION CHECK
    ↓
USER-VISIBLE REFERENCES
    ↓
USER VERIFICATION
```

## 2. What SOMA may say

Preferred language:

- "This protocol is described by [source]."
- "It is presented here as a disease-management/supportive practice, not as a cure."
- "We found the following research relevant to this component."
- "The cited study found..."
- "The evidence is limited/mixed/indirect/etc."
- "This study does not establish that the source protocol treats or cures the disease."
- "Verify the original study using the provided reference."

Avoid:

- "This cures..."
- "This treats..."
- "This will reverse..."
- "This replaces medication..."
- "Clinically proven" unless the specific evidence actually establishes the claim.

## 3. Protocol + evidence is not protocol validation

SOMA must preserve the difference between:

```text
A. Source protocol
B. Research about an ingredient/practice
C. Research about a similar intervention
D. Research directly testing the exact protocol
E. Clinical recommendation
```

A study about millet consumption does not automatically validate a particular Siridhanya combination, cooking method, Kashaya, dosage, duration, or disease-specific protocol.

Evidence matching therefore needs a `directness` field.

Recommended values:

- `DIRECT_EXACT_PROTOCOL`
- `DIRECT_COMPONENT`
- `RELATED_INTERVENTION`
- `MECHANISTIC`
- `OBSERVATIONAL_ASSOCIATION`
- `PRECLINICAL`
- `INDIRECT`
- `NO_RELEVANT_EVIDENCE_FOUND`

## 4. Research discovery requirement

When SOMA presents a Disease Management Protocol, the evidence service should actively search for supporting research relevant to the protocol's components and intended management context.

Search priority:

1. systematic reviews / meta-analyses;
2. clinical practice guidelines;
3. randomized controlled trials;
4. prospective human studies;
5. observational studies;
6. mechanistic / laboratory studies;
7. preclinical studies;
8. traditional-use documentation.

The search should also look for contradictory findings and safety evidence.

## 5. Evidence result shown to the user

Each protocol should expose an evidence section containing:

```text
Evidence status
Evidence directness
Research references
Study type
Population
Intervention/exposure
Comparator
Duration
Measured outcomes
Main findings
Limitations
Safety considerations
Conflict-of-interest / funding information when available
Original-source link

"Verify the original study" link
```

The user must be able to inspect the original publication independently.

## 6. Example — millet / diabetes management

A literature search demonstrates why this model is appropriate.

A 2021 systematic review/meta-analysis reported that long-term millet consumption was associated with reductions in fasting and post-prandial blood glucose in diabetic subjects and examined glycemic-index outcomes across different millet types and processing methods. See the PubMed record for PMID **34395493**: https://pubmed.ncbi.nlm.nih.gov/34395493/

However, another 2024 systematic review/meta-analysis of ancient grains found that pooled analyses using millet did not show significant effects for the selected diabetes outcomes, illustrating that the evidence base is not uniform. See the PubMed record for PMID **38553358**: https://pubmed.ncbi.nlm.nih.gov/38553358/

More broadly, systematic reviews of whole grains and dietary fibre provide evidence relevant to glycemic management, but they do not validate any particular Siridhanya or Kashaya protocol. For example, a 2020 systematic review/meta-analysis found improvements in several glycemic and cardiometabolic measures with higher fibre intake, while noting substantial heterogeneity in trial results. See the PubMed record for PMID **32142510**: https://pubmed.ncbi.nlm.nih.gov/32142510/

A 2024 systematic review/meta-analysis of whole grains included 37 randomized trials and found reductions in fasting blood glucose, with more modest/uncertain effects for HbA1c and HOMA-IR. See the PubMed record for PMID **38664726**: https://pubmed.ncbi.nlm.nih.gov/38664726/

Therefore SOMA should report something like:

```text
Source protocol:
Siridhanya-based dietary management protocol

Evidence found:
Relevant evidence exists for millet/whole-grain dietary patterns and some glycemic outcomes.

Evidence directness:
RELATED_INTERVENTION / DIRECT_COMPONENT depending on the specific study.

Evidence consistency:
MIXED / REQUIRES CONTEXT

What the research does NOT establish:
The cited research does not establish that the complete source protocol cures or treats diabetes, nor that it can replace prescribed diabetes care.
```

## 7. Research summary must be faithful to the paper

SOMA's AI must not invent a study conclusion.

For each paper, create a structured research record:

```yaml
study_id:
title:
authors:
year:
journal:
doi:
pubmed_id:
study_type:
population:
sample_size:
intervention:
comparator:
duration:
outcomes:
main_findings:
limitations:
safety_findings:
funding:
conflicts_of_interest:
evidence_directness:
source_url:
verification_url:
```

The user-facing summary must be generated from this structured record.

## 8. Evidence confidence

Do not use a single opaque AI confidence score.

Use a multidimensional assessment:

```text
Study quality
Evidence directness
Evidence consistency
Evidence magnitude
Evidence precision
Evidence duration
Safety certainty
External validity
```

A final plain-language status may then be derived:

- `Evidence relatively strong`
- `Evidence moderate`
- `Evidence limited`
- `Evidence mixed`
- `Evidence indirect`
- `Evidence insufficient`

## 9. Safety gate

A protocol cannot be surfaced merely because supporting research exists.

Before presentation, run:

```text
Ingredient identity
      ↓
Dose/concentration
      ↓
Drug interaction check
      ↓
Contraindication check
      ↓
Population-specific risk
      ↓
Disease-specific risk
      ↓
Medication-replacement risk
      ↓
Safety status
```

The presence of positive research does not cancel safety concerns.

## 10. Medication boundary

SOMA must never instruct a user to stop, reduce, replace, or alter prescribed medication on the basis of a source protocol or research summary.

If a source recommends medication withdrawal or substitution, SOMA should reproduce that as a source claim only when necessary for provenance and clearly mark it as **not a SOMA recommendation**.

## 11. High-risk conditions

For cancer, cardiovascular emergencies, severe infection, acute respiratory compromise, severe hypoglycemia/hyperglycemia, stroke symptoms, poisoning, pregnancy complications and other high-risk situations, disease-management content must not obscure the need for qualified medical assessment or emergency care.

Source protocols may remain available as historical/knowledge records where appropriate, but their presentation must use enhanced safety messaging and evidence labeling.

## 12. User verification is a first-class feature

Every research reference must provide an explicit verification action.

Preferred flow:

```text
SOMA summary
    ↓
Study metadata
    ↓
"Read original study"
    ↓
PubMed / publisher / DOI / repository
```

The user should never have to trust SOMA's summary alone.

## 13. Research freshness

Evidence records should store:

- search date;
- databases searched;
- search terms;
- inclusion/exclusion criteria;
- evidence date range;
- last reviewed date;
- superseded status.

This allows SOMA to rerun evidence searches when literature changes.

## 14. Source protocol and research protocol are separate objects

```text
SOURCE_PROTOCOL
    └── says what the source proposes

RESEARCH_EVIDENCE
    └── says what research tested

EVIDENCE_ASSESSMENT
    └── evaluates relationship between them

USER_PRESENTATION
    └── communicates both without conflating them
```

This separation is mandatory.

## 15. Final product language

The canonical SOMA framing is:

> **SOMA does not prescribe remedies or claim cures. SOMA documents disease-management practices, identifies relevant research, assesses how directly that research applies to the practice, and gives users the original references so they can verify the evidence themselves.**

This should become a product-wide policy across Web, mobile, AI, practitioner and research surfaces.
