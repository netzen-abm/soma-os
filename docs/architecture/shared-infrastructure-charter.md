# SOMA Shared Infrastructure Charter

**Status:** Foundational policy
**Date:** 2026-08-31

## Principle

Every reusable capability, tool, research adapter, safety rule, privacy
control, data model, and integration pattern should be designed first as
shared infrastructure.

Application surfaces consume the shared capability. They do not silently
create competing implementations.

## Applies to

- Web applications
- Android
- iOS
- desktop/native clients
- Telegram
- WhatsApp
- Messenger
- AI/RAG services
- agentic services
- research tools
- future ecosystem surfaces

## Shared-first rule

Before adding a capability to an application surface, determine whether it
belongs in a shared layer.

If reusable across two or more surfaces, implement the contract and core
behavior in shared infrastructure first unless there is a documented reason
not to.

## Capability layers

```text
Policy
  ↓
Contracts
  ↓
Shared capabilities
  ↓
Adapters
  ↓
Application surfaces
```

### Policy

Defines privacy, safety, evidence, security, and data-handling boundaries.

### Contracts

Defines stable interfaces and data semantics between components.

### Shared capabilities

Contains reusable business and technical primitives.

### Adapters

Connects external providers and transports to the shared contracts.

### Application surfaces

Presents capabilities to users through a specific channel or platform.

## Privacy by design and default

SOMA should operate without collecting personal data or sensitive personal data
unless a separately governed capability establishes a compelling necessity.

Default behavior:

- local-first storage;
- data minimization;
- no unnecessary identity collection;
- no silent telemetry containing personal data;
- explicit user control where data leaves the device;
- encryption for data that must be transmitted or stored remotely;
- no sale of personal data.

Privacy policy is infrastructure, not a client-specific feature.

## Safety by design and default

Safety controls belong in shared infrastructure so that one transport cannot
bypass safeguards applied by another transport.

For health-related capabilities:

- distinguish education from clinical advice;
- distinguish management support from treatment claims;
- expose evidence provenance;
- expose uncertainty and limitations;
- preserve safety warnings;
- require expert consultation where appropriate.

## Evidence infrastructure

Research discovery, source routing, provenance, deduplication, contradiction
handling, evidence assessment, and verification should be shared capabilities.

PubMed is one provider, not the evidence system.

National, international, and global sources can be combined according to the
research question.

## Provider isolation

External providers are adapters behind stable interfaces.

A provider outage must not corrupt shared evidence records or cause a client
to implement a private substitute that violates the shared policy.

## Transport isolation

A Telegram failure must not make Android, iOS, Web, or another transport fail.

Transport-specific code belongs at the adapter boundary.

## AI boundary

AI is an optional capability, not a prerequisite for core functionality.

Where AI is used, it consumes shared evidence and policy infrastructure. AI
must not silently bypass provenance, safety, privacy, or user-control rules.

## Data boundary

User data and research data are different domains.

Research infrastructure should work from research questions and public or
properly governed sources without requiring personal user data.

## New tool admission

Every new tool or external service must be evaluated for:

1. purpose;
2. licensing and cost;
3. privacy impact;
4. security impact;
5. portability;
6. provider dependency;
7. reusable value across surfaces;
8. failure behavior;
9. evidence/provenance implications.

Free/public tooling is preferred during the current foundation phase when it
meets the requirements.

## Maintainability

Shared infrastructure must be easier to maintain than duplicated client code.
Prefer:

- small modules;
- small functions;
- clear contracts;
- short readable lines;
- explicit errors;
- isolated provider adapters;
- focused tests.

## Change rule

A new capability should answer these questions before implementation:

- Can another surface reuse it?
- Does it belong in shared infrastructure?
- What policy governs it?
- What contract exposes it?
- What happens when its provider fails?
- What data does it require?
- Can the user operate without it?

## Architectural objective

SOMA should grow by adding capabilities to a common foundation, not by
recreating the same intelligence, safety, privacy, and evidence logic in each
application surface.
