use super::*;


use super::*;

fn request() -> AuthorizationRequest {
    AuthorizationRequest {
        principal_ref: "principal-1".into(),
        subject_ref: "person-1".into(),
        capability_id: "health.measurement.write".into(),
        capability_version: "1.0.0".into(),
        resource_type: "health_state_measurement".into(),
        resource_id: "measurement-1".into(),
        action: "write".into(),
        tenant_id: "tenant-1".into(),
        data_domain: "personal_health".into(),
    }
}

fn context() -> AuthorizedMeasurementContext {
    AuthorizedMeasurementContext::from_authorized_request(&request()).unwrap()
}

fn plan() -> MeasurementPlan {
    MeasurementPlan {
        id: "measurement-1".into(),
        schema_version: SCHEMA_VERSION.into(),
        subject_ref: "person-1".into(),
        intervention_ref: "intervention-1".into(),
        metric: "sleep_duration".into(),
        measurement_method: "daily_user_report".into(),
        unit: Some("hours".into()),
        observation_window_start: "2026-09-20T00:00:00Z".into(),
        observation_window_end: "2026-10-04T00:00:00Z".into(),
        baseline_ref: Some("observation-baseline-1".into()),
        uncertainty_method: Some("reported_uncertainty".into()),
        rationale: "Measure the predefined response metric during the intervention window".into(),
        created_at: "2026-09-16T00:00:00Z".into(),
        actor_ref: Some("principal-1".into()),
    }
}

#[test]
fn valid_measurement_plan_crosses_boundary() {
    assert!(MeasurementBoundary::validate(&plan(), &context()).is_ok());
}

#[test]
fn delegated_actor_does_not_become_subject() {
    let request = request();
    let context = AuthorizedMeasurementContext::from_authorized_request(&request).unwrap();
    assert_eq!(request.principal_ref, "principal-1");
    assert_eq!(context.subject_ref(), "person-1");
}

#[test]
fn subject_mismatch_is_rejected() {
    let mut candidate = plan();
    candidate.subject_ref = "person-2".into();
    assert_eq!(MeasurementBoundary::validate(&candidate, &context()), Err(MeasurementError::SubjectMismatch));
}

#[test]
fn missing_intervention_reference_is_rejected() {
    let mut candidate = plan();
    candidate.intervention_ref.clear();
    assert_eq!(
        MeasurementBoundary::validate(&candidate, &context()),
        Err(MeasurementError::InvalidMeasurementPlan)
    );
}

#[test]
fn measurement_fields_reject_control_characters() {
    let mut candidate = plan();
    candidate.metric = "sleep\nduration".into();
    assert_eq!(
        MeasurementBoundary::validate(&candidate, &context()),
        Err(MeasurementError::InvalidMeasurementPlan)
    );
}
