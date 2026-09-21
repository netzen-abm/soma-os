mod contract;
mod crypto;
mod store;
pub use contract::{
    AuthorizedIndexEntry, AuthorizationContext, StorageError, VaultAction,
    VaultAuthorizer, VaultKeyProvider, KEY_LEN,
};
pub use store::LocalFileVaultStore;

#[cfg(test)]
#[path = "local_health_vault_storage_tests.rs"]
mod tests;
