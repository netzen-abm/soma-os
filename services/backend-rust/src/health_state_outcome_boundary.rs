use crate::canonical_authorization::{AuthorizationDecision, AuthorizationRequest, CanonicalAuthorizationBoundary};
use super::contract::{AuthorizedOutcomeContext, Outcome, OutcomeError};

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