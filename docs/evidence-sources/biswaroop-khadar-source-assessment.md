# Biswaroop Roy Chowdhury & Dr. Khadar Valli — Evidence Source Assessment

## Purpose

This document registers the Biswaroop Roy Chowdhury public website/e-book ecosystem and the supplied Dr. Khadar Valli Siridhanya/Kashaya material as **knowledge sources** for SOMA's Food–Life Evidence Infrastructure.

These materials are source records. Their inclusion does **not** mean that SOMA validates their medical claims, treatment efficacy, safety, or causal explanations.

## 1. Source records

### Source A — Dr. Biswaroop Roy Chowdhury official website

Canonical source: https://biswaroop.com/

The official website describes Dr. Biswaroop Roy Chowdhury as an author/speaker associated with natural health and lifestyle-disease reversal and provides links to books, magazines, courses, research papers and other health material.

SOMA classification:

- `source_type`: `PUBLISHER_WEBSITE`
- `provenance_level`: `PRIMARY_PUBLISHER`
- `domain`: `nutrition_and_lifestyle_health`
- `evidence_class_default`: `DESCRIPTIVE`
- `clinical_validity`: `NOT_ESTABLISHED_BY_SOURCE_TYPE`

### Source B — Dr. Biswaroop Roy Chowdhury free e-book library

Canonical source: https://biswaroop.com/free_e_book/

The library contains a large collection of free health e-books in English and Hindi, with books covering nutrition, lifestyle, emergency medicine, cancer, heart care, diabetes, transplantation and related topics.

SOMA classification:

- `source_type`: `PUBLISHER_EBOOK_LIBRARY`
- `provenance_level`: `PRIMARY_PUBLISHER`
- `domain`: `nutrition_lifestyle_integrative_health`
- `evidence_class_default`: `DESCRIPTIVE`
- `clinical_validity`: `BOOK_SPECIFIC_ASSESSMENT_REQUIRED`

Each individual book must become a separate source record. The library page must never be treated as evidence for every claim contained in every book.

### Source C — Supplied Dr. Khadar Valli Siridhanya & Kashaya protocol material

The supplied document is titled **Siridhanya & Kashayas for leading healthy life and curing diseases** and contains material attributed to Dr. Khadar Valli and Dr. Sarala, including millet descriptions, lifestyle practices, food restrictions, Kashaya recipes, condition-specific protocols and cancer protocols. fileciteturn171file11

SOMA classification:

- `source_type`: `AUTHOR_ATTRIBUTED_PROTOCOL_DOCUMENT`
- `provenance_level`: `SECONDARY_OR_REPRODUCED_DOCUMENT_UNLESS_ORIGINAL_IS_VERIFIED`
- `domain`: `millets_traditional_food_practice_botanical_decoctions`
- `evidence_class_default`: `TRADITIONAL_REPORT` / `DESCRIPTIVE`
- `clinical_validity`: `NOT_ESTABLISHED`

The document contains explicit protocol claims linking specific millets/decoctions/oils to diseases and also contains cancer-specific protocols. These must be stored as **reported claims**, not as treatment recommendations. fileciteturn171file6

## 2. Why these sources are valuable to SOMA

They contain structured knowledge that is useful for research and evidence mapping:

- common and regional food names;
- millet identity and language mappings;
- botanical names;
- preparation methods;
- traditional food practices;
- fermented-food practices such as Ambali;
- Kashaya/decoction ingredient combinations;
- lifestyle routines;
- agriculture and food-system context;
- claimed relationships between foods/plants and health conditions;
- source attribution and historical context.

For example, the supplied material describes five Siridhanya categories and gives preparation guidance for fermented millet porridge/Ambali. fileciteturn171file0

It also provides condition-to-protocol mappings, such as diabetes, thyroid, PCOD, cardiovascular conditions, kidney conditions and others. These mappings are precisely the type of claim relationship SOMA should preserve as a provenance-rich record rather than flattening into a recommendation engine. fileciteturn171file1

## 3. Critical evidence boundary

SOMA must separate four things:

```text
WHAT THE SOURCE SAYS
        ↓
WHAT WAS ACTUALLY MEASURED
        ↓
WHAT INDEPENDENT EVIDENCE SUPPORTS
        ↓
WHAT, IF ANYTHING, MAY SAFELY BE RECOMMENDED
```

