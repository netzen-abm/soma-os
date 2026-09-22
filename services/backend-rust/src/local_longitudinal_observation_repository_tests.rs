use super::*;


use super::*;
use crate::canonical_authorization::AuthorizationRequest;
use crate::local_health_vault::{LocalHealthVaultCrypto, VaultRecordMetadata};
use crate::local_health_vault_storage::VaultAction;
use std::{
    collections::HashMap,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

const KEY: [u8; 32] = [7u8; 32];

struct Keys(HashMap<String, [u8; 32]>);

impl VaultKeyProvider for Keys {
    fn key_for(&self, key_ref: &str) -> Result<[u8; 32], StorageError> {
        self.0.get(key_ref).copied().ok_or(StorageError::KeyResolutionFailed)
    }
}

#[derive(Clone, Copy)]
struct SubjectAuthorizer;

impl VaultAuthorizer for SubjectAuthorizer {
    fn authorize(&self, context: &AuthorizationContext, record_id: &str, action: VaultAction) -> bool {
        match action {
            VaultAction::List => true,
            _ => record_id == "*" || record_id.starts_with(&context.subject_ref),
        }
    }
}

fn root() -> PathBuf {
    std::env::temp_dir().join(format!(
        "soma-longitudinal-observation-test-{}",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ))
}

fn context() -> AuthorizedObservationAccessContext {
    AuthorizedObservationAccessContext::from_authorized_request(&AuthorizationRequest {
        principal_ref: "principal-1".into(),
        subject_ref: "person-1".into(),
        capability_id: "health.read".into(),
        capability_version: "1.0.0".into(),
        resource_type: "health_record".into(),
        resource_id: "record-1".into(),
        action: "read".into(),
        tenant_id: "tenant-1".into(),
        data_domain: "personal_health".into(),
    })
    .unwrap()
}

fn observation() -> LocalHealthVaultRecord {
    LocalHealthVaultCrypto::encrypt_record(
        &KEY,
        VaultRecordMetadata {
            record_id: "person-1-observation-1",
            subject_ref: "person-1",
            entity_type: "observation",
            content_type: "application/json",
            key_ref: "key-1",
            provenance_ref: "provenance-1",
            created_at: "2026-09-13T00:00:00Z",
            updated_at: "2026-09-13T00:00:00Z",
        },
        br#"{"id":"person-1-observation-1","entity_type":"observation","concept":"heart_rate"}"#,
    )
    .unwrap()
}

fn repository(root: &PathBuf) -> LocalLongitudinalObservationRepository<Keys, SubjectAuthorizer> {
    let mut keys = HashMap::new();
    keys.insert("key-1".into(), KEY);
    LocalLongitudinalObservationRepository::new(
        LocalFileVaultStore::new(root, Keys(keys), SubjectAuthorizer).unwrap(),
    )
}

#[test]
fn adapter_reads_observation_from_existing_vault() {
    let root = root();
    let repository = repository(&root);
    let context = context();
    let vault_context = LocalLongitudinalObservationRepository::<Keys, SubjectAuthorizer>::vault_context(&context);
    repository.vault.put(observation(), &vault_context).unwrap();
    assert!(repository.get("person-1-observation-1", &context).is_ok());
    assert_eq!(repository.put_reference("person-1-observation-1", &context), Ok(()));
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn timeline_keeps_observed_time_unknown() {
    let root = root();
    let repository = repository(&root);
    let context = context();
    let vault_context = LocalLongitudinalObservationRepository::<Keys, SubjectAuthorizer>::vault_context(&context);
    repository.vault.put(observation(), &vault_context).unwrap();
    let timeline = repository.timeline(&context).unwrap();
    assert_eq!(timeline.len(), 1);
    assert_eq!(timeline[0].observed_at, None);
    assert_eq!(timeline[0].recorded_at, "2026-09-13T00:00:00Z");
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn non_observation_entity_is_rejected() {
    let root = root();
    let repository = repository(&root);
    let context = context();
    let vault_context = LocalLongitudinalObservationRepository::<Keys, SubjectAuthorizer>::vault_context(&context);
    let record = LocalHealthVaultCrypto::encrypt_record(
        &KEY,
        VaultRecordMetadata {
            record_id: "person-1-other-1",
            subject_ref: "person-1",
            entity_type: "other_health_entity",
            content_type: "application/json",
            key_ref: "key-1",
            provenance_ref: "provenance-1",
            created_at: "2026-09-13T00:00:00Z",
            updated_at: "2026-09-13T00:00:00Z",
        },
        b"{}",
    )
    .unwrap();
    repository.vault.put(record, &vault_context).unwrap();
    assert_eq!(
        repository.put_reference("person-1-other-1", &context),
        Err(ObservationRepositoryError::InvalidObservation)
    );
    std::fs::remove_dir_all(root).unwrap();
}
