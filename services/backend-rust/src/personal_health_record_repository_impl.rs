use std::cmp::Ordering;

use crate::local_health_vault::LocalHealthVaultRecord;
use crate::local_health_vault_storage::{
    AuthorizationContext, AuthorizedIndexEntry, LocalFileVaultStore, VaultAuthorizer, VaultKeyProvider,
};
use super::contract::{PersonalHealthRecordRepository, RepositoryError, RepositoryQuery};

pub struct LocalPersonalHealthRecordRepository<K, A> {
    pub(crate) vault: LocalFileVaultStore<K, A>,
}

impl<K, A> LocalPersonalHealthRecordRepository<K, A>
where
    K: VaultKeyProvider,
    A: VaultAuthorizer,
{
    pub fn new(vault: LocalFileVaultStore<K, A>) -> Self {
        Self { vault }
    }
}

impl<K, A> PersonalHealthRecordRepository for LocalPersonalHealthRecordRepository<K, A>
where
    K: VaultKeyProvider,
    A: VaultAuthorizer,
{
    fn put_reference(&self, record_id: &str, context: &AuthorizationContext) -> Result<(), RepositoryError> {
        self.vault.verify(record_id, context)?;
        self.list(context)?.into_iter().find(|e| e.record_id == record_id).map(|_| ()).ok_or(RepositoryError::NotFound)
    }

    fn get(&self, record_id: &str, context: &AuthorizationContext) -> Result<LocalHealthVaultRecord, RepositoryError> {
        Ok(self.vault.get(record_id, context)?)
    }

    fn list(&self, context: &AuthorizationContext) -> Result<Vec<AuthorizedIndexEntry>, RepositoryError> {
        Ok(self.vault.list(context)?.into_iter().filter(|e| e.record_state == "ACTIVE").collect())
    }

    fn query(&self, context: &AuthorizationContext, query: &RepositoryQuery) -> Result<Vec<AuthorizedIndexEntry>, RepositoryError> {
        Ok(self.list(context)?.into_iter().filter(|e| {
            query.entity_type.as_ref().is_none_or(|v| e.entity_type == *v)
                && query.classification.as_ref().is_none_or(|v| e.classification == *v)
                && query.content_type.as_ref().is_none_or(|v| e.content_type == *v)
        }).collect())
    }

    fn timeline(&self, context: &AuthorizationContext) -> Result<Vec<AuthorizedIndexEntry>, RepositoryError> {
        let mut entries = self.list(context)?;
        entries.sort_by(|a, b| match a.created_at.cmp(&b.created_at) {
            Ordering::Equal => a.record_id.cmp(&b.record_id),
            ordering => ordering,
        });
        Ok(entries)
    }

    fn tombstone(&self, record_id: &str, context: &AuthorizationContext) -> Result<(), RepositoryError> {
        Ok(self.vault.tombstone(record_id, context)?)
    }

    fn verify(&self, record_id: &str, context: &AuthorizationContext) -> Result<(), RepositoryError> {
        Ok(self.vault.verify(record_id, context)?)
    }

    fn rebuild_index(&self, context: &AuthorizationContext) -> Result<Vec<AuthorizedIndexEntry>, RepositoryError> {
        self.list(context)
    }
}
