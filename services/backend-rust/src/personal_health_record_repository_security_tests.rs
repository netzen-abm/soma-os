use super::*;
use crate::local_health_vault::LocalHealthVaultCrypto;
use crate::local_health_vault_storage::VaultAction;
use sha2::{Digest, Sha256};
use std::{
    cell::Cell,
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

const KEY_LEN: usize = 32;

#[derive(Clone)]
struct Keys {
    calls: Rc<Cell<usize>>,
    values: HashMap<String, [u8; KEY_LEN]>,
}

impl VaultKeyProvider for Keys {
    fn key_for(&self, key_ref: &str) -> Result<[u8; KEY_LEN], StorageError> {
        self.calls.set(self.calls.get() + 1);
        self.values.get(key_ref).copied().ok_or(StorageError::KeyResolutionFailed)
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
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    std::env::temp_dir().join(format!("soma-phr-repository-test-{nanos}"))
}

fn record(id: &str, subject: &str, entity_type: &str, created_at: &str) -> LocalHealthVaultRecord {
    LocalHealthVaultCrypto::encrypt_record(
        &[7u8; KEY_LEN],
        crate::local_health_vault::VaultRecordMetadata {
            record_id: id,
            subject_ref: subject,
            entity_type,
            content_type: "application/json",
            key_ref: "vault-key-v1",
            provenance_ref: "p1",
            created_at,
            updated_at: created_at,
        },
        br#"{"concept":"heart_rate","value":60}"#,
    )
    .unwrap()
}

fn repository(root: &Path) -> (LocalPersonalHealthRecordRepository<Keys, SubjectAuthorizer>, Rc<Cell<usize>>) {
    let calls = Rc::new(Cell::new(0));
    let mut values = HashMap::new();
    values.insert("vault-key-v1".into(), [7u8; KEY_LEN]);
    let vault = LocalFileVaultStore::new(
        root,
        Keys {
            calls: calls.clone(),
            values,
        },
        SubjectAuthorizer,
    )
    .unwrap();
    (LocalPersonalHealthRecordRepository::new(vault), calls)
}


#[test]
    fn missing_reference_fails_closed() {
        let root = root();
        let (repository, _) = repository(&root);
        let context = AuthorizationContext {
            principal_ref: "person-1".into(),
            subject_ref: "person-1".into(),
            scope: "self".into(),
        };
        assert_eq!(repository.put_reference("person-1-missing", &context), Err(RepositoryError::NotFound));
        fs::remove_dir_all(root).unwrap();
}


#[test]
    fn encrypted_bundle_does_not_persist_plaintext_payload() {
        let root = root();
        let (repository, _) = repository(&root);
        let context = AuthorizationContext {
            principal_ref: "person-1".into(),
            subject_ref: "person-1".into(),
            scope: "self".into(),
        };
        repository
            .vault
            .put(record("person-1-record-1", "person-1", "observation", "2026-09-10T00:00:00Z"), &context)
            .unwrap();
        let subject_hash = hex::encode(Sha256::digest(context.subject_ref.as_bytes()));
        let record_hash = hex::encode(Sha256::digest(b"person-1-record-1"));
        let bundle_path = root.join(subject_hash).join(format!("{record_hash}.bundle"));
        let bytes = fs::read(bundle_path).unwrap();
        let persisted = String::from_utf8_lossy(&bytes);
        assert!(!persisted.contains("heart_rate"));
        assert!(!persisted.contains("\"value\":60"));
        fs::remove_dir_all(root).unwrap();
}


#[test]
    fn timeline_is_deterministic_and_rebuild_does_not_create_a_second_source_of_truth() {
        let root = root();
        let (repository, _) = repository(&root);
        let context = AuthorizationContext {
            principal_ref: "person-1".into(),
            subject_ref: "person-1".into(),
            scope: "self".into(),
        };
        repository
            .vault
            .put(record("person-1-record-2", "person-1", "observation", "2026-09-11T00:00:00Z"), &context)
            .unwrap();
        repository
            .vault
            .put(record("person-1-record-1", "person-1", "observation", "2026-09-10T00:00:00Z"), &context)
            .unwrap();
        let timeline = repository.timeline(&context).unwrap();
        let rebuilt = repository.rebuild_index(&context).unwrap();
        assert_eq!(timeline[0].record_id, "person-1-record-1");
        assert_eq!(timeline[1].record_id, "person-1-record-2");
        assert_eq!(timeline, rebuilt);
        fs::remove_dir_all(root).unwrap();
}
