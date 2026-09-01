# Agent Clarification and Feedback Loop

**Status:** Architecture contract
**Version:** 0.1.0

SOMA agents must adapt to users through governed clarification and feedback,
not uncontrolled self-modification.

## Interaction loop

```text
User request
    ↓
Intent interpretation
    ↓
Confidence / ambiguity check
    ↓
 ┌───────────────┐
 │ Missing or    │── yes ──→ Clarifying question
 │ ambiguous?    │              ↓
 └───────┬───────┘         User answer
         │ no                   ↓
         └──────────────→ Re-evaluate
                              ↓
                         Policy check
                              ↓
                         Action / answer
                              ↓
                         User feedback
                              ↓
                     Governed feedback record
```

## Clarification policy

Agents must ask before proceeding when:

- required information is missing;
- multiple materially different interpretations are plausible;
- the requested action has significant privacy, financial, legal, health,
  security, or external side-effect implications;
- authorization is ambiguous;
- a requested action exceeds the current capability contract.

Agents may use low-risk defaults only when the default is reversible,
transparent, within policy, and unlikely to materially change the outcome.

## Feedback record

Feedback should capture only the minimum information required to improve the
system. A feedback record may contain:

- agent ID and version;
- execution ID;
- user-visible task category;
- feedback type;
- correction or preference;
- provenance/reference to the interaction;
- timestamp;
- applicable tenant/scope;
- review status.

Sensitive conversation content should not be copied into feedback merely for
convenience.

## Adaptation boundary

Feedback is an evaluation/configuration signal. It does not directly modify
production agent behavior.

```text
Feedback
   ↓
Classify
   ↓
Evaluate
   ↓
Approve change
   ↓
Version configuration / policy / agent
   ↓
Test
   ↓
Release
```

## Cross-department learning

A user-specific preference belongs to that user's authorized context unless
explicitly promoted through governance. One user's correction must not become
a global rule automatically.

Department-level improvements require owner approval and evaluation before
being promoted to a shared agent version.

## Auditability

Every material adaptation must be attributable to a feedback record and a
versioned release/configuration change.
