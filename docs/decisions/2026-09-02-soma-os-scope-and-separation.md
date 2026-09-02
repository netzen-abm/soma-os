# Decision Record — SOMA-OS Scope and Separation

**Date:** 2026-09-02  
**Status:** Accepted  
**Decision type:** Product scope / architecture / ecosystem boundary

## 1. Decision

SOMA-OS and Janavani are separate projects.

They must not be mixed at the product, repository, roadmap, documentation,
architecture, or capability-priority level merely because both may eventually
consume reusable infrastructure patterns.

SOMA-OS is a health and wellbeing infrastructure project.

Its scope includes:

- health and wellbeing;
- integration across relevant medical streams;
- health and medical research;
- evidence infrastructure;
- health-related policy and policy analysis;
- privacy-first health data infrastructure;
- health information and decision-support capabilities;
- safe integration of health-related services, providers, and research sources.

SOMA-OS is not a civic participation platform and does not inherit Janavani's
product goals.

## 2. Product boundary

Janavani is outside the SOMA-OS product boundary.

No Janavani application, workflow, civic feature, or product requirement
should be introduced into SOMA-OS merely for reuse.

If a genuinely generic technical capability is reusable, it may be considered
as shared infrastructure only when it remains domain-neutral and does not
create coupling between the two products.

## 3. SOMA-OS positioning

SOMA-OS should be developed as health-oriented infrastructure rather than as
an AI chatbot or a generic agent platform.

AI, agents, MCP, research connectors, knowledge systems, and other protocols
are implementation capabilities. They are not the product identity.

The core proposition is:

> SOMA-OS is a privacy-first, evidence-informed, modular health and wellbeing
> infrastructure for integrating knowledge, research, policy, data,
> capabilities, and governed intelligence across health domains.

## 4. Medical-stream integration

The integration objective is interoperability and evidence synthesis across
health domains, not the assertion that every medical tradition or intervention
has equivalent evidentiary support.

The architecture must preserve distinctions among:

- established clinical evidence;
- emerging evidence;
- traditional or historical knowledge;
- mechanistic or observational evidence;
- low-quality or conflicting evidence;
- unsupported claims.

Integration must therefore be evidence-aware rather than equivalence-based.

## 5. Health and safety boundary

SOMA-OS may support evidence discovery, synthesis, education, management
information, research, policy analysis, and decision support where appropriate.

It must not silently convert research information into diagnosis,
prescription, treatment replacement, or claims of cure.

Safety, uncertainty, provenance, contraindications, conflicts, and evidence
quality must remain explicit wherever relevant.

## 6. Shared infrastructure principle

Reusable SOMA capabilities belong in shared infrastructure first.

Client surfaces and domain modules should consume shared contracts rather
than implement parallel authorization, privacy, evidence, safety, provenance,
or policy logic.

The preferred dependency direction is:

```text
Health application / surface
            |
            v
Shared SOMA capability infrastructure
            |
      +-----+-----+-----+
      |           |     |
   Policy      Evidence Safety
      |           |     |
      +-----+-----+-----+
            |
            v
 Health providers / research sources
```

## 7. Core platform spine

The working platform spine is:

```text
Identity
   -> Capability
   -> Policy
   -> Gateway
   -> Execution
   -> Evidence
   -> Audit
```

This spine should remain independent of any particular user interface,
provider, AI model, protocol, or external service.

## 8. Development strategy

The immediate priority is not to add every possible adapter or protocol.

The platform should first prove one complete health-oriented reference flow:

```text
Health question / task
        |
        v
Capability selection
        |
        v
Policy evaluation
        |
        v
Evidence / health data access
        |
        v
Governed execution
        |
        v
Traceable result
        |
        v
Audit / provenance
```

Once this reference flow is reliable, additional health applications and
surfaces can consume the same infrastructure.

## 9. Practical application domains

Potential SOMA-OS applications include:

1. evidence research and synthesis;
2. health and wellbeing information systems;
3. longitudinal personal health knowledge systems;
4. integration of research across medical streams;
5. clinical and professional decision support;
6. health-policy research and analysis;
7. institutional health intelligence;
8. governed health agents;
9. health-data interoperability and provenance;
10. future health research infrastructure.

These are application directions, not a commitment to implement all of them
immediately.

## 10. Strategic risks

The primary risks are:

- scope explosion;
- building architecture without a real reference product;
- excessive abstraction before operational need exists;
- treating AI capability as product identity;
- conflating evidence discovery with medical advice;
- treating different medical traditions as evidentially equivalent;
- assuming passing tests alone establishes production security.

The response is disciplined sequencing, explicit boundaries, adversarial
validation, and real end-to-end health use cases.

## 11. Relationship to other projects

SOMA-OS may independently develop generic technical patterns that could be
useful elsewhere.

Such reuse must never create a product dependency on Janavani.

The default rule is:

> Separate products, independent roadmaps, domain-specific priorities,
> reusable infrastructure only where genuinely generic.

## 12. Source of truth

This decision record is part of SOMA-OS institutional memory and should be
updated through subsequent architecture decision records when the scope or
boundary materially changes.
