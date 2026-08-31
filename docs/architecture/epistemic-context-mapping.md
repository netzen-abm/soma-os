# SOMA — Epistemic Context Mapping

## Status

Canonical shared-research capability for studying knowledge claims across different knowledge traditions without collapsing them into a single epistemic framework.

## Principle

> A knowledge claim should first be understood within the knowledge tradition that produced it, including its concepts, methods of observation, validation criteria and context, before comparison with another knowledge tradition.

SOMA does not assume that either a modern scientific framework or an indigenous knowledge framework is automatically correct. It makes the frameworks explicit and evaluates relationships between them.

## Capability flow

```text
Knowledge Source
      ↓
Claim
      ↓
Knowledge Tradition
      ↓
Native Concepts
      ↓
Epistemic / Validation Framework
      ↓
Contextual Interpretation
      ↓
Modern / External Evidence Search
      ↓
Concept Correspondence Assessment
      ↓
Convergence / Divergence / Uncertainty
```

## 1. KnowledgeTradition

```yaml
tradition_id:
name:
region:
language:
historical_period:
domain:
source_refs: []
```

Examples may include Ayurveda, Yoga, Tantra, Darśana traditions, ethnobotanical knowledge, community food practices, or modern biomedical science.

The registry records a tradition; it does not establish the truth of all propositions associated with it.

## 2. NativeConcept

```yaml
concept_id:
tradition_id:
term:
language:
transliteration:
definition_source:
context:
related_concepts: []
translation_status:
```

Translation status should support:

- `DIRECT_EQUIVALENT`
- `APPROXIMATE`
- `PARTIAL`
- `NO_ESTABLISHED_EQUIVALENT`
- `CONTESTED`

A translated term must not be treated as equivalent to a modern technical term merely because both appear to describe similar phenomena.

## 3. EpistemicFramework

```yaml
framework_id:
tradition_id:
name:
knowledge_objects: []
validation_modes: []
observation_modes: []
reasoning_modes: []
source_refs: []
```

For Indian knowledge traditions, this may include documented pramāṇa categories such as pratyakṣa, anumāna, śabda and yukti where the particular tradition supports those categories. The exact interpretation must remain tradition-specific.

## 4. ClaimContext

```yaml
claim_context_id:
claim_id:
tradition_id:
concept_refs: []
framework_id:
context_description:
original_language_statement:
translation:
interpretive_notes:
```

The purpose is to preserve what the claim means **within its originating context** before external comparison.

## 5. Methodological record

SOMA should capture how a claim or practice was investigated, where that information is available:

```yaml
method_id:
claim_id:
method_type:
observation_period:
participants_or_sources:
intervention_or_practice:
measurements:
validation_criteria:
limitations:
source_refs: []
```

Possible method types include:

- textual analysis;
- oral-history/ethnography;
- practitioner observation;
- longitudinal observation;
- embodied/experiential observation;
- laboratory measurement;
- clinical study;
- statistical analysis;
- comparative analysis.

The record describes a method. It does not automatically certify the method's conclusions.

## 6. Cross-framework comparison

```yaml
comparison_id:
claim_id:
internal_interpretation:
external_framework:
comparison_question:
correspondence:
convergence:
divergence:
unknowns:
category_error_risk:
assessment_status:
```

`correspondence` should distinguish:

- `CONCEPTUALLY_ALIGNED`
- `PARTIALLY_ALIGNED`
- `ANALOGICAL_ONLY`
- `NOT_EQUIVALENT`
- `UNRESOLVED`

## 7. Example: Yukti and Bayesian inference

The research question may ask whether Yukti and Bayesian inference have methodological similarities.

SOMA must not begin with:

```text
Yukti = Bayesian inference
```

Instead:

```text
Document Yukti within its native tradition
        ↓
Document Bayesian inference within modern statistics
        ↓
Compare their objects, premises, uncertainty handling,
reasoning operations and validation criteria
        ↓
Identify similarities and differences
        ↓
Report correspondence as aligned / partial / analogy / unresolved
```

