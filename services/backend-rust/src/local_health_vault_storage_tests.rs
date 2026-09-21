use super::*;
use crate::local_health_vault::{LocalHealthVaultCrypto, VaultRecordMetadata};
use std::{cell::Cell, collections::HashMap, fs, path::PathBuf, rc::Rc, time::{SystemTime, UNIX_EPOCH}};

#[derive(Clone)]
struct Keys { calls: Rc<Cell<usize>>, values: HashMap<String, [u8; KEY_LEN]> }
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
        match action { VaultAction::List => true, _ => record_id == "*" || record_id.starts_with(&context.subject_ref) }
    }
}
fn root() -> PathBuf {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    std::env::temp_dir().join(format!("soma-vault-storage-test-{nanos}"))
}
fn record(id: &str, subject: &str) -> LocalHealthVaultRecord {
    LocalHealthVaultCrypto::encrypt_record(
        &[7u8; KEY_LEN],
        VaultRecordMetadata {
            record_id: id, subject_ref: subject, entity_type: "observation",
            content_type: "application/json", key_ref: "vault-key-v1", provenance_ref: "p1",
            created_at: "2026-09-10T00:00:00Z", updated_at: "2026-09-10T00:00:00Z",
        },
        br#"{"concept":"heart_rate","value":60}"#,
    ).unwrap()
}
fn store(root: &Path) -> (LocalFileVaultStore<Keys, SubjectAuthorizer>, Rc<Cell<usize>>) {
    let calls = Rc::new(Cell::new(0));
    let mut values = HashMap::new();
    values.insert("vault-key-v1".into(), [7u8; KEY_LEN]);
    (LocalFileVaultStore::new(root, Keys { calls: calls.clone(), values }, SubjectAuthorizer).unwrap(), calls)
}
fn context(principal: &str, subject: &str) -> AuthorizationContext {
    AuthorizationContext { principal_ref: principal.into(), subject_ref: subject.into(), scope: "self".into() }
}
#[test]
fn actor_and_subject_are_distinct() {
    let context = context("principal-1", "person-1");
    assert_eq!(context.principal_ref, "principal-1");
    assert_eq!(context.subject_ref, "person-1");
    assert_ne!(context.principal_ref, context.subject_ref);
}
#[test]
fn put_get_round_trip_and_list_hide_plaintext() {
    let root = root(); let (store, _) = store(&root);
    let context = context("principal-1", "person-1");
    store.put(record("person-1-record-1", "person-1"), &context).unwrap();
    let got = store.get("person-1-record-1", &context).unwrap();
    assert!(!got.ciphertext.contains("heart_rate"));
    let entries = store.list(&context).unwrap();
    assert_eq!(entries.len(), 1); assert_eq!(entries[0].subject_ref, "person-1");
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn unauthorized_get_denies_before_key_resolution() {
    let root = root(); let (store, calls) = store(&root);
    let owner = context("principal-1", "person-1"); let other = context("principal-2", "person-2");
    store.put(record("person-1-record-1", "person-1"), &owner).unwrap();
    calls.set(0);
    assert_eq!(store.get("person-1-record-1", &other), Err(StorageError::AuthorizationDenied));
    assert_eq!(calls.get(), 0); fs::remove_dir_all(root).unwrap();
}
#[test]
fn tombstone_blocks_reads_and_updates_index() {
    let root = root(); let (store, _) = store(&root);
    let context = context("principal-1", "person-1");
    store.put(record("person-1-record-1", "person-1"), &context).unwrap();
    store.tombstone("person-1-record-1", &context).unwrap();
    assert_eq!(store.get("person-1-record-1", &context), Err(StorageError::Tombstoned));
    assert_eq!(store.list(&context).unwrap()[0].record_state, "TOMBSTONED");
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn ciphertext_tampering_fails_integrity() {
    let root = root(); let (store, _) = store(&root);
    let context = context("principal-1", "person-1");
    store.put(record("person-1-record-1", "person-1"), &context).unwrap();
    let path = store.path_for(&context.subject_ref, "person-1-record-1");
    let mut bundle: contract::StoredBundle = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    let mut bytes = base64::engine::general_purpose::STANDARD.decode(&bundle.record.ciphertext).unwrap();
    bytes[0] ^= 1; bundle.record.ciphertext = base64::engine::general_purpose::STANDARD.encode(bytes);
    fs::write(&path, serde_json::to_vec(&bundle).unwrap()).unwrap();
    assert_eq!(store.verify("person-1-record-1", &context), Err(StorageError::RecordIntegrityFailure));
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn index_binding_tampering_fails_integrity() {
    let root = root(); let (store, _) = store(&root);
    let context = context("principal-1", "person-1");
    store.put(record("person-1-record-1", "person-1"), &context).unwrap();
    let path = store.path_for(&context.subject_ref, "person-1-record-1");
    let mut bundle: contract::StoredBundle = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    bundle.index.record_id = "person-1-record-2".into();
    fs::write(&path, serde_json::to_vec(&bundle).unwrap()).unwrap();
    assert_eq!(store.verify("person-1-record-1", &context), Err(StorageError::IndexIntegrityFailure));
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn interrupted_temp_bundle_is_not_listed() {
    let root = root(); let (store, _) = store(&root);
    let context = context("principal-1", "person-1");
    let tmp = store.path_for(&context.subject_ref, "person-1-record-1").with_extension("bundle.tmp-interrupted");
    fs::create_dir_all(tmp.parent().unwrap()).unwrap();
    fs::write(tmp, b"partial").unwrap();
    assert!(store.list(&context).unwrap().is_empty());
    fs::remove_dir_all(root).unwrap();
}
