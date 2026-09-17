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

/// Outcome authorization adapter: consumes the canonical authorization decision
/// and mints the outcome context only after an explicit ALLOW.
pub struct OutcomeAuthorizationBoundary;

impl OutcomeAuthorizationBoundary {
    pub fn authorize<C: CanonicalAuthorizationBoundary>(
        boundary: &C,
        request: &AuthorizationRequest,
    ) -> Result<AuthorizedOutcomeContext, AuthorizationDecision> {
        match boundary.authorize(request) {
            AuthorizationDecision::Allow => {
                AuthorizedOutcomeContext::from_authorized_request(request).map_err(|_| AuthorizationDecision::Deny)
            }
            decision => Err(decision),
        }
    }
}

pub struct OutcomeBoundary;

impl OutcomeBoundary {
    pub fn validate(outcome: &Outcome, context: &AuthorizedOutcomeContext) -> Result<(), OutcomeError> {
        outcome.validate()?;
        if outcome.subject_ref != context.subject_ref() {
            return Err(OutcomeError::SubjectMismatch);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AllowBoundary;

    impl CanonicalAuthorizationBoundary for AllowBoundary {
        fn authorize(&self, _request: &AuthorizationRequest) -> AuthorizationDecision {
            AuthorizationDecision::Allow
        }
    }

    struct DenyBoundary;

    impl CanonicalAuthorizationBoundary for DenyBoundary {
        fn authorize(&self, _request: &AuthorizationRequest) -> AuthorizationDecision {
            AuthorizationDecision::Deny
        }
    }

    fn request() -> AuthorizationRequest {
        AuthorizationRequest {
            principal_ref: "principal-1".into(),
            subject_ref: "person-1".into(),
            capability_id: "health.outcome.write".into(),
            resource_type: "health_state_outcome".into(),
            resource_id: "outcome-1".into(),
            action: "write".into(),
            tenant_id: "tenant-1".into(),
            data_domain: "personal_health".into(),
        }
    }

    fn context() -> AuthorizedOutcomeContext {
        OutcomeAuthorizationBoundary::authorize(&AllowBoundary, &request()).unwrap()
    }

    fn outcome() -> Outcome {
        Outcome {
            id: "outcome-1".into(),
            schema_version: SCHEMA_VERSION.into(),
            subject_ref: "person-1".into(),
            intervention_ref: "intervention-1".into(),
            measurement_ref: "measurement-1".into(),
            metric: "defined_metric".into(),
            result: "recorded_result".into(),
            unit: Some("unit".into()),
            observation_window_start: "2026-09-16T00:00:00Z".into(),
            observation_window_end: "2026-09-30T00:00:00Z".into(),
            baseline_ref: Some("observation-1".into()),
            uncertainty: Some("not_assessed".into()),
            status: OutcomeStatus::Observed,
            interpretation_ref: None,
            rationale: "Defined result over a bounded observation window".into(),
            source_ref: "observation:response-1".into(),
            recorded_at: "2026-09-30T00:00:00Z".into(),
            actor_ref: Some("principal-1".into()),
        }
    }

    #[test]
    fn valid_outcome_crosses_validation_boundary() {
        assert!(OutcomeBoundary::validate(&outcome(), &context()).is_ok());
    }

    #[test]
    fn deny_cannot_mint_context() {
        assert_eq!(
            OutcomeAuthorizationBoundary::authorize(&DenyBoundary, &request()),
            Err(AuthorizationDecision::Deny)
        );
    }

    #[test]
    fn delegated_actor_does_not_become_subject() {
        let request = request();
        let context = AuthorizedOutcomeContext::from_authorized_request(&request).unwrap();
        assert_eq!(request.principal_ref, "principal-1");
        assert_eq!(context.subject_ref(), "person-1");
    }

    #[test]
    fn subject_mismatch_is_rejected() {
        let mut candidate = outcome();
        candidate.subject_ref = "person-2".into();
        assert_eq!(OutcomeBoundary::validate(&candidate, &context()), Err(OutcomeError::SubjectMismatch));
    }

    #[test]
    fn missing_measurement_reference_is_rejected() {
        let mut candidate = outcome();
        candidate.measurement_ref.clear();
        assert_eq!(OutcomeBoundary::validate(&candidate, &context()), Err(OutcomeError::InvalidOutcome));
    }

    #[test]
    fn references_reject_control_characters() {
        let mut candidate = outcome();
        candidate.baseline_ref = Some("bad\nref".into());
        assert_eq!(OutcomeBoundary::validate(&candidate, &context()), Err(OutcomeError::InvalidReference));
    }
}
