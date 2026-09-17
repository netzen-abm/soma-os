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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical_authorization::AuthorizationRequest;
    use crate::local_health_vault::{LocalHealthVaultCrypto, VaultRecordMetadata};
    use crate::local_health_vault_storage::VaultAction;
    use std::{
        collections::HashMap,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    const KEY: [u8; 32] = [7u8; 32];

    struct Keys(HashMap<String, [u8; 32]>);

    impl VaultKeyProvider for Keys {
        fn key_for(&self, key_ref: &str) -> Result<[u8; 32], StorageError> {
            self.0.get(key_ref).copied().ok_or(StorageError::KeyResolutionFailed)
        }
    }

    #[derive(Clone, Copy)]
    struct SubjectAuthorizer;

    impl VaultAuthorizer for SubjectAuthorizer {
        fn authorize(&self, context: &AuthorizationContext, record_id: &str, action: VaultAction) -> bool {
            match action {
                VaultAction::List => true,
                _ => record_id == "*" || record_id.starts_with(&context.subject_ref),
            }
        }
    }

    fn root() -> PathBuf {
        std::env::temp_dir().join(format!(
            "soma-longitudinal-observation-test-{}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ))
    }

    fn context() -> AuthorizedObservationAccessContext {
        AuthorizedObservationAccessContext::from_authorized_request(&AuthorizationRequest {
            principal_ref: "principal-1".into(),
            subject_ref: "person-1".into(),
            capability_id: "health.read".into(),
            resource_type: "health_record".into(),
            resource_id: "record-1".into(),
            action: "read".into(),
            tenant_id: "tenant-1".into(),
            data_domain: "personal_health".into(),
        })
        .unwrap()
    }

    fn observation() -> LocalHealthVaultRecord {
        LocalHealthVaultCrypto::encrypt_record(
            &KEY,
            VaultRecordMetadata {
                record_id: "person-1-observation-1",
                subject_ref: "person-1",
                entity_type: "observation",
                content_type: "application/json",
                key_ref: "key-1",
                provenance_ref: "provenance-1",
                created_at: "2026-09-13T00:00:00Z",
                updated_at: "2026-09-13T00:00:00Z",
            },
            br#"{"id":"person-1-observation-1","entity_type":"observation","concept":"heart_rate"}"#,
        )
        .unwrap()
    }

    fn repository(root: &PathBuf) -> LocalLongitudinalObservationRepository<Keys, SubjectAuthorizer> {
        let mut keys = HashMap::new();
        keys.insert("key-1".into(), KEY);
        LocalLongitudinalObservationRepository::new(
            LocalFileVaultStore::new(root, Keys(keys), SubjectAuthorizer).unwrap(),
        )
    }

    #[test]
    fn adapter_reads_observation_from_existing_vault() {
        let root = root();
        let repository = repository(&root);
        let context = context();
        let vault_context = LocalLongitudinalObservationRepository::<Keys, SubjectAuthorizer>::vault_context(&context);
        repository.vault.put(observation(), &vault_context).unwrap();
        assert!(repository.get("person-1-observation-1", &context).is_ok());
        assert_eq!(repository.put_reference("person-1-observation-1", &context), Ok(()));
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn timeline_keeps_observed_time_unknown() {
        let root = root();
        let repository = repository(&root);
        let context = context();
        let vault_context = LocalLongitudinalObservationRepository::<Keys, SubjectAuthorizer>::vault_context(&context);
        repository.vault.put(observation(), &vault_context).unwrap();
        let timeline = repository.timeline(&context).unwrap();
        assert_eq!(timeline.len(), 1);
        assert_eq!(timeline[0].observed_at, None);
        assert_eq!(timeline[0].recorded_at, "2026-09-13T00:00:00Z");
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn non_observation_entity_is_rejected() {
        let root = root();
        let repository = repository(&root);
        let context = context();
        let vault_context = LocalLongitudinalObservationRepository::<Keys, SubjectAuthorizer>::vault_context(&context);
        let record = LocalHealthVaultCrypto::encrypt_record(
            &KEY,
            VaultRecordMetadata {
                record_id: "person-1-other-1",
                subject_ref: "person-1",
                entity_type: "other_health_entity",
                content_type: "application/json",
                key_ref: "key-1",
                provenance_ref: "provenance-1",
                created_at: "2026-09-13T00:00:00Z",
                updated_at: "2026-09-13T00:00:00Z",
            },
            b"{}",
        )
        .unwrap();
        repository.vault.put(record, &vault_context).unwrap();
        assert_eq!(
            repository.put_reference("person-1-other-1", &context),
            Err(ObservationRepositoryError::InvalidObservation)
        );
        std::fs::remove_dir_all(root).unwrap();
    }
}
