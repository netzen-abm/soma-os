# SOMA Personal Health Record v1

**Status:** Architecture contract / design baseline  
**Date:** 2026-09-08  
**Scope:** Local-first personal health organization, longitudinal health understanding, governed sharing and emergency access

## 1. Decision

SOMA is **not a video platform** and must not become a generic content-hosting destination.

SOMA is a **research-oriented health and wellness intelligence environment** with an optional, user-controlled personal health record capability.

The personal health record (PHR) is a supporting pillar of SOMA's health-intelligence mission:

> **Understand your health, organize your health information, learn from your history, and share the right information with the right person when it matters.**

The PHR must remain:

- local-first;
- privacy-preserving;
- safety-bounded;
- interoperable;
- longitudinal;
- provenance-aware;
- user-controlled;
- usable without mandatory account creation for safe local capabilities.

## 2. Product boundary

SOMA should combine five complementary experiences:

1. **Research & Evidence** — discover and understand health knowledge and research.
2. **Health Library** — organize trusted and user-provided health information.
3. **Personal Health Record** — maintain a longitudinal personal health profile.
4. **Health Intelligence** — analyze the user's authorized data into understandable summaries, trends, questions and safety signals.
5. **Sharing & Emergency Access** — disclose minimum-necessary information to clinicians or trusted people under explicit user-controlled policy.

Media is an input/discovery surface inside Research & Evidence. It is not SOMA's identity.

## 3. Core user value

A user should be able to use SOMA as a **personal health organizer and medical-information assistant**, without turning it into an autonomous doctor.

The user can optionally maintain:

- basic personal health context;
- allergies and sensitivities;
- diagnoses or conditions explicitly recorded by the user or imported from records;
- medical history;
- surgeries/procedures;
- hospitalizations;
- prescriptions;
- medication list;
- medication start/stop/change history;
- dosage, frequency, route and indication as recorded;
- supplements and other substances voluntarily recorded;
- laboratory and measurement results;
- symptoms and observations;
- vaccination/immunization records where applicable;
- clinician/facility information where the user chooses to store it;
- reports, prescriptions and other documents;
- lifestyle, nutrition, activity, sleep and other wellness observations;
- goals and longitudinal progress.

SOMA must distinguish **user-reported information**, **imported records**, **machine-derived information**, and **clinician-authored information**.

## 4. Canonical domain rule

The PHR must not create a second canonical health model.

Existing canonical health contracts remain authoritative. The PHR is a governed composition and user-facing organization of those entities.

Conceptually:

`Health State + Evidence Graph + Documents + Medication/Prescription records + Observations + Goals + Outcomes + Provenance -> Personal Health Record view`

The PHR may introduce view/index metadata, but it must not silently duplicate or fork canonical clinical/health semantics.

## 5. Local-first data architecture

### Default trust boundary

Sensitive personal health information should remain on the user's device by default.

Recommended architecture:

`User Device -> Encrypted Health Vault -> Local Health Intelligence Runtime -> Local Reports/Views`

External services are optional capabilities rather than the default health-data path:

`User Device -> Explicit Policy/Consent -> Minimized Data -> External Capability -> Result -> Provenance -> Device`

The default system should work without sending the user's complete health record to a cloud AI provider.

### Important security qualification

"Device-only" does **not** mean risk-free.

SOMA must explicitly account for:

- device theft/loss;
- device compromise or malware;
- malicious apps and OS-level compromise;
- insecure device backups;
- cloud backup configuration;
- screenshots and screen recording;
- exported files;
- copied documents;
- shared links;
- recipient-device compromise;
- unlocked-device access;
- recovery-key loss;
- user-selected emergency disclosure.

Security must therefore be based on layered controls, not the claim that local storage alone guarantees privacy.

## 6. Health Vault requirements

The Health Vault should provide:

- encryption at rest;
- authenticated encryption for sensitive records;
- explicit key-management/recovery design;
- record-level or collection-level access controls where justified;
- tamper-evident provenance for imported/derived records;
- local audit history for access, export and sharing;
- secure deletion semantics appropriate to the storage technology;
- controlled export;
- controlled backup;
- versioning and correction history;
- offline operation;
- migration/version compatibility.

The vault should be designed so that application modules cannot silently exfiltrate health data.

## 7. Identity and account model

Login remains optional where safe local operation is possible.

