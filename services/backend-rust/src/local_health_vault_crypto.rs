use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use ring::{
    aead,
    rand::{SecureRandom, SystemRandom},
};

use super::contract::{
    AuthenticatedMetadata, LocalHealthVaultRecord, VaultError, VaultRecordMetadata,
    CLASS, ENCRYPTION, INTEGRITY, SCHEMA,
};

const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;

pub struct LocalHealthVaultCrypto;

impl LocalHealthVaultCrypto {
    pub fn encrypt_record(
        key: &[u8],
        metadata: VaultRecordMetadata<'_>,
        plaintext: &[u8],
    ) -> Result<LocalHealthVaultRecord, VaultError> {
        if key.len() != KEY_LEN {
            return Err(VaultError::InvalidKeyLength);
        }
        let values = [
            metadata.record_id, metadata.subject_ref, metadata.entity_type,
            metadata.content_type, metadata.key_ref, metadata.provenance_ref,
            metadata.created_at, metadata.updated_at,
        ];
        if values.iter().any(|value| value.is_empty()) {
            return Err(VaultError::InvalidMetadata);
        }
        let authenticated = AuthenticatedMetadata {
            record_id: metadata.record_id,
            subject_ref: metadata.subject_ref,
            entity_type: metadata.entity_type,
            schema_version: SCHEMA,
            classification: CLASS,
            content_type: metadata.content_type,
            key_ref: metadata.key_ref,
            provenance_ref: metadata.provenance_ref,
            created_at: metadata.created_at,
            updated_at: metadata.updated_at,
            tombstone: false,
        };
        let aad = serde_json::to_vec(&authenticated).map_err(|_| VaultError::InvalidMetadata)?;
        let key = aead::LessSafeKey::new(
            aead::UnboundKey::new(&aead::AES_256_GCM, key)
                .map_err(|_| VaultError::InvalidKeyLength)?,
        );
        let mut nonce_bytes = [0u8; NONCE_LEN];
        SystemRandom::new().fill(&mut nonce_bytes).map_err(|_| VaultError::NonceGenerationFailed)?;
        let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);
        let mut ciphertext = plaintext.to_vec();
        key.seal_in_place_append_tag(nonce, aead::Aad::from(aad.as_slice()), &mut ciphertext)
            .map_err(|_| VaultError::EncryptionFailed)?;
        Ok(LocalHealthVaultRecord {
            record_id: metadata.record_id.to_string(),
            subject_ref: metadata.subject_ref.to_string(),
            entity_type: metadata.entity_type.to_string(),
            schema_version: SCHEMA.to_string(),
            classification: CLASS.to_string(),
            content_type: metadata.content_type.to_string(),
            ciphertext: BASE64.encode(ciphertext),
            nonce: BASE64.encode(nonce_bytes),
            key_ref: metadata.key_ref.to_string(),
            encryption_algorithm: ENCRYPTION.to_string(),
            integrity_algorithm: INTEGRITY.to_string(),
            provenance_ref: metadata.provenance_ref.to_string(),
            created_at: metadata.created_at.to_string(),
            updated_at: metadata.updated_at.to_string(),
            tombstone: false,
        })
    }

    pub fn decrypt_record(key: &[u8], record: &LocalHealthVaultRecord) -> Result<Vec<u8>, VaultError> {
        if key.len() != KEY_LEN || record.schema_version != SCHEMA || record.classification != CLASS
            || record.encryption_algorithm != ENCRYPTION || record.integrity_algorithm != INTEGRITY || record.tombstone {
            return Err(VaultError::InvalidEnvelope);
        }
        let nonce_vec = BASE64.decode(&record.nonce).map_err(|_| VaultError::InvalidEnvelope)?;
        if nonce_vec.len() != NONCE_LEN {
            return Err(VaultError::InvalidEnvelope);
        }
        let nonce = aead::Nonce::try_assume_unique_for_key(&nonce_vec)
            .map_err(|_| VaultError::InvalidEnvelope)?;
        let mut ciphertext = BASE64.decode(&record.ciphertext).map_err(|_| VaultError::InvalidEnvelope)?;
        let metadata = AuthenticatedMetadata {
            record_id: &record.record_id, subject_ref: &record.subject_ref, entity_type: &record.entity_type,
            schema_version: &record.schema_version, classification: &record.classification,
            content_type: &record.content_type, key_ref: &record.key_ref, provenance_ref: &record.provenance_ref,
            created_at: &record.created_at, updated_at: &record.updated_at, tombstone: record.tombstone,
        };
        let aad = serde_json::to_vec(&metadata).map_err(|_| VaultError::InvalidEnvelope)?;
        let key = aead::LessSafeKey::new(
            aead::UnboundKey::new(&aead::AES_256_GCM, key).map_err(|_| VaultError::InvalidKeyLength)?,
        );
        let plaintext = key.open_in_place(nonce, aead::Aad::from(aad.as_slice()), &mut ciphertext)
            .map_err(|_| VaultError::AuthenticationFailed)?;
        Ok(plaintext.to_vec())
    }
}
