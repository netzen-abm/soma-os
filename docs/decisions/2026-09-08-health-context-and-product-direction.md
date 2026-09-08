# Decision — SOMA Health Contexts and Product Direction

**Date:** 2026-09-08
**Status:** Accepted architecture/product direction

## Decision

SOMA-OS will remain a single personal health intelligence system with a shared canonical health domain and reusable health contexts.

The product will not split into separate Athlete, Veteran, Rehabilitation, or Media products. These are governed contexts/capabilities over the same person-centered longitudinal health record.

## Product identity

SOMA is a research-oriented health and wellness environment and personal health intelligence system.

Media is a supporting research/discovery input, not the product identity and not a generic video platform.

## User value

The system should allow a person to maintain a privacy-first digital health memory containing health history, prescriptions, medications, measurements, reports, goals, observations, and selected contextual information. SOMA can then organize, monitor, contextualize, and explain that information using evidence-aware intelligence.

## Contexts

Initial contexts:

- general health;
- athlete performance;
- service veteran/ex-service health;
- rehabilitation/recovery.

Contexts share the canonical Health State, Health Evidence Graph, Personal Health Record, Local Health Vault, Identity, Policy, Safety, Provenance, Audit, Sharing, and Research Contribution infrastructure.

## Athlete direction

Professional and competitive athletes are a first-class supported use case. Performance, training load, physiology, recovery, injury, rehabilitation, nutrition, and medical information can be tracked together while preserving the distinction between observed signals, hypotheses, evidence, and causal conclusions.

Team roles receive only explicitly authorized scopes. Athlete health/performance data is sensitive and is not team-owned by default.

SOMA does not provide autonomous return-to-play clearance, performance medical diagnosis, or treatment authorization.

## Veteran direction

Serving, retired, and ex-service personnel are a first-class supported use case. The system may preserve a longitudinal service-to-health context, including documented service history, exposures, injuries, rehabilitation, medications, measurements, reports, and current follow-up.

SOMA must not infer service causality merely from temporal association. It may organize evidence and formulate questions for appropriate clinical, occupational, or legal review.

## Medication and monitoring direction

For long-term or multi-medication users, SOMA should support medication reconciliation, duplication detection, interaction/safety signals, timeline review, missing-information identification, and clinician/pharmacist question generation.

Potential tests or monitoring topics are candidate prompts for professional confirmation, not diagnoses or medical orders.

## Privacy direction

The Local Health Vault is the default health-data trust boundary. Contexts inherit its authorization, consent, provenance, audit, deletion, export, and external-processing controls.

Device-first/local processing is the preferred architecture. Device-only storage is not represented as an absolute security guarantee because device compromise, backups, screenshots, exports, and recipient-device risks remain.

## Research direction

Any contribution of personal or contextual data to research is voluntary, purpose-specific, consented, minimum-necessary, provenance-preserving, and withdrawable where possible. Context membership never implies research consent.

## Rationale

A single longitudinal model avoids fragmented health memory and permits the same infrastructure to support changing life circumstances. Specialized contexts provide domain-specific usefulness without creating duplicated databases or policy/security models.

## Related architecture

- Local Health Vault
- Personal Health Record
- Health Evidence Graph
- Health Context Framework
- Health Media Intelligence
- Cross-paradigm evidence
- Research contribution
- Identity/Authorization Enforcement
- Policy Kernel

## Guardrail

New context proposals must demonstrate reusable requirements and must not introduce a parallel canonical health model, unrestricted AI access, automatic clinical authority, or silent external data transfer.
