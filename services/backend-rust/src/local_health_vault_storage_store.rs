use std::{fs, io::Write, path::{Path, PathBuf}};
use sha2::{Digest, Sha256};
use crate::local_health_vault::LocalHealthVaultRecord;
use super::contract::{
    AuthorizedIndexEntry, AuthorizationContext, IndexPlaintext, StorageError, StoredBundle,
    VaultAction, VaultAuthorizer, VaultKeyProvider,
};
use super::crypto::{decrypt_index, encrypt_index, validate_binding, validate_record_crypto};

pub struct LocalFileVaultStore<K, A> {
    root: PathBuf,
    key_provider: K,
    authorizer: A,
}
impl<K, A> LocalFileVaultStore<K, A>
where K: VaultKeyProvider, A: VaultAuthorizer
{
    pub fn new(root: impl AsRef<Path>, key_provider: K, authorizer: A) -> Result<Self, StorageError> {
        fs::create_dir_all(root.as_ref()).map_err(|_| StorageError::Io)?;
        Ok(Self { root: root.as_ref().to_path_buf(), key_provider, authorizer })
    }
    pub fn put(&self, record: LocalHealthVaultRecord, context: &AuthorizationContext) -> Result<(), StorageError> {
        if !self.authorizer.authorize(context, &record.record_id, VaultAction::Write)
            || context.subject_ref != record.subject_ref || record.tombstone {
            return Err(StorageError::AuthorizationDenied);
        }
        let key = self.key_provider.key_for(&record.key_ref)
            .map_err(|_| StorageError::KeyResolutionFailed)?;
        validate_record_crypto(&key, &record)?;
        let index = IndexPlaintext {
            record_id: record.record_id.clone(), subject_ref: record.subject_ref.clone(),
            entity_type: record.entity_type.clone(), schema_version: record.schema_version.clone(),
            classification: record.classification.clone(), content_type: record.content_type.clone(),
            key_ref: record.key_ref.clone(), provenance_ref: record.provenance_ref.clone(),
            created_at: record.created_at.clone(), updated_at: record.updated_at.clone(),
            record_state: "ACTIVE".into(),
        };
        let bundle = StoredBundle { record, index: encrypt_index(&key, &index)? };
        let bytes = serde_json::to_vec(&bundle).map_err(|_| StorageError::Serialization)?;
        atomic_write(&self.path_for(&bundle.record.subject_ref, &bundle.record.record_id), &bytes)
    }
    pub fn get(&self, record_id: &str, context: &AuthorizationContext) -> Result<LocalHealthVaultRecord, StorageError> {
        if !self.authorizer.authorize(context, record_id, VaultAction::Read) {
            return Err(StorageError::AuthorizationDenied);
        }
        let bundle = self.load(&context.subject_ref, record_id)?;
        let key = self.key_provider.key_for(&bundle.record.key_ref)
            .map_err(|_| StorageError::KeyResolutionFailed)?;
        let index = decrypt_index(&key, &bundle.index)?;
        validate_binding(&bundle, &index)?;
        if index.record_state == "TOMBSTONED" || bundle.record.tombstone {
            return Err(StorageError::Tombstoned);
        }
        validate_record_crypto(&key, &bundle.record)?;
        Ok(bundle.record)
    }
    pub fn list(&self, context: &AuthorizationContext) -> Result<Vec<AuthorizedIndexEntry>, StorageError> {
        if !self.authorizer.authorize(context, "*", VaultAction::List) {
            return Err(StorageError::AuthorizationDenied);
        }
        let subject_dir = self.subject_path(&context.subject_ref);
        let directory = match fs::read_dir(subject_dir) {
            Ok(directory) => directory,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(_) => return Err(StorageError::Io),
        };
        let mut entries = Vec::new();
        for item in directory {
            let path = item.map_err(|_| StorageError::Io)?.path();
            if !path.is_file() || path.extension().and_then(|v| v.to_str()) != Some("bundle") { continue; }
            let bundle: StoredBundle = serde_json::from_slice(&fs::read(&path).map_err(|_| StorageError::Io)?)
                .map_err(|_| StorageError::InvalidRecord)?;
            let key = self.key_provider.key_for(&bundle.record.key_ref)
                .map_err(|_| StorageError::KeyResolutionFailed)?;
            let index = decrypt_index(&key, &bundle.index)?;
            validate_binding(&bundle, &index)?;
            if index.subject_ref == context.subject_ref
                && self.authorizer.authorize(context, &index.record_id, VaultAction::Read) {
                entries.push(index.into());
            }
        }
        Ok(entries)
    }
    pub fn tombstone(&self, record_id: &str, context: &AuthorizationContext) -> Result<(), StorageError> {
        if !self.authorizer.authorize(context, record_id, VaultAction::Tombstone) {
            return Err(StorageError::AuthorizationDenied);
        }
        let mut bundle = self.load(&context.subject_ref, record_id)?;
        let key = self.key_provider.key_for(&bundle.record.key_ref)
            .map_err(|_| StorageError::KeyResolutionFailed)?;
        let mut index = decrypt_index(&key, &bundle.index)?;
        validate_binding(&bundle, &index)?;
        if index.subject_ref != context.subject_ref { return Err(StorageError::AuthorizationDenied); }
        bundle.record.tombstone = true;
        index.record_state = "TOMBSTONED".into();
        bundle.index = encrypt_index(&key, &index)?;
        let bytes = serde_json::to_vec(&bundle).map_err(|_| StorageError::Serialization)?;
        atomic_write(&self.path_for(&context.subject_ref, record_id), &bytes)
    }
    pub fn verify(&self, record_id: &str, context: &AuthorizationContext) -> Result<(), StorageError> {
        if !self.authorizer.authorize(context, record_id, VaultAction::Verify) {
            return Err(StorageError::AuthorizationDenied);
        }
        let bundle = self.load(&context.subject_ref, record_id)?;
        let key = self.key_provider.key_for(&bundle.record.key_ref)
            .map_err(|_| StorageError::KeyResolutionFailed)?;
        let index = decrypt_index(&key, &bundle.index)?;
        validate_binding(&bundle, &index)?;
        if !bundle.record.tombstone { validate_record_crypto(&key, &bundle.record)?; }
        Ok(())
    }
    fn load(&self, subject_ref: &str, record_id: &str) -> Result<StoredBundle, StorageError> {
        let bytes = fs::read(self.path_for(subject_ref, record_id)).map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound { StorageError::NotFound } else { StorageError::Io }
        })?;
        serde_json::from_slice(&bytes).map_err(|_| StorageError::InvalidRecord)
    }
    fn subject_path(&self, subject_ref: &str) -> PathBuf {
        self.root.join(hex::encode(Sha256::digest(subject_ref.as_bytes())))
    }
    fn path_for(&self, subject_ref: &str, record_id: &str) -> PathBuf {
        self.subject_path(subject_ref).join(format!("{}.bundle", hex::encode(Sha256::digest(record_id.as_bytes()))))
    }
}
fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), StorageError> {
    if let Some(parent) = path.parent() { fs::create_dir_all(parent).map_err(|_| StorageError::Io)?; }
    let tmp = path.with_extension(format!("bundle.tmp-{}-{}",
        std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| StorageError::Io)?.as_nanos()));
    let mut file = fs::File::create(&tmp).map_err(|_| StorageError::Io)?;
    if file.write_all(bytes).is_err() || file.sync_all().is_err() {
        let _ = fs::remove_file(&tmp); return Err(StorageError::Io);
    }
    if fs::rename(&tmp, path).is_err() {
        let _ = fs::remove_file(&tmp); return Err(StorageError::Io);
    }
    Ok(())
}
