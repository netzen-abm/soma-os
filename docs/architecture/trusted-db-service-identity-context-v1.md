# Trusted DB Service Identity and Transaction Context v1

**Status:** Design gate — proposed
**Baseline:** `main` after PostgreSQL protected-data isolation v1, legacy classification/quarantine v1, protected-data bypass audit v1, and canonical runtime architecture v1.
**Purpose:** Establish the trust boundary required before final protected-data constraints and PostgreSQL Row-Level Security (RLS) are enabled.

## 1. Decision

SOMA separates four security layers:

`Authenticated Principal → IdentityContext → Policy Kernel → Protected Data Access → Trusted DB Service Identity → PostgreSQL`

The database role is **not** an end-user identity and does not replace application authorization.

A protected transaction may establish tenant/data-domain context only through an approved SOMA persistence path. An arbitrary application caller, transport, model, agent, or end user must not be able to self-assert protected scope by directly setting PostgreSQL session or transaction configuration.

PostgreSQL RLS remains defense in depth. The Policy Kernel remains the canonical authorization authority.

## 2. Threat model

The boundary must resist:

- forged caller metadata;
- forged transport metadata;
- model/AI-inferred tenant or domain;
- direct SQL through an application connection;
- arbitrary `set_config` attempts;
- cross-tenant access using an identical pseudonymous hash;
- cross-domain access using an identical pseudonymous hash;
- connection-pool scope leakage;
- background jobs using end-user scope without an explicit identity path;
- service-to-service calls that lack verified service identity;
- privileged migration/admin access being confused with application access.

A caller-supplied `tenant_id` or `data_domain` is never authoritative merely because it is syntactically valid.

## 3. Trust chain

### 3.1 Application boundary

The application authenticates the principal and constructs a canonical `IdentityContext` from verified authentication provenance.

### 3.2 Authorization boundary

The identity context is evaluated against the canonical `PolicyRequest` through Identity Authorization Enforcement and the Policy Kernel.

### 3.3 Protected data boundary

`ProtectedDataAccess` accepts only an authorized operation with explicit target tenant and data-domain binding. It must not infer either dimension from hashes, transport, model output, or caller metadata.

### 3.4 Database trust boundary

The persistence adapter converts the already-authorized target into a trusted database transaction context. This conversion is an internal capability of the approved persistence service, not a public caller-controlled operation.

The database must be configured so that the ordinary application service identity cannot arbitrarily create a trusted context outside that path.

## 4. Database identity classes

At minimum, SOMA should distinguish:

| Identity | Purpose | Application request access | Schema/migration authority |
|---|---|---:|---:|
| Application service role | Normal protected persistence | Yes | No |
| Migration/owner role | Schema changes and controlled migrations | No | Yes |
| Operational/admin role | Explicit privileged operations | No by default | Controlled |
| Test role/environment | Integration verification | Test-only | Test-only |

Exact role names and deployment credentials are implementation details and must not be hard-coded into source control.

The application service role must not own protected tables if table ownership would create an unintended RLS bypass. The migration/owner role must be separately governed.

## 5. Trusted context establishment

The implementation must choose and document a mechanism that makes context establishment authoritative.

Acceptable design candidates include a narrowly scoped database function or equivalent trusted boundary whose execution privilege is granted only to the approved persistence service role. The mechanism must ensure that arbitrary direct SQL cannot impersonate the trusted path merely by setting the same GUC keys.

Requirements:

1. context values originate from the verified authorization boundary;
2. the caller cannot provide a second, conflicting scope that wins;
3. partial or malformed scope fails closed;
4. context is transaction-local;
5. rollback removes the effective context;
6. a pooled connection cannot retain a previous request's scope;
7. missing context produces denial under future RLS policies;
8. the trusted mechanism is not exposed as an end-user capability;
9. service identity and end-user identity remain separate;
10. background/service-to-service access follows an explicit identity contract.

## 6. Direct SQL resistance

Before RLS is enabled, tests must demonstrate that the ordinary application path cannot bypass the intended trust boundary by:

- executing `set_config` directly for protected scope;
- selecting protected rows without trusted context;
- selecting another tenant's rows by changing local configuration;
- using a matching `anonymized_user_hash` across tenants/domains;
- inserting partial or forged scope;
- using a second connection from the same pool to reuse another request's scope.

The security invariant is:

> **Knowing the context key names is not sufficient to acquire protected authority.**

## 7. Transaction contract

The protected persistence sequence is:

`authorization decision → trusted context establishment → protected operations → commit`

A transaction must never grant authorization merely because it successfully begins or commits.

For multi-record protected writes, the eventual provider-neutral Unit-of-Work contract must preserve this sequence atomically.

On failure:

- the transaction rolls back;
- no partial protected mutation is committed;
- trusted context is no longer effective after transaction termination;
- a pooled connection is safe for the next request.

## 8. Background and service-to-service identity

Background jobs and internal services must not borrow an end-user identity implicitly.

Each such actor needs:

- an explicit principal type (`agent`, `service`, or `application` as appropriate);
- verified authentication provenance;
- explicit tenant/data-domain scope;
- an authorization decision under the same Policy Kernel;
- auditable operation identity.

A scheduler, queue, worker, AI model, or transport must not become an authorization authority by itself.

## 9. Administrative and migration access

Migration/owner access is operationally privileged and must be treated as a separate trust domain.

The design must document:

- who can obtain migration/admin credentials;
- how production administrative access is audited;
- whether privileged roles bypass RLS by PostgreSQL semantics;
- how privileged access is prevented from being used as ordinary application access;
- how emergency access is revoked and reviewed.

No credentials belong in Git history, Docker manifests, tests, or committed documentation.

## 10. Relationship to RLS

RLS should be enabled only after the trusted context mechanism is verified.

Conceptually, protected rows will be visible only when:

`row.tenant_id = trusted_transaction_tenant AND row.data_domain = trusted_transaction_domain`

The exact implementation must also define behavior for:

- NULL context;
- malformed context;
- wrong tenant/domain;
- INSERT;
- SELECT;
- UPDATE;
- DELETE;
- table owners;
- migration roles;
- administrative roles.

Missing or invalid trusted context must deny access. There must be no fail-open fallback to unrestricted visibility.

## 11. Legacy data boundary

Rows with NULL tenant/data-domain scope remain transition data.

The trusted DB boundary must not promote such rows. Only the explicit verified legacy classification/promotion contract may establish protected eligibility.

No default scope may be assigned solely to satisfy database constraints.

## 12. Required verification

### Positive

- authorized same-tenant/same-domain read succeeds;
- authorized scoped write succeeds;
- transaction-local context is visible only within its transaction;
- concurrent transactions remain isolated.

### Negative

- unverified identity denied;
- expired/revoked identity denied;
- wrong tenant denied;
- wrong data domain denied;
- missing tenant denied;
- missing domain denied;
- forged caller metadata denied;
- direct `set_config` cannot acquire trusted protected authority;
- direct SQL without trusted context cannot read protected rows;
- cross-tenant identical hashes do not produce protected existence access;
- rollback does not leak context;
- pool reuse does not leak context;
- legacy NULL-scope rows remain inaccessible through protected paths.

### Privilege/integrity

- application role cannot perform schema/migration operations;
- application role cannot become an unintended table owner/RLS bypass;
- trusted context mechanism is callable only by the intended service role/path;
- privileged access is separately auditable;
- no production credentials are committed.

## 13. Deployment implications

Deployment must supply database roles and credentials through the platform's secret-management mechanism. Repository configuration may define placeholders and required environment-variable names, but must not contain real credentials.

Production rollout should be staged:

1. establish roles and privileges;
2. deploy trusted context mechanism;
3. run adversarial direct-SQL tests;
4. verify pool/concurrency/rollback behavior;
5. verify privileged-role boundaries;
6. introduce final constraints;
7. enable RLS in a controlled enforcement phase;
8. verify backup/restore and operational recovery.

## 14. Non-goals

This design does not:

- replace the Policy Kernel;
- define end-user authentication providers;
- define delegation;
- automatically classify legacy records;
- enable RLS immediately;
- make PostgreSQL the canonical authorization authority;
- solve all credential rotation or secret-management operations.

## 15. Gate status

**Proposed design gate.**

Implementation may proceed only after the design is reviewed against the existing IdentityContext, Identity Authorization Enforcement, Protected Data Access, PostgreSQL isolation, legacy classification, and bypass-audit contracts.

The next implementation gate is a minimal trusted database service identity + trusted transaction context proof, followed by final constraints and RLS only after adversarial verification succeeds.