This turns the topic into a legitimate comparative methodology study rather than a forced equivalence.

## 8. Example: Śabda Pramāṇa and scientific evidence

SOMA may investigate:

- what constitutes authoritative testimony within the relevant tradition;
- how source reliability is established;
- whether testimony is independent or derivative;
- how testimony interacts with perception and inference;
- how modern scientific evidence establishes reliability.

The result must distinguish conceptual comparison from the claim that one system can simply replace the other.

## 9. Example: Ayurvedic qualitative data

If an Ayurvedic assessment contains qualitative categories, SOMA may investigate ways to represent them without destroying their original semantics.

Possible workflow:

```text
Native qualitative category
       ↓
Original definition
       ↓
Practitioner interpretation
       ↓
Inter-rater protocol where appropriate
       ↓
Operational representation
       ↓
Reliability analysis
       ↓
Statistical model if appropriate
```

Operationalization must preserve the original concept and clearly identify what has been transformed.

## 10. Reductionism research track

SOMA may support research examining whether reductionist models adequately represent particular biological, experiential, ecological or systems-level phenomena.

The system should compare:

- reductionist model;
- systems model;
- object of inquiry;
- measurable variables;
- omitted variables;
- explanatory scope;
- predictive performance;
- limitations.

It must not label reductionism as inherently invalid merely because a non-reductionist framework exists.

## 11. Ethics of subjectivity

Experiential or practitioner knowledge can be recorded without treating subjective observation as automatically objective evidence.

SOMA should capture:

- observer position;
- training/competence;
- observation protocol;
- duration;
- reproducibility where applicable;
- inter-observer agreement where applicable;
- reflexivity;
- conflicts of interest;
- corroborating evidence.

This supports rigorous study of subjective knowledge while preserving the phenomenon being studied.

## 12. Research status

Each epistemic comparison must have a status:

- `RESEARCH_QUESTION`
- `SOURCE_DOCUMENTED`
- `PRELIMINARY_COMPARISON`
- `EVIDENCE_SUPPORTED`
- `EVIDENCE_MIXED`
- `UNRESOLVED`
- `CONTESTED`

No comparison should be presented as established merely because a conceptual similarity has been identified.

## 13. Relationship to Food–Life Evidence

This capability extends the Food–Life Evidence Infrastructure:

```text
Food / Plant / Practice
       ↓
Traditional knowledge claim
       ↓
Epistemic context
       ↓
Native validation method
       ↓
Analytical evidence
       ↓
Modern research
       ↓
Cross-framework comparison
       ↓
Evidence assessment
       ↓
Safety
       ↓
Management information
```

This is particularly important when SOMA represents Ayurveda, Siridhanya, Kashaya, ethnobotanical practices or other Indian knowledge sources.

## 14. AI/RAG rule

The AI layer must preserve epistemic labels during retrieval.

It should be able to answer:

> "Within the source tradition, this concept means..."

and separately:

> "Modern research has investigated the following related phenomenon..."

and separately:

> "The correspondence between these concepts is uncertain / partial / not established."

The AI must never silently translate a traditional concept into a biomedical diagnosis or mechanism.

## 15. Shared infrastructure

Epistemic Context Mapping is a reusable SOMA capability.

It should be available to:

- research;
- evidence registry;
- Food–Life knowledge;
- AI/RAG;
- Web;
- Android;
- iOS;
- practitioner/reviewer interfaces;
- future knowledge domains.

No client surface should create its own epistemic mapping model.

## 16. Design decision

SOMA adopts **methodological pluralism with explicit epistemic context**.

This means:

1. preserve a knowledge system's own categories and context;
2. document its stated methods of knowing and validation;
3. study claims using appropriate methods;
4. compare frameworks only after their internal structures are understood;
5. explicitly identify convergence, divergence and uncertainty;
6. never manufacture equivalence between concepts;
7. never treat tradition, modern science, or AI output as automatically authoritative.
