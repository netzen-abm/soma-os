use super::{AuthorizedHealthStateAccessContext, HealthStateQuery, HealthStateRepository,
    HealthStateRepositoryError, LocalHealthStateRepository, HEALTH_STATE_CONTENT_TYPE};
use crate::canonical_authorization::{AuthorizationDecision, AuthorizationRequest, CanonicalAuthorizationBoundary};
use crate::local_health_vault::{LocalHealthVaultCrypto, LocalHealthVaultRecord, VaultRecordMetadata};
use crate::local_health_vault_storage::{AuthorizationContext, LocalFileVaultStore, StorageError, VaultAction, VaultAuthorizer, VaultKeyProvider};
use std::{cell::Cell, collections::HashMap, fs, path::PathBuf, rc::Rc, time::{SystemTime, UNIX_EPOCH}};

const KEY_LEN: usize = 32;

#[derive(Clone)] struct Keys { values: HashMap<String, [u8; KEY_LEN]>, calls: Rc<Cell<usize>> }
impl VaultKeyProvider for Keys {
    fn key_for(&self, key_ref: &str) -> Result<[u8; KEY_LEN], StorageError> {
        self.calls.set(self.calls.get() + 1);
        self.values.get(key_ref).copied().ok_or(StorageError::KeyResolutionFailed)
    }
}
#[derive(Clone, Copy)] struct SubjectAuthorizer;
impl VaultAuthorizer for SubjectAuthorizer {
    fn authorize(&self, context: &AuthorizationContext, record_id: &str, action: VaultAction) -> bool {
        match action { VaultAction::List => true, _ => record_id == "*" || record_id.starts_with(&context.subject_ref) }
    }
}
struct AllowBoundary;
impl CanonicalAuthorizationBoundary for AllowBoundary {
    fn authorize(&self, _request: &AuthorizationRequest) -> AuthorizationDecision { AuthorizationDecision::Allow }
}
fn root() -> PathBuf {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    std::env::temp_dir().join(format!("soma-health-state-repository-test-{nanos}"))
}
fn request() -> AuthorizationRequest {
    AuthorizationRequest {
        principal_ref: "principal-1".into(), subject_ref: "person-1".into(),
        capability_id: "health.state.write".into(), capability_version: "1.0.0".into(),
        resource_type: "health_state".into(), resource_id: "record-1".into(),
        action: "write".into(), tenant_id: "tenant-1".into(), data_domain: "personal_health".into(),
    }
}
fn record(entity_type: &str) -> LocalHealthVaultRecord {
    LocalHealthVaultCrypto::encrypt_record(
        &[7u8; KEY_LEN],
        VaultRecordMetadata {
            record_id: "person-1-record-1", subject_ref: "person-1", entity_type,
            content_type: HEALTH_STATE_CONTENT_TYPE, key_ref: "vault-key-v1",
            provenance_ref: "provenance-1", created_at: "2026-09-14T00:00:00Z",
            updated_at: "2026-09-14T00:00:00Z",
        },
        br#"{"id":"person-1-record-1","subject_ref":"person-1","entity_type":"observation","schema_version":"1.1.0","status":"active","recorded_time":"2026-09-14T00:00:00Z","concept":"heart_rate","uncertainty":"known","provenance":{"origin":"device","method":"measurement"},"classification":"personal_health"}"#,
    ).unwrap()
}
fn repository(root: &std::path::Path) -> (LocalHealthStateRepository<Keys, SubjectAuthorizer>, Rc<Cell<usize>>) {
    let calls = Rc::new(Cell::new(0));
    let mut values = HashMap::new();
    values.insert("vault-key-v1".into(), [7u8; KEY_LEN]);
    let vault = LocalFileVaultStore::new(root, Keys { values, calls: calls.clone() }, SubjectAuthorizer).unwrap();
    (LocalHealthStateRepository::new(vault), calls)
}

#[test]
fn authorized_context_is_minted_only_from_allow() {
    let context = AllowBoundary.authorize(&request());
    assert_eq!(context, AuthorizationDecision::Allow);
    let access = AuthorizedHealthStateAccessContext::from_authorized_request(&request()).unwrap();
    assert_eq!(access.subject_ref(), "person-1");
    assert_eq!(access.scope(), "tenant-1:personal_health");
}
#[test]
fn repository_accepts_canonical_health_state_entity_types() {
    let root = root(); let (repository, _) = repository(&root);
    let context = AuthorizedHealthStateAccessContext::from_authorized_request(&request()).unwrap();
    repository.put(record("observation"), &context).unwrap();
    let records = repository.query(&context, &HealthStateQuery::default()).unwrap();
    assert_eq!(records.len(), 1); assert_eq!(records[0].entity_type, "observation");
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn repository_rejects_non_health_state_entity_types() {
    let root = root(); let (repository, _) = repository(&root);
    let context = AuthorizedHealthStateAccessContext::from_authorized_request(&request()).unwrap();
    assert_eq!(repository.put(record("evidence_claim"), &context), Err(HealthStateRepositoryError::InvalidEntity));
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn timeline_does_not_fabricate_effective_time() {
    let root = root(); let (repository, _) = repository(&root);
    let context = AuthorizedHealthStateAccessContext::from_authorized_request(&request()).unwrap();
    repository.put(record("observation"), &context).unwrap();
    let timeline = repository.timeline(&context).unwrap();
    assert_eq!(timeline[0].effective_time, None);
    assert_eq!(timeline[0].recorded_at, "2026-09-14T00:00:00Z");
    fs::remove_dir_all(root).unwrap();
}
