//! Ephemeral capability-permission lease contract for SOMA shared infrastructure.
//!
//! This is an application-level authorization/activation lease. It does not
//! attempt to revoke an operating-system permission, which remains controlled
//! by the host platform. SOMA must nevertheless stop using the resource as soon
//! as the governed purpose completes, and must require a fresh authorization
//! decision before the next activation.

use crate::canonical_authorization::AuthorizationDecision;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionState {
    Active,
    Released,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EphemeralPermissionLease {
    capability_id: String,
    resource_type: String,
    purpose: String,
    state: PermissionState,
    expires_at_epoch_seconds: u64,
}

impl EphemeralPermissionLease {
    /// Activate only after the canonical authorization boundary has returned ALLOW.
    /// Deny, consent, human-review, and degrade states cannot mint a lease.
    pub fn activate(
        decision: AuthorizationDecision,
        capability_id: String,
        resource_type: String,
        purpose: String,
        expires_at_epoch_seconds: u64,
    ) -> Result<Self, &'static str> {
        if !decision.allows_protected_execution()
            || capability_id.trim().is_empty()
            || resource_type.trim().is_empty()
            || purpose.trim().is_empty()
            || expires_at_epoch_seconds == 0
        {
            return Err("invalid_permission_lease");
        }

        Ok(Self {
            capability_id,
            resource_type,
            purpose,
            state: PermissionState::Active,
            expires_at_epoch_seconds,
        })
    }

    pub fn is_active(&self, now_epoch_seconds: u64) -> bool {
        self.state == PermissionState::Active && now_epoch_seconds < self.expires_at_epoch_seconds
    }

    /// Release immediately when the authorized purpose is complete, cancelled,
    /// failed, or authorization/consent is withdrawn.
    pub fn release(&mut self) {
        self.state = PermissionState::Released;
    }

    /// Convert an expired active lease to an explicit terminal state.
    pub fn expire_if_due(&mut self, now_epoch_seconds: u64) {
        if self.state == PermissionState::Active && now_epoch_seconds >= self.expires_at_epoch_seconds {
            self.state = PermissionState::Expired;
        }
    }

    pub fn capability_id(&self) -> &str {
        &self.capability_id
    }

    pub fn resource_type(&self) -> &str {
        &self.resource_type
    }

    pub fn purpose(&self) -> &str {
        &self.purpose
    }

    pub fn state(&self) -> PermissionState {
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::{AuthorizationDecision, EphemeralPermissionLease, PermissionState};

    fn lease() -> EphemeralPermissionLease {
        EphemeralPermissionLease::activate(
            AuthorizationDecision::Allow,
            "health.camera.capture".into(),
            "camera".into(),
            "capture_one_measurement".into(),
            100,
        )
        .unwrap()
    }

    #[test]
    fn permission_requires_canonical_allow() {
        for decision in [
            AuthorizationDecision::Deny,
            AuthorizationDecision::RequireConsent,
            AuthorizationDecision::RequireHumanReview,
            AuthorizationDecision::Degrade,
        ] {
            assert!(EphemeralPermissionLease::activate(
                decision,
                "health.camera.capture".into(),
                "camera".into(),
                "capture".into(),
                100,
            )
            .is_err());
        }
        assert!(EphemeralPermissionLease::activate(
            AuthorizationDecision::Allow,
            "health.camera.capture".into(),
            "camera".into(),
            "capture".into(),
            100,
        )
        .is_ok());
    }

    #[test]
    fn permission_is_active_only_during_authorized_window() {
        let mut lease = lease();
        assert!(lease.is_active(99));
        assert!(!lease.is_active(100));
        lease.expire_if_due(100);
        assert_eq!(lease.state(), PermissionState::Expired);
    }

    #[test]
    fn purpose_completion_releases_permission_immediately() {
        let mut lease = lease();
        lease.release();
        assert_eq!(lease.state(), PermissionState::Released);
        assert!(!lease.is_active(50));
    }

    #[test]
    fn released_permission_cannot_be_reactivated_in_place() {
        let mut lease = lease();
        lease.release();
        assert!(!lease.is_active(50));
        assert_eq!(lease.state(), PermissionState::Released);
    }

    #[test]
    fn invalid_permission_lease_is_rejected() {
        assert!(EphemeralPermissionLease::activate(AuthorizationDecision::Allow, "".into(), "camera".into(), "capture".into(), 100).is_err());
        assert!(EphemeralPermissionLease::activate(AuthorizationDecision::Allow, "health.camera.capture".into(), "".into(), "capture".into(), 100).is_err());
        assert!(EphemeralPermissionLease::activate(AuthorizationDecision::Allow, "health.camera.capture".into(), "camera".into(), "".into(), 100).is_err());
        assert!(EphemeralPermissionLease::activate(AuthorizationDecision::Allow, "health.camera.capture".into(), "camera".into(), "capture".into(), 0).is_err());
    }
}
