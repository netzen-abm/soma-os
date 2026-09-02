# Systems Biology and SOMA-OS

**Date:** 2026-09-02  
**Status:** Research-verified working direction  
**Scope:** Systems biology, health intelligence, nutrition, personalized nutrition, functional medicine

## Executive conclusion

Systems biology is a scientifically established research approach and is highly relevant to SOMA-OS. It should influence SOMA's scientific architecture, especially the health-state model, evidence graph, nutrition intelligence, intervention/experiment engine, and research infrastructure.

SOMA should **not** claim that its software is itself a systems-biology platform merely because it integrates health data. Systems biology conventionally involves systematic analysis of complex biological systems, often combining experimental data with computational/mathematical models and, in many contexts, multi-omics data. SOMA can build infrastructure that is *systems-oriented* and can consume systems-biology evidence without pretending to perform validated biological simulation.

## 1. What systems biology means

NIH describes systems biology as an approach that examines how components of biological systems interact at organism, tissue, or cellular levels. NIH also describes computational biomodelling/systems biology as using computer-based simulation to understand and predict interactions within biological systems.

The NCBI Medical Subject Headings definition emphasizes comprehensive analysis of complex biological systems by monitoring responses to perturbations and using large-scale computerized data collection and analysis to develop and test models.

Therefore, the important characteristics are:

- systems rather than isolated variables;
- interactions among components;
- multiple scales where appropriate;
- perturbation/response analysis;
- quantitative or computational modelling where appropriate;
- testable hypotheses and models;
- integration of experimental and observational data.

## 2. Why it fits SOMA

SOMA's proposed health-intelligence loop already has a compatible systems orientation:

```text
Health context
    -> health state
    -> evidence
    -> hypothesis
    -> intervention
    -> measurement
    -> outcome
    -> adaptation
```

This is compatible with systems thinking because an intervention is treated as a perturbation and the resulting response is measured in context rather than assuming one isolated variable determines health.

However, the loop is a **product/decision architecture**, not by itself a systems-biology model.

## 3. Nutrition is a particularly strong fit

The literature explicitly connects systems biology with personalized nutrition. Reviews describe the need to account for interacting biological processes, tissues, nutrients, environmental factors, genetics and metabolic flexibility when developing personalized nutrition approaches.

Systems-biology approaches in nutrition commonly integrate genomics/transcriptomics, proteomics, metabolomics and computational modelling to investigate how dietary interventions influence biological systems.

This supports SOMA's decision to make nutrition and metabolic/lifestyle health the first product wedge, while also warning against simplistic personalization claims.

## 4. Functional medicine relationship

A 2019 article by Jeffrey S. Bland explicitly describes Functional Medicine as having a relationship to systems-biology thinking and as a clinical framework for applying systems concepts to complex chronic disease.

This is useful for SOMA as an intellectual bridge, but the article is an editorial/perspective and should not be treated as proof that the complete Functional Medicine care model is clinically validated.

SOMA should therefore distinguish:

```text
Systems biology = scientific research approach
Functional medicine = clinical/practice framework
SOMA = governed health-intelligence infrastructure
```

They can interoperate without being treated as equivalent.

## 5. Proposed SOMA systems model

The canonical SOMA Health State should eventually represent interacting domains such as:

```text
Person
├── Biology
│   ├── biomarkers
│   ├── physiology
│   ├── symptoms
│   └── relevant laboratory data
├── Behaviors
│   ├── nutrition
│   ├── activity
│   ├── sleep
│   └── substance exposure
├── Context
│   ├── environment
│   ├── schedule
│   ├── resources
│   └── social context
├── Goals
├── Interventions
├── Responses
└── Outcomes
```

The model should represent relationships and uncertainty, not merely accumulate fields.

## 6. Perturbation/response model

A future SOMA Health Experiment can explicitly model:

```text
Baseline state
    -> intervention / perturbation
    -> observation window
    -> measured response
    -> interpretation
    -> adaptation
```

This creates a principled connection between systems thinking and the product's "Don't merely recommend. Learn." philosophy.

## 7. Important scientific boundary

SOMA must not infer biological causality merely from correlations in personal data.

A change in a biomarker after an intervention may be consistent with a hypothesis, but it does not automatically establish mechanism or causality.

Likewise, integrating many data streams does not automatically make a model biologically accurate.

Where SOMA performs modelling, it should preserve:

- model assumptions;
- input provenance;
- uncertainty;
- validation status;
- calibration;
- population applicability;
- known limitations;
- whether an output is exploratory, predictive, or clinically validated.

## 8. Architecture consequence

Systems biology should inform, but not replace, the shared SOMA architecture:

```text
Identity
  -> Capability
  -> Policy
  -> Gateway
  -> Execution
  -> Evidence
  -> Audit
             |
             v
       Health State Model
             |
             v
      Evidence / Knowledge Graph
             |
             v
       Intervention Engine
             |
             v
      Measurement / Outcomes
```

The biological model belongs above the governed infrastructure, not instead of it.

## 9. Research roadmap implication

SOMA Research should eventually support systems-biology-oriented evidence objects such as:

- biological mechanism;
- pathway/network;
- intervention;
- perturbation;
- biomarker response;
- phenotype/outcome;
- population;
- context;
- model;
- validation evidence.

These should remain linked to primary sources and evidence quality.

## 10. What we should not do yet

Do not begin by attempting to build a universal human digital twin, whole-body mechanistic simulator, or multi-omics prediction engine.

Those would introduce enormous scientific, validation, data, computational and regulatory burdens before SOMA has demonstrated value with simpler evidence-aware workflows.

The correct sequence is:

1. structured health state;
2. evidence graph;
3. contextual interventions;
4. measured outcomes;
5. validated prediction where justified;
6. increasingly sophisticated systems models only when evidence and use cases support them.

## 11. Working decision

Systems biology is adopted as a **scientific design influence and research domain** for SOMA-OS, especially for nutrition, metabolic health, complex chronic disease research and personalized health.

It is **not** adopted as a blanket claim that every SOMA capability is systems biology or that systems-biology reasoning alone validates clinical recommendations.

This distinction should remain explicit in product, research, marketing and safety documentation.
