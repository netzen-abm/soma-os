use super::contract::{AuthorizedHealthStateEvidenceLinkContext, HealthStateEvidenceLink, HealthStateEvidenceLinkError, HealthStateEvidenceRelationship};

pub struct HealthStateEvidenceLinkBoundary;

impl HealthStateEvidenceLinkBoundary {
    pub fn validate(
        link: &HealthStateEvidenceLink,
        context: &AuthorizedHealthStateEvidenceLinkContext,
    ) -> Result<(), HealthStateEvidenceLinkError> {
        if link.schema_version != SCHEMA_VERSION
            || link.id.trim().is_empty()
            || link.health_state_ref.trim().is_empty()
            || link.evidence_ref.trim().is_empty()
            || link.provenance.method.trim().is_empty()
            || link.provenance.created_at.as_deref().is_some_and(|value| value.trim().is_empty())
            || link.provenance.actor_ref.as_deref().is_some_and(|value| value.trim().is_empty())
        {
            return Err(HealthStateEvidenceLinkError::InvalidLink);
        }

        if context.subject_ref().trim().is_empty() {
            return Err(HealthStateEvidenceLinkError::InvalidAuthorizationContext);
        }

        Ok(())
    }

    /// Validates a personal-response linkage without interpreting it as causal.
    pub fn validate_personal_response(
        link: &HealthStateEvidenceLink,
        context: &AuthorizedHealthStateEvidenceLinkContext,
    ) -> Result<(), HealthStateEvidenceLinkError> {
        Self::validate(link, context)?;
        if link.relationship != HealthStateEvidenceRelationship::PersonalResponseObservedAfterIntervention {
            return Err(HealthStateEvidenceLinkError::InvalidLink);
        }
        Ok(())
    }
}