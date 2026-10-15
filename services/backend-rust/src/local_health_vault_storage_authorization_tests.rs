use super::*;

#[test]
fn denied_write_precedes_key_resolution_and_record_construction() {
    let root = root();
    let (store, calls) = store(&root);
    let denied = context("principal-2", "person-2");
    let built = Cell::new(false);

    let result = store.put_with(
        "person-1-record-2",
        "person-1",
        "vault-key-v1",
        &denied,
        |_| {
            built.set(true);
            Ok::<_, StorageError>(record("person-1-record-2", "person-1"))
        },
    );

    assert_eq!(result, Err(StorageError::AuthorizationDenied));
    assert_eq!(calls.get(), 0, "denied writes must not resolve encryption keys");
    assert!(!built.get(), "denied writes must not construct encrypted records");
    fs::remove_dir_all(root).unwrap();
}
