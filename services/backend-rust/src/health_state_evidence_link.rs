#[path = "health_state_evidence_link_boundary.rs"]
mod boundary;
#[path = "health_state_evidence_link_contract.rs"]
mod contract;
#[cfg(test)]
#[path = "health_state_evidence_link_tests.rs"]
mod tests;

#[allow(unused_imports)]
pub use boundary::HealthStateEvidenceLinkBoundary;
#[allow(unused_imports)]
pub use contract::{
    AuthorizedHealthStateEvidenceLinkContext, HealthStateEvidenceLink, HealthStateEvidenceLinkError,
    HealthStateEvidenceRelationship, LinkProvenance, LinkUncertainty,
};
