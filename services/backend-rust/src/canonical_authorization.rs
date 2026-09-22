//! Canonical Rust-side authorization contract and adapter boundary.
//!
//! This module never evaluates policy independently. It consumes the authoritative
//! SOMA authorization decision and mints protected contexts only from ALLOW.

#[path = "canonical_authorization_boundary.rs"]
mod boundary;
#[path = "canonical_authorization_contract.rs"]
mod contract;

pub use boundary::CanonicalAuthorizationBoundary;
pub use contract::{AuthorizationDecision, AuthorizationRequest};

#[cfg(test)]
#[path = "canonical_authorization/tests.rs"]
mod tests;
