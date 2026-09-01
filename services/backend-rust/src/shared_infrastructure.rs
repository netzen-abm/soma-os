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
    Infrastructure,
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
/// New domain features should register a reusable capability here before being
/// implemented as an application-specific module. This keeps capabilities
/// discoverable and reusable across web, mobile, bots, AI and protocol adapters.
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
        status: CapabilityStatus::Experimental,
    },
    CapabilityDescriptor {
        id: "knowledge.evidence",
        domain: CapabilityDomain::Evidence,
        status: CapabilityStatus::Experimental,
    },
    CapabilityDescriptor {
        id: "data.evidence_migration",
        domain: CapabilityDomain::Data,
        status: CapabilityStatus::Experimental,
    },
    CapabilityDescriptor {
        id: "spatial.point_mapping",
        domain: CapabilityDomain::Spatial,
        status: CapabilityStatus::Proposed,
    },
    CapabilityDescriptor {
        id: "infrastructure.capability_adapter_boundary",
        domain: CapabilityDomain::Infrastructure,
        status: CapabilityStatus::Validated,
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
    fn evidence_capabilities_are_shared_but_not_overclaimed() {
        assert_eq!(
            capability("knowledge.provenance").unwrap().status,
            CapabilityStatus::Experimental
        );
        assert_eq!(
            capability("knowledge.evidence").unwrap().status,
            CapabilityStatus::Experimental
        );
        assert_eq!(
            capability("data.evidence_migration").unwrap().status,
            CapabilityStatus::Experimental
        );
    }

    #[test]
    fn adapter_boundary_is_a_shared_infrastructure_capability() {
        let capability = capability("infrastructure.capability_adapter_boundary")
            .expect("capability adapter boundary");
        assert_eq!(capability.domain, CapabilityDomain::Infrastructure);
        assert_eq!(capability.status, CapabilityStatus::Validated);
    }

    #[test]
    fn security_capability_is_not_claimed_validated() {
        assert_eq!(
            capability("security.authenticated_encryption")
                .unwrap()
                .status,
            CapabilityStatus::Proposed
        );
    }
}
