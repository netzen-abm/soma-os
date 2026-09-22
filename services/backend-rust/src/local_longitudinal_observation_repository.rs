#[cfg(test)]
#[path = "local_longitudinal_observation_repository_tests.rs"]
mod tests;
//! Local longitudinal observation provider backed by the existing Health Vault.
//!
//! This adapter owns no second clinical payload store. It composes the existing
//! LocalFileVaultStore and exposes the provider-neutral observation repository
//! contract while preserving the vault's encryption, integrity, tombstone, and
//! authorization boundary.

use crate::local_health_vault::LocalHealthVaultRecord;
use crate::local_health_vault_storage::{
    AuthorizationContext, LocalFileVaultStore, StorageError, VaultAuthorizer, VaultKeyProvider,
};
use crate::longitudinal_observation_repository::{
    sort_timeline, AuthorizedObservationAccessContext, LongitudinalObservationRepository, ObservationQuery,
    ObservationRepositoryError, ObservationTimelineEntry,
};

/// Local provider for longitudinal observations.
///
/// The vault remains the protected persistence authority. This type is only an
/// adapter over that authority; it does not create another database, index, or
/// authorization engine.
pub struct LocalLongitudinalObservationRepository<K, A> {
    vault: LocalFileVaultStore<K, A>,
}

impl<K, A> LocalLongitudinalObservationRepository<K, A>
where
    K: VaultKeyProvider,
    A: VaultAuthorizer,
{
    pub fn new(vault: LocalFileVaultStore<K, A>) -> Self {
        Self {
            vault,
        }
    }

    fn vault_context(context: &AuthorizedObservationAccessContext) -> AuthorizationContext {
        AuthorizationContext {
            principal_ref: context.principal_ref().to_owned(),
            subject_ref: context.subject_ref().to_owned(),
            scope: context.scope().to_owned(),
        }
    }

    fn map_error(error: StorageError) -> ObservationRepositoryError {
        match error {
            StorageError::AuthorizationDenied => ObservationRepositoryError::AuthorizationDenied,
            StorageError::NotFound => ObservationRepositoryError::NotFound,
            StorageError::Tombstoned => ObservationRepositoryError::Tombstoned,
            StorageError::IndexIntegrityFailure | StorageError::RecordIntegrityFailure => {
                ObservationRepositoryError::IntegrityFailure
            }
            StorageError::InvalidRecord
            | StorageError::KeyResolutionFailed
            | StorageError::Io
            | StorageError::Serialization
            | StorageError::NonceGenerationFailed => ObservationRepositoryError::StorageFailure,
        }
    }

    fn is_observation(record: &LocalHealthVaultRecord) -> bool {
        record.entity_type == "observation"
    }
}

impl<K, A> LongitudinalObservationRepository for LocalLongitudinalObservationRepository<K, A>
where
    K: VaultKeyProvider,
    A: VaultAuthorizer,
{
    type Observation = LocalHealthVaultRecord;

    fn put_reference(
        &self,
        observation_id: &str,
        context: &AuthorizedObservationAccessContext,
    ) -> Result<(), ObservationRepositoryError> {
        let vault_context = Self::vault_context(context);
        let record = self.vault.get(observation_id, &vault_context).map_err(Self::map_error)?;
        if !Self::is_observation(&record) {
            return Err(ObservationRepositoryError::InvalidObservation);
        }
        Ok(())
    }

    fn get(
        &self,
        observation_id: &str,
        context: &AuthorizedObservationAccessContext,
    ) -> Result<Self::Observation, ObservationRepositoryError> {
        let vault_context = Self::vault_context(context);
        let record = self.vault.get(observation_id, &vault_context).map_err(Self::map_error)?;
        if !Self::is_observation(&record) {
            return Err(ObservationRepositoryError::InvalidObservation);
        }
        Ok(record)
    }

    fn query(
        &self,
        context: &AuthorizedObservationAccessContext,
        query: &ObservationQuery,
    ) -> Result<Vec<Self::Observation>, ObservationRepositoryError> {
        let vault_context = Self::vault_context(context);
        let entries = self.vault.list(&vault_context).map_err(Self::map_error)?;
        let mut observations = Vec::new();

        for entry in entries {
            if entry.entity_type != "observation"
                || query.classification.as_ref().is_some_and(|value| entry.classification != *value)
            {
                continue;
            }

            // The encrypted vault intentionally does not expose payload fields through
            // its metadata index. Concept/status filtering therefore remains outside
            // this storage adapter rather than forcing a second plaintext index.
            observations.push(self.vault.get(&entry.record_id, &vault_context).map_err(Self::map_error)?);
        }

        Ok(observations)
    }

    fn timeline(
        &self,
        context: &AuthorizedObservationAccessContext,
    ) -> Result<Vec<ObservationTimelineEntry>, ObservationRepositoryError> {
        let vault_context = Self::vault_context(context);
        let entries = self.vault.list(&vault_context).map_err(Self::map_error)?;
        let mut timeline = entries
            .into_iter()
            .filter(|entry| entry.entity_type == "observation")
            .map(|entry| ObservationTimelineEntry {
                observation_id: entry.record_id,
                subject_ref: entry.subject_ref,
                // The current vault index does not expose observed_at. Preserve the
                // distinction required by the contract instead of fabricating it from
                // recorded_at.
                observed_at: None,
                recorded_at: entry.created_at,
            })
            .collect::<Vec<_>>();
        sort_timeline(&mut timeline);
        Ok(timeline)
    }

    fn tombstone(
        &self,
        observation_id: &str,
        context: &AuthorizedObservationAccessContext,
    ) -> Result<(), ObservationRepositoryError> {
        let vault_context = Self::vault_context(context);
        self.vault.tombstone(observation_id, &vault_context).map_err(Self::map_error)
    }

    fn verify(
        &self,
        observation_id: &str,
        context: &AuthorizedObservationAccessContext,
    ) -> Result<(), ObservationRepositoryError> {
        let vault_context = Self::vault_context(context);
        self.vault.verify(observation_id, &vault_context).map_err(Self::map_error)
    }
}
