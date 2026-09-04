# SOMA-OS Decision — Policy Kernel Resource Scope v1

**Date:** 2026-09-04  
**Status:** Proposed for review  
**Branch:** `feature/policy-kernel-resource-scope-v1`

## Decision

Strengthen the shared Policy Kernel so authorization grants bind to the
concrete `resource_id`, not only the resource type.

The grant identity becomes:

```text
principal_id
principal_type
capability_id
resource_type
resource_id
action
```

This is a narrow security hardening step. It does not claim to implement
complete production authorization.

## Why

The previous evaluator validated `resource_id` in the request but omitted it
from the grant lookup key. Consequently, one positive grant for a resource
type could authorize another resource instance of the same type. That is an
authorization-isolation defect.

## Boundary

The evaluator remains deterministic, side-effect free, and fail-closed.
Caller context cannot create an `ALLOW` or expand resource scope.

The following remain intentionally separate future contracts:

- tenant/data-domain isolation;
- consent lifecycle and revocation;
- human-review lifecycle;
- credential/session lifecycle;
- service-to-service identity;
- grant expiry;
- delegation;
- grant provenance;
- policy decision audit persistence.

## Compatibility

The change is breaking for the in-memory grant contract and therefore bumps
the Policy Kernel contract version from `0.2.0` to `0.3.0`. Existing v0.2 grant
stores must be explicitly migrated or adapted; they must not be silently
interpreted as v0.3 grants.

## Verification requirement

Before merge, CI must demonstrate:

1. exact resource instance is allowed when explicitly granted;
2. a different `resource_id` is denied;
3. principal identity/type, capability, resource type, and action remain
   isolated;
4. malformed capability declarations fail closed;
5. caller context cannot alter resource scope;
6. existing deny/consent/review/degrade semantics remain covered.

No forced merge is permitted. The change should be integrated only after the
normal CI and security-review gates are satisfied.