A source statement such as:

```text
"Millet X helps condition Y"
```

must **not** become:

```text
Millet X treats condition Y
```

and must not become:

```text
SOMA recommends Millet X for condition Y
```

without separate evidence assessment and safety review.

## 4. Claim representation

Every extracted claim from these sources should use a structure conceptually equivalent to:

```yaml
claim_id:
subject:
predicate:
object:
source_id:
source_location:
quoted_or_paraphrased_statement:
claim_type:
reported_by:
evidence_class:
independent_evidence_refs: []
safety_evidence_refs: []
contradictory_evidence_refs: []
confidence: unknown
clinical_recommendation_status: not_established
review_status: pending
```

### Recommended `claim_type` values

- `TRADITIONAL_USE_CLAIM`
- `NUTRITIONAL_COMPOSITION_CLAIM`
- `PREPARATION_CLAIM`
- `LIFESTYLE_CLAIM`
- `AGRICULTURAL_CLAIM`
- `MECHANISTIC_CLAIM`
- `DISEASE_ASSOCIATION_CLAIM`
- `THERAPEUTIC_EFFICACY_CLAIM`
- `SAFETY_CLAIM`
- `PERSONAL_TESTIMONIAL`

## 5. Example: Siridhanya claim handling

The supplied source states that specific millets have benefits for conditions including diabetes, anemia, constipation, neurological conditions and cancers. fileciteturn171file4

SOMA should ingest this as multiple source claims, for example:

```text
Claim C-001
Subject: foxtail millet
Predicate: reported_benefit_for
Object: diabetes
Source: Khadar-Siridhanya-2020
Claim type: DISEASE_ASSOCIATION_CLAIM
Evidence class: TRADITIONAL_REPORT / AUTHOR_ASSERTION
Clinical recommendation: NOT_ESTABLISHED
```

Then independently attach:

```text
Nutritional evidence
Analytical evidence
Preclinical evidence
Human observational evidence
Clinical-trial evidence
Systematic reviews
Safety evidence
Contradictory evidence
```

This allows the same food to have a rich evidence graph without treating one author's assertion as the final scientific answer.

## 6. Cancer claims require elevated safeguards

The supplied document contains a section titled **Siridhanya and Kashaya for curing Cancer** and provides cancer-type-specific combinations of millets and herbal decoctions. fileciteturn171file6

These records must be assigned an elevated review state:

```text
HIGH-RISK HEALTH CLAIM
        ↓
SOURCE PRESERVATION
        ↓
INDEPENDENT EVIDENCE SEARCH
        ↓
SAFETY / INTERACTION REVIEW
        ↓
CLINICAL EVIDENCE ASSESSMENT
        ↓
NO AUTOMATIC TREATMENT RECOMMENDATION
```

They must never be surfaced by a general consumer interface as if SOMA had established that the protocol cures cancer.

## 7. Protocol data model

A protocol should be stored separately from a claim.

```text
Protocol
├── protocol_id
├── protocol_name
├── source_id
├── source_version
├── reported_indication
├── ingredients[]
├── quantities
├── preparation_method
├── administration_method
├── schedule
├── duration
├── exclusions
├── source_claim_refs[]
├── safety_refs[]
├── evidence_refs[]
├── review_status
└── recommendation_status
```

This distinction is essential because a protocol can be accurately transcribed from a historical source even when its claimed efficacy has not been established.

## 8. Ingredient normalization

The Khadar material contains common names, regional names and botanical names. For example, it maps several Siridhanya millets across English, Hindi, Marathi, Tamil, Kannada and Telugu. fileciteturn171file11

SOMA should therefore normalize each ingredient through:

```text
Source name
   ↓
Canonical common name
   ↓
Scientific/botanical identity
   ↓
Taxonomic reference
   ↓
Regional names
   ↓
Plant/food identity confidence
```

Identity uncertainty must remain visible. Similar vernacular names must not be silently merged.

## 9. Preparation is part of evidence context

The same food or botanical material can differ by:

- species/cultivar;
- plant part;
- harvest stage;
- geography;
- drying;
- fermentation;
- cooking;
- extraction method;
- concentration;
- storage;
- contamination/adulteration.

