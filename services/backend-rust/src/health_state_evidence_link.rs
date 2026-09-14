//! Bounded Health State ↔ Evidence Graph linkage capability.
//!
//! This module owns only the cross-domain link contract and validation boundary.
//! Health State and Evidence Graph remain distinct canonical domains; this type
//! does not own either domain's persistence, payloads, or policy engine.

use crate::canonical_authorization::AuthorizationRequest;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const SCHEMA_VERSION: &str = "1.0.0";

/// Directional, provenance-preserving reference between canonical health-state
/// and evidence entities. The referenced entities remain owned by their domains.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HealthStateEvidenceLink {
    pub id: String,
    pub schema_version: String,
    pub health_state_ref: String,
    pub evidence_ref: String,
    pub relationship: HealthStateEvidenceRelationship,
    pub provenance: LinkProvenance,
    #[serde(default)]
    pub context: Option<serde_json::Value>,
    pub uncertainty: Option<LinkUncertainty>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStateEvidenceRelationship {
    EvidenceInformsInterpretation,
    EvidenceInformsHypothesis,
    EvidenceSupportsContextualDecision,
    PersonalResponseObservedAfterIntervention,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkProvenance {
    pub method: String,
    pub created_at: Option<String>,
    pub actor_ref: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LinkUncertainty {
    Known,
    Estimated,
    Reported,
    Inferred,
    Missing,
    Conflicting,
    NotApplicable,
}

/// Authorization-bound context for creating or consuming a cross-domain link.
///
/// Fields are private so a caller cannot manufacture a governed linkage context
/// from arbitrary subject/scope strings. The canonical authorization boundary
/// supplies the request; this module only validates its structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedHealthStateEvidenceLinkContext {
    subject_ref: String,
    tenant_id: String,
    data_domain: String,
    capability_id: String,
    resource_type: String,
    resource_id: String,
    action: String,
}

impl AuthorizedHealthStateEvidenceLinkContext {
    pub(crate) fn from_authorized_request(
        request: &AuthorizationRequest,
    ) -> Result<Self, HealthStateEvidenceLinkError> {
        let values = [
            &request.principal_ref,
            &request.capability_id,
            &request.resource_type,
            &request.resource_id,
            &request.action,
            &request.tenant_id,
            &request.data_domain,
        ];
        if values.iter().any(|value| value.trim().is_empty() || value.chars().any(char::is_control)) {
            return Err(HealthStateEvidenceLinkError::InvalidAuthorizationContext);
        }

        Ok(Self {
            subject_ref: request.principal_ref.clone(),
            tenant_id: request.tenant_id.clone(),
            data_domain: request.data_domain.clone(),
            capability_id: request.capability_id.clone(),
            resource_type: request.resource_type.clone(),
            resource_id: request.resource_id.clone(),
            action: request.action.clone(),
        })
    }

    pub fn subject_ref(&self) -> &str { &self.subject_ref }
    pub fn tenant_id(&self) -> &str { &self.tenant_id }
    pub fn data_domain(&self) -> &str { &self.data_domain }
    pub fn capability_id(&self) -> &str { &self.capability_id }
    pub fn resource_type(&self) -> &str { &self.resource_type }
    pub fn resource_id(&self) -> &str { &self.resource_id }
    pub fn action(&self) -> &str { &self.action }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HealthStateEvidenceLinkError {
    #[error("invalid authorization context")]
    InvalidAuthorizationContext,
    #[error("invalid health-state/evidence link")]
    InvalidLink,
    #[error("link subject does not match authorized subject")]
    SubjectMismatch,
}

/// Smallest canonical validation boundary for cross-domain linkage.
///
/// It validates the link contract and binds access to an already-authorized
/// context. It deliberately does not resolve either reference, infer causation,
/// or persist the link.
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
            || link
                .provenance
                .created_at
                .as_deref()
                .is_some_and(|value| value.trim().is_empty())
            || link
                .provenance
                .actor_ref
                .as_deref()
                .is_some_and(|value| value.trim().is_empty())
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

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> AuthorizationRequest {
        AuthorizationRequest {
            principal_ref: "person-1".into(),
            capability_id: "health.evidence.link".into(),
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
                created_at: Some("2026-09-14T05:00:00Z".into()),
                actor_ref: Some("person-1".into()),
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
}
