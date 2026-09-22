#[cfg(test)]
#[path = "longitudinal_observation_repository_tests.rs"]
mod tests;
//! Provider-neutral longitudinal observation and timeline contract.
//!
//! Protected observation access must cross the canonical authorization boundary
//! before a provider can read payloads or resolve vault keys. This module keeps
//! the resulting access context non-forgeable at the application boundary.

use std::cmp::Ordering;
use thiserror::Error;

use crate::canonical_authorization::AuthorizationRequest;

/// Authorization-bound context for protected observation access.
///
/// The fields are intentionally private. Callers cannot construct a context from
/// arbitrary subject/scope strings; the canonical authorization boundary mints it
/// only after an authoritative `ALLOW` decision and request validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedObservationAccessContext {
    principal_ref: String,
    subject_ref: String,
    scope: String,
    capability_id: String,
    capability_version: String,
    resource_type: String,
    resource_id: String,
    action: String,
}

impl AuthorizedObservationAccessContext {
    pub(crate) fn from_authorized_request(request: &AuthorizationRequest) -> Result<Self, ObservationRepositoryError> {
        if [
            &request.principal_ref,
            &request.subject_ref,
            &request.capability_id,
            &request.capability_version,
            &request.resource_type,
            &request.resource_id,
            &request.action,
            &request.tenant_id,
            &request.data_domain,
        ]
        .iter()
        .any(|value| value.trim().is_empty() || value.chars().any(char::is_control))
        {
            return Err(ObservationRepositoryError::AuthorizationDenied);
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
    pub fn capability_version(&self) -> &str {
        &self.capability_version
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationQuery {
    pub concept: Option<String>,
    pub status: Option<String>,
    pub classification: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationTimelineEntry {
    pub observation_id: String,
    pub subject_ref: String,
    /// Health-relevant time. None means the observation time is unknown.
    pub observed_at: Option<String>,
    pub recorded_at: String,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ObservationRepositoryError {
    #[error("authorization denied")]
    AuthorizationDenied,
    #[error("observation not found")]
    NotFound,
    #[error("observation is tombstoned")]
    Tombstoned,
    #[error("repository integrity failure")]
    IntegrityFailure,
    #[error("repository storage failure")]
    StorageFailure,
    #[error("invalid observation")]
    InvalidObservation,
}

/// Canonical longitudinal observation repository boundary.
///
/// This contract deliberately does not define a competing clinical payload model.
/// Canonical Health State observations remain the domain representation; a provider
/// stores or indexes references to those entities according to SOMA's governed
/// storage boundary. Implementations must preserve provenance and uncertainty and
/// must not infer causation.
pub trait LongitudinalObservationRepository {
    type Observation;

    fn put_reference(
        &self,
        observation_id: &str,
        context: &AuthorizedObservationAccessContext,
    ) -> Result<(), ObservationRepositoryError>;

    fn get(
        &self,
        observation_id: &str,
        context: &AuthorizedObservationAccessContext,
    ) -> Result<Self::Observation, ObservationRepositoryError>;

    fn query(
        &self,
        context: &AuthorizedObservationAccessContext,
        query: &ObservationQuery,
    ) -> Result<Vec<Self::Observation>, ObservationRepositoryError>;

    fn timeline(
        &self,
        context: &AuthorizedObservationAccessContext,
    ) -> Result<Vec<ObservationTimelineEntry>, ObservationRepositoryError>;

    fn tombstone(
        &self,
        observation_id: &str,
        context: &AuthorizedObservationAccessContext,
    ) -> Result<(), ObservationRepositoryError>;

    fn verify(
        &self,
        observation_id: &str,
        context: &AuthorizedObservationAccessContext,
    ) -> Result<(), ObservationRepositoryError>;
}

/// Shared deterministic timeline ordering helper for all repository providers.
///
/// Known observation time is authoritative for chronology. Recorded time is only a
/// deterministic secondary key; it is never substituted into an observation when
/// observation time is unknown. Stable ID closes the ordering relation.
pub fn sort_timeline(entries: &mut [ObservationTimelineEntry]) {
    entries.sort_by(|left, right| {
        match (&left.observed_at, &right.observed_at) {
            (Some(left_time), Some(right_time)) => match left_time.cmp(right_time) {
                Ordering::Equal => left.recorded_at.cmp(&right.recorded_at),
                ordering => ordering,
            },
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => left.recorded_at.cmp(&right.recorded_at),
        }
        .then_with(|| left.observation_id.cmp(&right.observation_id))
    });
}
