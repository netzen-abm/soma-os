# SOMA Food–Life Evidence Infrastructure

## Status

**Canonical architecture draft — stabilization branch**

This document defines the shared evidence model for food, plants, nutrition, traditional knowledge, analytical measurements, safety, clinical evidence, and outcomes.

It is an infrastructure specification, not a clinical protocol and not a claim that any food, plant, traditional practice, compound, or intervention treats a disease.

## 1. Core principle

> SOMA must never confuse knowledge about a food or plant with evidence that the food or plant treats a disease.

Every claim must retain its provenance, evidence type, context, safety status, and confidence.

WHO's 2025–2034 traditional medicine strategy explicitly emphasizes strengthening evidence, safety and regulation, health-system integration, and cross-sector value. SOMA therefore treats evidence and safety as first-class infrastructure rather than presentation-layer metadata.

## 2. Evidence chain

```text
Identity
  ↓
Composition
  ↓
Context
  ↓
Traditional Knowledge
  ↓
Analytical Evidence
  ↓
Experimental Evidence
  ↓
Clinical Evidence
  ↓
Outcome Evidence
  ↓
Safety Assessment
  ↓
Evidence Confidence
```

The chain is not necessarily linear. A record may have evidence at one level and no evidence at another. Missing evidence must remain explicit.

## 3. Canonical entities

### 3.1 FoodPlantIdentity

Represents what the entity is.

Required fields:

- `id`
- `canonical_name`
- `scientific_name` where applicable
- `local_names`
- `traditional_names`
- `entity_type` (`food`, `plant`, `cultivar`, `ingredient`, `prepared_food`)
- `taxonomy_reference`
- `identity_evidence_refs`

### 3.2 ContextRecord

Represents conditions under which the entity exists or was measured.

- `geography`
- `season`
- `cultivation_or_wild_status`
- `soil_or_environment_context`
- `processing_state`
- `harvest_context`
- `sample_id`

### 3.3 NutritionProfile

Represents measured or sourced nutritional composition.

- `energy`
- `macronutrients`
- `micronutrients`
- `fiber`
- `measurement_method`
- `source_ref`
- `measurement_date`
- `confidence`

### 3.4 PhytochemicalProfile

Represents detected or reported compounds.

- `compound_id`
- `compound_name`
- `concentration`
- `unit`
- `analytical_method`
- `sample_id`
- `laboratory_or_source`
- `detection_limit` where available
- `quality_flags`

### 3.5 AnalyticalAssay

Represents an actual measurement rather than a derived claim.

- `assay_id`
- `sample_id`
- `method`
- `instrument`
- `method_version`
- `operator_or_lab` where collection is permitted
- `raw_data_reference`
- `processed_data_reference`
- `quality_control_status`
- `timestamp`

HPLC belongs here. HPLC output is analytical evidence; it is not itself clinical evidence.

### 3.6 TraditionalKnowledgeRecord

Represents a documented traditional-use statement without automatically treating it as biomedical fact.

- `record_id`
- `tradition_or_community`
- `source_reference`
- `original_statement`
- `translation`
- `reported_use`
- `historical_or_current_context`
- `knowledge_rights_status`
- `evidence_class`

### 3.7 ExperimentalEvidence

Represents laboratory or preclinical evidence.

- `study_id`
- `model_type`
- `intervention_or_compound`
- `endpoint`
- `result`
- `limitations`
- `publication_reference`
- `evidence_quality`

### 3.8 ClinicalEvidence

Represents human evidence.

- `study_id`
- `population`
- `intervention`
- `comparator`
- `outcome`
- `effect_measure`
- `follow_up`
- `study_design`
- `registration_reference`
- `publication_reference`
- `risk_of_bias`
- `evidence_quality`

### 3.9 SafetyRecord

Safety is independent from efficacy.

- `safety_id`
- `population`
- `contraindications`
- `drug_interactions`
- `adverse_events`
- `dose_or_exposure_context`
- `source_reference`
- `regulatory_status`
- `confidence`

### 3.10 OutcomeRecord

Represents observed outcomes rather than inferred benefits.

- `outcome_id`
- `outcome_type`
- `measurement`
- `baseline`
- `follow_up`
- `population`
- `source_reference`
- `causal_interpretation`
- `confidence`

## 4. Evidence classification

SOMA should distinguish at minimum:

