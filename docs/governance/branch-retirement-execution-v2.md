# Branch Retirement Execution v2

## Target

Exactly nine persistent active branches:
- main
- development
- security/current
- architecture/current
- feature/current
- health/current
- research/current
- ai-agent/current
- integration/current

No obsolete branch is merged merely to reduce branch count.

## Retirement classification

Every non-canonical branch must be classified:
- MERGE_REQUIRED — unique, still-required implementation.
- PRESERVE_THEN_DELETE — unique historical/security/audit/research material; preserve provenance, then retire.
- REDUNDANT_DELETE — fully superseded or duplicate.
- UNKNOWN — insufficient evidence; do not delete.

Deletion requires comparison with main, review of unique commits/files, preservation of unique history, no open PR dependency, dependency confirmation, and an authorized GitHub branch-delete operation.

## Current state

All nine canonical branches exist.

The current repository inventory contains 33 non-canonical branches requiring controlled retirement:
arch/behavioral-evidence-epistemic-layer-v1
arch/canonical-governed-operation-contract-v1
arch/capability-registry-canonicalization-v1
architecture/health-media-intelligence-v1
architecture/local-health-vault-contract-v1
architecture/local-health-vault-storage-index-v1
audit/evidence-canonicalization-v1
design/canonical-domain-runtime-polyglot-v1-main
design/canonical-domain-runtime-polyglot-v1
design/db-constraints-trusted-context-rls-v1
design/hybrid-web-platform-v1
design/identity-authorization-enforcement-v1
design/legacy-scope-classification-quarantine-v1
design/protected-data-access-enforcement-v1
design/trusted-db-service-identity-context-v1
feat/canonical-principal-subject-scope
feat/canonical-principal-subject-scope-v2
feat/canonical-principal-subject-scope-v3
feat/canonical-principal-subject-scope-v4-clean
feat/python-authorization-subject-convergence-v1
feat/unpaywall-access-location-adapter-v1
feature/health-state-evidence-schema-v1
feature/policy-kernel-grant-identity-hardening
feature/shared-policy-kernel-hardening
feature/shared-policy-kernel-hardening-v2
fix/protected-db-context-rustfmt-v1
governance/automation-architecture-v1
impl/evidence-assessment-chain-hardening-v1
security/legacy-data-classification-quarantine-v1-clean
security/legacy-data-classification-quarantine-v1
security/protected-context-conformance-v1
security/protected-data-bypass-audit-v1-main
security/protected-data-bypass-audit-v1

## Safety constraint

The connected GitHub capability currently does not expose an authorized remote branch-delete mutation. Do not force-move refs to simulate deletion. Execute deletion only through an authorized branch-delete path and then verify the repository has exactly nine branches.

## Shared-infrastructure constraint

Retained work must converge into the single SOMA shared infrastructure. No parallel identity, authorization, policy, Health State, observation, evidence, safety, causality, intervention/response/outcome, provenance, or protected-data foundation may be introduced.
