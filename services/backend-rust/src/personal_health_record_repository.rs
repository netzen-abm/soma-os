//! Canonical Personal Health Record repository facade.
//!
//! The facade preserves the repository contract while separating implementation,
//! contract, and test boundaries without creating a second persistence model.

#[path = "personal_health_record_repository_contract.rs"]
mod contract;
#[path = "personal_health_record_repository_implementation.rs"]
mod implementation;


pub use contract::{PersonalHealthRecordRepository, RepositoryError, RepositoryQuery};
pub use implementation::LocalPersonalHealthRecordRepository;


#[cfg(test)]
#[path = "personal_health_record_repository_tests.rs"]
mod tests;
#[cfg(test)]
#[path = "personal_health_record_repository_security_tests.rs"]
mod security_tests;
