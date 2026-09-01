# Architecture Decision: OpenClaw, MCP, and the SOMA Agent Ecosystem

**Date:** 2026-09-01
**Status:** Accepted as architecture guidance; implementation gated

## Context

OpenClaw is a local-first personal AI assistant architecture centered on a
Gateway that coordinates sessions, tools, events, and messaging channels. It
also exposes tools, skills, plugins, nodes, and model providers as extension
points. The upstream project explicitly recommends plugins for new
capabilities and warns that inbound messages are untrusted and that tools may
run on the host unless sandboxing is configured.

SOMA has a broader ecosystem goal: shared capabilities must serve multiple
surfaces and providers without turning one transport, model provider,
protocol, or agent runtime into a core dependency.

## Decision

1. SOMA will treat an agent runtime such as OpenClaw as an **optional runtime
   adapter**, not as SOMA core.
2. SOMA will support **MCP (Model Context Protocol) as an optional capability
   and adapter boundary** for tool/context interoperability, not as the
   canonical SOMA domain model.
3. MCP servers will be governed as external capability providers behind the
   Agent Gateway and policy layer.
4. Agent Registry remains the authoritative catalog for approved agents and
   reusable capabilities.
5. Agent Runtime, Memory Bank, Agent Identity, Agent Gateway, Model Armor, and
   Agent Observability remain SOMA shared infrastructure contracts regardless
   of which runtime or protocol adapter is used.
6. MCP is not permission. Identity, authorization, data classification,
   jurisdiction, consent, safety, and audit policy are evaluated by SOMA
   before protected tool access.
7. An MCP server failure must not disable unrelated SOMA capabilities.
8. MCP tool outputs and instructions are untrusted input and must pass through
   validation/armor before being used for protected actions.

## MCP placement

```text
                         SOMA CORE
                            |
                   Shared Capability Kernel
                            |
                 Policy + Identity + Provenance
                            |
                     Agent Gateway
                            |
                      Model Armor
                            |
                     Agent Runtime
                            |
             +--------------+--------------+
             |                             |
        Native tools                 MCP adapter
                                           |
                                  +--------+--------+
                                  |        |        |
                               MCP S1   MCP S2    MCP S3
```

MCP is therefore a transport/interoperability boundary. The canonical SOMA
capability contract remains protocol-neutral.

## OpenClaw placement

OpenClaw may be integrated later as one possible agent runtime/channel
adapter. SOMA must remain operational without OpenClaw.

Potential mapping:

- OpenClaw Gateway -> SOMA Agent Runtime adapter;
- OpenClaw tools/skills/plugins -> SOMA capability adapters;
- OpenClaw channels -> SOMA transport adapters;
- OpenClaw local workspace/memory -> SOMA Memory Bank adapter where a governed
  contract can be enforced;
- OpenClaw security controls -> defense-in-depth only; SOMA policy remains the
  authority for protected ecosystem data.

The mapping is intentionally conceptual until an implementation is required.

## Enterprise and sovereignty boundary

For enterprise use, MCP servers must be classified by:

- owner and organization;
- execution environment;
- tenant scope;
- permitted data classes;
- data residency/jurisdiction;
- network egress policy;
- credentials and secret handling;
- retention and logging;
- tool risk level;
- approval requirement;
- version and integrity evidence.

A local MCP server is not automatically trusted merely because it runs locally.

## Long-running agents

MCP sessions must not be used as a substitute for durable execution state.
Long-running work belongs in Agent Runtime state, with re-authorization on
resume and idempotency for external side effects.

## User feedback and clarification

Agents must ask for missing context when a requested action is ambiguous or
high-impact. Feedback is recorded as governed evaluation/configuration input,
not as an immediate uncontrolled behavior change.

## Implementation gate

Do not add a concrete MCP server or OpenClaw integration until all of the
following are defined for that integration:

- capability contract;
- identity/authentication model;
- authorization scopes;
- data boundary;
- trust classification;
- tool risk classification;
- Model Armor checks;
- audit events;
- failure/degradation behavior;
- test suite;
- rollback/unregistration path;
- owner and lifecycle policy.

## Consequence

This approach lets SOMA benefit from the growing agent/tool ecosystem while
avoiding protocol or runtime lock-in. It also preserves the shared-first rule:
features are built once as governed capabilities, then exposed through the
appropriate adapters and surfaces.
