use super::{
    AuthorizedHealthStateAccessContext, HealthStateQuery, HealthStateRepository, HealthStateRepositoryError,
    LocalHealthStateRepository, HEALTH_STATE_CONTENT_TYPE,
};
use crate::canonical_authorization::{AuthorizationDecision, AuthorizationRequest, CanonicalAuthorizationBoundary};
use crate::local_health_vault::{LocalHealthVaultCrypto, LocalHealthVaultRecord, VaultRecordMetadata};
use crate::local_health_vault_storage::{
    AuthorizationContext, LocalFileVaultStore, StorageError, VaultAction, VaultAuthorizer, VaultKeyProvider,
};
use std::{
    cell::Cell,
    collections::HashMap,
    fs,
    path::PathBuf,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

const KEY_LEN: usize = 32;

#[derive(Clone)]
struct Keys {
    values: HashMap<String, [u8; KEY_LEN]>,
    calls: Rc<Cell<usize>>,
}
impl VaultKeyProvider for Keys {
    fn key_for(&self, key_ref: &str) -> Result<[u8; KEY_LEN], StorageError> {
        self.calls.set(self.calls.get() + 1);
        self.values.get(key_ref).copied().ok_or(StorageError::KeyResolutionFailed)
    }
}
#[derive(Clone, Copy)]
struct SubjectAuthorizer;
impl VaultAuthorizer for SubjectAuthorizer {
    fn authorize(&self, context: &AuthorizationContext, record_id: &str, action: VaultAction) -> bool {
        match action {
            VaultAction::List => true,
            _ => record_id == "*" || record_id.starts_with(&context.subject_ref),
        }
    }
}
struct AllowBoundary;
impl CanonicalAuthorizationBoundary for AllowBoundary {
    fn authorize(&self, _request: &AuthorizationRequest) -> AuthorizationDecision {
        AuthorizationDecision::Allow
    }
}
fn root() -> PathBuf {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    std::env::temp_dir().join(format!("soma-health-state-repository-test-{nanos}"))
}
fn request() -> AuthorizationRequest {
    AuthorizationRequest {
        principal_ref: "principal-1".into(),
        subject_ref: "person-1".into(),
        capability_id: "health.state.write".into(),
        capability_version: "1.0.0".into(),
        resource_type: "health_state".into(),
        resource_id: "record-1".into(),
        action: "write".into(),
        tenant_id: "tenant-1".into(),
        data_domain: "personal_health".into(),
    }
}
fn record(entity_type: &str) -> LocalHealthVaultRecord {
    LocalHealthVaultCrypto::encrypt_record(
        &[7u8; KEY_LEN],
        VaultRecordMetadata {
            record_id: "person-1-record-1", subject_ref: "person-1", entity_type,
            content_type: HEALTH_STATE_CONTENT_TYPE, key_ref: "vault-key-v1",
            provenance_ref: "provenance-1", created_at: "2026-09-14T00:00:00Z",
            updated_at: "2026-09-14T00:00:00Z",
        },
        br#"{"id":"person-1-record-1","subject_ref":"person-1","entity_type":"observation","schema_version":"1.1.0","status":"active","recorded_time":"2026-09-14T00:00:00Z","concept":"heart_rate","uncertainty":"known","provenance":{"origin":"device","method":"measurement"},"classification":"personal_health"}"#,
    ).unwrap()
}
fn repository(root: &std::path::Path) -> (LocalHealthStateRepository<Keys, SubjectAuthorizer>, Rc<Cell<usize>>) {
    let calls = Rc::new(Cell::new(0));
    let mut values = HashMap::new();
    values.insert("vault-key-v1".into(), [7u8; KEY_LEN]);
    let vault = LocalFileVaultStore::new(
        root,
        Keys {
            values,
            calls: calls.clone(),
        },
        SubjectAuthorizer,
    )
    .unwrap();
    (LocalHealthStateRepository::new(vault), calls)
}

#[test]
fn authorized_context_is_minted_only_from_allow() {
    let context = AllowBoundary.authorize(&request());
    assert_eq!(context, AuthorizationDecision::Allow);
    let access = AuthorizedHealthStateAccessContext::from_authorized_request(&request()).unwrap();
    assert_eq!(access.subject_ref(), "person-1");
    assert_eq!(access.scope(), "tenant-1:personal_health");
}
#[test]
fn repository_accepts_canonical_health_state_entity_types() {
    let root = root();
    let (repository, _) = repository(&root);
    let context = AuthorizedHealthStateAccessContext::from_authorized_request(&request()).unwrap();
    repository.put(record("observation"), &context).unwrap();
    let records = repository.query(&context, &HealthStateQuery::default()).unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].entity_type, "observation");
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn repository_rejects_non_health_state_entity_types() {
    let root = root();
    let (repository, _) = repository(&root);
    let context = AuthorizedHealthStateAccessContext::from_authorized_request(&request()).unwrap();
    assert_eq!(repository.put(record("evidence_claim"), &context), Err(HealthStateRepositoryError::InvalidEntity));
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn timeline_does_not_fabricate_effective_time() {
    let root = root();
    let (repository, _) = repository(&root);
    let context = AuthorizedHealthStateAccessContext::from_authorized_request(&request()).unwrap();
    repository.put(record("observation"), &context).unwrap();
    let timeline = repository.timeline(&context).unwrap();
    assert_eq!(timeline[0].effective_time, None);
    assert_eq!(timeline[0].recorded_at, "2026-09-14T00:00:00Z");
    fs::remove_dir_all(root).unwrap();
}


#[test]
fn longitudinal_observation_to_outcome_persists_through_existing_vault() {
    use crate::health_state_evidence_link::{
        AuthorizedHealthStateEvidenceLinkContext, HealthStateEvidenceLink, HealthStateEvidenceLinkBoundary,
        HealthStateEvidenceRelationship, LinkProvenance, LinkUncertainty,
    };
    use crate::health_state_intervention::{
        AuthorizedInterventionContext, InterventionIntent, InterventionIntentBoundary, InterventionIntentStatus,
    };
    use crate::health_state_measurement::{AuthorizedMeasurementContext, MeasurementBoundary, MeasurementPlan};
    use crate::health_state_outcome::{
        Outcome, OutcomeAuthorizationBoundary, OutcomeBoundary, OutcomeStatus,
    };
    use crate::longitudinal_observation_repository::{
        AuthorizedObservationAccessContext, LongitudinalObservationRepository,
    };

    let root = root();
    let (repository, _) = repository(&root);
    let request = request();
    assert_eq!(AllowBoundary.authorize(&request), AuthorizationDecision::Allow);
    let health_context = AuthorizedHealthStateAccessContext::from_authorized_request(&request).unwrap();

    // Persist the observation through the canonical Health State repository.
    repository.put(record("observation"), &health_context).unwrap();

    // The longitudinal adapter reads the same underlying vault directory and does
    // not introduce a second clinical payload store.
    let mut key_values = HashMap::new();
    key_values.insert("vault-key-v1".to_owned(), [7u8; KEY_LEN]);
    let observation_repository = crate::local_longitudinal_observation_repository::LocalLongitudinalObservationRepository::new(
        LocalFileVaultStore::new(&root, Keys { values: key_values, calls: Rc::new(Cell::new(0)) }, SubjectAuthorizer).unwrap(),
    );
    let observation_context = AuthorizedObservationAccessContext::from_authorized_request(&request).unwrap();
    let observed = observation_repository.get("person-1-record-1", &observation_context).unwrap();
    assert_eq!(observed.subject_ref, "person-1");
    assert_eq!(observed.entity_type, "observation");
    let observation_payload = LocalHealthVaultCrypto::decrypt_record(&[7u8; KEY_LEN], &observed).unwrap();
    let observation_json: serde_json::Value = serde_json::from_slice(&observation_payload).unwrap();
    assert_eq!(observation_json["id"], "person-1-record-1");
    assert_eq!(observation_json["concept"], "heart_rate");
    assert_eq!(observation_repository.put_reference("person-1-record-1", &observation_context), Ok(()));

    // Validate explicit evidence linkage without converting association into causality.
    let link_context = AuthorizedHealthStateEvidenceLinkContext::from_authorized_request(&request).unwrap();
    let link = HealthStateEvidenceLink {
        id: "person-1-link-1".into(),
        schema_version: "1.0.0".into(),
        health_state_ref: "person-1-outcome-1".into(),
        evidence_ref: "person-1-record-1".into(),
        relationship: HealthStateEvidenceRelationship::EvidenceInformsInterpretation,
        provenance: LinkProvenance {
            method: "integration-test".into(),
            created_at: Some("2026-10-13T00:00:00Z".into()),
            actor_ref: Some("principal-1".into()),
        },
        context: None,
        uncertainty: Some(LinkUncertainty::Reported),
    };
    HealthStateEvidenceLinkBoundary::validate(&link, &link_context).unwrap();

    let intervention_context = AuthorizedInterventionContext::from_authorized_request(&request).unwrap();
    let intervention = InterventionIntent {
        id: "person-1-intervention-1".into(),
        schema_version: "1.0.0".into(),
        subject_ref: "person-1".into(),
        intervention_type: "bounded_lifestyle_change".into(),
        description: "Test-only intervention intent".into(),
        intended_change: "Observe a predefined health measure".into(),
        status: InterventionIntentStatus::Proposed,
        rationale: "Integration proof, not a clinical recommendation".into(),
        evidence_refs: vec!["person-1-record-1".into()],
        safety_refs: vec!["safety-review-1".into()],
        causality_refs: vec![],
        planned_start: Some("2026-10-01T00:00:00Z".into()),
        planned_end: Some("2026-10-08T00:00:00Z".into()),
        created_at: "2026-10-01T00:00:00Z".into(),
        actor_ref: Some("principal-1".into()),
    };
    InterventionIntentBoundary::validate(&intervention, &intervention_context).unwrap();

    let measurement_context = AuthorizedMeasurementContext::from_authorized_request(&request).unwrap();
    let measurement = MeasurementPlan {
        id: "person-1-measurement-1".into(),
        schema_version: "1.0.0".into(),
        subject_ref: "person-1".into(),
        intervention_ref: "person-1-intervention-1".into(),
        metric: "heart_rate".into(),
        measurement_method: "existing_observation".into(),
        unit: Some("beats_per_minute".into()),
        observation_window_start: "2026-10-01T00:00:00Z".into(),
        observation_window_end: "2026-10-08T00:00:00Z".into(),
        baseline_ref: Some("person-1-record-1".into()),
        uncertainty_method: Some("preserve_source_uncertainty".into()),
        rationale: "Validate traceability of a bounded measurement plan".into(),
        created_at: "2026-10-01T00:00:00Z".into(),
        actor_ref: Some("principal-1".into()),
    };
    MeasurementBoundary::validate(&measurement, &measurement_context).unwrap();

    let outcome = Outcome {
        id: "person-1-outcome-1".into(),
        schema_version: "1.0.0".into(),
        subject_ref: "person-1".into(),
        intervention_ref: intervention.id.clone(),
        measurement_ref: measurement.id.clone(),
        metric: measurement.metric.clone(),
        result: "observation_retrieved".into(),
        unit: measurement.unit.clone(),
        observation_window_start: measurement.observation_window_start.clone(),
        observation_window_end: measurement.observation_window_end.clone(),
        baseline_ref: Some("person-1-record-1".into()),
        uncertainty: Some("reported; no causal conclusion".into()),
        status: OutcomeStatus::Observed,
        interpretation_ref: None,
        rationale: "Persisted integration-test result; not evidence of efficacy".into(),
        source_ref: link.id.clone(),
        recorded_at: "2026-10-13T00:00:00Z".into(),
        actor_ref: Some("principal-1".into()),
    };
    let outcome_context = OutcomeAuthorizationBoundary::authorize(&AllowBoundary, &request()).unwrap();
    OutcomeBoundary::validate(&outcome, &outcome_context).unwrap();

    // Persist the validated outcome using the existing Health State repository.
    // Keep the encrypted payload's references explicit and independently verifiable.
    let outcome_payload = serde_json::json!({
        "id": outcome.id,
        "schema_version": outcome.schema_version,
        "subject_ref": outcome.subject_ref,
        "intervention_ref": outcome.intervention_ref,
        "measurement_ref": outcome.measurement_ref,
        "metric": outcome.metric,
        "result": outcome.result,
        "unit": outcome.unit,
        "observation_window_start": outcome.observation_window_start,
        "observation_window_end": outcome.observation_window_end,
        "baseline_ref": outcome.baseline_ref,
        "uncertainty": outcome.uncertainty,
        "status": "observed",
        "rationale": outcome.rationale,
        "source_ref": outcome.source_ref,
        "recorded_at": outcome.recorded_at,
        "actor_ref": outcome.actor_ref
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
    ).unwrap();
    repository.put(outcome_record, &health_context).unwrap();

    let persisted = repository.get("person-1-outcome-1", &health_context).unwrap();
    repository.verify("person-1-outcome-1", &health_context).unwrap();
    let persisted_bytes = LocalHealthVaultCrypto::decrypt_record(&[7u8; KEY_LEN], &persisted).unwrap();
    let persisted_json: serde_json::Value = serde_json::from_slice(&persisted_bytes).unwrap();
    assert_eq!(persisted_json["subject_ref"], "person-1");
    assert_eq!(persisted_json["intervention_ref"], "person-1-intervention-1");
    assert_eq!(persisted_json["measurement_ref"], "person-1-measurement-1");
    assert_eq!(persisted_json["baseline_ref"], "person-1-record-1");
    assert_eq!(persisted_json["source_ref"], "person-1-link-1");

    let timeline = repository.timeline(&health_context).unwrap();
    assert!(timeline.iter().any(|entry| entry.record_id == "person-1-record-1"));
    assert!(timeline.iter().any(|entry| entry.record_id == "person-1-outcome-1"));
    assert!(timeline.iter().all(|entry| entry.subject_ref == "person-1"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn subject_mismatch_fails_before_key_resolution_or_persistence() {
    let root = root();
    let (repository, key_calls) = repository(&root);
    let mut other_subject_request = request();
    other_subject_request.subject_ref = "person-2".into();
    let context = AuthorizedHealthStateAccessContext::from_authorized_request(&other_subject_request).unwrap();

    assert_eq!(
        repository.put(record("observation"), &context),
        Err(HealthStateRepositoryError::AuthorizationDenied)
    );
    assert_eq!(key_calls.get(), 0, "authorization failure must occur before key resolution");
    assert!(repository.query(&context, &HealthStateQuery::default()).unwrap().is_empty());
    fs::remove_dir_all(root).unwrap();
}
