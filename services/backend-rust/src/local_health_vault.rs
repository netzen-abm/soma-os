use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use ring::{
    aead,
    rand::{SecureRandom, SystemRandom},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;
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
struct AuthenticatedMetadata<'a> {
    record_id: &'a str,
    subject_ref: &'a str,
    entity_type: &'a str,
    schema_version: &'a str,
    classification: &'a str,
    content_type: &'a str,
    key_ref: &'a str,
    provenance_ref: &'a str,
    created_at: &'a str,
    updated_at: &'a str,
    tombstone: bool,
}

pub struct LocalHealthVaultCrypto;

impl LocalHealthVaultCrypto {
    /// Encrypt one canonical health record payload. Key lifecycle and platform
    /// keystore integration are intentionally outside this primitive.
    pub fn encrypt_record(
        key: &[u8],
        key_ref: &str,
        record_id: &str,
        subject_ref: &str,
        entity_type: &str,
        content_type: &str,
        provenance_ref: &str,
        created_at: &str,
        updated_at: &str,
        plaintext: &[u8],
    ) -> Result<LocalHealthVaultRecord, VaultError> {
        if key.len() != KEY_LEN {
            return Err(VaultError::InvalidKeyLength);
        }
        if [record_id, subject_ref, entity_type, content_type, key_ref, provenance_ref, created_at, updated_at]
            .iter()
            .any(|value| value.is_empty())
        {
            return Err(VaultError::InvalidMetadata);
        }

        let metadata = AuthenticatedMetadata {
            record_id,
            subject_ref,
            entity_type,
            schema_version: SCHEMA_VERSION,
            classification: CLASSIFICATION,
            content_type,
            key_ref,
            provenance_ref,
            created_at,
            updated_at,
            tombstone: false,
        };
        let aad = serde_json::to_vec(&metadata).map_err(|_| VaultError::InvalidMetadata)?;

        let unbound = aead::UnboundKey::new(&aead::AES_256_GCM, key).map_err(|_| VaultError::InvalidKeyLength)?;
        let sealing_key = aead::LessSafeKey::new(unbound);
        let rng = SystemRandom::new();
        let mut nonce_bytes = [0u8; NONCE_LEN];
        rng.fill(&mut nonce_bytes).map_err(|_| VaultError::NonceGenerationFailed)?;
        let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);

        let mut ciphertext = plaintext.to_vec();
        sealing_key
            .seal_in_place_append_tag(nonce, aead::Aad::from(aad.as_slice()), &mut ciphertext)
            .map_err(|_| VaultError::EncryptionFailed)?;

