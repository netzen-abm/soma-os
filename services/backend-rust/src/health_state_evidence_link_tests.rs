use super::*;

    use super::*;

    fn request() -> AuthorizationRequest {
        AuthorizationRequest {
            principal_ref: "principal-1".into(),
            subject_ref: "person-1".into(),
            capability_id: "health.evidence.link".into(),
            capability_version: "1.0.0".into(),
            resource_type: "health_state_evidence_link".into(),
            resource_id: "link-1".into(),
            action: "read".into(),
            tenant_id: "tenant-1".into(),
            data_domain: "personal_health".into(),
        }
    }

    fn context() -> AuthorizedHealthStateEvidenceLinkContext {
        AuthorizedHealthStateEvidenceLinkContext::from_authorized_request(&request()).unwrap()
    }

    fn link(relationship: HealthStateEvidenceRelationship) -> HealthStateEvidenceLink {
        HealthStateEvidenceLink {
            id: "link-1".into(),
            schema_version: SCHEMA_VERSION.into(),
            health_state_ref: "hs-1".into(),
            evidence_ref: "claim-1".into(),
            relationship,
            provenance: LinkProvenance {
                method: "human-curated".into(),
                created_at: Some("2026-09-14T05:00:00Z".into()),
                actor_ref: Some("principal-1".into()),
            },
            context: None,
            uncertainty: Some(LinkUncertainty::Reported),
        }
    }

    #[test]
    fn valid_cross_domain_link_passes_boundary() {
        assert!(HealthStateEvidenceLinkBoundary::validate(
            &link(HealthStateEvidenceRelationship::EvidenceInformsHypothesis),
            &context()
        )
        .is_ok());
    }

    #[test]
    fn serde_values_match_machine_readable_schema() {
        let link = link(HealthStateEvidenceRelationship::EvidenceInformsHypothesis);
        let value = serde_json::to_value(&link).unwrap();
        assert_eq!(value["relationship"], "EVIDENCE_INFORMS_HYPOTHESIS");
        assert_eq!(value["uncertainty"], "reported");

        let round_trip: HealthStateEvidenceLink = serde_json::from_value(value).unwrap();
        assert_eq!(round_trip, link);
    }

    #[test]
    fn malformed_link_is_rejected() {
        let mut candidate = link(HealthStateEvidenceRelationship::EvidenceInformsHypothesis);
        candidate.health_state_ref.clear();
        assert_eq!(
            HealthStateEvidenceLinkBoundary::validate(&candidate, &context()),
            Err(HealthStateEvidenceLinkError::InvalidLink)
        );
    }

    #[test]
    fn malformed_authorization_context_is_rejected() {
        let mut request = request();
        request.data_domain.clear();
        assert_eq!(
            AuthorizedHealthStateEvidenceLinkContext::from_authorized_request(&request),
            Err(HealthStateEvidenceLinkError::InvalidAuthorizationContext)
        );
    }

    #[test]
    fn delegated_actor_does_not_become_subject() {
        let request = request();
        let context = AuthorizedHealthStateEvidenceLinkContext::from_authorized_request(&request).unwrap();
        assert_eq!(request.principal_ref, "principal-1");
        assert_eq!(context.subject_ref(), "person-1");
    }

    #[test]
    fn personal_response_is_explicitly_bounded() {
        assert!(HealthStateEvidenceLinkBoundary::validate_personal_response(
            &link(HealthStateEvidenceRelationship::PersonalResponseObservedAfterIntervention),
            &context()
        )
        .is_ok());

        assert_eq!(
            HealthStateEvidenceLinkBoundary::validate_personal_response(
                &link(HealthStateEvidenceRelationship::EvidenceInformsHypothesis),
                &context()
            ),
            Err(HealthStateEvidenceLinkError::InvalidLink)
        );
    }
