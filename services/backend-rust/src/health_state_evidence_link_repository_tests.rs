use super::*;
use crate::canonical_authorization::{AuthorizationDecision, AuthorizationRequest, CanonicalAuthorizationBoundary};
use crate::local_health_vault_storage::{AuthorizationContext, VaultAction};
use std::{
    fs,
    path::PathBuf,
    sync::{Arc, atomic::{AtomicBool, Ordering}},
    time::{SystemTime, UNIX_EPOCH},
};

const KEY: [u8; 32] = [7u8; 32];

struct Keys;
impl VaultKeyProvider for Keys {
    fn key_for(&self, key_ref: &str) -> Result<[u8; 32], StorageError> {
        if key_ref == "vault-key-v1" { Ok(KEY) } else { Err(StorageError::KeyResolutionFailed) }
    }
}

struct AllowDomain;
impl CanonicalAuthorizationBoundary for AllowDomain {
    fn authorize(&self, request: &AuthorizationRequest) -> AuthorizationDecision {
        if request.principal_ref.trim().is_empty()
            || request.subject_ref.trim().is_empty()
            || request.tenant_id.trim().is_empty()
            || request.data_domain.trim().is_empty()
        {
            AuthorizationDecision::Deny
        } else {
            AuthorizationDecision::Allow
        }
    }
}

struct VaultPolicy {
    allow: AtomicBool,
}
impl VaultAuthorizer for VaultPolicy {
    fn authorize(&self, context: &AuthorizationContext, _record_id: &str, _action: VaultAction) -> bool {
        self.allow.load(Ordering::SeqCst)
            && !context.principal_ref.is_empty()
            && !context.subject_ref.is_empty()
            && !context.scope.is_empty()
            && context.subject_ref == "person-1"
    }
}

fn root() -> PathBuf {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    std::env::temp_dir().join(format!("soma-evidence-link-repository-test-{nanos}"))
}

fn request(action: &str) -> AuthorizationRequest {
    AuthorizationRequest {
        principal_ref: "clinician-1".into(),
        subject_ref: "person-1".into(),
        capability_id: format!("health.evidence.link.{action}"),
        capability_version: "1.0.0".into(),
        resource_type: ENTITY_TYPE.into(),
        resource_id: "link-1".into(),
        action: action.into(),
        tenant_id: "tenant-1".into(),
        data_domain: "personal_health".into(),
    }
}

fn link() -> HealthStateEvidenceLink {
    HealthStateEvidenceLink {
        id: "link-1".into(),
        schema_version: crate::health_state_evidence_link::contract::SCHEMA_VERSION.into(),
        health_state_ref: "observation-1".into(),
        evidence_ref: "evidence-1".into(),
        relationship: crate::health_state_evidence_link::HealthStateEvidenceRelationship::EvidenceInformsHypothesis,
        provenance: crate::health_state_evidence_link::LinkProvenance {
            method: "human-curated".into(),
            created_at: Some("2026-10-15T04:00:00Z".into()),
            actor_ref: Some("clinician-1".into()),
        },
        context: None,
        uncertainty: Some(crate::health_state_evidence_link::LinkUncertainty::Reported),
    }
}

fn repository(root: &std::path::Path, allow_vault: bool) -> HealthStateEvidenceLinkRepository<Keys, VaultPolicy> {
    HealthStateEvidenceLinkRepository::new(
        root,
        Arc::new(Keys),
        VaultPolicy { allow: AtomicBool::new(allow_vault) },
        "vault-key-v1",
    ).unwrap()
}

fn authorized(action: &str) -> AuthorizedHealthStateEvidenceLinkContext {
    HealthStateEvidenceLinkRepository::<Keys, VaultPolicy>::authorize_context(&AllowDomain, &request(action)).unwrap()
}

#[test]
fn authorized_write_readback_preserves_link_provenance_and_uncertainty() {
    let root = root();
    let repository = repository(&root, true);
    let write = authorized("write");
    let original = link();
    repository.put(&original, &write).unwrap();

    let read = authorized("read");
    let restored = repository.get("link-1", &read).unwrap();
    assert_eq!(restored, original);
    assert_eq!(restored.provenance.actor_ref.as_deref(), Some("clinician-1"));
    assert_eq!(restored.uncertainty, Some(crate::health_state_evidence_link::LinkUncertainty::Reported));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn denied_vault_write_does_not_persist_record() {
    let root = root();
    let repository = repository(&root, false);
    let write = authorized("write");
    assert_eq!(
        repository.put(&link(), &write),
        Err(HealthStateEvidenceLinkRepositoryError::AuthorizationDenied)
    );

    // A separate repository with vault access still cannot read a record that
    // the denied write failed to persist.
    let allowed_repository = repository(&root, true);
    assert_eq!(
        allowed_repository.get("link-1", &authorized("read")),
        Err(HealthStateEvidenceLinkRepositoryError::StorageFailure)
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn operation_context_must_match_action_and_resource() {
    let root = root();
    let repository = repository(&root, true);
    let wrong_action = authorized("read");
    assert_eq!(
        repository.put(&link(), &wrong_action),
        Err(HealthStateEvidenceLinkRepositoryError::InvalidContext)
    );
    let mut wrong_resource_request = request("write");
    wrong_resource_request.resource_id = "different-link".into();
    let wrong_resource = HealthStateEvidenceLinkRepository::<Keys, VaultPolicy>::authorize_context(
        &AllowDomain,
        &wrong_resource_request,
    ).unwrap();
    assert_eq!(
        repository.put(&link(), &wrong_resource),
        Err(HealthStateEvidenceLinkRepositoryError::InvalidContext)
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn cross_subject_read_is_denied_by_vault_scope() {
    let root = root();
    let repository = repository(&root, true);
    repository.put(&link(), &authorized("write")).unwrap();
    let mut other_request = request("read");
    other_request.subject_ref = "person-2".into();
    let other_context = HealthStateEvidenceLinkRepository::<Keys, VaultPolicy>::authorize_context(
        &AllowDomain,
        &other_request,
    ).unwrap();
    assert_eq!(
        repository.get("link-1", &other_context),
        Err(HealthStateEvidenceLinkRepositoryError::AuthorizationDenied)
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn invalid_link_is_rejected_before_vault_write() {
    let root = root();
    let repository = repository(&root, true);
    let mut invalid = link();
    invalid.evidence_ref.clear();
    assert_eq!(
        repository.put(&invalid, &authorized("write")),
        Err(HealthStateEvidenceLinkRepositoryError::InvalidLink)
    );
    assert_eq!(
        repository.get("link-1", &authorized("read")),
        Err(HealthStateEvidenceLinkRepositoryError::StorageFailure)
    );
    fs::remove_dir_all(root).unwrap();
}
