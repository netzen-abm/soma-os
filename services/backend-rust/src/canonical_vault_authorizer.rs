//! Adapter from the Local Health Vault compatibility authorization interface to
//! SOMA's canonical Rust authorization boundary.
//!
//! This module is deliberately an adapter, not a second policy engine. The
//! canonical boundary owns the authorization decision. The vault adapter only
//! translates the vault operation into the canonical request envelope and
//! preserves the existing subject-scope precondition before protected execution.

use crate::canonical_authorization::{
    AuthorizationDecision, AuthorizationRequest, CanonicalAuthorizationBoundary,
};
use crate::local_health_vault_storage::{AuthorizationContext, VaultAction, VaultAuthorizer};

const RESOURCE_TYPE: &str = "health_vault_record";
const DATA_DOMAIN: &str = "personal_health";

/// Canonical authorization adapter for Local Health Vault operations.
///
/// `tenant_id` is supplied by the trusted SOMA runtime rather than inferred
/// from caller input. `AuthorizationContext` carries independent actor and
/// subject identities; neither is inferred from the other.
pub struct CanonicalVaultAuthorizer<B> {
    boundary: B,
    tenant_id: String,
}

impl<B> CanonicalVaultAuthorizer<B> {
    pub fn new(boundary: B, tenant_id: impl Into<String>) -> Self {
        Self {
            boundary,
            tenant_id: tenant_id.into(),
        }
    }
}

impl<B> VaultAuthorizer for CanonicalVaultAuthorizer<B>
where
    B: CanonicalAuthorizationBoundary,
{
    fn authorize(
        &self,
        context: &AuthorizationContext,
        record_id: &str,
        action: VaultAction,
    ) -> bool {
        if context.principal_ref.trim().is_empty()
            || context.subject_ref.trim().is_empty()
            || context.scope.trim().is_empty()
        {
            return false;
        }

        let request = AuthorizationRequest {
            principal_ref: context.principal_ref.clone(),
            subject_ref: context.subject_ref.clone(),
            capability_id: capability_id(action).to_owned(),
            resource_type: RESOURCE_TYPE.to_owned(),
            resource_id: record_id.to_owned(),
            action: action_name(action).to_owned(),
            tenant_id: self.tenant_id.clone(),
            data_domain: DATA_DOMAIN.to_owned(),
        };

        matches!(
            self.boundary.authorize(&request),
            AuthorizationDecision::Allow
        )
    }
}

fn capability_id(action: VaultAction) -> &'static str {
    match action {
        VaultAction::Write => "health.vault.write",
        VaultAction::Read => "health.vault.read",
        VaultAction::List => "health.vault.list",
        VaultAction::Tombstone => "health.vault.tombstone",
        VaultAction::Verify => "health.vault.verify",
    }
}

fn action_name(action: VaultAction) -> &'static str {
    match action {
        VaultAction::Write => "write",
        VaultAction::Read => "read",
        VaultAction::List => "list",
        VaultAction::Tombstone => "tombstone",
        VaultAction::Verify => "verify",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct RecordingBoundary {
        decision: AuthorizationDecision,
        request: RefCell<Option<AuthorizationRequest>>,
    }

    impl CanonicalAuthorizationBoundary for RecordingBoundary {
        fn authorize(&self, request: &AuthorizationRequest) -> AuthorizationDecision {
            *self.request.borrow_mut() = Some(request.clone());
            self.decision
        }
    }

    fn context() -> AuthorizationContext {
        AuthorizationContext {
            principal_ref: "clinician-1".into(),
            subject_ref: "person-1".into(),
            scope: "tenant-1:personal_health".into(),
        }
    }

    #[test]
    fn only_canonical_allow_authorizes_vault_execution() {
        for decision in [
            AuthorizationDecision::Deny,
            AuthorizationDecision::RequireConsent,
            AuthorizationDecision::RequireHumanReview,
            AuthorizationDecision::Degrade,
        ] {
            let boundary = RecordingBoundary {
                decision,
                request: RefCell::new(None),
            };
            let authorizer = CanonicalVaultAuthorizer::new(boundary, "tenant-1");
            assert!(!authorizer.authorize(&context(), "record-1", VaultAction::Read));
        }
        let boundary = RecordingBoundary {
            decision: AuthorizationDecision::Allow,
            request: RefCell::new(None),
        };
        let authorizer = CanonicalVaultAuthorizer::new(boundary, "tenant-1");
        assert!(authorizer.authorize(&context(), "record-1", VaultAction::Read));
    }

    #[test]
    fn delegated_vault_request_preserves_actor_and_subject() {
        let boundary = RecordingBoundary {
            decision: AuthorizationDecision::Allow,
            request: RefCell::new(None),
        };
        let authorizer = CanonicalVaultAuthorizer::new(boundary, "tenant-1");
        assert!(authorizer.authorize(&context(), "record-1", VaultAction::Tombstone));
        let request = authorizer.boundary.request.borrow();
        let request = request.as_ref().expect("canonical request must be recorded");
        assert_eq!(request.principal_ref, "clinician-1");
        assert_eq!(request.subject_ref, "person-1");
        assert_ne!(request.principal_ref, request.subject_ref);
        assert_eq!(request.capability_id, "health.vault.tombstone");
        assert_eq!(request.resource_type, RESOURCE_TYPE);
        assert_eq!(request.resource_id, "record-1");
        assert_eq!(request.action, "tombstone");
        assert_eq!(request.tenant_id, "tenant-1");
        assert_eq!(request.data_domain, DATA_DOMAIN);
    }

    #[test]
    fn empty_identity_scope_fails_closed_before_policy_evaluation() {
        let boundary = RecordingBoundary {
            decision: AuthorizationDecision::Allow,
            request: RefCell::new(None),
        };
        let authorizer = CanonicalVaultAuthorizer::new(boundary, "tenant-1");
        let context = AuthorizationContext {
            principal_ref: "clinician-1".into(),
            subject_ref: "person-1".into(),
            scope: "".into(),
        };
        assert!(!authorizer.authorize(&context, "record-1", VaultAction::Read));
        assert!(authorizer.boundary.request.borrow().is_none());
    }
}
