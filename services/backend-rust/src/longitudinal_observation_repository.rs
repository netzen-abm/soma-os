use std::cmp::Ordering;
use thiserror::Error;

/// Provider-neutral authorization context. Implementations must enforce subject scope
/// before accessing protected observation payloads or resolving vault keys.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationAuthorizationContext {
    pub subject_ref: String,
    pub scope: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationQuery {
    pub concept: Option<String>,
    pub status: Option<String>,
    pub classification: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationTimelineEntry {
    pub observation_id: String,
    pub subject_ref: String,
    /// Health-relevant time. None means the observation time is unknown.
    pub observed_at: Option<String>,
    pub recorded_at: String,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ObservationRepositoryError {
    #[error("authorization denied")]
    AuthorizationDenied,
    #[error("observation not found")]
    NotFound,
    #[error("observation is tombstoned")]
    Tombstoned,
    #[error("repository integrity failure")]
    IntegrityFailure,
    #[error("repository storage failure")]
    StorageFailure,
    #[error("invalid observation")]
    InvalidObservation,
}

/// Canonical longitudinal observation repository boundary.
///
/// This contract deliberately does not define a clinical payload model. Canonical
/// Health State observations remain the domain representation; a provider stores or
/// indexes references to those entities according to SOMA's governed storage boundary.
/// Implementations must preserve provenance and uncertainty and must not infer causation.
pub trait LongitudinalObservationRepository {
    type Observation;

    fn put_reference(
        &self,
        observation_id: &str,
        context: &ObservationAuthorizationContext,
    ) -> Result<(), ObservationRepositoryError>;

    fn get(
        &self,
        observation_id: &str,
        context: &ObservationAuthorizationContext,
    ) -> Result<Self::Observation, ObservationRepositoryError>;

    fn query(
        &self,
        context: &ObservationAuthorizationContext,
        query: &ObservationQuery,
    ) -> Result<Vec<Self::Observation>, ObservationRepositoryError>;

    fn timeline(
        &self,
        context: &ObservationAuthorizationContext,
    ) -> Result<Vec<ObservationTimelineEntry>, ObservationRepositoryError>;

    fn tombstone(
        &self,
        observation_id: &str,
        context: &ObservationAuthorizationContext,
    ) -> Result<(), ObservationRepositoryError>;

    fn verify(
        &self,
        observation_id: &str,
        context: &ObservationAuthorizationContext,
    ) -> Result<(), ObservationRepositoryError>;
}

/// Shared deterministic timeline ordering helper for all repository providers.
///
/// Known observation time is authoritative for chronology. Recorded time is only a
/// deterministic fallback/tie-breaker; it is never substituted into the observation
/// itself when observation time is unknown. Stable ID closes the ordering relation.
pub fn sort_timeline(entries: &mut [ObservationTimelineEntry]) {
    entries.sort_by(|left, right| {
        match (&left.observed_at, &right.observed_at) {
            (Some(left_time), Some(right_time)) => match left_time.cmp(right_time) {
                Ordering::Equal => left.recorded_at.cmp(&right.recorded_at),
                ordering => ordering,
            },
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => left.recorded_at.cmp(&right.recorded_at),
        }
        .then_with(|| left.observation_id.cmp(&right.observation_id))
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context() -> ObservationAuthorizationContext {
        ObservationAuthorizationContext {
            subject_ref: "person-1".into(),
            scope: "self".into(),
        }
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
    fn authorization_context_is_subject_scoped() {
        let value = context();
        assert_eq!(value.subject_ref, "person-1");
        assert_eq!(value.scope, "self");
    }

    #[test]
    fn contract_does_not_expose_causal_relationships() {
        let query = ObservationQuery {
            concept: Some("heart_rate".into()),
            status: Some("active".into()),
            classification: Some("personal_health".into()),
        };
        assert_eq!(query.concept.as_deref(), Some("heart_rate"));
        // Causal semantics belong to evidence assessment, not the observation repository.
    }
}
