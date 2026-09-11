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

/// Canonical authorization decision consumer boundary for Rust infrastructure.
///
/// Implementations are adapters to the already-governed authorization system;
/// they must not evaluate policy independently, widen identity scope, or infer
/// authorization from model output or caller metadata.
pub trait CanonicalAuthorizationBoundary {
    fn authorize(
        &self,
        principal_ref: &str,
        capability_id: &str,
        resource_type: &str,
        resource_id: &str,
        action: &str,
        tenant_id: &str,
        data_domain: &str,
    ) -> AuthorizationDecision;
}

#[cfg(test)]
mod tests {
    use super::AuthorizationDecision;

    #[test]
    fn only_allow_can_cross_protected_execution_boundary() {
        assert!(AuthorizationDecision::Allow.allows_protected_execution());
        assert!(!AuthorizationDecision::Deny.allows_protected_execution());
        assert!(!AuthorizationDecision::RequireConsent.allows_protected_execution());
        assert!(!AuthorizationDecision::RequireHumanReview.allows_protected_execution());
        assert!(!AuthorizationDecision::Degrade.allows_protected_execution());
    }
}
