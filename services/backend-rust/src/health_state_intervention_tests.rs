use super::*;


use super::*;

fn request() -> AuthorizationRequest {
    AuthorizationRequest {
        principal_ref: "principal-1".into(),
        subject_ref: "person-1".into(),
        capability_id: "health.intervention.write".into(),
        capability_version: "1.0.0".into(),
        resource_type: "health_state_intervention".into(),
        resource_id: "intervention-1".into(),
        action: "write".into(),
        tenant_id: "tenant-1".into(),
        data_domain: "personal_health".into(),
    }
}

fn context() -> AuthorizedInterventionContext {
    AuthorizedInterventionContext::from_authorized_request(&request()).unwrap()
}

fn intent() -> InterventionIntent {
    InterventionIntent {
        id: "intervention-1".into(),
        schema_version: SCHEMA_VERSION.into(),
        subject_ref: "person-1".into(),
        intervention_type: "lifestyle_change".into(),
        description: "Bounded change for a defined observation window".into(),
        intended_change: "Change one specified behavior".into(),
        status: InterventionIntentStatus::Proposed,
        rationale: "Testable hypothesis with explicit evidence and safety references".into(),
        evidence_refs: vec!["pubmed:123".into()],
        safety_refs: vec!["safety:123".into()],
        causality_refs: vec!["causality:123".into()],
        planned_start: Some("2026-09-20T00:00:00Z".into()),
        planned_end: Some("2026-10-04T00:00:00Z".into()),
        created_at: "2026-09-15T00:00:00Z".into(),
        actor_ref: Some("principal-1".into()),
    }
}

#[test]
fn valid_intent_crosses_validation_boundary() {
    assert!(InterventionIntentBoundary::validate(&intent(), &context()).is_ok());
}

#[test]
fn delegated_actor_does_not_become_subject() {
    let request = request();
    let context = AuthorizedInterventionContext::from_authorized_request(&request).unwrap();
    assert_eq!(request.principal_ref, "principal-1");
    assert_eq!(context.subject_ref(), "person-1");
}

#[test]
fn subject_mismatch_is_rejected() {
    let mut candidate = intent();
    candidate.subject_ref = "person-2".into();
    assert_eq!(
        InterventionIntentBoundary::validate(&candidate, &context()),
        Err(InterventionIntentError::SubjectMismatch)
    );
}

#[test]
fn empty_intended_change_is_rejected() {
    let mut candidate = intent();
    candidate.intended_change.clear();
    assert_eq!(
        InterventionIntentBoundary::validate(&candidate, &context()),
        Err(InterventionIntentError::InvalidIntent)
    );
}

#[test]
fn references_reject_control_characters() {
    let mut candidate = intent();
    candidate.evidence_refs.push("bad\nref".into());
    assert_eq!(
        InterventionIntentBoundary::validate(&candidate, &context()),
        Err(InterventionIntentError::InvalidReference)
    );
}
