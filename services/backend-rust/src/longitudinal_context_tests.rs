use super::*;


use super::*;
use crate::health_state_evidence_link::{HealthStateEvidenceRelationship, LinkProvenance, LinkUncertainty};

fn request() -> AuthorizationRequest {
    AuthorizationRequest {
        principal_ref: "principal-1".into(),
        subject_ref: "person-1".into(),
        capability_id: "health.context.read".into(),
        capability_version: "1.0.0".into(),
        resource_type: "longitudinal_context".into(),
        resource_id: "person-1".into(),
        action: "read".into(),
        tenant_id: "tenant-1".into(),
        data_domain: "personal_health".into(),
    }
}

fn context() -> AuthorizedLongitudinalContextAccessContext {
    AuthorizedLongitudinalContextAccessContext::from_authorized_request(&request()).unwrap()
}

fn health_state_entry(id: &str, subject: &str) -> HealthStateTimelineEntry {
    HealthStateTimelineEntry {
        record_id: id.into(),
        subject_ref: subject.into(),
        effective_time: Some("2026-09-01T00:00:00Z".into()),
        recorded_at: "2026-09-01T01:00:00Z".into(),
    }
}

fn observation_entry(id: &str, subject: &str) -> ObservationTimelineEntry {
    ObservationTimelineEntry {
        observation_id: id.into(),
        subject_ref: subject.into(),
        observed_at: None,
        recorded_at: "2026-09-02T00:00:00Z".into(),
    }
}

fn link() -> HealthStateEvidenceLink {
    HealthStateEvidenceLink {
        id: "link-1".into(),
        schema_version: "1.0.0".into(),
        health_state_ref: "hs-1".into(),
        evidence_ref: "claim-1".into(),
        relationship: HealthStateEvidenceRelationship::EvidenceInformsHypothesis,
        provenance: LinkProvenance {
            method: "human-curated".into(),
            created_at: Some("2026-09-02T00:00:00Z".into()),
            actor_ref: Some("principal-1".into()),
        },
        context: None,
        uncertainty: Some(LinkUncertainty::Reported),
    }
}

#[test]
fn assembly_is_read_only_and_preserves_evidence_references() {
    let result = LongitudinalContextAssembly::assemble(
        &context(),
        vec![health_state_entry("hs-1", "person-1")],
        vec![observation_entry("obs-1", "person-1")],
        vec![link()],
    )
    .unwrap();
    assert_eq!(result.subject_ref, "person-1");
    assert_eq!(result.health_state_timeline.len(), 1);
    assert_eq!(result.observation_timeline.len(), 1);
    assert_eq!(result.evidence_links[0].evidence_ref, "claim-1");
}

#[test]
fn cross_subject_projection_is_rejected() {
    assert_eq!(
        LongitudinalContextAssembly::assemble(
            &context(),
            vec![health_state_entry("hs-1", "person-2")],
            vec![],
            vec![],
        ),
        Err(LongitudinalContextError::SubjectMismatch)
    );
}

#[test]
fn malformed_evidence_reference_is_rejected() {
    let mut candidate = link();
    candidate.evidence_ref.clear();
    assert_eq!(
        LongitudinalContextAssembly::assemble(&context(), vec![], vec![], vec![candidate]),
        Err(LongitudinalContextError::InvalidEvidenceLink)
    );
}

#[test]
fn unknown_observation_time_remains_unknown() {
    let result = LongitudinalContextAssembly::assemble(
        &context(),
        vec![],
        vec![observation_entry("obs-1", "person-1")],
        vec![],
    )
    .unwrap();
    assert_eq!(result.observation_timeline[0].observed_at, None);
}
