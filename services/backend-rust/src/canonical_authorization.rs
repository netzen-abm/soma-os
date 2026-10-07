//! Canonical Rust-side authorization contract and adapter boundary.
//!
//! This module never evaluates policy independently. It consumes the authoritative
//! SOMA authorization decision and mints protected contexts only from ALLOW.

mod contract;

pub use crate::canonical_authorization_boundary::CanonicalAuthorizationBoundary;
pub use contract::{AuthorizationDecision, AuthorizationRequest};

#[cfg(test)]
mod tests;
