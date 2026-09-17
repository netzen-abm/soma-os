//! Read-only assembly of governed longitudinal context from canonical SOMA projections.
//!
//! This module does not own health data, evidence, relationships, or policy. It
//! assembles already-authorized projections and explicit cross-domain references
//! into a deterministic context view. It never dereferences an Evidence Graph
//! record, performs clinical or causal inference, or grants agent authority.

use crate::canonical_authorization::AuthorizationRequest;
use crate::health_state_evidence_link::HealthStateEvidenceLink;
use crate::health_state_repository::HealthStateTimelineEntry;
use crate::longitudinal_observation_repository::ObservationTimelineEntry;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedLongitudinalContextAccessContext {
    subject_ref: String,
    scope: String,
    capability_id: String,
    resource_type: String,
    resource_id: String,
    action: String,
}

impl AuthorizedLongitudinalContextAccessContext {
    pub(crate) fn from_authorized_request(request: &AuthorizationRequest) -> Result<Self, LongitudinalContextError> {
        let values = [
            &request.principal_ref,
            &request.subject_ref,
            &request.capability_id,
            &request.resource_type,
            &request.resource_id,
            &request.action,
            &request.tenant_id,
            &request.data_domain,
        ];
        if values.iter().any(|value| value.trim().is_empty() || value.chars().any(char::is_control)) {
            return Err(LongitudinalContextError::AuthorizationDenied);
        }
        Ok(Self {
            subject_ref: request.subject_ref.clone(),
            scope: format!("{}:{}", request.tenant_id, request.data_domain),
            capability_id: request.capability_id.clone(),
            resource_type: request.resource_type.clone(),
            resource_id: request.resource_id.clone(),
            action: request.action.clone(),
        })
    }

    pub fn subject_ref(&self) -> &str {
        &self.subject_ref
    }
    pub fn scope(&self) -> &str {
        &self.scope
    }
    pub fn capability_id(&self) -> &str {
        &self.capability_id
    }
    pub fn resource_type(&self) -> &str {
        &self.resource_type
    }
    pub fn resource_id(&self) -> &str {
        &self.resource_id
    }
    pub fn action(&self) -> &str {
        &self.action
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernedLongitudinalContext {
    pub subject_ref: String,
    pub health_state_timeline: Vec<HealthStateTimelineEntry>,
    pub observation_timeline: Vec<ObservationTimelineEntry>,
    pub evidence_links: Vec<HealthStateEvidenceLink>,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum LongitudinalContextError {
    #[error("authorization denied")]
    AuthorizationDenied,
    #[error("context projection contains data for another subject")]
    SubjectMismatch,
    #[error("context projection contains an invalid evidence link")]
    InvalidEvidenceLink,
}

pub struct LongitudinalContextAssembly;

impl LongitudinalContextAssembly {
    pub fn assemble(
        context: &AuthorizedLongitudinalContextAccessContext,
        mut health_state_timeline: Vec<HealthStateTimelineEntry>,
        mut observation_timeline: Vec<ObservationTimelineEntry>,
        evidence_links: Vec<HealthStateEvidenceLink>,
    ) -> Result<GovernedLongitudinalContext, LongitudinalContextError> {
        let subject = context.subject_ref();
        if health_state_timeline.iter().any(|entry| entry.subject_ref != subject)
            || observation_timeline.iter().any(|entry| entry.subject_ref != subject)
        {
            return Err(LongitudinalContextError::SubjectMismatch);
        }
        if evidence_links.iter().any(|link| {
            link.health_state_ref.trim().is_empty() || link.evidence_ref.trim().is_empty() || link.id.trim().is_empty()
        }) {
            return Err(LongitudinalContextError::InvalidEvidenceLink);
        }
        health_state_timeline.sort_by(|left, right| {
            left.effective_time
                .cmp(&right.effective_time)
                .then_with(|| left.recorded_at.cmp(&right.recorded_at))
                .then_with(|| left.record_id.cmp(&right.record_id))
        });
        observation_timeline.sort_by(|left, right| {
            match (&left.observed_at, &right.observed_at) {
                (Some(a), Some(b)) => a.cmp(b),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => left.recorded_at.cmp(&right.recorded_at),
            }
            .then_with(|| left.recorded_at.cmp(&right.recorded_at))
            .then_with(|| left.observation_id.cmp(&right.observation_id))
        });
        Ok(GovernedLongitudinalContext {
            subject_ref: subject.to_owned(),
            health_state_timeline,
            observation_timeline,
            evidence_links,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::health_state_evidence_link::{HealthStateEvidenceRelationship, LinkProvenance, LinkUncertainty};

    fn request() -> AuthorizationRequest {
        AuthorizationRequest {
            principal_ref: "principal-1".into(),
            subject_ref: "person-1".into(),
            capability_id: "health.context.read".into(),
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
                vec![]
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
}
