# SOMA-OS Health Intelligence Market and Product Strategy

**Date:** 2026-09-02  
**Status:** Strategic working baseline  
**Scope:** Health, wellbeing, nutrition, functional nutrition, lifestyle medicine, functional medicine, research, policy, evidence and health infrastructure

## 1. Strategic conclusion

SOMA-OS should not enter the market as another generic wellness application, calorie tracker, supplement marketplace, or AI health chatbot.

The strongest defensible opportunity identified so far is:

> **A governed, evidence-aware health intelligence layer connecting fragmented health knowledge and personal data into safer, measurable and continuously improving health action.**

Working positioning:

# SOMA-OS — The Health Intelligence Infrastructure

Working category:

**Evidence-aware health intelligence**

Core product question:

> What actually works for this person, under these conditions, and how do we know?

## 2. Market signal

The global wellness economy was approximately **$6.8 trillion in 2024**, according to the Global Wellness Institute, and was projected toward approximately **$9.8 trillion by 2029**. The scale confirms a large market, but scale alone is not a product strategy.

At the same time, noncommunicable diseases remain a dominant global health burden. WHO identifies unhealthy diet, physical inactivity, tobacco, harmful alcohol use and air pollution among major risk factors.

The implication for SOMA is not to compete for attention with more wellness content. The opportunity is to improve the quality of interpretation, action, measurement and learning around health information.

## 3. Market gaps identified

### Gap 1 — Information abundance without trustworthy interpretation

People can access enormous quantities of nutrition and wellness information but often cannot distinguish evidence, hypothesis, expert opinion, marketing and misinformation.

**SOMA opportunity:** preserve provenance, evidence quality, uncertainty and source type in every important knowledge object.

### Gap 2 — Personalization is ahead of evidence

Personalized nutrition and health platforms are growing, but systematic reviews indicate heterogeneous effects and generally limited or low-certainty evidence for many personalized interventions.

**SOMA opportunity:** personalize cautiously and expose the evidence basis rather than presenting personalization as inherently superior.

### Gap 3 — Functional medicine has a useful systems orientation but incomplete model-level evidence

Functional medicine emphasizes systems thinking, root-cause investigation, lifestyle and patient partnership. Individual elements may have evidence, but the complete care model requires stronger systematic practice-based evidence.

**SOMA opportunity:** provide an evidence infrastructure that can represent hypotheses without promoting them to established clinical facts.

### Gap 4 — Lifestyle medicine has validated pillars but implementation is difficult

Lifestyle medicine provides a useful evidence-based framework around nutrition, physical activity, restorative sleep, stress management, social connection and avoidance of risky substances.

**SOMA opportunity:** turn static pillars into measurable, adaptive interventions and follow-up loops.

### Gap 5 — Nutrition is usually treated as a recommendation, not a dynamic system

Most products answer what a person should eat. Fewer systems rigorously track context, intervention, adherence, outcome and adaptation.

**SOMA opportunity:** treat nutrition as an experiment-and-learning problem rather than a fixed prescription engine.

### Gap 6 — Data collection without a closed learning loop

Wearables, labs, questionnaires and health applications can collect substantial data, but collection does not automatically create useful knowledge.

**SOMA opportunity:** connect observation to hypothesis, intervention, measurement, outcome and adaptation.

### Gap 7 — Fragmented care and knowledge ecosystems

Clinical care, dietetics, coaching, laboratories, wearables, research databases, traditional knowledge and wellness services frequently operate as separate silos.

**SOMA opportunity:** provide neutral shared infrastructure and adapters rather than creating another silo.

### Gap 8 — Evidence and safety integration across health traditions

Traditional and integrative medicine contain knowledge worth researching, but responsible integration requires evidence, safety, quality, regulation and provenance.

**SOMA opportunity:** integrate knowledge without flattening epistemic differences.

### Gap 9 — Health misinformation

Recent systematic reviews identify substantial nutrition and wellness misinformation online, including influencer- and algorithm-amplified content.

**SOMA opportunity:** make evidence provenance and uncertainty visible at the point where users encounter a health claim.

## 4. Competitive landscape and lesson

Relevant market examples reviewed include Levels, ZOE, InsideTracker, Nourish, GOQii and HealthifyMe.

Their capabilities demonstrate strong demand for combinations of data, coaching, personalized nutrition, biomarkers, AI and digital health services.

