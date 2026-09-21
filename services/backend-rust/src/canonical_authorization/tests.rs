use super::{AuthorizationDecision, AuthorizationRequest, CanonicalAuthorizationBoundary};

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
        capability_id: "health.read".into(),
        capability_version: "1.0.0".into(),
        resource_type: "health_record".into(),
        resource_id: "record-1".into(),
        action: "read".into(),
        tenant_id: "tenant-1".into(),
        data_domain: "personal_health".into(),
    }
}

mod protected;
mod domain;