A user should have a compelling reason to create an account, such as:

- encrypted multi-device synchronization;
- encrypted backup/recovery;
- controlled clinician sharing across devices;
- family/delegation workflows;
- research contribution;
- cross-device continuity;
- organization/practitioner membership.

Account creation must never be required merely for analytics, advertising, model training, convenience, or implementation simplicity.

## 8. Personal Health Timeline

The primary PHR experience should be longitudinal rather than a collection of disconnected forms.

Example timeline:

`History -> Condition -> Medication -> Test -> Result -> Symptom -> Intervention -> Response -> Outcome`

Users should be able to inspect:

- what happened;
- when it happened;
- what source recorded it;
- what changed afterwards;
- what is known versus inferred;
- what information is missing;
- what evidence supports an interpretation;
- what contradictory evidence exists.

## 9. Medication intelligence

Medication management is a high-value capability but also a high-risk one.

SOMA may provide:

- medication reconciliation;
- duplicate-therapy flags;
- recorded dose/frequency inconsistencies;
- medication timeline;
- adherence logging where the user chooses;
- known interaction/safety-signal lookup from authoritative sources;
- contraindication or caution prompts where evidence and context support them;
- questions to discuss with a doctor or pharmacist;
- reminders and organization;
- detection of potentially important changes in the user's recorded regimen.

SOMA must **not** autonomously:

- prescribe;
- discontinue medication;
- change dosage;
- declare a medication safe for the individual;
- declare an interaction clinically irrelevant;
- replace pharmacist/clinician review for high-consequence decisions.

Medication intelligence should follow:

`Recorded medication data -> normalization -> evidence lookup -> interaction/safety assessment -> context analysis -> uncertainty -> user-readable signal -> clinician/pharmacist question`

## 10. Long-term medication and polypharmacy view

For users taking multiple medicines over long periods, SOMA should produce a simple, understandable medication report.

Possible sections:

- current medicines;
- why each medicine is recorded as being taken;
- duration;
- dose/frequency;
- overlapping medicines;
- known interaction signals;
- monitoring considerations from authoritative references;
- recorded laboratory trends relevant to the medication context;
- reported symptoms/events after medication changes;
- missing information;
- questions to ask the clinician/pharmacist.

The report must distinguish:

**Observed:** directly recorded facts.  
**Documented:** information present in an imported source.  
**Derived:** computationally generated relationship.  
**Evidence-linked:** supported by an external evidence source.  
**Uncertain:** insufficient information or conflicting evidence.

## 11. Test and monitoring suggestions

SOMA may identify **candidate monitoring questions** based on recorded history, medication history, documented conditions and authoritative monitoring guidance.

Examples of output framing:

> "Your medication history may make it useful to ask your clinician whether periodic X monitoring is appropriate. SOMA has not determined that you need this test."

The system must never silently convert a candidate into:

- a diagnosis;
- a medical order;
- a treatment requirement;
- a claim that failure to test is dangerous.

Every candidate should preserve the reason, source, date, applicability and uncertainty.

## 12. Personal Health Report

SOMA should generate a simple periodic report designed for the user, not only for clinicians.

A report may contain:

### A. Health snapshot
- recorded conditions/context;
- current medications;
- recent measurements;
- active goals;
- important allergies/sensitivities;
- recent changes.

### B. Trends
- weight/body composition where recorded;
- blood pressure;
- glucose-related measurements;
- lipids;
- sleep/activity;
- symptoms;
- other user-selected measures.

### C. Medication view
- current regimen;
- changes;
- reconciliation issues;
- evidence-linked safety questions.

### D. Questions for clinician
- unresolved findings;
- missing information;
- candidate monitoring questions;
- questions generated from the user's documented history.

### E. Evidence and uncertainty
- which statements are directly recorded;
- which are derived;
- evidence sources;
- contradictions;
- uncertainty.

The report must never present an algorithmic health summary as an authoritative diagnosis.

## 13. Doctor sharing

The user should be able to create a purpose-specific clinician package rather than handing over the entire vault.

Example:

`Select purpose -> Select records -> Review preview -> Confirm recipient/scope -> Encrypt/share -> Audit -> Expire/revoke where possible`

Possible packages:

- medication reconciliation;
- recent laboratory results;
- chronic-condition history;
- hospitalization summary;
- symptom timeline;
- full longitudinal record;
- selected research/evidence notes.