Therefore SOMA should never treat an ingredient name alone as equivalent to a protocol intervention.

The supplied Ambali description, for example, specifies soaking, cooking, fermentation and covering conditions. fileciteturn171file0

## 10. Safety model

Some supplied protocols involve medicinal plants, concentrated decoctions, oils, fasting/dietary restrictions, or instructions concerning existing medication.

SOMA must separately assess:

- herb-drug interactions;
- contraindications;
- pregnancy/lactation considerations;
- renal/hepatic impairment;
- pediatric/geriatric use;
- allergy risk;
- toxic plant misidentification;
- contamination/adulteration;
- dose and concentration;
- duration;
- replacement or discontinuation of conventional treatment.

The source itself contains a warning not to stop regular medicines suddenly, but that source statement must still be treated as source content rather than as a substitute for independent clinical guidance. fileciteturn171file1

## 11. Biswaroop e-books: ingestion policy

The official free e-book library should be used as a **bibliographic discovery layer**.

For every selected book:

1. capture title;
2. capture author attribution;
3. capture publication/launch date if available;
4. capture official source URL;
5. capture the actual e-book URL if available;
6. record language;
7. record edition/version;
8. preserve page/section references;
9. extract claims separately from explanatory text;
10. attach independent evidence;
11. assess safety;
12. assign review status.

The official library currently lists numerous books and dates, including recent publications and older works. citeturn0search0

## 12. Source hierarchy

Recommended hierarchy for SOMA:

### Tier 1 — Primary scientific/regulatory evidence

- regulatory documents;
- clinical guidelines;
- systematic reviews/meta-analyses;
- registered clinical trials;
- peer-reviewed primary research;
- validated laboratory datasets.

### Tier 2 — Structured scientific/technical datasets

- food composition databases;
- botanical/taxonomic databases;
- analytical datasets;
- pharmacovigilance databases.

### Tier 3 — Traditional and expert knowledge

- traditional pharmacopeia;
- practitioner-authored protocols;
- ethnobotanical documentation;
- historical texts;
- expert books.

### Tier 4 — Public educational material

- websites;
- magazines;
- interviews;
- videos;
- testimonials;
- social posts.

Tier 3 and Tier 4 sources are valuable knowledge inputs but cannot automatically override stronger evidence or establish clinical efficacy.

## 13. Copyright and ingestion

SOMA should preserve metadata, short excerpts where legally permissible, bibliographic references and source links rather than indiscriminately copying entire copyrighted books into a public-facing knowledge layer.

Full-text ingestion should be governed by the applicable license, permission, ownership and access controls.

For research use, the preferred record is:

```text
metadata + provenance + claim extraction + citation + evidence links
```

rather than uncontrolled full-text republication.

## 14. AI/RAG rule

The AI layer must retrieve source claims with provenance.

A generated answer should be able to distinguish:

```text
"Dr. Khadar Valli's material states..."

from

"Clinical evidence indicates..."

from

"SOMA has insufficient evidence to determine..."
```

The model must never merge these into one undifferentiated answer.

## 15. Recommended user-facing evidence labels

Use plain-language labels such as:

- **Traditional report**
- **Author claim**
- **Nutritional evidence**
- **Laboratory evidence**
- **Human observational evidence**
- **Clinical trial evidence**
- **Systematic-review evidence**
- **Safety evidence**
- **Evidence uncertain**
- **Not established as treatment**

## 16. Decision

The Biswaroop and Khadar material should be **included in SOMA's knowledge/evidence graph**, but **not promoted to the clinical recommendation layer by default**.

This is the correct architectural treatment because it preserves valuable traditional and expert knowledge while maintaining the central SOMA distinction:

> **A claim can be faithfully recorded without being scientifically validated.**

## 17. Next implementation step

Before creating clinical recommendation APIs, implement the following shared capability:

```text
Evidence Source Registry
        ↓
Claim Extraction Registry
        ↓
Protocol Registry
        ↓
Independent Evidence Linker
        ↓
Safety / Interaction Registry
        ↓
Evidence Assessment
        ↓
Controlled Retrieval API
```

Only after this chain is operational should SOMA expose protocol-oriented AI retrieval to consumer applications.
