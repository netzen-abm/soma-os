use std::cmp::Ordering;

use thiserror::Error;

use crate::local_health_vault::LocalHealthVaultRecord;
use crate::local_health_vault_storage::{
    AuthorizationContext, AuthorizedIndexEntry, LocalFileVaultStore, StorageError, VaultAuthorizer, VaultKeyProvider,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RepositoryQuery {
    pub entity_type: Option<String>,
    pub classification: Option<String>,
    pub content_type: Option<String>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RepositoryError {
    #[error("authorization denied")]
    AuthorizationDenied,
    #[error("record not found")]
    NotFound,
    #[error("record is tombstoned")]
    Tombstoned,
    #[error("repository integrity failure")]
    IntegrityFailure,
    #[error("repository storage failure")]
    StorageFailure,
}

impl From<StorageError> for RepositoryError {
    fn from(error: StorageError) -> Self {
        match error {
            StorageError::AuthorizationDenied => Self::AuthorizationDenied,
            StorageError::NotFound => Self::NotFound,
            StorageError::Tombstoned => Self::Tombstoned,
            StorageError::IndexIntegrityFailure | StorageError::RecordIntegrityFailure => Self::IntegrityFailure,
            StorageError::InvalidRecord
            | StorageError::KeyResolutionFailed
            | StorageError::Io
            | StorageError::Serialization
            | StorageError::NonceGenerationFailed => Self::StorageFailure,
        }
    }
}

pub trait PersonalHealthRecordRepository {
    fn put_reference(&self, record_id: &str, context: &AuthorizationContext) -> Result<(), RepositoryError>;

    fn get(&self, record_id: &str, context: &AuthorizationContext) -> Result<LocalHealthVaultRecord, RepositoryError>;

    fn list(&self, context: &AuthorizationContext) -> Result<Vec<AuthorizedIndexEntry>, RepositoryError>;

    fn query(
        &self,
        context: &AuthorizationContext,
        query: &RepositoryQuery,
    ) -> Result<Vec<AuthorizedIndexEntry>, RepositoryError>;

    fn timeline(&self, context: &AuthorizationContext) -> Result<Vec<AuthorizedIndexEntry>, RepositoryError>;

    fn tombstone(&self, record_id: &str, context: &AuthorizationContext) -> Result<(), RepositoryError>;

    fn verify(&self, record_id: &str, context: &AuthorizationContext) -> Result<(), RepositoryError>;

    fn rebuild_index(&self, context: &AuthorizationContext) -> Result<Vec<AuthorizedIndexEntry>, RepositoryError>;
}

/// Provider-neutral PHR repository facade over the governed Local Health Vault.
///
/// The repository deliberately owns no second clinical payload store. Its query and
/// timeline projections are derived from the vault's protected index metadata. This
/// keeps Health State and other canonical records authoritative while allowing a
/// future persistent repository provider to implement the same trait.
pub struct LocalPersonalHealthRecordRepository<K, A> {
    vault: LocalFileVaultStore<K, A>,
}

impl<K, A> LocalPersonalHealthRecordRepository<K, A>
where
    K: VaultKeyProvider,
    A: VaultAuthorizer,
{
    pub fn new(vault: LocalFileVaultStore<K, A>) -> Self {
        Self {
            vault,
        }
    }
}

impl<K, A> PersonalHealthRecordRepository for LocalPersonalHealthRecordRepository<K, A>
where
    K: VaultKeyProvider,
    A: VaultAuthorizer,
{
    fn put_reference(&self, record_id: &str, context: &AuthorizationContext) -> Result<(), RepositoryError> {
        let entries = self.list(context)?;
        if !entries.iter().any(|entry| entry.record_id == record_id) {
            return Err(RepositoryError::NotFound);
        }
        self.vault.verify(record_id, context).map_err(Into::into)
    }

    fn get(&self, record_id: &str, context: &AuthorizationContext) -> Result<LocalHealthVaultRecord, RepositoryError> {
        self.vault.get(record_id, context).map_err(Into::into)
    }

    fn list(&self, context: &AuthorizationContext) -> Result<Vec<AuthorizedIndexEntry>, RepositoryError> {
        self.vault
            .list(context)
            .map(|entries| entries.into_iter().filter(|entry| entry.record_state == "ACTIVE").collect())
            .map_err(Into::into)
    }

    fn query(
        &self,
        context: &AuthorizationContext,
        query: &RepositoryQuery,
    ) -> Result<Vec<AuthorizedIndexEntry>, RepositoryError> {
        self.list(context).map(|entries| {
            entries
                .into_iter()
                .filter(|entry| {
                    query.entity_type.as_ref().is_none_or(|value| entry.entity_type == *value)
                        && query.classification.as_ref().is_none_or(|value| entry.classification == *value)
                        && query.content_type.as_ref().is_none_or(|value| entry.content_type == *value)
                })
                .collect()
        })
    }

    fn timeline(&self, context: &AuthorizationContext) -> Result<Vec<AuthorizedIndexEntry>, RepositoryError> {
        let mut entries = self.list(context)?;
        entries.sort_by(|left, right| match left.created_at.cmp(&right.created_at) {
            Ordering::Equal => left.record_id.cmp(&right.record_id),
            ordering => ordering,
        });
        Ok(entries)
    }

    fn tombstone(&self, record_id: &str, context: &AuthorizationContext) -> Result<(), RepositoryError> {
        self.vault.tombstone(record_id, context).map_err(Into::into)
    }

    fn verify(&self, record_id: &str, context: &AuthorizationContext) -> Result<(), RepositoryError> {
        self.vault.verify(record_id, context).map_err(Into::into)
    }

    fn rebuild_index(&self, context: &AuthorizationContext) -> Result<Vec<AuthorizedIndexEntry>, RepositoryError> {
        self.list(context)
    }
}

#[cfg(test)]
mod tests {
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
            subject_ref: "person-1".into(),
            scope: "self".into(),
        };
        let other = AuthorizationContext {
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

    #[test]
    fn missing_reference_fails_closed() {
        let root = root();
        let (repository, _) = repository(&root);
        let context = AuthorizationContext {
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
}
