use crate::{
    health_state_evidence_link::AuthorizedHealthStateEvidenceLinkContext,
    health_state_intervention::AuthorizedInterventionContext,
    health_state_measurement::AuthorizedMeasurementContext,
    health_state_repository::AuthorizedHealthStateAccessContext,
    longitudinal_context::AuthorizedLongitudinalContextAccessContext,
    longitudinal_observation_repository::AuthorizedObservationAccessContext,
    protected_db_context::AuthorizedProtectedDbContext,
};
use super::{AuthorizationDecision, AuthorizationRequest};

pub trait CanonicalAuthorizationBoundary {
    fn authorize(&self, request: &AuthorizationRequest) -> AuthorizationDecision;

    fn authorize_protected_context(
        &self, request: &AuthorizationRequest,
    ) -> Result<AuthorizedProtectedDbContext, AuthorizationDecision> {
        allow_context(self.authorize(request), || AuthorizedProtectedDbContext::from_authorized_request(request))
    }

    fn authorize_health_state_evidence_link_context(
        &self, request: &AuthorizationRequest,
    ) -> Result<AuthorizedHealthStateEvidenceLinkContext, AuthorizationDecision> {
        allow_context(self.authorize(request), || AuthorizedHealthStateEvidenceLinkContext::from_authorized_request(request))
    }

    fn authorize_health_state_repository_context(
        &self, request: &AuthorizationRequest,
    ) -> Result<AuthorizedHealthStateAccessContext, AuthorizationDecision> {
        allow_context(self.authorize(request), || AuthorizedHealthStateAccessContext::from_authorized_request(request))
    }

    fn authorize_longitudinal_context(
        &self, request: &AuthorizationRequest,
    ) -> Result<AuthorizedLongitudinalContextAccessContext, AuthorizationDecision> {
        allow_context(self.authorize(request), || AuthorizedLongitudinalContextAccessContext::from_authorized_request(request))
    }

    fn authorize_longitudinal_observation_context(
        &self, request: &AuthorizationRequest,
    ) -> Result<AuthorizedObservationAccessContext, AuthorizationDecision> {
        allow_context(self.authorize(request), || AuthorizedObservationAccessContext::from_authorized_request(request))
    }

    fn authorize_intervention_context(
        &self, request: &AuthorizationRequest,
    ) -> Result<AuthorizedInterventionContext, AuthorizationDecision> {
        allow_context(self.authorize(request), || AuthorizedInterventionContext::from_authorized_request(request))
    }

    fn authorize_measurement_context(
        &self, request: &AuthorizationRequest,
    ) -> Result<AuthorizedMeasurementContext, AuthorizationDecision> {
        allow_context(self.authorize(request), || AuthorizedMeasurementContext::from_authorized_request(request))
    }
}

fn allow_context<T, F>(
    decision: AuthorizationDecision, mint: F,
) -> Result<T, AuthorizationDecision>
where
    F: FnOnce() -> Result<T, impl Into<()>>,
{
    match decision {
        AuthorizationDecision::Allow => mint().map_err(|_| AuthorizationDecision::Deny),
        decision => Err(decision),
    }
}