The default should be **minimum necessary disclosure**, not full-record disclosure.

## 14. Emergency Profile

SOMA should provide an explicit **Emergency Profile** separate from the full PHR.

The emergency profile is a deliberately small, user-approved disclosure set intended for situations such as:

- hospitalization;
- accident;
- unconsciousness/incapacity;
- inability to communicate;
- urgent assistance by a close family member or trusted person.

Potential fields:

- name/preferred name;
- emergency contacts;
- critical allergies;
- critical medical conditions explicitly selected by the user;
- essential medications;
- anticoagulant/other high-consequence medication flags where the user chooses to disclose;
- implanted devices where applicable;
- blood-group information if the user records it, clearly labelled as user-provided unless verified;
- treating clinician/contact details where selected;
- preferred hospital/facility;
- advance-care or emergency instruction references where legally appropriate;
- emergency notes;
- record freshness timestamp.

The emergency profile must be visibly labelled as:

> **User-provided emergency information — verify clinically where possible.**

It must not be presented as a substitute for emergency medical assessment.

## 15. Emergency access model

Emergency access must not be a hidden backdoor into the Health Vault.

Preferred model:

`Emergency Profile -> explicit disclosure policy -> trusted recipient/access mechanism -> minimal dataset -> audit`

Possible modes:

1. **User-present sharing** — user unlocks and presents/exports the profile.
2. **Trusted-person delegation** — user grants a scoped emergency capability in advance.
3. **Break-glass access** — future high-assurance feature requiring explicit governance, strong audit, and careful legal/safety review.

Break-glass access should not be implemented casually. A QR code or link alone is not an authorization model.

## 16. Sharing contract principles

Every sharing grant should define, where technically possible:

- who receives access;
- what data is shared;
- why it is shared;
- when it starts;
- when it expires;
- whether re-sharing is prohibited/controlled;
- whether download/export is allowed;
- whether access is revocable;
- consent/policy version;
- audit events;
- provenance of the shared records.

Emergency access should be narrower than ordinary account access.

## 17. AI and agent boundary

SOMA's AI may assist with:

- summarization;
- organization;
- trend detection;
- medication-list normalization;
- evidence retrieval;
- contradiction search;
- question generation;
- document extraction;
- longitudinal pattern surfacing;
- research discovery.

AI must not receive unrestricted Health Vault access merely because it is useful.

The intended path is:

`User intent -> authorization -> minimum necessary context -> local analysis where possible -> evidence retrieval -> safety/policy checks -> result -> provenance -> audit`

For external AI providers:

`Health data -> explicit policy/consent -> minimization/redaction -> external call -> result -> local provenance`

## 18. Medical document handling

Prescriptions, discharge summaries, lab reports and medical documents should preserve their source.

Pipeline:

`Document -> integrity/MIME -> scan -> OCR/layout -> structured extraction -> source coordinates -> normalization -> verification -> canonical record -> provenance`

Extracted fields should retain, where possible:

- source document reference;
- page/location;
- extraction method;
- OCR confidence;
- original text/span;
- verification status;
- transformation history.

An OCR result is not automatically a clinical fact.

## 19. Health intelligence loop

The PHR connects SOMA's health intelligence loop:

`Observe -> Understand -> Hypothesize -> Intervene -> Measure -> Learn -> Adapt`

But SOMA must preserve the epistemic boundary:

`Observation != Interpretation != Recommendation`

A personal pattern is not automatically a causal relationship.

## 20. Research-oriented integration

The PHR should connect users to research without silently turning personal health data into research data.

Potential future flow:

`Personal observation -> optional research eligibility signal -> explicit purpose-specific consent -> contribution record -> de-identification/minimization -> governed research use -> attribution preference`

Research participation must remain voluntary, purpose-specific, withdrawable where technically and legally possible, and auditable.

## 21. Safety architecture

SOMA should implement a risk-tiered approach.

### Lower-risk
- organizing records;
- storing documents;
- displaying user-entered history;
- reminders;
- trend visualization;
- research discovery.

### Moderate-risk
- medication reconciliation;
- interaction/safety signals;
- longitudinal interpretation;
- monitoring questions;
- evidence-linked health summaries.

### High-risk
- diagnosis;
- medication changes;
- emergency clinical decisions;
- treatment selection;
- autonomous clinical action.

