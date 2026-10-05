#[path = "health_state_evidence_link_contract.rs"]
mod contract;
#[path = "health_state_evidence_link_boundary.rs"]
mod boundary;
#[cfg(test)]
#[path = "health_state_evidence_link_tests.rs"]
mod tests;

pub use contract::{
    AuthorizedHealthStateEvidenceLinkContext, HealthStateEvidenceLink,
    HealthStateEvidenceLinkError, HealthStateEvidenceRelationship, LinkProvenance,
    LinkUncertainty,
};
pub use boundary::HealthStateEvidenceLinkBoundary;
