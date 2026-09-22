use super::*;


use super::*;

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
fn authorized_context_binds_request_scope() {
    let context = AuthorizedObservationAccessContext::from_authorized_request(&request()).unwrap();
    assert_eq!(context.subject_ref(), "person-1");
    assert_eq!(context.scope(), "tenant-1:personal_health");
    assert_eq!(context.capability_id(), "health.read");
    assert_eq!(context.resource_type(), "health_record");
    assert_eq!(context.resource_id(), "record-1");
    assert_eq!(context.action(), "read");
}

#[test]
fn malformed_authorized_request_cannot_create_context() {
    let mut request = request();
    request.action = "".into();
    assert_eq!(
        AuthorizedObservationAccessContext::from_authorized_request(&request),
        Err(ObservationRepositoryError::AuthorizationDenied)
    );
}

#[test]
fn timeline_prefers_observed_time_and_keeps_unknown_time_unknown() {
    let mut entries = vec![
        ObservationTimelineEntry {
            observation_id: "b".into(),
            subject_ref: "person-1".into(),
            observed_at: None,
            recorded_at: "2026-09-01T00:00:00Z".into(),
        },
        ObservationTimelineEntry {
            observation_id: "a".into(),
            subject_ref: "person-1".into(),
            observed_at: Some("2026-08-01T00:00:00Z".into()),
            recorded_at: "2026-09-10T00:00:00Z".into(),
        },
        ObservationTimelineEntry {
            observation_id: "c".into(),
            subject_ref: "person-1".into(),
            observed_at: Some("2026-08-01T00:00:00Z".into()),
            recorded_at: "2026-09-09T00:00:00Z".into(),
        },
    ];

    sort_timeline(&mut entries);
    assert_eq!(entries[0].observation_id, "c");
    assert_eq!(entries[1].observation_id, "a");
    assert_eq!(entries[2].observation_id, "b");
    assert_eq!(entries[2].observed_at, None);
}

#[test]
fn equal_times_use_stable_id_as_final_tie_breaker() {
    let mut entries = vec![
        ObservationTimelineEntry {
            observation_id: "z".into(),
            subject_ref: "person-1".into(),
            observed_at: Some("2026-09-01T00:00:00Z".into()),
            recorded_at: "2026-09-01T00:00:00Z".into(),
        },
        ObservationTimelineEntry {
            observation_id: "a".into(),
            subject_ref: "person-1".into(),
            observed_at: Some("2026-09-01T00:00:00Z".into()),
            recorded_at: "2026-09-01T00:00:00Z".into(),
        },
    ];
    sort_timeline(&mut entries);
    assert_eq!(entries[0].observation_id, "a");
    assert_eq!(entries[1].observation_id, "z");
}

#[test]
fn observation_contract_does_not_expose_causal_relationships() {
    let query = ObservationQuery {
        concept: Some("heart_rate".into()),
        status: Some("active".into()),
        classification: Some("personal_health".into()),
    };
    assert_eq!(query.concept.as_deref(), Some("heart_rate"));
    // Causal semantics belong to evidence assessment, not the observation repository.
}
