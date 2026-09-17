//! Provider-neutral Health State repository backed by the existing Health Vault.
//!
//! The repository stores canonical Health State entities without creating a second
//! clinical store or semantic model. Protected access is bound to the canonical
//! authorization decision boundary before the vault is reached.

use std::cmp::Ordering;

use thiserror::Error;

use crate::canonical_authorization::AuthorizationRequest;
use crate::local_health_vault::LocalHealthVaultRecord;
use crate::local_health_vault_storage::{
    AuthorizationContext, LocalFileVaultStore, StorageError, VaultAuthorizer, VaultKeyProvider,
};

const HEALTH_STATE_CONTENT_TYPE: &str = "application/json";

const ALLOWED_ENTITY_TYPES: [&str; 8] =
    ["person", "observation", "interpretation", "goal", "context", "intervention", "response", "outcome"];

/// Authorization-bound context for protected Health State repository access.
///
/// Fields are private so callers cannot manufacture repository authority from
/// arbitrary subject or scope strings. The canonical authorization boundary mints
/// this context only after an authoritative `ALLOW` decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedHealthStateAccessContext {
    principal_ref: String,
    subject_ref: String,
    scope: String,
    capability_id: String,
    resource_type: String,
    resource_id: String,
    action: String,
}

impl AuthorizedHealthStateAccessContext {
    pub(crate) fn from_authorized_request(request: &AuthorizationRequest) -> Result<Self, HealthStateRepositoryError> {
        let values = [
            &request.principal_ref,
            &request.subject_ref,
            &request.capability_id,
            &request.resource_type,
            &request.resource_id,
            &request.action,
            &request.tenant_id,
            &request.data_domain,
        ];
        if values.iter().any(|value| value.trim().is_empty() || value.chars().any(char::is_control)) {
            return Err(HealthStateRepositoryError::AuthorizationDenied);
        }

        Ok(Self {
            principal_ref: request.principal_ref.clone(),
            subject_ref: request.subject_ref.clone(),
            scope: format!("{}:{}", request.tenant_id, request.data_domain),
            capability_id: request.capability_id.clone(),
            resource_type: request.resource_type.clone(),
            resource_id: request.resource_id.clone(),
            action: request.action.clone(),
        })
    }

    pub fn principal_ref(&self) -> &str {
        &self.principal_ref
    }

    pub fn subject_ref(&self) -> &str {
        &self.subject_ref
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }

    pub fn capability_id(&self) -> &str {
        &self.capability_id
    }

    pub fn resource_type(&self) -> &str {
        &self.resource_type
    }

    pub fn resource_id(&self) -> &str {
        &self.resource_id
    }

