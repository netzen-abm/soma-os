#[path = "health_state_evidence_link_boundary.rs"]
mod boundary;
#[path = "health_state_evidence_link_contract.rs"]
mod contract;
#[cfg(test)]
#[path = "health_state_evidence_link_tests.rs"]
mod tests;

pub(super) use contract::{
    AuthorizedHealthStateEvidenceLinkContext, HealthStateEvidenceLink, HealthStateEvidenceLinkError,
    HealthStateEvidenceRelationship, LinkProvenance, LinkUncertainty,
};
