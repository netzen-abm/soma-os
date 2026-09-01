# SOMA MCP Adapter Contract

**Status:** Foundational contract
**Version:** 0.1.0

MCP (Model Context Protocol) is a shared interoperability capability for
connecting governed SOMA agents and applications to external tools, resources,
and prompts. MCP is not the SOMA authorization model and is not a trust
boundary.

## Placement

```text
Agent
  ↓
Agent Identity
  ↓
Policy Decision
  ↓
Model Armor
  ↓
Agent Gateway
  ↓
MCP Adapter
  ↓
MCP Server
```

## Server registration

Every MCP server must have a registry record containing:

- stable server ID and version;
- owner and lifecycle status;
- transport type;
- endpoint/environment classification;
- trust classification;
- permitted tenants and jurisdictions;
- data classes handled;
- authentication mechanism;
- available capabilities/tools/resources/prompts;
- risk classification;
- approval requirements;
- health status;
- integrity/version evidence;
- audit policy;
- rollback and unregister procedure.

## Discovery

Discovery is capability- and policy-aware. A discovered tool is not
necessarily authorized for the requesting agent or user.

```text
registered
    ↓
available
    ↓
policy-eligible
    ↓
authorized
    ↓
invocable
```

## Tool execution

Before invocation, SOMA must establish:

1. agent identity;
2. acting user/delegator identity where applicable;
3. requested capability and tool;
4. input data classification;
5. tenant and jurisdiction scope;
6. authorization decision;
7. risk and approval requirements;
8. Model Armor validation;
9. audit correlation ID.

MCP tool descriptions, prompts, resource contents, and tool results are
untrusted inputs. They must not override SOMA policy.

## Production data

MCP servers must not receive unrestricted production credentials. Protected
access should use narrowly scoped, short-lived authorization issued through
the Agent Gateway or an equivalent policy-controlled mechanism.

A local MCP server is not implicitly trusted.

## Failure isolation

MCP server failure must be isolated from unrelated SOMA capabilities. The
adapter must expose explicit health and failure states and support timeout,
cancellation, circuit breaking, and controlled retry where appropriate.

Retries for side-effecting operations require idempotency protection.

## Observability

Each invocation must produce structured audit metadata including:

- correlation ID;
- execution ID;
- agent ID/version;
- MCP server ID/version;
- tool identifier/version where available;
- policy decision;
- approval decision;
- start/end timestamps;
- outcome/error class.

Do not persist unrestricted prompts, secrets, PII, or chain-of-thought as
telemetry merely for debugging.

## MCP Apps

Interactive MCP Apps may be supported as an adapter extension. UI delivery
must remain subject to the same identity, policy, data, and safety controls as
tool invocation.

## MCP server mode

SOMA may later expose selected shared capabilities as MCP servers. Such
servers must expose only explicitly approved capability contracts and must
not bypass SOMA's authorization, provenance, privacy, safety, or audit rules.

## Implementation gate

No production MCP integration is complete until it has:

- contract tests;
- authentication tests;
- authorization tests;
- malicious/untrusted-content tests;
- data-boundary tests;
- jurisdiction tests where applicable;
- failure and timeout tests;
- idempotency tests for side effects;
- audit-event tests;
- registration/rollback tests.