    pub fn action(&self) -> &str {
        &self.action
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HealthStateQuery {
    pub entity_type: Option<String>,
    pub classification: Option<String>,
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthStateTimelineEntry {
    pub record_id: String,
    pub subject_ref: String,
    /// Health-relevant time is intentionally unavailable from the current vault
    /// metadata projection; `None` means unknown rather than fabricated.
    pub effective_time: Option<String>,
    pub recorded_at: String,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HealthStateRepositoryError {
    #[error("authorization denied")]
    AuthorizationDenied,
    #[error("health state not found")]
    NotFound,
    #[error("health state is tombstoned")]
    Tombstoned,
    #[error("repository integrity failure")]
    IntegrityFailure,
    #[error("repository storage failure")]
    StorageFailure,
    #[error("invalid Health State entity")]
    InvalidEntity,
}

impl From<StorageError> for HealthStateRepositoryError {
    fn from(error: StorageError) -> Self {
        match error {
            StorageError::AuthorizationDenied => Self::AuthorizationDenied,
            StorageError::NotFound => Self::NotFound,
            StorageError::Tombstoned => Self::Tombstoned,
            StorageError::IndexIntegrityFailure | StorageError::RecordIntegrityFailure => Self::IntegrityFailure,
            StorageError::InvalidRecord
            | StorageError::KeyResolutionFailed
            | StorageError::Io
            | StorageError::Serialization
            | StorageError::NonceGenerationFailed => Self::StorageFailure,
        }
    }
}

pub trait HealthStateRepository {
    fn put(
        &self,
        record: LocalHealthVaultRecord,
        context: &AuthorizedHealthStateAccessContext,
    ) -> Result<(), HealthStateRepositoryError>;

    fn get(
        &self,
        record_id: &str,
        context: &AuthorizedHealthStateAccessContext,
    ) -> Result<LocalHealthVaultRecord, HealthStateRepositoryError>;

    fn query(
        &self,
        context: &AuthorizedHealthStateAccessContext,
        query: &HealthStateQuery,
    ) -> Result<Vec<LocalHealthVaultRecord>, HealthStateRepositoryError>;

    fn timeline(
        &self,
        context: &AuthorizedHealthStateAccessContext,
    ) -> Result<Vec<HealthStateTimelineEntry>, HealthStateRepositoryError>;

    fn tombstone(
        &self,
        record_id: &str,
        context: &AuthorizedHealthStateAccessContext,
    ) -> Result<(), HealthStateRepositoryError>;

    fn verify(
        &self,
        record_id: &str,
        context: &AuthorizedHealthStateAccessContext,
    ) -> Result<(), HealthStateRepositoryError>;
}

/// Local Health State repository adapter over the existing Local Health Vault.
pub struct LocalHealthStateRepository<K, A> {
    vault: LocalFileVaultStore<K, A>,
}

impl<K, A> LocalHealthStateRepository<K, A>
where
    K: VaultKeyProvider,
    A: VaultAuthorizer,
{
    pub fn new(vault: LocalFileVaultStore<K, A>) -> Self {
        Self {
            vault,
        }
    }

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
where
    K: VaultKeyProvider,
    A: VaultAuthorizer,
{
    fn put(
        &self,
        record: LocalHealthVaultRecord,
        context: &AuthorizedHealthStateAccessContext,
    ) -> Result<(), HealthStateRepositoryError> {
        Self::validate_entity(&record)?;
        if record.subject_ref != context.subject_ref() {
            return Err(HealthStateRepositoryError::AuthorizationDenied);
        }
        self.vault.put(record, &Self::vault_context(context)).map_err(Into::into)
    }

    fn get(
        &self,
        record_id: &str,
        context: &AuthorizedHealthStateAccessContext,
    ) -> Result<LocalHealthVaultRecord, HealthStateRepositoryError> {
        let record =
            self.vault.get(record_id, &Self::vault_context(context)).map_err(HealthStateRepositoryError::from)?;
        Self::validate_entity(&record)?;
        Ok(record)
    }

    fn query(
        &self,
        context: &AuthorizedHealthStateAccessContext,
        query: &HealthStateQuery,
    ) -> Result<Vec<LocalHealthVaultRecord>, HealthStateRepositoryError> {
        let entries = self.vault.list(&Self::vault_context(context)).map_err(HealthStateRepositoryError::from)?;
        let mut records = Vec::new();
        for entry in entries {
            if !ALLOWED_ENTITY_TYPES.contains(&entry.entity_type.as_str())
                || query.entity_type.as_ref().is_some_and(|value| entry.entity_type != *value)
                || query.classification.as_ref().is_some_and(|value| entry.classification != *value)
                || query.content_type.as_ref().is_some_and(|value| entry.content_type != *value)
            {
                continue;
            }
            let record = self.get(&entry.record_id, context)?;
            records.push(record);
        }
        Ok(records)
    }

    fn timeline(
        &self,
        context: &AuthorizedHealthStateAccessContext,
    ) -> Result<Vec<HealthStateTimelineEntry>, HealthStateRepositoryError> {
        let entries = self.vault.list(&Self::vault_context(context)).map_err(HealthStateRepositoryError::from)?;
        let mut timeline = entries
            .into_iter()
            .filter(|entry| ALLOWED_ENTITY_TYPES.contains(&entry.entity_type.as_str()))
            .map(|entry| HealthStateTimelineEntry {
                record_id: entry.record_id,
                subject_ref: entry.subject_ref,
                effective_time: None,
                recorded_at: entry.created_at,
            })
            .collect::<Vec<_>>();
        timeline.sort_by(|left, right| {
            match (&left.effective_time, &right.effective_time) {
                (Some(left_time), Some(right_time)) => match left_time.cmp(right_time) {
                    Ordering::Equal => left.recorded_at.cmp(&right.recorded_at),
                    ordering => ordering,
                },
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => left.recorded_at.cmp(&right.recorded_at),
            }
            .then_with(|| left.record_id.cmp(&right.record_id))
        });
        Ok(timeline)
    }

    fn tombstone(
        &self,
        record_id: &str,
        context: &AuthorizedHealthStateAccessContext,
    ) -> Result<(), HealthStateRepositoryError> {
        self.vault.tombstone(record_id, &Self::vault_context(context)).map_err(Into::into)
    }

    fn verify(
        &self,
        record_id: &str,
        context: &AuthorizedHealthStateAccessContext,
    ) -> Result<(), HealthStateRepositoryError> {
        self.vault.verify(record_id, &Self::vault_context(context)).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical_authorization::{AuthorizationDecision, CanonicalAuthorizationBoundary};
    use crate::local_health_vault::{LocalHealthVaultCrypto, VaultRecordMetadata};
    use std::{
        cell::Cell,
        collections::HashMap,
        fs,
        path::PathBuf,
        rc::Rc,
        time::{SystemTime, UNIX_EPOCH},
    };

    const KEY_LEN: usize = 32;

    #[derive(Clone)]
    struct Keys {
        values: HashMap<String, [u8; KEY_LEN]>,
        calls: Rc<Cell<usize>>,
    }

    impl VaultKeyProvider for Keys {
        fn key_for(&self, key_ref: &str) -> Result<[u8; KEY_LEN], StorageError> {
            self.calls.set(self.calls.get() + 1);
            self.values.get(key_ref).copied().ok_or(StorageError::KeyResolutionFailed)
        }
    }

    #[derive(Clone, Copy)]
    struct SubjectAuthorizer;

    impl VaultAuthorizer for SubjectAuthorizer {
        fn authorize(
            &self,
            context: &AuthorizationContext,
            record_id: &str,
            action: crate::local_health_vault_storage::VaultAction,
        ) -> bool {
            match action {
                crate::local_health_vault_storage::VaultAction::List => true,
                _ => record_id == "*" || record_id.starts_with(&context.subject_ref),
            }
        }
    }

    struct AllowBoundary;

    impl CanonicalAuthorizationBoundary for AllowBoundary {
        fn authorize(&self, _request: &AuthorizationRequest) -> AuthorizationDecision {
            AuthorizationDecision::Allow
        }
    }

    fn root() -> PathBuf {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        std::env::temp_dir().join(format!("soma-health-state-repository-test-{nanos}"))
    }

    fn request() -> AuthorizationRequest {
        AuthorizationRequest {
            principal_ref: "principal-1".into(),
            subject_ref: "person-1".into(),
            capability_id: "health.state.write".into(),
            resource_type: "health_state".into(),
            resource_id: "record-1".into(),
            action: "write".into(),
            tenant_id: "tenant-1".into(),
            data_domain: "personal_health".into(),
        }
    }

    fn record(entity_type: &str) -> LocalHealthVaultRecord {
        LocalHealthVaultCrypto::encrypt_record(
            &[7u8; KEY_LEN],
            VaultRecordMetadata {
                record_id: "person-1-record-1",
                subject_ref: "person-1",
                entity_type,
                content_type: HEALTH_STATE_CONTENT_TYPE,
                key_ref: "vault-key-v1",
                provenance_ref: "provenance-1",
                created_at: "2026-09-14T00:00:00Z",
                updated_at: "2026-09-14T00:00:00Z",
            },
            br#"{"id":"person-1-record-1","subject_ref":"person-1","entity_type":"observation","schema_version":"1.1.0","status":"active","recorded_time":"2026-09-14T00:00:00Z","concept":"heart_rate","uncertainty":"known","provenance":{"origin":"device","method":"measurement"},"classification":"personal_health"}"#,
        )
        .unwrap()
    }

    fn repository(root: &std::path::Path) -> (LocalHealthStateRepository<Keys, SubjectAuthorizer>, Rc<Cell<usize>>) {
        let calls = Rc::new(Cell::new(0));
        let mut values = HashMap::new();
        values.insert("vault-key-v1".into(), [7u8; KEY_LEN]);
        let vault = LocalFileVaultStore::new(
            root,
            Keys {
                values,
                calls: calls.clone(),
            },
            SubjectAuthorizer,
        )
        .unwrap();
        (LocalHealthStateRepository::new(vault), calls)
    }

    #[test]
    fn authorized_context_is_minted_only_from_allow() {
        let context = AllowBoundary.authorize(&request());
        assert_eq!(context, AuthorizationDecision::Allow);
        let access = AuthorizedHealthStateAccessContext::from_authorized_request(&request()).unwrap();
        assert_eq!(access.subject_ref(), "person-1");
        assert_eq!(access.scope(), "tenant-1:personal_health");
    }

    #[test]
    fn repository_accepts_canonical_health_state_entity_types() {
        let root = root();
        let (repository, _) = repository(&root);
        let context = AuthorizedHealthStateAccessContext::from_authorized_request(&request()).unwrap();
        repository.put(record("observation"), &context).unwrap();
        let records = repository.query(&context, &HealthStateQuery::default()).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].entity_type, "observation");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn repository_rejects_non_health_state_entity_types() {
        let root = root();
        let (repository, _) = repository(&root);
        let context = AuthorizedHealthStateAccessContext::from_authorized_request(&request()).unwrap();
        assert_eq!(repository.put(record("evidence_claim"), &context), Err(HealthStateRepositoryError::InvalidEntity));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn timeline_does_not_fabricate_effective_time() {
        let root = root();
        let (repository, _) = repository(&root);
        let context = AuthorizedHealthStateAccessContext::from_authorized_request(&request()).unwrap();
        repository.put(record("observation"), &context).unwrap();
        let timeline = repository.timeline(&context).unwrap();
        assert_eq!(timeline[0].effective_time, None);
        assert_eq!(timeline[0].recorded_at, "2026-09-14T00:00:00Z");
        fs::remove_dir_all(root).unwrap();
    }
}
