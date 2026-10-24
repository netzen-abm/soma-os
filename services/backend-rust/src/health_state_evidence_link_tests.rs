use super::contract::SCHEMA_VERSION;
use super::*;
use crate::canonical_authorization::AuthorizationRequest;

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
            created_at: None,
            actor_ref: None,
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
    assert!(value.get("context").is_none());
    assert!(value["provenance"].get("created_at").is_none());
    assert!(value["provenance"].get("actor_ref").is_none());

    let round_trip: HealthStateEvidenceLink = serde_json::from_value(value).unwrap();
    assert_eq!(round_trip, link);
}

#[test]
fn shared_json_fixture_round_trips_through_serde() {
    let fixture = include_str!("../../../tests/fixtures/health-state-evidence-link-v1.json");
    let decoded: HealthStateEvidenceLink = serde_json::from_str(fixture).unwrap();
    assert_eq!(decoded, link(HealthStateEvidenceRelationship::EvidenceInformsHypothesis));
    let encoded = serde_json::to_value(decoded).unwrap();
    let fixture_value: serde_json::Value = serde_json::from_str(fixture).unwrap();
    assert_eq!(encoded, fixture_value);
}

#[test]
fn serde_rejects_unknown_root_and_provenance_fields() {
    let fixture = include_str!("../../../tests/fixtures/health-state-evidence-link-v1.json");
    let mut value: serde_json::Value = serde_json::from_str(fixture).unwrap();
    value["unexpected"] = serde_json::Value::Bool(true);
    assert!(serde_json::from_value::<HealthStateEvidenceLink>(value).is_err());

    let mut value: serde_json::Value = serde_json::from_str(fixture).unwrap();
    value["provenance"]["unexpected"] = serde_json::Value::Bool(true);
    assert!(serde_json::from_value::<HealthStateEvidenceLink>(value).is_err());
}

#[test]
fn non_object_context_is_rejected() {
    let mut candidate = link(HealthStateEvidenceRelationship::EvidenceInformsHypothesis);
    candidate.context = Some(serde_json::json!("not-an-object"));
    assert_eq!(
        HealthStateEvidenceLinkBoundary::validate(&candidate, &context()),
        Err(HealthStateEvidenceLinkError::InvalidLink)
    );
}

#[test]
fn malformed_provenance_timestamp_is_rejected() {
    let mut candidate = link(HealthStateEvidenceRelationship::EvidenceInformsHypothesis);
    candidate.provenance.created_at = Some("not-a-timestamp".into());
    assert_eq!(
        HealthStateEvidenceLinkBoundary::validate(&candidate, &context()),
        Err(HealthStateEvidenceLinkError::InvalidLink)
    );
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
