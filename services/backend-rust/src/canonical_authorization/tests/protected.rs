use super::{AuthorizationDecision, AuthorizationRequest, AllowBoundary, CanonicalAuthorizationBoundary, DenyBoundary, request};

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
            subject_ref: "person-1".to_owned(),
            capability_id: "health.read".to_owned(),
            capability_version: "1.0.0".to_owned(),
            resource_type: "health_record".to_owned(),
            resource_id: "record-1".to_owned(),
            action: "read".to_owned(),
            tenant_id: "tenant-1".to_owned(),
            data_domain: "personal_health".to_owned(),
        };

        assert_eq!(request.principal_ref, "principal-1");
        assert_eq!(request.subject_ref, "person-1");
        assert_eq!(request.capability_id, "health.read");
        assert_eq!(request.capability_version, "1.0.0");
        assert_eq!(request.resource_type, "health_record");
        assert_eq!(request.resource_id, "record-1");
        assert_eq!(request.action, "read");
        assert_eq!(request.tenant_id, "tenant-1");
        assert_eq!(request.data_domain, "personal_health");
    }
