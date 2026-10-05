#[path = "health_state_outcome_boundary.rs"]
mod boundary;
#[path = "health_state_outcome_contract.rs"]
mod contract;
#[cfg(test)]
#[path = "health_state_outcome_tests.rs"]
mod tests;

#[allow(unused_imports)]
pub use boundary::{OutcomeAuthorizationBoundary, OutcomeBoundary};
#[allow(unused_imports)]
pub use contract::{AuthorizedOutcomeContext, Outcome, OutcomeError, OutcomeStatus};