        Ok(LocalHealthVaultRecord {
            record_id: record_id.to_string(),
            subject_ref: subject_ref.to_string(),
            entity_type: entity_type.to_string(),
            schema_version: SCHEMA_VERSION.to_string(),
            classification: CLASSIFICATION.to_string(),
            content_type: content_type.to_string(),
            ciphertext: BASE64.encode(ciphertext),
            nonce: BASE64.encode(nonce_bytes),
            key_ref: key_ref.to_string(),
            encryption_algorithm: ENCRYPTION_ALGORITHM.to_string(),
            integrity_algorithm: INTEGRITY_ALGORITHM.to_string(),
            provenance_ref: provenance_ref.to_string(),
            created_at: created_at.to_string(),
            updated_at: updated_at.to_string(),
            tombstone: false,
        })
    }

    /// Authenticate metadata and decrypt a record. Any metadata alteration is
    /// treated as tampering because the metadata is authenticated as AAD.
    pub fn decrypt_record(key: &[u8], record: &LocalHealthVaultRecord) -> Result<Vec<u8>, VaultError> {
        if key.len() != KEY_LEN
            || record.schema_version != SCHEMA_VERSION
            || record.classification != CLASSIFICATION
            || record.encryption_algorithm != ENCRYPTION_ALGORITHM
            || record.integrity_algorithm != INTEGRITY_ALGORITHM
            || record.tombstone
        {
            return Err(VaultError::InvalidEnvelope);
        }

        let nonce_vec = BASE64.decode(&record.nonce).map_err(|_| VaultError::InvalidEnvelope)?;
        if nonce_vec.len() != NONCE_LEN {
            return Err(VaultError::InvalidEnvelope);
        }
        let nonce = aead::Nonce::try_assume_unique_for_key(&nonce_vec).map_err(|_| VaultError::InvalidEnvelope)?;
        let mut ciphertext = BASE64.decode(&record.ciphertext).map_err(|_| VaultError::InvalidEnvelope)?;
        let metadata = AuthenticatedMetadata {
            record_id: &record.record_id,
            subject_ref: &record.subject_ref,
            entity_type: &record.entity_type,
            schema_version: &record.schema_version,
            classification: &record.classification,
            content_type: &record.content_type,
            key_ref: &record.key_ref,
            provenance_ref: &record.provenance_ref,
            created_at: &record.created_at,
            updated_at: &record.updated_at,
            tombstone: record.tombstone,
        };
        let aad = serde_json::to_vec(&metadata).map_err(|_| VaultError::InvalidEnvelope)?;
        let unbound = aead::UnboundKey::new(&aead::AES_256_GCM, key).map_err(|_| VaultError::InvalidKeyLength)?;
        let opening_key = aead::LessSafeKey::new(unbound);
        let plaintext = opening_key
            .open_in_place(nonce, aead::Aad::from(aad.as_slice()), &mut ciphertext)
            .map_err(|_| VaultError::AuthenticationFailed)?;
        Ok(plaintext.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: [u8; KEY_LEN] = [7u8; KEY_LEN];
    const WRONG_KEY: [u8; KEY_LEN] = [8u8; KEY_LEN];

    fn sample() -> LocalHealthVaultRecord {
        LocalHealthVaultCrypto::encrypt_record(
            &KEY,
            "vault-key-v1",
            "record-1",
            "person-1",
            "observation",
            "application/json",
            "provenance-1",
            "2026-09-08T00:00:00Z",
            "2026-09-08T00:00:00Z",
            br#"{"concept":"heart_rate","value":60,"unit":"bpm"}"#,
        )
        .unwrap()
    }

    #[test]
    fn round_trip_preserves_plaintext() {
        let record = sample();
        let plaintext = LocalHealthVaultCrypto::decrypt_record(&KEY, &record).unwrap();
        assert_eq!(plaintext, br#"{"concept":"heart_rate","value":60,"unit":"bpm"}"#);
        assert!(!record.ciphertext.contains("heart_rate"));
    }

    #[test]
    fn wrong_key_fails_closed() {
        let record = sample();
        assert_eq!(LocalHealthVaultCrypto::decrypt_record(&WRONG_KEY, &record), Err(VaultError::AuthenticationFailed));
    }

    #[test]
    fn ciphertext_tampering_fails() {
        let mut record = sample();
        let mut bytes = BASE64.decode(&record.ciphertext).unwrap();
        bytes[0] ^= 1;
        record.ciphertext = BASE64.encode(bytes);
        assert_eq!(LocalHealthVaultCrypto::decrypt_record(&KEY, &record), Err(VaultError::AuthenticationFailed));
    }

    #[test]
    fn metadata_tampering_fails() {
        let mut record = sample();
        record.subject_ref = "person-2".to_string();
        assert_eq!(LocalHealthVaultCrypto::decrypt_record(&KEY, &record), Err(VaultError::AuthenticationFailed));
    }

    #[test]
    fn invalid_key_length_is_rejected() {
        let error = LocalHealthVaultCrypto::encrypt_record(
            &[0u8; 31],
            "key",
            "record",
            "person",
            "observation",
            "application/json",
            "provenance",
            "2026-09-08T00:00:00Z",
            "2026-09-08T00:00:00Z",
            b"secret",
        )
        .unwrap_err();
        assert_eq!(error, VaultError::InvalidKeyLength);
    }

    #[test]
    fn nonce_is_randomized() {
        let a = sample();
        let b = sample();
        assert_ne!(a.nonce, b.nonce);
        assert_ne!(a.ciphertext, b.ciphertext);
    }
}
