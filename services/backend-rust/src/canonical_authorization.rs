//! Canonical Rust-side authorization decision contract.
//!
//! This module is deliberately a contract/adapter boundary, not a second policy
//! engine. Policy evaluation remains owned by SOMA's canonical authorization
//! decision boundary. Rust protected-data providers consume only the resulting
//! decision and must fail closed unless the decision is explicitly ALLOW.

use crate::health_state_evidence_link::AuthorizedHealthStateEvidenceLinkContext;
use crate::health_state_intervention::AuthorizedInterventionContext;
use crate::health_state_measurement::AuthorizedMeasurementContext;
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

/// Canonical authorization request envelope consumed by the Rust adapter boundary.
///
/// `principal_ref` identifies the actor making the request. `subject_ref` identifies
/// the health/data subject on whose behalf the protected operation is performed.
/// They are intentionally distinct so delegated access cannot collapse actor and
/// subject identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationRequest {
    pub principal_ref: String,
    pub subject_ref: String,
    pub capability_id: String,
    pub resource_type: String,
    pub resource_id: String,
    pub action: String,
    pub tenant_id: String,
    pub data_domain: String,
}

/// Canonical authorization decision consumer boundary for Rust infrastructure.
///
/// Implementations are adapters to the already-governed authorization system;
/// they must not evaluate policy independently, widen identity scope, or infer
/// authorization from model output or caller metadata.
pub trait CanonicalAuthorizationBoundary {
    fn authorize(&self, request: &AuthorizationRequest) -> AuthorizationDecision;

    /// Mint a protected execution context only from an authoritative ALLOW and
    /// a structurally valid governed request.
    fn authorize_protected_context(
        &self,
        request: &AuthorizationRequest,
    ) -> Result<AuthorizedProtectedDbContext, AuthorizationDecision> {
        match self.authorize(request) {
            AuthorizationDecision::Allow => {
                AuthorizedProtectedDbContext::from_authorized_request(request).map_err(|_| AuthorizationDecision::Deny)
            }
            decision => Err(decision),
        }
    }

    /// Mint a Health State ↔ Evidence linkage context only from an authoritative
    /// ALLOW and a structurally valid governed request.
    fn authorize_health_state_evidence_link_context(
        &self,
        request: &AuthorizationRequest,
    ) -> Result<AuthorizedHealthStateEvidenceLinkContext, AuthorizationDecision> {
        match self.authorize(request) {
            AuthorizationDecision::Allow => AuthorizedHealthStateEvidenceLinkContext::from_authorized_request(request)
                .map_err(|_| AuthorizationDecision::Deny),
            decision => Err(decision),
        }
    }

    /// Mint a Health State repository access context only from an authoritative
    /// ALLOW and a structurally valid governed request.
    fn authorize_health_state_repository_context(
        &self,
        request: &AuthorizationRequest,
    ) -> Result<AuthorizedHealthStateAccessContext, AuthorizationDecision> {
        match self.authorize(request) {
            AuthorizationDecision::Allow => AuthorizedHealthStateAccessContext::from_authorized_request(request)
                .map_err(|_| AuthorizationDecision::Deny),
            decision => Err(decision),
        }
    }

    /// Mint a longitudinal-context read context only from an authoritative ALLOW
    /// and a structurally valid governed request.
    fn authorize_longitudinal_context(
        &self,
        request: &AuthorizationRequest,
    ) -> Result<AuthorizedLongitudinalContextAccessContext, AuthorizationDecision> {
        match self.authorize(request) {
            AuthorizationDecision::Allow => {
                AuthorizedLongitudinalContextAccessContext::from_authorized_request(request)
                    .map_err(|_| AuthorizationDecision::Deny)
            }
            decision => Err(decision),
        }
    }

    /// Mint an intervention context only from an authoritative ALLOW and a
    /// structurally valid governed request. This does not authorize execution.
    fn authorize_intervention_context(
        &self,
        request: &AuthorizationRequest,
    ) -> Result<AuthorizedInterventionContext, AuthorizationDecision> {
        match self.authorize(request) {
            AuthorizationDecision::Allow => {
                AuthorizedInterventionContext::from_authorized_request(request).map_err(|_| AuthorizationDecision::Deny)
            }
            decision => Err(decision),
        }
    }

    /// Mint a measurement context only from an authoritative ALLOW and a
    /// structurally valid governed request. This does not validate clinical
    /// appropriateness or authorize an intervention.
    fn authorize_measurement_context(
        &self,
        request: &AuthorizationRequest,
    ) -> Result<AuthorizedMeasurementContext, AuthorizationDecision> {
        match self.authorize(request) {
            AuthorizationDecision::Allow => {
                AuthorizedMeasurementContext::from_authorized_request(request).map_err(|_| AuthorizationDecision::Deny)
            }
            decision => Err(decision),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AuthorizationDecision, AuthorizationRequest, CanonicalAuthorizationBoundary};

    #[test]
    fn only_allow_can_cross_protected_execution_boundary() {
        assert!(AuthorizationDecision::Allow.allows_protected_execution());
        assert!(!AuthorizationDecision::Deny.allows_protected_execution());
        assert!(!AuthorizationDecision::RequireConsent.allows_protected_execution());
        assert!(!AuthorizationDecision::RequireHumanReview.allows_protected_execution());
        assert!(!AuthorizationDecision::Degrade.allows_protected_execution());
    }

    fn request() -> AuthorizationRequest {
        AuthorizationRequest {
            principal_ref: "principal-1".into(),
            subject_ref: "person-1".into(),
            capability_id: "health.read".into(),
            resource_type: "health_record".into(),
            resource_id: "record-1".into(),
            action: "read".into(),
            tenant_id: "tenant-1".into(),
            data_domain: "personal_health".into(),
        }
    }

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

    #[test]
    fn authorization_request_keeps_actor_and_subject_distinct() {
        let request = request();
        assert_eq!(request.principal_ref, "principal-1");
        assert_eq!(request.subject_ref, "person-1");
    }

    #[test]
    fn protected_context_preserves_actor_and_subject() {
        let context = AllowBoundary.authorize_protected_context(&request()).unwrap();
        assert_eq!(context.principal_ref(), "principal-1");
        assert_eq!(context.subject_ref(), "person-1");
        assert_eq!(context.capability_id(), "health.read");
        assert_eq!(context.resource_type(), "health_record");
        assert_eq!(context.resource_id(), "record-1");
        assert_eq!(context.action(), "read");
        assert_eq!(context.tenant_id(), "tenant-1");
        assert_eq!(context.data_domain(), "personal_health");
        assert_eq!(DenyBoundary.authorize_protected_context(&request()), Err(AuthorizationDecision::Deny));
    }
}
