use super::*;
use crate::health_state_evidence_link::{AuthorizedHealthStateEvidenceLinkContext, HealthStateEvidenceLinkBoundary};
use crate::health_state_intervention::{AuthorizedInterventionContext, InterventionIntentBoundary};
use crate::health_state_measurement::{AuthorizedMeasurementContext, MeasurementBoundary};
use crate::health_state_outcome::{OutcomeAuthorizationBoundary, OutcomeBoundary};
use crate::longitudinal_observation_repository::{
    AuthorizedObservationAccessContext, LongitudinalObservationRepository,
};

#[path = "longitudinal_workflow_domain_fixtures.rs"]
mod domain_fixtures;
use domain_fixtures::{intervention, link, measurement, outcome};

#[test]
fn longitudinal_contracts_and_observation_outcome_persistence_are_verified() {
    let root = root();
    let (repository, _) = repository(&root);
    let write_request = request();
    assert_eq!(AllowBoundary.authorize(&write_request), AuthorizationDecision::Allow);
    let health_context = AuthorizedHealthStateAccessContext::from_authorized_request(&write_request).unwrap();
    repository.put(record("observation"), &health_context).unwrap();

    let mut key_values = HashMap::new();
    key_values.insert("vault-key-v1".to_owned(), [7u8; KEY_LEN]);
    let observation_repository =
        crate::local_longitudinal_observation_repository::LocalLongitudinalObservationRepository::new(
            LocalFileVaultStore::new(
                &root,
                Keys {
                    values: key_values,
                    calls: Rc::new(Cell::new(0)),
                },
                SubjectAuthorizer,
            )
            .unwrap(),
        );
    let mut observation_request = write_request.clone();
    observation_request.capability_id = "health.vault.read".into();
    observation_request.resource_type = "health_observation".into();
    observation_request.resource_id = "person-1-record-1".into();
    observation_request.action = "read".into();
    assert_eq!(AllowBoundary.authorize(&observation_request), AuthorizationDecision::Allow);
    let observation_context =
        AuthorizedObservationAccessContext::from_authorized_request(&observation_request).unwrap();
    let observed = observation_repository.get("person-1-record-1", &observation_context).unwrap();
    assert_eq!(observed.subject_ref, "person-1");
    assert_eq!(observed.entity_type, "observation");
    let bytes = LocalHealthVaultCrypto::decrypt_record(&[7u8; KEY_LEN], &observed).unwrap();
    let payload: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(payload["id"], "person-1-record-1");
    assert_eq!(payload["concept"], "heart_rate");
    assert_eq!(observation_repository.put_reference("person-1-record-1", &observation_context), Ok(()));

    let mut link_request = write_request.clone();
    link_request.capability_id = "health.evidence.link".into();
    link_request.resource_type = "health_state_evidence_link".into();
    link_request.resource_id = "person-1-link-1".into();
    assert_eq!(AllowBoundary.authorize(&link_request), AuthorizationDecision::Allow);
    let link_context = AuthorizedHealthStateEvidenceLinkContext::from_authorized_request(&link_request).unwrap();
    let evidence_link = link();
    // This repository persists observations and outcomes today; evidence-link, intervention,
    // and measurement are contract-validated here because no dedicated persistence boundary exists.
    HealthStateEvidenceLinkBoundary::validate(&evidence_link, &link_context).unwrap();

    let mut intervention_request = write_request.clone();
    intervention_request.capability_id = "health.intervention.write".into();
    intervention_request.resource_type = "health_state_intervention".into();
    intervention_request.resource_id = "person-1-intervention-1".into();
    assert_eq!(AllowBoundary.authorize(&intervention_request), AuthorizationDecision::Allow);
    let intervention_context = AuthorizedInterventionContext::from_authorized_request(&intervention_request).unwrap();
    let intervention = intervention();
    InterventionIntentBoundary::validate(&intervention, &intervention_context).unwrap();

    let mut measurement_request = write_request.clone();
    measurement_request.capability_id = "health.measurement.write".into();
    measurement_request.resource_type = "health_state_measurement".into();
    measurement_request.resource_id = "person-1-measurement-1".into();
    assert_eq!(AllowBoundary.authorize(&measurement_request), AuthorizationDecision::Allow);
    let measurement_context = AuthorizedMeasurementContext::from_authorized_request(&measurement_request).unwrap();
    let measurement = measurement();
    MeasurementBoundary::validate(&measurement, &measurement_context).unwrap();

    let mut outcome_request = write_request.clone();
    outcome_request.capability_id = "health.outcome.write".into();
    outcome_request.resource_type = "health_state_outcome".into();
    outcome_request.resource_id = "person-1-outcome-1".into();
    assert_eq!(AllowBoundary.authorize(&outcome_request), AuthorizationDecision::Allow);
    let outcome_context = OutcomeAuthorizationBoundary::authorize(&AllowBoundary, &outcome_request).unwrap();
    let outcome = outcome(&intervention, &measurement, &evidence_link);
    OutcomeBoundary::validate(&outcome, &outcome_context).unwrap();

    let outcome_payload = serde_json::json!({
        "id": outcome.id, "schema_version": outcome.schema_version, "subject_ref": outcome.subject_ref,
        "intervention_ref": outcome.intervention_ref, "measurement_ref": outcome.measurement_ref,
        "metric": outcome.metric, "result": outcome.result, "unit": outcome.unit,
        "observation_window_start": outcome.observation_window_start,
        "observation_window_end": outcome.observation_window_end, "baseline_ref": outcome.baseline_ref,
        "uncertainty": outcome.uncertainty, "status": "observed", "rationale": outcome.rationale,
        "source_ref": outcome.source_ref, "recorded_at": outcome.recorded_at, "actor_ref": outcome.actor_ref
    });
    let outcome_bytes = serde_json::to_vec(&outcome_payload).unwrap();
    let outcome_record = LocalHealthVaultCrypto::encrypt_record(
        &[7u8; KEY_LEN],
        VaultRecordMetadata {
            record_id: "person-1-outcome-1",
            subject_ref: "person-1",
            entity_type: "outcome",
            content_type: HEALTH_STATE_CONTENT_TYPE,
            key_ref: "vault-key-v1",
            provenance_ref: "person-1-link-1",
            created_at: "2026-10-13T00:00:00Z",
            updated_at: "2026-10-13T00:00:00Z",
        },
        &outcome_bytes,
    )
    .unwrap();
    let persistence_context = AuthorizedHealthStateAccessContext::from_authorized_request(&outcome_request).unwrap();
    repository.put(outcome_record, &persistence_context).unwrap();
    let persisted = repository.get("person-1-outcome-1", &persistence_context).unwrap();
    repository.verify("person-1-outcome-1", &persistence_context).unwrap();
    let persisted_bytes = LocalHealthVaultCrypto::decrypt_record(&[7u8; KEY_LEN], &persisted).unwrap();
    let persisted_json: serde_json::Value = serde_json::from_slice(&persisted_bytes).unwrap();
    assert_eq!(persisted_json["subject_ref"], "person-1");
    assert_eq!(persisted_json["intervention_ref"], "person-1-intervention-1");
    assert_eq!(persisted_json["measurement_ref"], "person-1-measurement-1");
    assert_eq!(persisted_json["baseline_ref"], "person-1-record-1");
    assert_eq!(persisted_json["source_ref"], "person-1-link-1");

    let mut timeline_request = write_request;
    timeline_request.capability_id = "health.timeline.read".into();
    timeline_request.resource_type = "health_state_timeline".into();
    timeline_request.resource_id = "person-1".into();
    timeline_request.action = "read".into();
    assert_eq!(AllowBoundary.authorize(&timeline_request), AuthorizationDecision::Allow);
    let timeline_context = AuthorizedHealthStateAccessContext::from_authorized_request(&timeline_request).unwrap();
    let timeline = repository.timeline(&timeline_context).unwrap();
    assert!(timeline.iter().any(|entry| entry.record_id == "person-1-record-1"));
    assert!(timeline.iter().any(|entry| entry.record_id == "person-1-outcome-1"));
    assert!(timeline.iter().all(|entry| entry.subject_ref == "person-1"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn subject_mismatch_fails_before_key_resolution_or_persistence() {
    let root = root();
    let (repository, key_calls) = repository(&root);
    let mut request = request();
    request.subject_ref = "person-2".into();
    let context = AuthorizedHealthStateAccessContext::from_authorized_request(&request).unwrap();
    assert_eq!(repository.put(record("observation"), &context), Err(HealthStateRepositoryError::AuthorizationDenied));
    assert_eq!(key_calls.get(), 0, "authorization failure must occur before key resolution");
    assert!(repository.query(&context, &HealthStateQuery::default()).unwrap().is_empty());
    fs::remove_dir_all(root).unwrap();
}