High-risk capabilities require substantially stronger controls and should not be enabled merely by adding an AI model.

## 22. Privacy architecture

Privacy requirements:

- local-first by default;
- data minimization;
- explicit purpose;
- explicit consent for external processing;
- no hidden health-data monetization;
- no health-data advertising profile by default;
- no silent model-training use;
- user-controlled export;
- user-controlled deletion subject to unavoidable technical/legal limitations;
- transparent sharing history;
- auditable external transfers;
- separation between health data and telemetry/analytics;
- no transmission of the complete health profile when a derived non-sensitive query is sufficient.

## 23. Interoperability

The PHR should eventually import/export through adapters for:

- FHIR;
- HL7 where applicable;
- Apple Health;
- Android Health Connect;
- wearable/device adapters;
- CSV/JSON;
- PDF/document sources;
- DICOM and imaging metadata where appropriate.

Interoperability standards are adapters. They do not replace the canonical SOMA health domain model.

## 24. Product architecture

Recommended high-level structure:

`Research & Evidence`
`       |`
`Health Intelligence <-> Personal Health Record`
`       |                    |`
`Evidence Graph          Health Vault`
`       |                    |`
`Policy/Consent <-> Identity/Authorization`
`       |                    |`
`Audit/Provenance <---- Sharing/Emergency`

The Health Media Intelligence layer feeds research discovery; it does not become the center of the product.

## 25. What SOMA should become

The strongest product identity is:

> **SOMA — a private, research-oriented health intelligence environment with a personal health record that helps people organize, understand, monitor and safely share their health information.**

The user should feel that SOMA is:

- their health library;
- their health organizer;
- their longitudinal health memory;
- their evidence-aware research companion;
- their medication and document organizer;
- their preparation tool for conversations with clinicians;
- their controlled emergency health profile.

It should **not** feel like:

- a social-media health feed;
- a video streaming service;
- an AI doctor;
- a diagnostic engine;
- a supplement marketplace;
- a medical-record vendor that owns the user's health data.

## 26. Non-goals for v1

Do not initially build:

- autonomous diagnosis;
- autonomous prescription;
- autonomous medication changes;
- automatic emergency break-glass access;
- unrestricted cloud AI processing;
- full hospital EHR replacement;
- insurance claims administration;
- uncontrolled health-data sharing;
- generic video hosting.

## 27. Recommended implementation sequence

### Phase 1 — Local Health Vault foundation

- canonical record references;
- encrypted local storage;
- provenance;
- import/export;
- access audit;
- local backup/recovery design.

### Phase 2 — Personal Health Profile

- timeline;
- medications;
- prescriptions;
- medical history;
- documents;
- observations;
- goals;
- progress.

### Phase 3 — Health Intelligence

- local summaries;
- medication reconciliation;
- evidence-linked safety signals;
- trend analysis;
- clinician-question generation;
- candidate monitoring prompts.

### Phase 4 — Sharing

- clinician packages;
- purpose/scope grants;
- encrypted exchange;
- expiration/revocation;
- audit trail.

### Phase 5 — Emergency Profile

- minimum-necessary emergency card;
- trusted-person delegation;
- secure presentation/export;
- emergency access audit.

### Phase 6 — Interoperability and research

- FHIR/Health Connect/Apple Health/device adapters;
- governed research contribution;
- longitudinal research insights with explicit consent.

## 28. Architectural acceptance criteria

The PHR architecture is not production-ready until:

- local-first storage is implemented and threat-modelled;
- canonical health contracts remain the single source of health semantics;
- sensitive data access is authorization-gated;
- external processing is explicit and minimized;
- sharing is scoped and auditable;
- emergency data is separate from unrestricted vault access;
- medication analysis is safety-bounded;
- test suggestions are explicitly non-prescriptive;
- provenance is retained through import/extraction/derivation;
- user-reported and machine-derived information are distinguishable;
- deletion/export/backup semantics are documented;
- adversarial privacy tests exist;
- adversarial authorization tests exist;
- emergency disclosure tests exist;
- CI validates the machine-readable contracts;
- deployment architecture does not silently turn local health data into server-side data.

## 29. Governing principle

> **Your health record belongs under your control. SOMA's job is to make it useful without making it exposed.**

And for health intelligence:

> **Help the user understand more, without pretending the system knows more than the evidence supports.**
