//! Canonical Health State repository contracts and authorization-bound context.

use thiserror::Error;

use crate::canonical_authorization::AuthorizationRequest;
use crate::local_health_vault::LocalHealthVaultRecord;
use crate::local_health_vault_storage::StorageError;

pub const HEALTH_STATE_CONTENT_TYPE: &str = "application/json";
pub const ALLOWED_ENTITY_TYPES: [&str; 8] = [
    "person", "observation", "interpretation", "goal",
    "context", "intervention", "response", "outcome",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedHealthStateAccessContext {
    principal_ref: String,
    subject_ref: String,
    scope: String,
    capability_id: String,
    capability_version: String,
    resource_type: String,
    resource_id: String,
    action: String,
}

impl AuthorizedHealthStateAccessContext {
    pub(crate) fn from_authorized_request(
        request: &AuthorizationRequest,
    ) -> Result<Self, HealthStateRepositoryError> {
        let values = [
            &request.principal_ref, &request.subject_ref, &request.capability_id,
            &request.capability_version, &request.resource_type, &request.resource_id,
            &request.action, &request.tenant_id, &request.data_domain,
        ];
        if values.iter().any(|value| value.trim().is_empty() || value.chars().any(char::is_control)) {
            return Err(HealthStateRepositoryError::AuthorizationDenied);
        }
        Ok(Self {
            principal_ref: request.principal_ref.clone(),
            subject_ref: request.subject_ref.clone(),
            scope: format!("{}:{}", request.tenant_id, request.data_domain),
            capability_id: request.capability_id.clone(),
            capability_version: request.capability_version.clone(),
            resource_type: request.resource_type.clone(),
            resource_id: request.resource_id.clone(),
            action: request.action.clone(),
        })
    }

    pub fn principal_ref(&self) -> &str { &self.principal_ref }
    pub fn subject_ref(&self) -> &str { &self.subject_ref }
    pub fn scope(&self) -> &str { &self.scope }
    pub fn capability_id(&self) -> &str { &self.capability_id }
    pub fn capability_version(&self) -> &str { &self.capability_version }
    pub fn resource_type(&self) -> &str { &self.resource_type }
    pub fn resource_id(&self) -> &str { &self.resource_id }
    pub fn action(&self) -> &str { &self.action }
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
            StorageError::InvalidRecord | StorageError::KeyResolutionFailed
            | StorageError::Io | StorageError::Serialization
            | StorageError::NonceGenerationFailed => Self::StorageFailure,
        }
    }
}

pub trait HealthStateRepository {
    fn put(&self, record: LocalHealthVaultRecord, context: &AuthorizedHealthStateAccessContext)
        -> Result<(), HealthStateRepositoryError>;
    fn get(&self, record_id: &str, context: &AuthorizedHealthStateAccessContext)
        -> Result<LocalHealthVaultRecord, HealthStateRepositoryError>;
    fn query(&self, context: &AuthorizedHealthStateAccessContext, query: &HealthStateQuery)
        -> Result<Vec<LocalHealthVaultRecord>, HealthStateRepositoryError>;
    fn timeline(&self, context: &AuthorizedHealthStateAccessContext)
        -> Result<Vec<HealthStateTimelineEntry>, HealthStateRepositoryError>;
    fn tombstone(&self, record_id: &str, context: &AuthorizedHealthStateAccessContext)
        -> Result<(), HealthStateRepositoryError>;
    fn verify(&self, record_id: &str, context: &AuthorizedHealthStateAccessContext)
        -> Result<(), HealthStateRepositoryError>;
}
