//! Canonical Rust-side authorization contract and adapter boundary.
//!
//! This module never evaluates policy independently. It consumes the authoritative
//! SOMA authorization decision and mints protected contexts only from ALLOW.

pub use crate::canonical_authorization_boundary::CanonicalAuthorizationBoundary;
pub use crate::canonical_authorization_contract::{AuthorizationDecision, AuthorizationRequest};

#[cfg(test)]
mod tests;
