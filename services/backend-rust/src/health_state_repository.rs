//! Canonical Health State repository boundary.

#[path = "health_state_repository_contract.rs"]
mod contract;
#[path = "health_state_repository_impl.rs"]
mod implementation;

pub use contract::{
    AuthorizedHealthStateAccessContext, HealthStateQuery, HealthStateRepository,
    HealthStateRepositoryError, HealthStateTimelineEntry, ALLOWED_ENTITY_TYPES,
    HEALTH_STATE_CONTENT_TYPE,
};
pub use implementation::LocalHealthStateRepository;

#[cfg(test)]
#[path = "health_state_repository_tests.rs"]
mod tests;
