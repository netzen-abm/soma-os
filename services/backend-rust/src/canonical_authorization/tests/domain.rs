use super::{AuthorizationDecision, AuthorizationRequest, AllowBoundary, CanonicalAuthorizationBoundary, DenyBoundary, request};

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
