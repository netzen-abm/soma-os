//! SOMA shared infrastructure capability registry.
//!
//! Domain applications must consume reusable capabilities from this layer rather
//! than implementing their own copies of privacy, provenance, spatial, evidence,
//! or data-processing primitives.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityDomain {
    Privacy,
    Security,
    Data,
    Provenance,
    Evidence,
    Spatial,
    Knowledge,
    Intelligence,
    Transport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityStatus {
    Proposed,
    Experimental,
    Validated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityDescriptor {
    pub id: &'static str,
    pub domain: CapabilityDomain,
    pub status: CapabilityStatus,
}

/// Canonical registry of shared capabilities.
///
/// This is intentionally small at the foundation stage. New domain features
/// should reference a shared capability here before being implemented as an
/// application-specific module.
pub const CAPABILITIES: &[CapabilityDescriptor] = &[
    CapabilityDescriptor {
        id: "privacy.local_first",
        domain: CapabilityDomain::Privacy,
        status: CapabilityStatus::Validated,
    },
    CapabilityDescriptor {
        id: "security.authenticated_encryption",
        domain: CapabilityDomain::Security,
        status: CapabilityStatus::Proposed,
    },
    CapabilityDescriptor {
        id: "knowledge.provenance",
        domain: CapabilityDomain::Provenance,
        status: CapabilityStatus::Proposed,
    },
    CapabilityDescriptor {
        id: "knowledge.evidence",
        domain: CapabilityDomain::Evidence,
        status: CapabilityStatus::Proposed,
    },
    CapabilityDescriptor {
        id: "spatial.point_mapping",
        domain: CapabilityDomain::Spatial,
        status: CapabilityStatus::Proposed,
    },
];

pub fn capability(id: &str) -> Option<&'static CapabilityDescriptor> {
    CAPABILITIES.iter().find(|item| item.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_privacy_capability_is_registered() {
        let capability = capability("privacy.local_first").expect("privacy capability");
        assert_eq!(capability.domain, CapabilityDomain::Privacy);
        assert_eq!(capability.status, CapabilityStatus::Validated);
    }

    #[test]
    fn security_and_knowledge_capabilities_are_not_claimed_validated() {
        assert_eq!(
            capability("security.authenticated_encryption")
                .unwrap()
                .status,
            CapabilityStatus::Proposed
        );
        assert_eq!(
            capability("knowledge.provenance").unwrap().status,
            CapabilityStatus::Proposed
        );
    }
}
