# SOMA — Protocol Research Evidence Pipeline

## Purpose

This shared infrastructure defines how SOMA presents source-derived disease-management protocols together with relevant research, when available, without presenting the protocol as a remedy, cure, prescription, or replacement for professional care.

## Canonical policy

> **SOMA provides disease-management information, not remedies or cures.**

A protocol may be shown when it is clearly identified as source-derived and is accompanied by an evidence search result. If no relevant supporting research is found, SOMA must say so explicitly rather than manufacture support or imply validation.

Users should be encouraged to consult an appropriately qualified healthcare professional, especially for diagnosis, medication changes, complex disease, pregnancy, severe symptoms, or high-risk conditions.

## Pipeline

```text
Source Protocol
      ↓
Normalize ingredients / practices
      ↓
Identify disease-management context
      ↓
Search relevant research
      ↓
Search contradictory evidence
      ↓
Search safety / interactions
      ↓
Assess evidence directness
      ↓
Generate faithful study summaries
      ↓
Attach original verification links
      ↓
Present protocol + evidence status
      ↓
Professional consultation notice
```

## Two legitimate outcomes

### Outcome A — relevant research available

Show:

1. the source-derived management protocol;
2. the source and provenance;
3. relevant research;
4. evidence directness;
5. study summary;
6. limitations;
7. safety information;
8. original study/reference link;
9. a clear statement that the research does not automatically validate the complete protocol.

### Outcome B — no relevant supporting research found

Show:

1. the source-derived management protocol;
2. the source and provenance;
3. **"No directly relevant supporting research was identified in the searched sources."**;
4. what adjacent or indirect evidence, if any, was found;
5. the search date and sources searched;
6. safety information where available;
7. a professional-consultation notice.

Never fill the evidence section with unrelated studies merely to make the protocol appear supported.

## Evidence directness

Every research record must state how closely it relates to the protocol:

- `DIRECT_EXACT_PROTOCOL` — the study tested the same protocol or substantially equivalent intervention.
- `DIRECT_COMPONENT` — the study tested a major component of the protocol.
- `RELATED_INTERVENTION` — similar food, botanical, preparation, or dietary intervention.
- `MECHANISTIC` — biological/chemical mechanism evidence only.
- `OBSERVATIONAL_ASSOCIATION` — association in human observational data.
- `PRECLINICAL` — laboratory or animal evidence.
- `INDIRECT` — useful contextual evidence but not directly applicable.
- `NO_RELEVANT_EVIDENCE_FOUND` — no sufficiently relevant research identified.

The UI must display this distinction.

## Evidence hierarchy

Prefer, in order where available and relevant:

1. systematic reviews and meta-analyses;
2. evidence-based clinical guidelines;
3. randomized controlled trials;
4. prospective human studies;
5. observational human studies;
6. mechanistic research;
7. preclinical research;
8. traditional-use documentation.

The hierarchy is not an automatic quality score. Study quality and directness must still be assessed.

## Study summary contract

Every cited study should be represented by:

```yaml
study_id:
title:
authors:
year:
journal:
doi:
pubmed_id:
source_url:
verification_url:
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
evidence_quality:
reviewed_at:
```

The generated summary must distinguish:

- what the researchers measured;
- what they found;
- what they did not establish;
- important limitations.

## User-facing presentation

Recommended structure:

### Disease-management protocol

**Source:** [source]

**What the source describes:** [faithful summary]

**How SOMA classifies it:** Disease-management / supportive practice; not a cure or remedy claim.

### Relevant research

**Evidence status:** [strong/moderate/limited/mixed/insufficient]

**Directness:** [exact/component/related/mechanistic/etc.]

**Study:** [title]

**Study type:** [type]

**Summary:** [faithful concise summary]

**Limitations:** [limitations]

**What this does not establish:** [explicit boundary]

**Original source:** [verification link]

### Safety and professional consultation

[relevant safety information]

> This information is not a diagnosis or prescription. Do not stop, replace, or alter prescribed treatment based on this protocol. Consult an appropriately qualified healthcare professional before using a disease-management protocol, particularly when you have a medical condition, take medication, are pregnant, or have severe or worsening symptoms.

## No-evidence declaration

When no relevant supporting research is identified, use a transparent declaration rather than silence:

> **Evidence declaration:** We did not identify directly relevant supporting research for this exact protocol in the sources searched as of [date]. This does not prove that the protocol is ineffective; it means that SOMA has not identified sufficient directly relevant evidence to establish its effectiveness. Any related or indirect research is shown separately and is not presented as validation of the protocol.

## Contradictory evidence

The evidence engine must actively search for contradictory or null findings.

If relevant evidence is mixed, show both supportive and contradictory evidence where practical.

Never suppress negative findings simply because a positive study was found.

## Search provenance

Store:

- search timestamp;
- databases/search systems used;
- search terms;
- filters;
- date range;
- inclusion criteria;
- exclusion criteria;
- evidence retrieved;
- evidence rejected and reason where material.

This makes the evidence assessment reproducible.

## Safety before presentation

A research match is not sufficient for publication of a protocol in a high-risk context.

Run safety checks for:

- medication interactions;
- contraindications;
- dose/concentration;
- botanical identity;
- contamination/adulteration;
- pregnancy/lactation;
- age-related risk;
- renal/hepatic impairment;
- disease-specific risks;
- medication replacement or withdrawal claims.

## High-risk disease gate

For cancer, cardiovascular emergencies, severe infection, stroke, poisoning, severe hypoglycemia/hyperglycemia, acute respiratory compromise, pregnancy complications and other urgent/high-risk situations:

- the protocol must not be presented as a substitute for medical care;
- professional consultation messaging is mandatory;
- emergency guidance should take priority where symptoms indicate an emergency;
- source-derived claims must remain clearly labelled as source claims;
- evidence directness must be visible.

## AI/RAG requirements

The AI layer must never transform:

```text
source claim + related research
```

into:

```text
clinically proven treatment
```

It must preserve provenance throughout retrieval and generation.

A response should be able to say:

> "The source describes this as a disease-management practice. We found research on one component, but no study identified here tested the complete protocol."

This is the desired behavior.

## Shared infrastructure requirement

The pipeline is a reusable SOMA capability.

It must serve:

- Web application;
- Android;
- iOS;
- research interface;
- practitioner/reviewer interface;
- AI/RAG;
- future applications.

No application should implement its own private evidence-search or protocol-validation logic.

## Governance

The evidence engine must remain separate from clinical decision authority.

SOMA provides:

- source discovery;
- evidence retrieval;
- evidence classification;
- provenance;
- safety information;
- transparent uncertainty;
- user verification paths.

Qualified professionals remain responsible for diagnosis and clinical decisions.

## Design decision

This pipeline intentionally supports both:

**Protocol + relevant study references**, when evidence exists;

and:

**Protocol + explicit no-relevant-evidence declaration**, when it does not.

That is preferable to either suppressing traditional/source knowledge or overstating scientific support.
