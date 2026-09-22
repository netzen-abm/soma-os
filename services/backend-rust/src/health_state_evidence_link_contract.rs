//! Bounded Health State ↔ Evidence Graph linkage capability.
//!
//! This module owns only the cross-domain link contract and validation boundary.
//! Health State and Evidence Graph remain distinct canonical domains; this type
//! does not own either domain's persistence, payloads, or policy engine.

use crate::canonical_authorization::AuthorizationRequest;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub(super) const SCHEMA_VERSION: &str = "1.0.0";

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
    #[serde(rename = "EVIDENCE_INFORMS_INTERPRETATION")]
    EvidenceInformsInterpretation,
    #[serde(rename = "EVIDENCE_INFORMS_HYPOTHESIS")]
    EvidenceInformsHypothesis,
    #[serde(rename = "EVIDENCE_SUPPORTS_CONTEXTUAL_DECISION")]
    EvidenceSupportsContextualDecision,
    #[serde(rename = "PERSONAL_RESPONSE_OBSERVED_AFTER_INTERVENTION")]
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
    #[serde(rename = "known")]
    Known,
    #[serde(rename = "estimated")]
    Estimated,
    #[serde(rename = "reported")]
    Reported,
    #[serde(rename = "inferred")]
    Inferred,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "conflicting")]
    Conflicting,
    #[serde(rename = "not_applicable")]
    NotApplicable,
}

/// Authorization-bound context for creating or consuming a cross-domain link.
///
/// Fields are private so a caller cannot manufacture a governed linkage context
/// from arbitrary subject/scope strings. The canonical authorization boundary
/// supplies the request only after an authoritative ALLOW; this module validates
/// the request structure but does not evaluate policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedHealthStateEvidenceLinkContext {
    subject_ref: String,
    tenant_id: String,
    data_domain: String,
    capability_id: String,
    capability_version: String,
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
            &request.subject_ref,
            &request.capability_id,
            &request.capability_version,
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
            subject_ref: request.subject_ref.clone(),
            tenant_id: request.tenant_id.clone(),
            data_domain: request.data_domain.clone(),
            capability_id: request.capability_id.clone(),
            capability_version: request.capability_version.clone(),
            resource_type: request.resource_type.clone(),
            resource_id: request.resource_id.clone(),
            action: request.action.clone(),
        })
    }

    pub fn subject_ref(&self) -> &str {
        &self.subject_ref
    }
    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }
    pub fn data_domain(&self) -> &str {
        &self.data_domain
    }
    pub fn capability_id(&self) -> &str {
        &self.capability_id
    }
    pub fn capability_version(&self) -> &str {
        &self.capability_version
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

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HealthStateEvidenceLinkError {
    #[error("invalid authorization context")]
    InvalidAuthorizationContext,
    #[error("invalid health-state/evidence link")]
    InvalidLink,
    #[error("link subject does not match authorized subject")]
    SubjectMismatch,
}