SOMA should not attempt to copy their entire feature sets. The strategic distinction should be infrastructure and governance:

- evidence-aware rather than claim-heavy;
- source-traceable rather than opaque;
- safety-aware rather than unrestricted;
- adaptive rather than static;
- interoperable rather than siloed;
- outcome-oriented rather than engagement-only;
- protocol-neutral rather than dependent on one channel or provider.

Competitor products are useful market evidence, not architectural authorities.

## 5. Recommended first product wedge

Do not launch with “AI for all health.”

Start with:

# SOMA Metabolic & Lifestyle Health Intelligence

Initial domain:

- nutrition;
- metabolic health;
- physical activity;
- sleep;
- stress;
- weight/body composition;
- basic laboratory information;
- symptoms and health context;
- personal goals and constraints.

The initial domain is deliberately narrower than the long-term SOMA vision.

## 6. First user journey

The reference flow should be:

```text
1. Health Context
       ->
2. Health State
       ->
3. Evidence Retrieval
       ->
4. Safety / Policy Check
       ->
5. Personalized Assessment
       ->
6. Small Intervention / Experiment
       ->
7. Measurement
       ->
8. Outcome
       ->
9. Adaptation
```

The core product philosophy is:

> **Don't merely recommend. Learn.**

The system should be able to distinguish what was suggested, why it was suggested, what evidence supported it, what happened after implementation, and what should be reconsidered.

## 7. Health Evidence Graph

Every important health claim or intervention should be representable as a structured evidence object.

Minimum conceptual fields:

- claim or intervention;
- target population;
- conditions/context;
- intervention parameters;
- outcome;
- evidence type;
- evidence strength;
- population characteristics;
- dose/intensity;
- duration;
- contraindications;
- safety considerations;
- conflicts of interest;
- source;
- publication date;
- evidence status;
- uncertainty;
- provenance.

The graph must preserve disagreement and competing explanations where evidence conflicts.

## 8. Evidence status model

A useful working evidence scale is:

- **E0 — Unknown**
- **E1 — Plausible**
- **E2 — Preliminary**
- **E3 — Supported**
- **E4 — Well-supported**

This is a product-layer communication model, not a replacement for formal evidence-grading systems.

Evidence strength and safety must remain separate dimensions.

Working safety classification:

- low concern;
- context dependent;
- clinically significant caution;
- contraindicated;
- insufficient safety evidence.

## 9. Evidence Passport

A recommendation or intervention should be accompanied by an **Evidence Passport™** containing, where applicable:

- intervention;
- evidence level;
- relevant population;
- studied outcomes;
- evidence basis;
- safety considerations;
- limitations;
- last evidence review;
- source references;
- confidence/uncertainty.

The purpose is not to create a decorative badge. It is to make the reasoning chain inspectable.

## 10. Health experiment model

A SOMA health experiment should contain:

```text
Hypothesis
   -> Intervention
   -> Duration
   -> Measures
   -> Result
   -> Interpretation
   -> Next action
```

Example structure:

> Hypothesis: a specific lifestyle change may improve a selected measurable outcome under the person's current conditions.
>
> Intervention: define a bounded change.
>
> Measure: define the outcome and observation window.
>
> Result: record what happened.
>
> Adaptation: continue, modify, stop or investigate further.

This is intentionally more defensible than promising a universally “perfect” personalized diet.

## 11. Practitioner layer

A later **SOMA Practitioner** product can provide:

- longitudinal patient health context;
- evidence explorer;
- intervention builder;
- safety checks;
- follow-up tracking;
- multidisciplinary care coordination;
- outcome tracking;
- research-ready structured observations.

Professional users remain responsible for diagnosis and clinical decisions.

## 12. Research layer

A later **SOMA Research** capability can support:

- evidence maps;
- intervention registries;
- longitudinal outcome datasets;
- hypothesis registries;
- systematic-review candidate discovery;
- real-world evidence workflows;
- research provenance.

Research use requires consent, governance, privacy controls, appropriate de-identification/pseudonymization, and ethics processes where applicable.

## 13. India as a design advantage

India provides an unusually relevant environment for the SOMA model because of:

- dietary diversity;
- chronic disease burden;
- affordability constraints;
- modern medicine;
- traditional medical knowledge;
- preventive health needs;
- multilingual populations;
- family-centered health behavior;
- regional food systems;
- digital-health adoption.

