use serde::{Deserialize, Serialize};
use thiserror::Error;

const SCHEMA_VERSION: &str = "1.0.0";
const CLASSIFICATION: &str = "sensitive_personal_health";
const ENCRYPTION_ALGORITHM: &str = "AES-256-GCM";
const INTEGRITY_ALGORITHM: &str = "AES-256-GCM-AEAD";

#[derive(Debug, Error, PartialEq, Eq)]
pub enum VaultError {
    #[error("vault key must be exactly 32 bytes")]
    InvalidKeyLength,
    #[error("vault record metadata is invalid")]
    InvalidMetadata,
    #[error("random nonce generation failed")]
    NonceGenerationFailed,
    #[error("vault record encryption failed")]
    EncryptionFailed,
    #[error("vault record authentication failed or record was tampered with")]
    AuthenticationFailed,
    #[error("vault record is unsupported or malformed")]
    InvalidEnvelope,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocalHealthVaultRecord {
    pub record_id: String,
    pub subject_ref: String,
    pub entity_type: String,
    pub schema_version: String,
    pub classification: String,
    pub content_type: String,
    pub ciphertext: String,
    pub nonce: String,
    pub key_ref: String,
    pub encryption_algorithm: String,
    pub integrity_algorithm: String,
    pub provenance_ref: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub tombstone: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VaultRecordMetadata<'a> {
    pub record_id: &'a str,
    pub subject_ref: &'a str,
    pub entity_type: &'a str,
    pub content_type: &'a str,
    pub key_ref: &'a str,
    pub provenance_ref: &'a str,
    pub created_at: &'a str,
    pub updated_at: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct AuthenticatedMetadata<'a> {
    pub(crate) record_id: &'a str,
    pub(crate) subject_ref: &'a str,
    pub(crate) entity_type: &'a str,
    pub(crate) schema_version: &'a str,
    pub(crate) classification: &'a str,
    pub(crate) content_type: &'a str,
    pub(crate) key_ref: &'a str,
    pub(crate) provenance_ref: &'a str,
    pub(crate) created_at: &'a str,
    pub(crate) updated_at: &'a str,
    pub(crate) tombstone: bool,
}

pub(crate) const SCHEMA: &str = SCHEMA_VERSION;
pub(crate) const CLASS: &str = CLASSIFICATION;
pub(crate) const ENCRYPTION: &str = ENCRYPTION_ALGORITHM;
pub(crate) const INTEGRITY: &str = INTEGRITY_ALGORITHM;
