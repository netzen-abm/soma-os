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
    fn repository_is_reference_based_and_queries_derived_metadata() {
        let root = root();
        let (repository, _) = repository(&root);
        let context = AuthorizationContext {
            principal_ref: "person-1".into(),
            subject_ref: "person-1".into(),
            scope: "self".into(),
        };
        let vault = record("person-1-record-1", "person-1", "observation", "2026-09-10T00:00:00Z");
        repository.vault.put(vault, &context).unwrap();
        repository.put_reference("person-1-record-1", &context).unwrap();
        let entries = repository
            .query(
                &context,
                &RepositoryQuery {
                    entity_type: Some("observation".into()),
                    ..Default::default()
                },
            )
            .unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].record_id, "person-1-record-1");
        fs::remove_dir_all(root).unwrap();
}


#[test]
    fn unauthorized_reference_verification_happens_before_key_resolution() {
        let root = root();
        let (repository, calls) = repository(&root);
        let owner = AuthorizationContext {
            principal_ref: "person-1".into(),
            subject_ref: "person-1".into(),
            scope: "self".into(),
        };
        let other = AuthorizationContext {
            principal_ref: "person-2".into(),
            subject_ref: "person-2".into(),
            scope: "self".into(),
        };
        repository
            .vault
            .put(record("person-1-record-1", "person-1", "observation", "2026-09-10T00:00:00Z"), &owner)
            .unwrap();
        calls.set(0);
        assert_eq!(repository.put_reference("person-1-record-1", &other), Err(RepositoryError::AuthorizationDenied));
        assert_eq!(calls.get(), 0);
        fs::remove_dir_all(root).unwrap();
}


#[test]
    fn tombstoned_records_are_not_visible_or_registrable() {
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
        repository.tombstone("person-1-record-1", &context).unwrap();
        assert!(repository.list(&context).unwrap().is_empty());
        assert_eq!(repository.put_reference("person-1-record-1", &context), Err(RepositoryError::NotFound));
        assert!(matches!(repository.get("person-1-record-1", &context), Err(RepositoryError::Tombstoned)));
        fs::remove_dir_all(root).unwrap();
}

