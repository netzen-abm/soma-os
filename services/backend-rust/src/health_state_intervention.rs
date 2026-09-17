//! Bounded, governed intervention-intent contract for the canonical Health State domain.
//!
//! This module records what is intended to change without executing the change,
//! claiming efficacy, or creating a separate experiment/action engine.

use crate::canonical_authorization::AuthorizationRequest;
use thiserror::Error;

const SCHEMA_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterventionIntentStatus {
    Proposed,
    Authorized,
    Executed,
    Cancelled,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterventionIntent {
    pub id: String,
    pub schema_version: String,
    pub subject_ref: String,
    pub intervention_type: String,
    pub description: String,
    pub intended_change: String,
    pub status: InterventionIntentStatus,
    pub rationale: String,
    pub evidence_refs: Vec<String>,
    pub safety_refs: Vec<String>,
    pub causality_refs: Vec<String>,
    pub planned_start: Option<String>,
    pub planned_end: Option<String>,
    pub created_at: String,
    pub actor_ref: Option<String>,
}

impl InterventionIntent {
    pub fn validate(&self) -> Result<(), InterventionIntentError> {
        for value in [
            &self.id,
            &self.schema_version,
            &self.subject_ref,
            &self.intervention_type,
            &self.description,
            &self.intended_change,
            &self.rationale,
            &self.created_at,
        ] {
            if value.trim().is_empty() || value.chars().any(char::is_control) {
                return Err(InterventionIntentError::InvalidIntent);
            }
        }
        if self.schema_version != SCHEMA_VERSION {
            return Err(InterventionIntentError::InvalidIntent);
        }
        for refs in [&self.evidence_refs, &self.safety_refs, &self.causality_refs] {
            if refs.iter().any(|value| value.trim().is_empty() || value.chars().any(char::is_control)) {
                return Err(InterventionIntentError::InvalidReference);
            }
        }
        if self.planned_start.as_deref().is_some_and(|value| value.trim().is_empty())
            || self.planned_end.as_deref().is_some_and(|value| value.trim().is_empty())
            || self.actor_ref.as_deref().is_some_and(|value| value.trim().is_empty())
        {
            return Err(InterventionIntentError::InvalidIntent);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedInterventionContext {
    subject_ref: String,
    tenant_id: String,
    data_domain: String,
    capability_id: String,
    resource_type: String,
    resource_id: String,
    action: String,
}

impl AuthorizedInterventionContext {
    pub(crate) fn from_authorized_request(request: &AuthorizationRequest) -> Result<Self, InterventionIntentError> {
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
            return Err(InterventionIntentError::InvalidAuthorizationContext);
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

    pub fn subject_ref(&self) -> &str { &self.subject_ref }
    pub fn tenant_id(&self) -> &str { &self.tenant_id }
    pub fn data_domain(&self) -> &str { &self.data_domain }
    pub fn capability_id(&self) -> &str { &self.capability_id }
    pub fn resource_type(&self) -> &str { &self.resource_type }
    pub fn resource_id(&self) -> &str { &self.resource_id }
    pub fn action(&self) -> &str { &self.action }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum InterventionIntentError {
    #[error("invalid intervention intent")]
    InvalidIntent,
    #[error("invalid intervention reference")]
    InvalidReference,
    #[error("invalid authorization context")]
    InvalidAuthorizationContext,
    #[error("intervention subject does not match authorized subject")]
    SubjectMismatch,
}

pub struct InterventionIntentBoundary;

impl InterventionIntentBoundary {
    pub fn validate(
        intent: &InterventionIntent,
        context: &AuthorizedInterventionContext,
    ) -> Result<(), InterventionIntentError> {
        intent.validate()?;
        if intent.subject_ref != context.subject_ref() {
            return Err(InterventionIntentError::SubjectMismatch);
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
            capability_id: "health.intervention.write".into(),
            resource_type: "health_state_intervention".into(),
            resource_id: "intervention-1".into(),
            action: "write".into(),
            tenant_id: "tenant-1".into(),
            data_domain: "personal_health".into(),
        }
    }

    fn context() -> AuthorizedInterventionContext {
        AuthorizedInterventionContext::from_authorized_request(&request()).unwrap()
    }

    fn intent() -> InterventionIntent {
        InterventionIntent {
            id: "intervention-1".into(),
            schema_version: SCHEMA_VERSION.into(),
            subject_ref: "person-1".into(),
            intervention_type: "lifestyle_change".into(),
            description: "Bounded change for a defined observation window".into(),
            intended_change: "Change one specified behavior".into(),
            status: InterventionIntentStatus::Proposed,
            rationale: "Testable hypothesis with explicit evidence and safety references".into(),
            evidence_refs: vec!["pubmed:123".into()],
            safety_refs: vec!["safety:123".into()],
            causality_refs: vec!["causality:123".into()],
            planned_start: Some("2026-09-20T00:00:00Z".into()),
            planned_end: Some("2026-10-04T00:00:00Z".into()),
            created_at: "2026-09-15T00:00:00Z".into(),
            actor_ref: Some("principal-1".into()),
        }
    }

    #[test]
    fn valid_intent_crosses_validation_boundary() {
        assert!(InterventionIntentBoundary::validate(&intent(), &context()).is_ok());
    }

    #[test]
    fn delegated_actor_does_not_become_subject() {
        let request = request();
        let context = AuthorizedInterventionContext::from_authorized_request(&request).unwrap();
        assert_eq!(request.principal_ref, "principal-1");
        assert_eq!(context.subject_ref(), "person-1");
    }

    #[test]
    fn subject_mismatch_is_rejected() {
        let mut candidate = intent();
        candidate.subject_ref = "person-2".into();
        assert_eq!(InterventionIntentBoundary::validate(&candidate, &context()), Err(InterventionIntentError::SubjectMismatch));
    }

    #[test]
    fn empty_intended_change_is_rejected() {
        let mut candidate = intent();
        candidate.intended_change.clear();
        assert_eq!(InterventionIntentBoundary::validate(&candidate, &context()), Err(InterventionIntentError::InvalidIntent));
    }

    #[test]
    fn references_reject_control_characters() {
        let mut candidate = intent();
        candidate.evidence_refs.push("bad\nref".into());
        assert_eq!(InterventionIntentBoundary::validate(&candidate, &context()), Err(InterventionIntentError::InvalidReference));
    }
}
