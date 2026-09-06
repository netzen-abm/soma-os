# SOMA — Trusted DB Service Identity & Transaction Context v1

**Status:** Design baseline — implementation gate
**Baseline:** `main` after canonical runtime architecture v1

## 1. Decision

SOMA must establish a trusted database service-identity boundary before PostgreSQL RLS is enabled for protected data.

The canonical security chain remains:

`IdentityContext → Identity Authorization Enforcement → Policy Kernel → ProtectedDataAccess → trusted DB context → PostgreSQL`

PostgreSQL constraints and RLS are defense-in-depth. They do not become the authorization authority.

## 2. Threat model

The primary threat is scope spoofing at the database boundary.

A caller must not gain protected access merely by supplying `tenant_id` or `data_domain`, setting transaction-local PostgreSQL configuration, choosing a transport, invoking an agent, or using a model-generated value.

The following are untrusted unless explicitly converted by the canonical authorization path:

- caller-supplied tenant/data-domain metadata;
- HTTP/Telegram/WhatsApp/MCP/agent transport metadata;
- client-side claims;
- model or AI output;
- search results;
- arbitrary application SQL;
- arbitrary database sessions.

## 3. Trust boundary

Only an approved SOMA persistence service identity may establish protected transaction context.

The service identity is distinct from the end-user/application principal represented by `IdentityContext`.

Conceptually:

```text
Authenticated principal
        ↓
Verified IdentityContext
        ↓
Policy Kernel authorization
        ↓
ProtectedDataAccess
        ↓
Approved persistence service identity
        ↓
Trusted transaction context
        ↓
PostgreSQL
```

No lower layer may promote untrusted metadata into security authority.

## 4. Database role model

The implementation should separate at least these operational roles:

### Application/persistence role

- used by the approved SOMA persistence path;
- limited to required protected-data operations;
- must not own protected tables;
- must not have migration/DDL privileges;
- must not be an unrestricted administrative role;
- must not expose credentials to end users or untrusted surfaces.

### Migration/owner role

- owns schema objects or performs controlled migrations;
- is not used by normal application requests;
- is operationally separated from the application role;
- access is explicitly audited.

### Administrative/operational role

- reserved for controlled maintenance and incident response;
- must not silently become an end-user authorization bypass;
- privileged access requires an explicit operational policy and audit trail.

Exact PostgreSQL role names and deployment credentials belong to environment configuration, never committed source.

## 5. Trusted context establishment

The existing transaction-local context mechanism is useful for isolation but is not, by itself, a trust proof.

The implementation must establish a mechanism whereby:

1. the canonical application authorization path has already succeeded;
2. the request is executed by the approved persistence identity;
3. the scope values are derived from the verified authorization context;
4. only the approved persistence path can establish protected transaction context;
5. malformed, partial, conflicting, or absent scope fails closed;
6. context is transaction-local;
7. rollback and transaction completion cannot leak context to another request.

The mechanism may use a tightly controlled PostgreSQL function, role privilege boundary, or equivalent design, but the implementation must prove why an arbitrary application session cannot forge the same trust signal.

## 6. SECURITY DEFINER consideration

A `SECURITY DEFINER` function may be appropriate if it is the narrowest safe mechanism for establishing trusted transaction context.

If used, the implementation must explicitly verify:

- fixed, trusted `search_path`;
- owner is a controlled role;
- `EXECUTE` is granted only to the intended application role;
- `PUBLIC` cannot execute it;
- function arguments are validated;
- no dynamic SQL is required where avoidable;
- the function cannot be used to escalate privileges or alter unrelated security state;
- rollback clears transaction-local state;
- function ownership and deployment are separately controlled.

A SECURITY DEFINER function must not be treated as automatically safe merely because it is defined by a privileged role.

## 7. Context representation

The protected context must contain exactly the scope required by the protected data boundary:

- `tenant_id`
- `data_domain`

The implementation must not accept a caller-provided "trusted" flag, arbitrary role name, model confidence, transport identity, or similar assertion as a substitute for database trust.

The database context is an enforcement input, not an end-user identity.

## 8. Transaction and Unit-of-Work boundary

The protected transaction must conceptually encompass:

`authorization decision → trusted context establishment → persistence operations → commit`

For multi-record protected writes, the eventual provider-neutral Unit-of-Work contract must preserve atomicity across related records.

A successful transaction must never be interpreted as proof that authorization occurred.

## 9. Required adversarial tests

### Identity and authorization

- unverified identity cannot establish protected context;
- expired/revoked identity cannot establish protected context;
- caller metadata cannot broaden scope;
- wrong tenant/domain cannot establish an authorized context.

### Database trust

- arbitrary application SQL cannot set a trusted context accepted by RLS;
- direct `set_config` attempts do not create trusted authorization state;
- unauthorized database role cannot invoke context-establishing privileged functions;
- migration/owner role is not used by normal application persistence;
- missing context denies protected access;
- malformed/partial context denies access.

### Transaction isolation

- rollback removes effective context;
- pooled connections do not leak context;
- concurrent transactions remain isolated;
- a second transaction cannot observe another transaction's scope.

### Data isolation

- wrong tenant is denied;
- wrong domain is denied;
- same hash across tenants remains isolated;
- same hash across domains remains isolated;
- legacy NULL-scope rows remain inaccessible through protected paths.

## 10. Credential and deployment boundary

This design does not prescribe a particular secret manager, but production deployment must provide credentials through environment/secret management rather than repository source.

The implementation must explicitly distinguish:

- application runtime credentials;
- migration credentials;
- operational/admin credentials;
- test database credentials.

Hard-coded production-like database credentials are a separate security defect and must not be introduced or preserved as part of this boundary.

## 11. Acceptance criteria

This gate is complete only when evidence demonstrates:

1. a normal protected request reaches PostgreSQL through the approved persistence identity;
2. the trusted context cannot be forged by caller or transport metadata;
3. direct SQL cannot self-establish equivalent trusted scope;
4. database privileges prevent unauthorized context establishment;
5. role separation is documented and testable;
6. transaction-local isolation and rollback behavior are verified;
7. concurrent pool reuse remains isolated;
8. migration/admin access is separated from normal application access;
9. no credentials are committed to source;
10. the design remains compatible with the canonical Policy Kernel and ProtectedDataAccess boundary;
11. RLS is still treated as defense-in-depth rather than the primary authorization authority.

## 12. Explicit non-goals

This gate does not:

- replace IdentityContext;
- replace the Policy Kernel;
- infer tenant/domain from database state;
- enable final RLS policies automatically;
- make PostgreSQL the canonical authorization authority;
- define end-user authentication providers;
- define delegation;
- silently classify legacy data;
- declare protected persistence product-ready.

## 13. Implementation order

1. Verify the current main schema and persistence path.
2. Define PostgreSQL role/privilege ownership and deployment assumptions.
3. Implement the narrowest trusted-context establishment mechanism.
4. Add direct-SQL and privilege-boundary adversarial tests.
5. Re-run existing isolation tests to prevent regression.
6. Only after trust is proven, introduce final constraints and then controlled RLS.
7. Update product-readiness from evidence only.

## 14. Status

**Design baseline:** accepted as the prerequisite for database-level protected-data enforcement.

**Implementation status:** not yet implemented.

**Security principle:** transaction-local does not mean trusted; trust must be established by an authenticated and privilege-controlled persistence boundary.
