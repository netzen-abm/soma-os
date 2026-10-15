use thiserror::Error;

use crate::local_health_vault_storage::StorageError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HealthStateEvidenceLinkRepositoryError {
    #[error("authorization denied")]
    AuthorizationDenied,
    #[error("invalid evidence-link context or operation")]
    InvalidContext,
    #[error("invalid evidence link")]
    InvalidLink,
    #[error("evidence-link serialization failed")]
    Serialization,
    #[error("vault cryptographic operation failed")]
    Cryptography,
    #[error("vault storage operation failed")]
    StorageFailure,
}

impl From<StorageError> for HealthStateEvidenceLinkRepositoryError {
    fn from(error: StorageError) -> Self {
        match error {
            StorageError::AuthorizationDenied => Self::AuthorizationDenied,
            StorageError::Serialization => Self::Serialization,
            StorageError::RecordIntegrityFailure | StorageError::IndexIntegrityFailure => Self::Cryptography,
            StorageError::NotFound
            | StorageError::Tombstoned
            | StorageError::InvalidRecord
            | StorageError::KeyResolutionFailed
            | StorageError::Io
            | StorageError::NonceGenerationFailed => Self::StorageFailure,
        }
    }
}
