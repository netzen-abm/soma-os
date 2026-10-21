use super::*;

#[derive(Clone, Copy)]
struct SelectiveReadAuthorizer;
impl VaultAuthorizer for SelectiveReadAuthorizer {
    fn authorize(&self, context: &AuthorizationContext, record_id: &str, action: VaultAction) -> bool {
        match action {
            VaultAction::List => true,
            VaultAction::Write => record_id.starts_with(&context.subject_ref),
            VaultAction::Read => record_id.starts_with(&context.subject_ref) && !record_id.contains("denied"),
            VaultAction::Tombstone | VaultAction::Verify => record_id.starts_with(&context.subject_ref),
        }
    }
}

#[test]
fn list_does_not_resolve_keys_for_records_denied_read_access() {
    let root = root();
    let calls = Rc::new(Cell::new(0));
    let mut values = HashMap::new();
    values.insert("vault-key-v1".into(), [7u8; KEY_LEN]);
    let store = LocalFileVaultStore::new(
        &root,
        Keys {
            calls: calls.clone(),
            values,
        },
        SelectiveReadAuthorizer,
    )
    .unwrap();
    let context = context("principal-1", "person-1");
    store.put(record("person-1-allowed", "person-1"), &context).unwrap();
    store.put(record("person-1-denied", "person-1"), &context).unwrap();

    // Exclude setup-time key resolution. Listing must enforce per-record Read
    // authorization before resolving a key or decrypting that record's index.
    calls.set(0);
    let entries = store.list(&context).unwrap();

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].record_id, "person-1-allowed");
    assert_eq!(calls.get(), 1, "denied records must not trigger key resolution");
    fs::remove_dir_all(root).unwrap();
}
