//! Conformance tests for protected vault authorization composition.
//!
//! These tests deliberately exercise the existing LocalFileVaultStore boundary
//! without introducing another authorization implementation into production.

use crate::local_health_vault::{LocalHealthVaultCrypto, VaultRecordMetadata};
use crate::local_health_vault_storage::{
    AuthorizationContext, LocalFileVaultStore, StorageError, VaultAction, VaultAuthorizer,
    VaultKeyProvider,
};
use sha2::Digest;
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
        self.values
            .get(key_ref)
            .copied()
            .ok_or(StorageError::KeyResolutionFailed)
    }
}

#[derive(Clone, Copy)]
struct SelectiveAuthorizer;

impl VaultAuthorizer for SelectiveAuthorizer {
    fn authorize(
        &self,
        _context: &AuthorizationContext,
        record_id: &str,
        action: VaultAction,
    ) -> bool {
        match action {
            VaultAction::List => true,
            VaultAction::Read => record_id != "person-1-record-2",
            _ => true,
        }
    }
}

#[derive(Clone, Copy)]
struct DenyListAuthorizer;

impl VaultAuthorizer for DenyListAuthorizer {
    fn authorize(
        &self,
        _context: &AuthorizationContext,
        _record_id: &str,
        action: VaultAction,
    ) -> bool {
        !matches!(action, VaultAction::List)
    }
}

fn root(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("soma-{label}-{nanos}"))
}

fn record(
    id: &str,
    subject: &str,
) -> crate::local_health_vault::LocalHealthVaultRecord {
    LocalHealthVaultCrypto::encrypt_record(
        &[7u8; KEY_LEN],
        VaultRecordMetadata {
            record_id: id,
            subject_ref: subject,
            entity_type: "observation",
            content_type: "application/json",
            key_ref: "vault-key-v1",
            provenance_ref: "p1",
            created_at: "2026-09-10T00:00:00Z",
            updated_at: "2026-09-10T00:00:00Z",
        },
        br#"{"concept":"heart_rate","value":60}"#,
    )
    .unwrap()
}

fn keys() -> Keys {
    let calls = Rc::new(Cell::new(0));
    let mut values = HashMap::new();
    values.insert("vault-key-v1".into(), [7u8; KEY_LEN]);
    Keys { calls, values }
}

fn context() -> AuthorizationContext {
    AuthorizationContext {
        subject_ref: "person-1".into(),
        scope: "self".into(),
    }
}

#[test]
fn list_requires_list_authorization_and_filters_each_record_by_read_authorization() {
    let root = root("list-read-conformance");
    let store = LocalFileVaultStore::new(root.clone(), keys(), SelectiveAuthorizer).unwrap();
    let context = context();

    store
        .put(record("person-1-record-1", "person-1"), &context)
        .unwrap();
    store
        .put(record("person-1-record-2", "person-1"), &context)
        .unwrap();

    let entries = store.list(&context).unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].record_id, "person-1-record-1");

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn list_denial_stops_before_filesystem_enumeration() {
    let root = root("list-denial-conformance");
    let calls = Rc::new(Cell::new(0));
    let key_provider = Keys {
        calls: calls.clone(),
        values: {
            let mut values = HashMap::new();
            values.insert("vault-key-v1".into(), [7u8; KEY_LEN]);
            values
        },
    };
    let store = LocalFileVaultStore::new(root.clone(), key_provider, DenyListAuthorizer).unwrap();

    assert_eq!(
        store.list(&context()),
        Err(StorageError::AuthorizationDenied)
    );
    assert_eq!(calls.get(), 0);

    let subject_dir = root.join(hex::encode(sha2::Sha256::digest(b"person-1")));
    assert!(!Path::new(&subject_dir).exists());
    fs::remove_dir_all(root).unwrap();
}
