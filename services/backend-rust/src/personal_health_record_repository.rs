#[path = "personal_health_record_repository_contract.rs"]
mod contract;
#[path = "personal_health_record_repository_impl.rs"]
mod implementation;
#[cfg(test)]
#[path = "personal_health_record_repository_security_tests.rs"]
mod security_tests;
#[cfg(test)]
#[path = "personal_health_record_repository_tests.rs"]
mod tests;

pub(super) use contract::{PersonalHealthRecordRepository, RepositoryError, RepositoryQuery};
pub(super) use implementation::LocalPersonalHealthRecordRepository;
