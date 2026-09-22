//! Bounded measurement contract for the canonical Health State domain.
//!
//! This module defines what should be measured, how, and over which window.
//! It does not store a measurement result or claim that an intervention worked.

use crate::canonical_authorization::AuthorizationRequest;
use thiserror::Error;

const SCHEMA_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasurementPlan {
    pub id: String,
    pub schema_version: String,
    pub subject_ref: String,
    pub intervention_ref: String,
    pub metric: String,
    pub measurement_method: String,
    pub unit: Option<String>,
    pub observation_window_start: String,
    pub observation_window_end: String,
    pub baseline_ref: Option<String>,
    pub uncertainty_method: Option<String>,
    pub rationale: String,
    pub created_at: String,
    pub actor_ref: Option<String>,
}

impl MeasurementPlan {
    pub fn validate(&self) -> Result<(), MeasurementError> {
        for value in [
            &self.id,
            &self.schema_version,
            &self.subject_ref,
            &self.intervention_ref,
            &self.metric,
            &self.measurement_method,
            &self.observation_window_start,
            &self.observation_window_end,
            &self.rationale,
            &self.created_at,
        ] {
            if value.trim().is_empty() || value.chars().any(char::is_control) {
                return Err(MeasurementError::InvalidMeasurementPlan);
            }
        }
        if self.schema_version != SCHEMA_VERSION {
            return Err(MeasurementError::InvalidMeasurementPlan);
        }
        for value in [
            self.unit.as_deref(),
            self.baseline_ref.as_deref(),
            self.uncertainty_method.as_deref(),
            self.actor_ref.as_deref(),
        ]
        .into_iter()
        .flatten()
        {
            if value.trim().is_empty() || value.chars().any(char::is_control) {
                return Err(MeasurementError::InvalidMeasurementPlan);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedMeasurementContext {
    subject_ref: String,
    tenant_id: String,
    data_domain: String,
    capability_id: String,
    capability_version: String,
    resource_type: String,
    resource_id: String,
    action: String,
}

impl AuthorizedMeasurementContext {
    pub(crate) fn from_authorized_request(request: &AuthorizationRequest) -> Result<Self, MeasurementError> {
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
            return Err(MeasurementError::InvalidAuthorizationContext);
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
pub enum MeasurementError {
    #[error("invalid measurement plan")]
    InvalidMeasurementPlan,
    #[error("invalid authorization context")]
    InvalidAuthorizationContext,
    #[error("measurement subject does not match authorized subject")]
    SubjectMismatch,
    #[error("measurement intervention reference is required")]
    MissingInterventionReference,
}

pub struct MeasurementBoundary;

impl MeasurementBoundary {
    pub fn validate(plan: &MeasurementPlan, context: &AuthorizedMeasurementContext) -> Result<(), MeasurementError> {
        plan.validate()?;
        if plan.subject_ref != context.subject_ref() {
            return Err(MeasurementError::SubjectMismatch);
        }
        if plan.intervention_ref.trim().is_empty() {
            return Err(MeasurementError::MissingInterventionReference);
        }
        Ok(())
    }
}
#[cfg(test)]
#[path = "health_state_measurement_tests.rs"]
mod tests;