```text
TRADITIONAL_REPORT
DESCRIPTIVE
ANALYTICAL
IN_VITRO
IN_VIVO
OBSERVATIONAL_HUMAN
CLINICAL_TRIAL
SYSTEMATIC_REVIEW
META_ANALYSIS
GUIDELINE
REGULATORY
```

Evidence classes must not be silently promoted. For example:

```text
Traditional use
    ≠
Clinical efficacy
```

and:

```text
Compound detected by HPLC
    ≠
Therapeutic effect in humans
```

## 5. Evidence confidence

Use a separate confidence field rather than encoding confidence into the entity name.

Recommended values:

- `unknown`
- `very_low`
- `low`
- `moderate`
- `high`
- `very_high`

Confidence must be accompanied by the reason for the assessment and the underlying source references.

## 6. Source provenance

Every externally derived record should carry:

- source organization
- source URL or persistent identifier
- retrieval date
- publication date where available
- source type
- license/usage status where relevant
- transformation history
- evidence assessor, if manually assessed

Examples of source classes include government repositories, WHO resources, peer-reviewed research, open datasets, analytical laboratories, and documented traditional knowledge sources.

## 7. Safety boundary

SOMA must maintain a hard boundary between:

1. information retrieval,
2. evidence synthesis,
3. decision support, and
4. clinical diagnosis or treatment.

The platform must not generate a treatment recommendation merely because an entity is associated with a condition in a source dataset.

Safety review must consider:

- medication interactions
- contraindications
- population-specific risks
- dose/exposure
- contamination/adulteration risk
- identity uncertainty
- evidence uncertainty
- regulatory status

## 8. Food-is-Medicine integration

Food-is-Medicine programs provide an important reference architecture for connecting nutrition interventions with healthcare workflows. Tufts describes Food-is-Medicine programs as integrating food-based nutritional interventions into healthcare, including medically tailored meals, groceries, and produce, with clinical referral, nutrition expertise, and outcome measurement.

SOMA should model these interventions as structured intervention records rather than generic food recommendations.

```text
Health/Social Context
        ↓
Eligibility / Need Assessment
        ↓
Nutrition Intervention
        ↓
Delivery Context
        ↓
Adherence / Exposure
        ↓
Clinical & Patient Outcomes
        ↓
Healthcare Utilization / Cost
```

This allows Food-is-Medicine evidence to coexist with botanical, nutritional, and traditional-knowledge evidence without collapsing them into one category.

## 9. HPLC and analytical-service boundary

HPLC and related analytical pipelines belong in a separate analytical-service layer.

```text
SOMA Core
   │
   ├── Identity
   ├── Provenance
   ├── Privacy
   ├── Governance
   └── Evidence registry
           │
           ▼
Analytical Services
   ├── HPLC
   ├── spectroscopy
   ├── metabolomics
   └── ML classification
```

Analytical services may produce measurements and classifications. SOMA Core stores their provenance and evidence status; it must not convert a model classification into a medical claim automatically.

## 10. Privacy-by-design requirements

The Food–Life Evidence infrastructure must follow the SOMA privacy boundary:

- no personal data collection by default;
- no personal sensitive data collection by default;
- user-owned data remains on the user's device whenever possible;
- if synchronization is explicitly required, minimize the data and encrypt it before transport;
- no telemetry should be required for core functionality;
- research datasets must be de-identified and independently governed;
- provenance about public scientific sources is not equivalent to collecting personal user data.

## 11. Shared-infrastructure rule

Any new food, botanical, nutrition, analytical, evidence, safety, or provenance capability must first be implemented as reusable SOMA infrastructure.

Applications and client surfaces consume the shared capability; they must not create parallel domain models or private evidence stores.

## 12. Initial external evidence anchors

The initial architecture has been informed by:

- WHO Global Traditional Medicine Strategy 2025–2034 — evidence, safety, regulation, integration, sustainability and rights.
- Tufts Food is Medicine Institute — medically tailored meals and healthcare integration.
- Open food/composition datasets — useful as data sources but not clinical evidence by themselves.
- Ethnobotanical research datasets — useful for mapping traditional knowledge and plant-use relationships.

These sources are evidence inputs. They are not automatically authoritative for every claim represented in SOMA.

## 13. Non-goals

This model does not by itself:

- diagnose disease;
- prescribe treatment;
- establish clinical efficacy;
- validate traditional claims;
- establish safety without evidence;
- infer causality from correlation;
- convert an ML classification into a clinical decision.

Those functions require separate governance, evidence and regulatory controls.
