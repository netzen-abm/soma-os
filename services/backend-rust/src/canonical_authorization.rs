//! Canonical Rust-side authorization decision contract.
//!
//! This module is deliberately a contract/adapter boundary, not a second policy
//! engine. Policy evaluation remains owned by SOMA's canonical authorization
//! decision boundary. Rust protected-data providers consume only the resulting
//! decision and must fail closed unless the decision is explicitly ALLOW.

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
/// Keeping authorization inputs in one explicit value object reduces positional
/// argument risk and makes the governed request scope auditable at the boundary.
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

/// Canonical authorization decision consumer boundary for Rust infrastructure.
///
/// Implementations are adapters to the already-governed authorization system;
/// they must not evaluate policy independently, widen identity scope, or infer
/// authorization from model output or caller metadata.
pub trait CanonicalAuthorizationBoundary {
    fn authorize(&self, request: &AuthorizationRequest) -> AuthorizationDecision;
}

#[cfg(test)]
mod tests {
    use super::{AuthorizationDecision, AuthorizationRequest};

    #[test]
    fn only_allow_can_cross_protected_execution_boundary() {
        assert!(AuthorizationDecision::Allow.allows_protected_execution());
        assert!(!AuthorizationDecision::Deny.allows_protected_execution());
        assert!(!AuthorizationDecision::RequireConsent.allows_protected_execution());
        assert!(!AuthorizationDecision::RequireHumanReview.allows_protected_execution());
        assert!(!AuthorizationDecision::Degrade.allows_protected_execution());
    }

    #[test]
    fn authorization_request_keeps_governance_scope_explicit() {
        let request = AuthorizationRequest {
            principal_ref: "principal-1".to_owned(),
            capability_id: "health.read".to_owned(),
            resource_type: "health_record".to_owned(),
            resource_id: "record-1".to_owned(),
            action: "read".to_owned(),
            tenant_id: "tenant-1".to_owned(),
            data_domain: "personal_health".to_owned(),
        };

        assert_eq!(request.principal_ref, "principal-1");
        assert_eq!(request.capability_id, "health.read");
        assert_eq!(request.resource_type, "health_record");
        assert_eq!(request.resource_id, "record-1");
        assert_eq!(request.action, "read");
        assert_eq!(request.tenant_id, "tenant-1");
        assert_eq!(request.data_domain, "personal_health");
    }
}
