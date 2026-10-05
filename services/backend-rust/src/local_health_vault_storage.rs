#[path = "local_health_vault_storage_contract.rs"]
mod contract;
#[path = "local_health_vault_storage_crypto.rs"]
mod crypto;
#[path = "local_health_vault_storage_store.rs"]
mod store;
pub(super) use contract::{
    AuthorizationContext, AuthorizedIndexEntry, StorageError, VaultAction, VaultAuthorizer, VaultKeyProvider,
};
pub(super) use contract::KEY_LEN;
pub(super) use store::LocalFileVaultStore;

#[cfg(test)]
#[path = "local_health_vault_storage_tests.rs"]
mod tests;
