mod contract;
mod boundary;
#[cfg(test)]
#[path = "health_state_outcome_tests.rs"]
mod tests;

pub use contract::{AuthorizedOutcomeContext, Outcome, OutcomeError, OutcomeStatus};
pub use boundary::{OutcomeAuthorizationBoundary, OutcomeBoundary};
