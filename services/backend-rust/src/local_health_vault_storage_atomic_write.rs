use std::{
    fs,
    io::Write,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use super::contract::StorageError;

/// Atomically replace a vault bundle after syncing its temporary file.
pub(super) fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), StorageError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|_| StorageError::Io)?;
    }
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|_| StorageError::Io)?.as_nanos();
    let tmp = path.with_extension(format!("bundle.tmp-{}-{nonce}", std::process::id()));
    let mut file = fs::File::create(&tmp).map_err(|_| StorageError::Io)?;
    if file.write_all(bytes).is_err() || file.sync_all().is_err() {
        let _ = fs::remove_file(&tmp);
        return Err(StorageError::Io);
    }
    if fs::rename(&tmp, path).is_err() {
        let _ = fs::remove_file(&tmp);
        return Err(StorageError::Io);
    }
    Ok(())
}
