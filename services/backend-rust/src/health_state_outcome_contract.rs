//! Bounded, governed outcome contract for the canonical Health State domain.
//!
//! An outcome records a defined result over an explicit observation window. It
//! does not itself establish efficacy, causality, clinical significance, or an
//! adaptation decision.

use crate::canonical_authorization::{AuthorizationDecision, AuthorizationRequest, CanonicalAuthorizationBoundary};
use thiserror::Error;

const SCHEMA_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutcomeStatus {
    Planned,
    Observed,
    Incomplete,
    Cancelled,
    Finalized,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    pub id: String,
    pub schema_version: String,
    pub subject_ref: String,
    pub intervention_ref: String,
    pub measurement_ref: String,
    pub metric: String,
    pub result: String,
    pub unit: Option<String>,
    pub observation_window_start: String,
    pub observation_window_end: String,
    pub baseline_ref: Option<String>,
    pub uncertainty: Option<String>,
    pub status: OutcomeStatus,
    pub interpretation_ref: Option<String>,
    pub rationale: String,
    pub source_ref: String,
    pub recorded_at: String,
    pub actor_ref: Option<String>,
}

impl Outcome {
    pub fn validate(&self) -> Result<(), OutcomeError> {
        for value in [
            &self.id,
            &self.schema_version,
            &self.subject_ref,
            &self.intervention_ref,
            &self.measurement_ref,
            &self.metric,
            &self.result,
            &self.observation_window_start,
            &self.observation_window_end,
            &self.rationale,
            &self.source_ref,
            &self.recorded_at,
        ] {
            if value.trim().is_empty() || value.chars().any(char::is_control) {
                return Err(OutcomeError::InvalidOutcome);
            }
        }
        if self.schema_version != SCHEMA_VERSION {
            return Err(OutcomeError::InvalidOutcome);
        }
        for value in [
            self.unit.as_deref(),
            self.baseline_ref.as_deref(),
            self.uncertainty.as_deref(),
            self.interpretation_ref.as_deref(),
            self.actor_ref.as_deref(),
        ]
        .into_iter()
        .flatten()
        {
            if value.trim().is_empty() || value.chars().any(char::is_control) {
                return Err(OutcomeError::InvalidReference);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedOutcomeContext {
    subject_ref: String,
    tenant_id: String,
    data_domain: String,
    capability_id: String,
    capability_version: String,
    resource_type: String,
    resource_id: String,
    action: String,
}

impl AuthorizedOutcomeContext {
    pub(crate) fn from_authorized_request(request: &AuthorizationRequest) -> Result<Self, OutcomeError> {
        let values = [
            &request.principal_ref,
            &request.subject_ref,
            &request.capability_id,
            &request.capability_version,
            &request.resource_type,
            &request.resource_id,
            &request.action,
            &request.tenant_id,
            &request.data_domain,
        ];
        if values.iter().any(|value| value.trim().is_empty() || value.chars().any(char::is_control)) {
            return Err(OutcomeError::InvalidAuthorizationContext);
        }
        Ok(Self {
            subject_ref: request.subject_ref.clone(),
            tenant_id: request.tenant_id.clone(),
            data_domain: request.data_domain.clone(),
            capability_id: request.capability_id.clone(),
            capability_version: request.capability_version.clone(),
            resource_type: request.resource_type.clone(),
            resource_id: request.resource_id.clone(),
            action: request.action.clone(),
        })
    }

    pub fn subject_ref(&self) -> &str {
        &self.subject_ref
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn data_domain(&self) -> &str {
        &self.data_domain
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

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OutcomeError {
    #[error("invalid outcome")]
    InvalidOutcome,
    #[error("invalid outcome reference")]
    InvalidReference,
    #[error("invalid authorization context")]
    InvalidAuthorizationContext,
    #[error("outcome subject does not match authorized subject")]
    SubjectMismatch,
}

