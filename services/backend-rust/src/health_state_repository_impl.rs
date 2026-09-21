//! Local Health State repository adapter over the existing Health Vault.

use std::cmp::Ordering;

use crate::local_health_vault::LocalHealthVaultRecord;
use crate::local_health_vault_storage::{
    AuthorizationContext, LocalFileVaultStore, VaultAuthorizer, VaultKeyProvider,
};
use super::{
    AuthorizedHealthStateAccessContext, HealthStateQuery, HealthStateRepository,
    HealthStateRepositoryError, HealthStateTimelineEntry, ALLOWED_ENTITY_TYPES,
    HEALTH_STATE_CONTENT_TYPE,
};

pub struct LocalHealthStateRepository<K, A> {
    vault: LocalFileVaultStore<K, A>,
}

impl<K, A> LocalHealthStateRepository<K, A>
where K: VaultKeyProvider, A: VaultAuthorizer
{
    pub fn new(vault: LocalFileVaultStore<K, A>) -> Self { Self { vault } }

    fn vault_context(context: &AuthorizedHealthStateAccessContext) -> AuthorizationContext {
        AuthorizationContext {
            principal_ref: context.principal_ref().to_owned(),
            subject_ref: context.subject_ref().to_owned(),
            scope: context.scope().to_owned(),
        }
    }

    fn validate_entity(record: &LocalHealthVaultRecord) -> Result<(), HealthStateRepositoryError> {
        if !ALLOWED_ENTITY_TYPES.contains(&record.entity_type.as_str())
            || record.content_type != HEALTH_STATE_CONTENT_TYPE
            || record.record_id.trim().is_empty()
            || record.subject_ref.trim().is_empty()
        {
            return Err(HealthStateRepositoryError::InvalidEntity);
        }
        Ok(())
    }
}

impl<K, A> HealthStateRepository for LocalHealthStateRepository<K, A>
where K: VaultKeyProvider, A: VaultAuthorizer
{
    fn put(&self, record: LocalHealthVaultRecord, context: &AuthorizedHealthStateAccessContext)
        -> Result<(), HealthStateRepositoryError> {
        Self::validate_entity(&record)?;
        if record.subject_ref != context.subject_ref() {
            return Err(HealthStateRepositoryError::AuthorizationDenied);
        }
        self.vault.put(record, &Self::vault_context(context)).map_err(Into::into)
    }

    fn get(&self, record_id: &str, context: &AuthorizedHealthStateAccessContext)
        -> Result<LocalHealthVaultRecord, HealthStateRepositoryError> {
        let record = self.vault.get(record_id, &Self::vault_context(context))
            .map_err(HealthStateRepositoryError::from)?;
        Self::validate_entity(&record)?;
        Ok(record)
    }

    fn query(&self, context: &AuthorizedHealthStateAccessContext, query: &HealthStateQuery)
        -> Result<Vec<LocalHealthVaultRecord>, HealthStateRepositoryError> {
        let entries = self.vault.list(&Self::vault_context(context))
            .map_err(HealthStateRepositoryError::from)?;
        let mut records = Vec::new();
        for entry in entries {
            if !ALLOWED_ENTITY_TYPES.contains(&entry.entity_type.as_str())
                || query.entity_type.as_ref().is_some_and(|v| entry.entity_type != *v)
                || query.classification.as_ref().is_some_and(|v| entry.classification != *v)
                || query.content_type.as_ref().is_some_and(|v| entry.content_type != *v) {
                continue;
            }
            records.push(self.get(&entry.record_id, context)?);
        }
        Ok(records)
    }

    fn timeline(&self, context: &AuthorizedHealthStateAccessContext)
        -> Result<Vec<HealthStateTimelineEntry>, HealthStateRepositoryError> {
        let entries = self.vault.list(&Self::vault_context(context))
            .map_err(HealthStateRepositoryError::from)?;
        let mut timeline = entries.into_iter()
            .filter(|e| ALLOWED_ENTITY_TYPES.contains(&e.entity_type.as_str()))
            .map(|e| HealthStateTimelineEntry {
                record_id: e.record_id,
                subject_ref: e.subject_ref,
                effective_time: None,
                recorded_at: e.created_at,
            })
            .collect::<Vec<_>>();
        timeline.sort_by(|left, right| {
            match (&left.effective_time, &right.effective_time) {
                (Some(a), Some(b)) => a.cmp(b).then_with(|| left.recorded_at.cmp(&right.recorded_at)),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => left.recorded_at.cmp(&right.recorded_at),
            }.then_with(|| left.record_id.cmp(&right.record_id))
        });
        Ok(timeline)
    }

    fn tombstone(&self, record_id: &str, context: &AuthorizedHealthStateAccessContext)
        -> Result<(), HealthStateRepositoryError> {
        self.vault.tombstone(record_id, &Self::vault_context(context)).map_err(Into::into)
    }

    fn verify(&self, record_id: &str, context: &AuthorizedHealthStateAccessContext)
        -> Result<(), HealthStateRepositoryError> {
        self.vault.verify(record_id, &Self::vault_context(context)).map_err(Into::into)
    }
}
