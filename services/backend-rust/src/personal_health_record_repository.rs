mod contract;
mod implementation;
#[cfg(test)]
mod tests {
    #[path = "personal_health_record_repository_tests.rs"]
    mod functional;
    #[path = "personal_health_record_repository_security_tests.rs"]
    mod security;
}
pub use contract::{PersonalHealthRecordRepository, RepositoryError, RepositoryQuery};
pub use implementation::LocalPersonalHealthRecordRepository;
