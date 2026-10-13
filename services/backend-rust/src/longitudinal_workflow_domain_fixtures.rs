use crate::health_state_evidence_link::{
    HealthStateEvidenceLink, HealthStateEvidenceRelationship, LinkProvenance, LinkUncertainty,
};
use crate::health_state_intervention::{InterventionIntent, InterventionIntentStatus};
use crate::health_state_measurement::MeasurementPlan;
use crate::health_state_outcome::{Outcome, OutcomeStatus};

pub(super) fn link() -> HealthStateEvidenceLink {
    HealthStateEvidenceLink {
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
    }
}

pub(super) fn intervention() -> InterventionIntent {
    InterventionIntent {
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
    }
}

pub(super) fn measurement() -> MeasurementPlan {
    MeasurementPlan {
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
    }
}

pub(super) fn outcome(
    intervention: &InterventionIntent,
    measurement: &MeasurementPlan,
    link: &HealthStateEvidenceLink,
) -> Outcome {
    Outcome {
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
    }
}
