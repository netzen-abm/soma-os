# SOMA Shareable Infrastructure Security Binding v1

## Purpose

This contract defines how SOMA's single shared infrastructure binds capability use, purpose-bound permission, canonical authorization, protected execution, and audit without creating a second authorization engine or a second protected-data model.

The design is ecosystem infrastructure. Web, mobile, device, messaging, MCP, AI, agent, research, and future protocol surfaces are adapters over the same governed operation and security substrate.

## Canonical flow

Surface / Agent / Workflow → Governed Capability Operation → Capability Registry + Permission Lifecycle → Authorization + Policy Decision Boundary → Canonical Authorization Decision → Protected Execution → Protected Data / Health State or device/external capability adapter → Outcome + Audit.

## Existing canonical contracts

SOMA already has distinct contracts for capability inventory, governed operation, purpose-bound permission lifecycle, authorization and policy decision boundary, protected health data and Health State, and semantic convergence. These contracts are complementary. They must not become parallel authority models.

## Security invariants

1. One authorization authority: policy evaluation remains in the canonical Authorization + Policy Decision Boundary / Policy Kernel path.
2. Permission is not authorization: a permission lifecycle record expresses a purpose-bound grant and its lifecycle; it does not bypass policy.
3. Purpose and scope are explicit: a surface cannot widen requested scope after a grant.
4. Automatic revocation is mandatory: sensitive capability access becomes inactive after expiry or completion; re-use requires a new governed operation.
5. Fresh consent for re-grant: a later operation cannot silently reuse a completed device/data grant when fresh consent is required.
6. Protected execution remains subject-bound: protected data APIs consume canonical authorized context rather than caller-supplied authority strings.
7. Adapters are replaceable: device, protocol, AI, agent, research, and surface implementations cannot create their own protected-data authority.
8. AI cannot self-authorize: model output is an input to governed workflows, not an authorization decision.
9. No second Health State: Health State remains the canonical personal-health model.
10. No second protected store: device or surface adapters must use the existing protected-data boundary.

## Permission lifecycle

REQUESTED → GRANTED → ACTIVE → REVOKED / EXPIRED.

Completion-driven revocation is required for temporary capability access. Expiry is a hard upper bound. Re-grant creates a new permission identity and must pass through the normal governed-operation and consent path.

Examples include camera, microphone, location, contacts/address book, and other sensitive device capabilities. The infrastructure is capability-neutral; each adapter declares the exact capability and minimum scope it requires.

## Architectural boundary

The shared infrastructure owns identity references and authority context; capability/version resolution; permission lifecycle semantics; policy evaluation; canonical authorization decision; protected execution context; provenance/audit references; and fail-closed behavior.

A surface owns only presentation, user interaction, local platform integration, and adapter-specific mechanics. It must not own a competing permission, authorization, Health State, or protected-data authority.

## Implementation rule

Before adding a new security primitive, first prove that the requirement cannot be represented by an existing canonical contract. New implementations must name their canonical owner and adapter boundary.

Line count alone is not a refactoring criterion. Split code only when a real responsibility, lifecycle, dependency, or security boundary warrants it.

## Validation

The repository must continuously validate the nine canonical branch names; semantic ownership and authorization convergence; capability permission lifecycle invariants; governed operation required authority fields; and protected-data subject binding.

Branch hygiene validation is intentionally non-destructive. Physical branch deletion requires an authorized GitHub branch-delete capability; force-moving a branch is not an acceptable substitute.

## Next implementation seam

The next bounded implementation layer is to bind the existing permission lifecycle contract to governed-operation execution semantics, with explicit completion revocation and fresh-consent enforcement at the shared execution boundary. This should extend existing contracts rather than introduce another permission or authorization engine.
