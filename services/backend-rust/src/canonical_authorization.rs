//! Canonical Rust-side authorization decision contract.
//!
//! This module is deliberately a contract/adapter boundary, not a second policy
//! engine. Policy evaluation remains owned by SOMA's canonical authorization
//! decision boundary. Rust protected-data providers consume only the resulting
//! decision and must fail closed unless the decision is explicitly ALLOW.

use crate::health_state_evidence_link::AuthorizedHealthStateEvidenceLinkContext;
use crate::health_state_intervention::AuthorizedInterventionContext;
use crate::health_state_repository::AuthorizedHealthStateAccessContext;
use crate::longitudinal_context::AuthorizedLongitudinalContextAccessContext;
use crate::protected_db_context::AuthorizedProtectedDbContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationDecision {
    Allow,
    Deny,
    RequireConsent,
    RequireHumanReview,
    Degrade,
}

impl AuthorizationDecision {
    pub fn allows_protected_execution(self) -> bool {
        matches!(self, Self::Allow)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationRequest {
    pub principal_ref: String,
    pub capability_id: String,
    pub resource_type: String,
    pub resource_id: String,
    pub action: String,
    pub tenant_id: String,
    pub data_domain: String,
}

pub trait CanonicalAuthorizationBoundary {
    fn authorize(&self, request: &AuthorizationRequest) -> AuthorizationDecision;

    fn authorize_protected_context(&self, request: &AuthorizationRequest) -> Result<AuthorizedProtectedDbContext, AuthorizationDecision> {
        match self.authorize(request) {
            AuthorizationDecision::Allow => AuthorizedProtectedDbContext::from_authorized_request(request).map_err(|_| AuthorizationDecision::Deny),
            decision => Err(decision),
        }
    }

    fn authorize_health_state_evidence_link_context(&self, request: &AuthorizationRequest) -> Result<AuthorizedHealthStateEvidenceLinkContext, AuthorizationDecision> {
        match self.authorize(request) {
            AuthorizationDecision::Allow => AuthorizedHealthStateEvidenceLinkContext::from_authorized_request(request).map_err(|_| AuthorizationDecision::Deny),
            decision => Err(decision),
        }
    }

    fn authorize_health_state_repository_context(&self, request: &AuthorizationRequest) -> Result<AuthorizedHealthStateAccessContext, AuthorizationDecision> {
        match self.authorize(request) {
            AuthorizationDecision::Allow => AuthorizedHealthStateAccessContext::from_authorized_request(request).map_err(|_| AuthorizationDecision::Deny),
            decision => Err(decision),
        }
    }

    fn authorize_longitudinal_context(&self, request: &AuthorizationRequest) -> Result<AuthorizedLongitudinalContextAccessContext, AuthorizationDecision> {
        match self.authorize(request) {
            AuthorizationDecision::Allow => AuthorizedLongitudinalContextAccessContext::from_authorized_request(request).map_err(|_| AuthorizationDecision::Deny),
            decision => Err(decision),
        }
    }

    /// Mint an intervention context only from an authoritative ALLOW and a
    /// structurally valid governed request. This does not authorize execution.
    fn authorize_intervention_context(&self, request: &AuthorizationRequest) -> Result<AuthorizedInterventionContext, AuthorizationDecision> {
        match self.authorize(request) {
            AuthorizationDecision::Allow => AuthorizedInterventionContext::from_authorized_request(request).map_err(|_| AuthorizationDecision::Deny),
            decision => Err(decision),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AuthorizationDecision, AuthorizationRequest, CanonicalAuthorizationBoundary};

    struct AllowBoundary;
    impl CanonicalAuthorizationBoundary for AllowBoundary {
        fn authorize(&self, _request: &AuthorizationRequest) -> AuthorizationDecision { AuthorizationDecision::Allow }
    }

    struct DenyBoundary;
    impl CanonicalAuthorizationBoundary for DenyBoundary {
        fn authorize(&self, _request: &AuthorizationRequest) -> AuthorizationDecision { AuthorizationDecision::Deny }
    }

    fn request() -> AuthorizationRequest {
        AuthorizationRequest {
            principal_ref: "person-1".into(), capability_id: "health.intervention.write".into(),
            resource_type: "health_state_intervention".into(), resource_id: "intervention-1".into(),
            action: "write".into(), tenant_id: "tenant-1".into(), data_domain: "personal_health".into(),
        }
    }

    #[test]
    fn only_allow_can_cross_protected_execution_boundary() {
        assert!(AuthorizationDecision::Allow.allows_protected_execution());
        assert!(!AuthorizationDecision::Deny.allows_protected_execution());
        assert!(!AuthorizationDecision::RequireConsent.allows_protected_execution());
        assert!(!AuthorizationDecision::RequireHumanReview.allows_protected_execution());
        assert!(!AuthorizationDecision::Degrade.allows_protected_execution());
    }

    #[test]
    fn intervention_context_requires_authoritative_allow() {
        let context = AllowBoundary.authorize_intervention_context(&request()).unwrap();
        assert_eq!(context.subject_ref(), "person-1");
        assert_eq!(context.resource_type(), "health_state_intervention");
        assert_eq!(DenyBoundary.authorize_intervention_context(&request()), Err(AuthorizationDecision::Deny));
    }

    #[test]
    fn malformed_allow_cannot_mint_intervention_context() {
        let mut request = request();
        request.data_domain.clear();
        assert_eq!(AllowBoundary.authorize_intervention_context(&request), Err(AuthorizationDecision::Deny));
    }
}
