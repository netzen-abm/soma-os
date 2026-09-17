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
mod tests {
    use super::*;

    fn request() -> AuthorizationRequest {
        AuthorizationRequest {
            principal_ref: "principal-1".into(),
            subject_ref: "person-1".into(),
            capability_id: "health.measurement.write".into(),
            resource_type: "health_state_measurement".into(),
            resource_id: "measurement-1".into(),
            action: "write".into(),
            tenant_id: "tenant-1".into(),
            data_domain: "personal_health".into(),
        }
    }

    fn context() -> AuthorizedMeasurementContext {
        AuthorizedMeasurementContext::from_authorized_request(&request()).unwrap()
    }

    fn plan() -> MeasurementPlan {
        MeasurementPlan {
            id: "measurement-1".into(),
            schema_version: SCHEMA_VERSION.into(),
            subject_ref: "person-1".into(),
            intervention_ref: "intervention-1".into(),
            metric: "sleep_duration".into(),
            measurement_method: "daily_user_report".into(),
            unit: Some("hours".into()),
            observation_window_start: "2026-09-20T00:00:00Z".into(),
            observation_window_end: "2026-10-04T00:00:00Z".into(),
            baseline_ref: Some("observation-baseline-1".into()),
            uncertainty_method: Some("reported_uncertainty".into()),
            rationale: "Measure the predefined response metric during the intervention window".into(),
            created_at: "2026-09-16T00:00:00Z".into(),
            actor_ref: Some("principal-1".into()),
        }
    }

    #[test]
    fn valid_measurement_plan_crosses_boundary() {
        assert!(MeasurementBoundary::validate(&plan(), &context()).is_ok());
    }

    #[test]
    fn delegated_actor_does_not_become_subject() {
        let request = request();
        let context = AuthorizedMeasurementContext::from_authorized_request(&request).unwrap();
        assert_eq!(request.principal_ref, "principal-1");
        assert_eq!(context.subject_ref(), "person-1");
    }

    #[test]
    fn subject_mismatch_is_rejected() {
        let mut candidate = plan();
        candidate.subject_ref = "person-2".into();
        assert_eq!(MeasurementBoundary::validate(&candidate, &context()), Err(MeasurementError::SubjectMismatch));
    }

    #[test]
    fn missing_intervention_reference_is_rejected() {
        let mut candidate = plan();
        candidate.intervention_ref.clear();
        assert_eq!(
            MeasurementBoundary::validate(&candidate, &context()),
            Err(MeasurementError::InvalidMeasurementPlan)
        );
    }

    #[test]
    fn measurement_fields_reject_control_characters() {
        let mut candidate = plan();
        candidate.metric = "sleep\nduration".into();
        assert_eq!(
            MeasurementBoundary::validate(&candidate, &context()),
            Err(MeasurementError::InvalidMeasurementPlan)
        );
    }
}
