use super::{AuthorizationDecision, AuthorizationRequest, CanonicalAuthorizationBoundary};

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

    #[test]
    fn protected_context_requires_authoritative_allow() {
        let context = AllowBoundary.authorize_protected_context(&request()).unwrap();
        assert_eq!(context.principal_ref(), "principal-1");
        assert_eq!(context.subject_ref(), "person-1");
        assert_eq!(context.capability_id(), "health.read");
        assert_eq!(context.capability_version(), "1.0.0");
        assert_eq!(context.resource_type(), "health_record");
        assert_eq!(context.resource_id(), "record-1");
        assert_eq!(context.action(), "read");
        assert_eq!(context.tenant_id(), "tenant-1");
        assert_eq!(context.data_domain(), "personal_health");

        assert_eq!(DenyBoundary.authorize_protected_context(&request()), Err(AuthorizationDecision::Deny));
    }

    #[test]
    fn malformed_allow_cannot_mint_protected_context() {
        let mut request = request();
        request.action = "".into();
        assert_eq!(AllowBoundary.authorize_protected_context(&request), Err(AuthorizationDecision::Deny));
    }

    #[test]
    fn health_state_evidence_link_context_requires_authoritative_allow() {
        let context = AllowBoundary.authorize_health_state_evidence_link_context(&request()).unwrap();
        assert_eq!(context.subject_ref(), "person-1");
        assert_eq!(context.resource_type(), "health_record");
        assert_eq!(context.resource_id(), "record-1");

        assert_eq!(
            DenyBoundary.authorize_health_state_evidence_link_context(&request()),
            Err(AuthorizationDecision::Deny)
        );
    }

    #[test]
    fn malformed_allow_cannot_mint_health_state_evidence_link_context() {
        let mut request = request();
        request.data_domain = "".into();
        assert_eq!(
            AllowBoundary.authorize_health_state_evidence_link_context(&request),
            Err(AuthorizationDecision::Deny)
        );
    }

    #[test]
    fn health_state_repository_context_requires_authoritative_allow() {
        let context = AllowBoundary.authorize_health_state_repository_context(&request()).unwrap();
        assert_eq!(context.subject_ref(), "person-1");
        assert_eq!(context.scope(), "tenant-1:personal_health");
        assert_eq!(
            DenyBoundary.authorize_health_state_repository_context(&request()),
            Err(AuthorizationDecision::Deny)
        );
    }

    #[test]
    fn malformed_allow_cannot_mint_health_state_repository_context() {
        let mut request = request();
        request.data_domain = "".into();
        assert_eq!(AllowBoundary.authorize_health_state_repository_context(&request), Err(AuthorizationDecision::Deny));
    }

    #[test]
    fn longitudinal_context_requires_authoritative_allow() {
        let context = AllowBoundary.authorize_longitudinal_context(&request()).unwrap();
        assert_eq!(context.subject_ref(), "person-1");
        assert_eq!(context.scope(), "tenant-1:personal_health");
        assert_eq!(DenyBoundary.authorize_longitudinal_context(&request()), Err(AuthorizationDecision::Deny));
    }

    #[test]
    fn malformed_allow_cannot_mint_longitudinal_context() {
        let mut request = request();
        request.data_domain = "".into();
        assert_eq!(AllowBoundary.authorize_longitudinal_context(&request), Err(AuthorizationDecision::Deny));
    }

    #[test]
    fn longitudinal_observation_context_requires_authoritative_allow() {
        let mut request = request();
        request.capability_id = "health.timeline.read".into();
        request.resource_type = "health_observation".into();
        request.resource_id = "observation-1".into();
        request.action = "read".into();
        let context = AllowBoundary.authorize_longitudinal_observation_context(&request).unwrap();
        assert_eq!(context.subject_ref(), "person-1");
        assert_eq!(context.capability_id(), "health.timeline.read");
        assert_eq!(context.capability_version(), "1.0.0");
        assert_eq!(context.resource_id(), "observation-1");
        assert_eq!(DenyBoundary.authorize_longitudinal_observation_context(&request), Err(AuthorizationDecision::Deny));
    }

    #[test]
    fn malformed_allow_cannot_mint_longitudinal_observation_context() {
        let mut request = request();
        request.capability_id = "health.timeline.read".into();
        request.resource_type = "health_observation".into();
        request.resource_id = "observation-1".into();
        request.action = "".into();
        assert_eq!(
            AllowBoundary.authorize_longitudinal_observation_context(&request),
            Err(AuthorizationDecision::Deny)
        );
    }

    #[test]
    fn intervention_context_requires_authoritative_allow() {
        let mut request = request();
        request.principal_ref = "person-1".into();
        request.subject_ref = "person-1".into();
        request.capability_id = "health.intervention.write".into();
        request.resource_type = "health_state_intervention".into();
        request.resource_id = "intervention-1".into();
        request.action = "write".into();
        let context = AllowBoundary.authorize_intervention_context(&request).unwrap();
        assert_eq!(context.subject_ref(), "person-1");
        assert_eq!(context.resource_type(), "health_state_intervention");
        assert_eq!(DenyBoundary.authorize_intervention_context(&request), Err(AuthorizationDecision::Deny));
    }

    #[test]
    fn malformed_allow_cannot_mint_intervention_context() {
        let mut request = request();
        request.data_domain = "".into();
        assert_eq!(AllowBoundary.authorize_intervention_context(&request), Err(AuthorizationDecision::Deny));
    }

    #[test]
    fn measurement_context_requires_authoritative_allow() {
        let mut request = request();
        request.principal_ref = "person-1".into();
        request.subject_ref = "person-1".into();
        request.capability_id = "health.measurement.write".into();
        request.resource_type = "health_state_measurement".into();
        request.resource_id = "measurement-1".into();
        request.action = "write".into();
        let context = AllowBoundary.authorize_measurement_context(&request).unwrap();
        assert_eq!(context.subject_ref(), "person-1");
        assert_eq!(context.resource_type(), "health_state_measurement");
        assert_eq!(DenyBoundary.authorize_measurement_context(&request), Err(AuthorizationDecision::Deny));
    }

    #[test]
    fn malformed_allow_cannot_mint_measurement_context() {
        let mut request = request();
        request.data_domain = "".into();
        assert_eq!(AllowBoundary.authorize_measurement_context(&request), Err(AuthorizationDecision::Deny));
    }
