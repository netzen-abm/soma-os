use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::local_health_vault::LocalHealthVaultRecord;

pub const NONCE_LEN: usize = 12;
pub const KEY_LEN: usize = 32;
pub const INDEX_SCHEMA_VERSION: &str = "1.0.0";

pub trait VaultKeyProvider {
    fn key_for(&self, key_ref: &str) -> Result<[u8; KEY_LEN], StorageError>;
}
pub trait VaultAuthorizer {
    fn authorize(&self, context: &AuthorizationContext, record_id: &str, action: VaultAction) -> bool;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationContext {
    pub principal_ref: String,
    pub subject_ref: String,
    pub scope: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultAction { Write, Read, List, Tombstone, Verify }

#[derive(Debug, Error, PartialEq, Eq)]
pub enum StorageError {
    #[error("authorization denied")] AuthorizationDenied,
    #[error("record not found")] NotFound,
    #[error("record is tombstoned")] Tombstoned,
    #[error("invalid storage record")] InvalidRecord,
    #[error("index integrity failure")] IndexIntegrityFailure,
    #[error("record integrity failure")] RecordIntegrityFailure,
    #[error("key resolution failed")] KeyResolutionFailed,
    #[error("storage I/O failure")] Io,
    #[error("serialization failure")] Serialization,
    #[error("random nonce generation failed")] NonceGenerationFailed,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct EncryptedIndex {
    pub schema_version: String,
    pub record_id: String,
    pub key_ref: String,
    pub nonce: String,
    pub ciphertext: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct IndexPlaintext {
    pub record_id: String,
    pub subject_ref: String,
    pub entity_type: String,
    pub schema_version: String,
    pub classification: String,
    pub content_type: String,
    pub key_ref: String,
    pub provenance_ref: String,
    pub created_at: String,
    pub updated_at: String,
    pub record_state: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct StoredBundle {
    pub record: LocalHealthVaultRecord,
    pub index: EncryptedIndex,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedIndexEntry {
    pub record_id: String,
    pub subject_ref: String,
    pub entity_type: String,
    pub schema_version: String,
    pub classification: String,
    pub content_type: String,
    pub key_ref: String,
    pub provenance_ref: String,
    pub created_at: String,
    pub updated_at: String,
    pub record_state: String,
}
impl From<IndexPlaintext> for AuthorizedIndexEntry {
    fn from(value: IndexPlaintext) -> Self {
        Self {
            record_id: value.record_id, subject_ref: value.subject_ref,
            entity_type: value.entity_type, schema_version: value.schema_version,
            classification: value.classification, content_type: value.content_type,
            key_ref: value.key_ref, provenance_ref: value.provenance_ref,
            created_at: value.created_at, updated_at: value.updated_at,
            record_state: value.record_state,
        }
    }
}
