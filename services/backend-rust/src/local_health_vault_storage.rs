#[path = "local_health_vault_storage_contract.rs"]
mod contract;
#[path = "local_health_vault_storage_crypto.rs"]
mod crypto;
#[path = "local_health_vault_storage_store.rs"]
mod store;
pub use contract::{
    AuthorizedIndexEntry, AuthorizationContext, StorageError, VaultAction,
    VaultAuthorizer, VaultKeyProvider, KEY_LEN,
};
pub use store::LocalFileVaultStore;

#[cfg(test)]
#[path = "local_health_vault_storage_tests.rs"]
mod tests;
