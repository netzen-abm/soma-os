use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use ring::{aead, rand::{SecureRandom, SystemRandom}};
use serde_json;
use crate::local_health_vault::LocalHealthVaultCrypto;
use super::contract::{
    EncryptedIndex, IndexPlaintext, StorageError, StoredBundle, INDEX_SCHEMA_VERSION,
    KEY_LEN, NONCE_LEN,
};

pub(crate) fn encrypt_index(
    key: &[u8; KEY_LEN], index: &IndexPlaintext,
) -> Result<EncryptedIndex, StorageError> {
    let aad = format!("soma-index-v1:{}:{}", index.record_id, index.key_ref);
    let unbound = aead::UnboundKey::new(&aead::AES_256_GCM, key)
        .map_err(|_| StorageError::KeyResolutionFailed)?;
    let sealing = aead::LessSafeKey::new(unbound);
    let mut nonce_bytes = [0u8; NONCE_LEN];
    SystemRandom::new().fill(&mut nonce_bytes)
        .map_err(|_| StorageError::NonceGenerationFailed)?;
    let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);
    let mut plaintext = serde_json::to_vec(index).map_err(|_| StorageError::Serialization)?;
    sealing.seal_in_place_append_tag(nonce, aead::Aad::from(aad.as_bytes()), &mut plaintext)
        .map_err(|_| StorageError::Serialization)?;
    Ok(EncryptedIndex {
        schema_version: INDEX_SCHEMA_VERSION.into(),
        record_id: index.record_id.clone(),
        key_ref: index.key_ref.clone(),
        nonce: BASE64.encode(nonce_bytes),
        ciphertext: BASE64.encode(plaintext),
    })
}

pub(crate) fn decrypt_index(
    key: &[u8; KEY_LEN], encrypted: &EncryptedIndex,
) -> Result<IndexPlaintext, StorageError> {
    if encrypted.schema_version != INDEX_SCHEMA_VERSION {
        return Err(StorageError::InvalidRecord);
    }
    let nonce_vec = BASE64.decode(&encrypted.nonce)
        .map_err(|_| StorageError::IndexIntegrityFailure)?;
    if nonce_vec.len() != NONCE_LEN {
        return Err(StorageError::IndexIntegrityFailure);
    }
    let nonce = aead::Nonce::try_assume_unique_for_key(&nonce_vec)
        .map_err(|_| StorageError::IndexIntegrityFailure)?;
    let mut ciphertext = BASE64.decode(&encrypted.ciphertext)
        .map_err(|_| StorageError::IndexIntegrityFailure)?;
    let aad = format!("soma-index-v1:{}:{}", encrypted.record_id, encrypted.key_ref);
    let opening = aead::LessSafeKey::new(
        aead::UnboundKey::new(&aead::AES_256_GCM, key)
            .map_err(|_| StorageError::KeyResolutionFailed)?,
    );
    let plaintext = opening.open_in_place(
        nonce, aead::Aad::from(aad.as_bytes()), &mut ciphertext,
    ).map_err(|_| StorageError::IndexIntegrityFailure)?;
    let index: IndexPlaintext = serde_json::from_slice(plaintext)
        .map_err(|_| StorageError::IndexIntegrityFailure)?;
    if index.record_id != encrypted.record_id || index.key_ref != encrypted.key_ref {
        return Err(StorageError::IndexIntegrityFailure);
    }
    Ok(index)
}

pub(crate) fn validate_binding(
    bundle: &StoredBundle, index: &IndexPlaintext,
) -> Result<(), StorageError> {
    if index.record_id != bundle.record.record_id
        || index.subject_ref != bundle.record.subject_ref
        || index.key_ref != bundle.record.key_ref
        || index.classification != bundle.record.classification
        || index.provenance_ref != bundle.record.provenance_ref
    {
        return Err(StorageError::IndexIntegrityFailure);
    }
    Ok(())
}

pub(crate) fn validate_record_crypto(
    key: &[u8; KEY_LEN], record: &crate::local_health_vault::LocalHealthVaultRecord,
) -> Result<(), StorageError> {
    LocalHealthVaultCrypto::decrypt_record(key, record)
        .map(|_| ())
        .map_err(|_| StorageError::RecordIntegrityFailure)
}
