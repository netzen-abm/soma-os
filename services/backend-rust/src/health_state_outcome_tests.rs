use super::*;

    use super::*;

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
            capability_id: "health.outcome.write".into(),
            capability_version: "1.0.0".into(),
            resource_type: "health_state_outcome".into(),
            resource_id: "outcome-1".into(),
            action: "write".into(),
            tenant_id: "tenant-1".into(),
            data_domain: "personal_health".into(),
        }
    }

    fn context() -> AuthorizedOutcomeContext {
        OutcomeAuthorizationBoundary::authorize(&AllowBoundary, &request()).unwrap()
    }

    fn outcome() -> Outcome {
        Outcome {
            id: "outcome-1".into(),
            schema_version: SCHEMA_VERSION.into(),
            subject_ref: "person-1".into(),
            intervention_ref: "intervention-1".into(),
            measurement_ref: "measurement-1".into(),
            metric: "defined_metric".into(),
            result: "recorded_result".into(),
            unit: Some("unit".into()),
            observation_window_start: "2026-09-16T00:00:00Z".into(),
            observation_window_end: "2026-09-30T00:00:00Z".into(),
            baseline_ref: Some("observation-1".into()),
            uncertainty: Some("not_assessed".into()),
            status: OutcomeStatus::Observed,
            interpretation_ref: None,
            rationale: "Defined result over a bounded observation window".into(),
            source_ref: "observation:response-1".into(),
            recorded_at: "2026-09-30T00:00:00Z".into(),
            actor_ref: Some("principal-1".into()),
        }
    }

    #[test]
    fn valid_outcome_crosses_validation_boundary() {
        assert!(OutcomeBoundary::validate(&outcome(), &context()).is_ok());
    }

    #[test]
    fn deny_cannot_mint_context() {
        assert_eq!(
            OutcomeAuthorizationBoundary::authorize(&DenyBoundary, &request()),
            Err(AuthorizationDecision::Deny)
        );
    }

    #[test]
    fn delegated_actor_does_not_become_subject() {
        let request = request();
        let context = AuthorizedOutcomeContext::from_authorized_request(&request).unwrap();
        assert_eq!(request.principal_ref, "principal-1");
        assert_eq!(context.subject_ref(), "person-1");
    }

    #[test]
    fn subject_mismatch_is_rejected() {
        let mut candidate = outcome();
        candidate.subject_ref = "person-2".into();
        assert_eq!(OutcomeBoundary::validate(&candidate, &context()), Err(OutcomeError::SubjectMismatch));
    }

    #[test]
    fn missing_measurement_reference_is_rejected() {
        let mut candidate = outcome();
        candidate.measurement_ref.clear();
        assert_eq!(OutcomeBoundary::validate(&candidate, &context()), Err(OutcomeError::InvalidOutcome));
    }

    #[test]
    fn references_reject_control_characters() {
        let mut candidate = outcome();
        candidate.baseline_ref = Some("bad\nref".into());
        assert_eq!(OutcomeBoundary::validate(&candidate, &context()), Err(OutcomeError::InvalidReference));
    }
