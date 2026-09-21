//! Canonical Rust-side authorization contract and adapter boundary.
//!
//! This module never evaluates policy independently. It consumes the authoritative
//! SOMA authorization decision and mints protected contexts only from ALLOW.

mod boundary;
mod contract;

pub use boundary::CanonicalAuthorizationBoundary;
pub use contract::{AuthorizationDecision, AuthorizationRequest};

#[cfg(test)]
mod tests;