The design goal should not be “India-only.” India should be a rigorous real-world design environment from which broadly reusable infrastructure can emerge.

## 14. What SOMA should not become

Avoid making SOMA primarily:

- a supplement marketplace;
- a generic AI health chatbot;
- a calorie tracker;
- an influencer wellness platform;
- a root-cause marketing engine without evidence;
- a diagnostic engine pretending to replace clinicians;
- an unlimited-testing platform;
- a biohacking product;
- a traditional-medicine claim aggregator;
- an engagement-maximization system that rewards sensational claims.

## 15. Architecture implication

The market strategy reinforces the shared-infrastructure architecture:

```text
Health Surface
      |
      v
Shared SOMA Infrastructure
      |
      +-- Identity
      +-- Capability Registry
      +-- Policy Kernel
      +-- Evidence Graph
      +-- Provenance
      +-- Safety
      +-- Privacy
      +-- Verification
      +-- Audit
      +-- Experiment / Outcome Loop
      |
      v
Provider / Research / Device Adapters
```

A new application should primarily compose these capabilities rather than recreate them.

## 16. Product learning loop

The long-term strategic moat is not merely access to AI or data.

It is the governed loop:

```text
Observe
  -> Understand
  -> Hypothesize
  -> Intervene
  -> Measure
  -> Learn
  -> Adapt
```

The system should continuously distinguish:

- what is known;
- what is inferred;
- what is hypothesized;
- what was attempted;
- what was observed;
- what remains uncertain.

## 17. Roadmap implication

The strategic roadmap is:

- **S0 — Repository stabilization:** substantially complete
- **S1 — Shared capability architecture:** substantially complete
- **S2 — Policy Kernel:** current hardening phase
- **S3 — Evidence Intelligence Layer:** next major build
- **S4 — Health State Model**
- **S5 — Nutrition Intelligence**
- **S6 — Lifestyle Intelligence**
- **S7 — Intervention / Experiment Engine**
- **S8 — Practitioner Platform**
- **S9 — Research Infrastructure**
- **S10 — External integrations**
- **S11 — Ecosystem expansion**

Expansion into many protocols, applications or agent surfaces should follow rather than precede a reliable end-to-end reference implementation.

## 18. Evidence discipline

SOMA must maintain separate epistemic categories for at least:

- clinical evidence;
- systematic reviews/meta-analyses;
- observational evidence;
- mechanistic plausibility;
- expert opinion;
- traditional knowledge;
- historical claims;
- commercial claims;
- self-published claims;
- unsupported claims.

A claim should never become stronger merely because it appears in multiple databases or is repeated by multiple secondary sources.

## 19. Strategic decision

The recommended strategy is therefore:

1. stabilize and secure the shared foundation;
2. build the evidence/provenance layer;
3. build one complete metabolic/lifestyle reference flow;
4. introduce bounded experiments and outcome measurement;
5. validate real-world usefulness;
6. add practitioner and research layers;
7. expand adapters and surfaces only when justified.

The product should earn expansion through evidence and use, not through feature accumulation.

## 20. Sources reviewed during strategy work

Representative research and institutional sources reviewed during this strategy work include:

- Global Wellness Institute — 2025 Global Wellness Economy Monitor;
- World Health Organization — Noncommunicable Diseases;
- World Health Organization — Traditional Medicine Strategy 2025–2034;
- American College of Lifestyle Medicine — lifestyle medicine and six-pillar materials;
- Institute for Functional Medicine — functional medicine and research materials;
- peer-reviewed systematic reviews on nutrition misinformation;
- peer-reviewed systematic reviews/meta-analyses on personalized nutrition;
- peer-reviewed research on slow-paced breathing/HRV;
- peer-reviewed research on whole grains/millets;
- peer-reviewed literature examining functional medicine outcomes;
- evidence and critical literature concerning exclusion-zone/fourth-phase water claims;
- NCBI/PubMed research infrastructure documentation.

Specific external source URLs should be stored in evidence records when individual claims are operationalized. This strategy document is a synthesis, not a substitute for source-level evidence records.

## 21. Relationship to project memory

This document extends `docs/SOMA-OS-PROJECT-MEMORY.md` and the strategic review in `docs/decisions/`.

The repository remains the institutional memory of SOMA-OS. Important strategic changes, market findings, product decisions, architecture decisions, security findings and evidence-model changes should continue to be recorded in version control.
