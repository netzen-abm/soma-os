# SOMA Shared Agent Platform

**Status:** Foundational architecture contract
**Version:** 0.1.0

This document defines the shared infrastructure for safe, reusable agents.
Agents are capabilities of the SOMA ecosystem, not application-specific bots.

## Architecture

```text
Agent Registry
      ↓
Agent Gateway
      ↓
Agent Identity + Policy
      ↓
Model Armor
      ↓
Agent Runtime
      ↕
Memory Bank
      ↓
Shared Capabilities / Tools
      ↓
Production Data (policy-gated)
      ↓
Agent Observability
```

## Agent Registry

The registry is the authoritative catalog for enterprise-approved agents.
Each agent record must declare:

- stable agent ID and semantic version;
- owner and lifecycle status;
- purpose and allowed tasks;
- required capabilities and tools;
- permitted data classes and jurisdictions;
- identity and authorization requirements;
- memory policy and retention class;
- model/provider adapters;
- supported surfaces;
- human-approval requirements;
- failure and rollback behavior;
- evaluation and release evidence.

Agents are discovered by capability and policy, not by arbitrary code import.
A department may reuse an approved agent without creating a competing copy.

## Agent Runtime

The runtime owns execution state for synchronous and long-running work.
Jobs must have an immutable execution ID, lifecycle state, deadline/lease,
retry policy, cancellation path, and idempotency key where side effects exist.

Long-running work must be resumable from durable state. A worker restart must
not silently duplicate an external side effect.

## Memory Bank

Memory is a governed capability, not an unrestricted transcript store.
Every memory item must carry:

- subject/tenant scope;
- purpose;
- sensitivity/data classification;
- jurisdiction;
- source and provenance;
- created/updated timestamps;
- retention/expiry policy;
- access policy;
- deletion status.

The default is minimum necessary context. Memory must not cross tenants,
users, jurisdictions, or authorization boundaries merely because an agent can
technically retrieve it.

## Agent Identity

Every agent execution receives a distinct identity and execution context.
Authorization is evaluated for the agent, user/delegator, tenant, capability,
data scope, and requested action.

Use short-lived credentials and least privilege. Agents must not inherit broad
human credentials by default.

## Agent Gateway

All external agent tool calls and production-data access should pass through a
single policy-enforcing gateway.

The gateway is responsible for authentication, authorization, data-boundary
checks, routing, rate limits, audit correlation, and capability allowlists.
Direct agent-to-provider access is prohibited where the provider handles
protected production data.

## Model Armor

Model Armor is an inline defense layer. It must treat model input and tool
output as untrusted.

Controls include:

- prompt-injection detection;
- tool/schema validation;
- untrusted-content isolation;
- output policy checks;
- PII and secret leakage detection;
- data-loss-prevention checks;
- unsafe-action detection;
- human approval for high-impact actions.

Armor is not a substitute for authorization. A blocked prompt must not be the
only control protecting production data.

## Agent Observability

Every execution must emit structured, privacy-aware audit events with a
correlation ID. The target telemetry model is OpenTelemetry-compatible.

Record sufficient operational provenance to reconstruct what happened:
agent version, execution ID, policy decision, capability/tool invocation,
result class, error, approval, and timestamps.

Do not store unrestricted chain-of-thought or sensitive payloads as telemetry.
Store concise decision metadata and references to governed artifacts instead.

## Production data boundary

Production data is never implicitly available to an agent. Access requires:

```text
request
  ↓
identity
  ↓
policy
  ↓
data classification
  ↓
jurisdiction check
  ↓
minimum necessary scope
  ↓
armor/tool validation
  ↓
approved gateway action
  ↓
audited result
```

A provider outage, memory failure, or model failure must fail closed for
protected actions and must not broaden permissions during fallback.

## Cross-department reuse

A reusable agent is published once in the registry with explicit ownership,
versioning, policy requirements, and evaluation evidence. Departments consume
that approved version through the gateway. Local customization is represented
as configuration or a governed extension, not as an untracked fork.

## Asynchronous continuity

Context across weeks is maintained through durable, scoped memory and runtime
state, not by relying on an ever-growing chat transcript. Each continuation
must re-authorize the current actor and re-evaluate policy before accessing
protected memory or production data.

## User interaction

When required information is missing or an action is ambiguous, the agent
should ask a clarifying question rather than infer a high-impact decision.
User feedback should be captured as a governed signal that can update agent
configuration or evaluation data only through approved workflows.
