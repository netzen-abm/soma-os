mod contract;
mod implementation;

pub use contract::{PersonalHealthRecordRepository, RepositoryError, RepositoryQuery};
pub use implementation::LocalPersonalHealthRecordRepository;

#[cfg(test)]
mod tests;
