use thiserror::Error;

use crate::local_health_vault_storage::StorageError;

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
    fn put_reference(&self, record_id: &str, context: &crate::local_health_vault_storage::AuthorizationContext) -> Result<(), RepositoryError>;
    fn get(&self, record_id: &str, context: &crate::local_health_vault_storage::AuthorizationContext) -> Result<crate::local_health_vault::LocalHealthVaultRecord, RepositoryError>;
    fn list(&self, context: &crate::local_health_vault_storage::AuthorizationContext) -> Result<Vec<crate::local_health_vault_storage::AuthorizedIndexEntry>, RepositoryError>;
    fn query(&self, context: &crate::local_health_vault_storage::AuthorizationContext, query: &RepositoryQuery) -> Result<Vec<crate::local_health_vault_storage::AuthorizedIndexEntry>, RepositoryError>;
    fn timeline(&self, context: &crate::local_health_vault_storage::AuthorizationContext) -> Result<Vec<crate::local_health_vault_storage::AuthorizedIndexEntry>, RepositoryError>;
    fn tombstone(&self, record_id: &str, context: &crate::local_health_vault_storage::AuthorizationContext) -> Result<(), RepositoryError>;
    fn verify(&self, record_id: &str, context: &crate::local_health_vault_storage::AuthorizationContext) -> Result<(), RepositoryError>;
    fn rebuild_index(&self, context: &crate::local_health_vault_storage::AuthorizationContext) -> Result<Vec<crate::local_health_vault_storage::AuthorizedIndexEntry>, RepositoryError>;
}
