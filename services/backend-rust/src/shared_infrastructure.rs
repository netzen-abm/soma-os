//! SOMA shared infrastructure capability registry projection.
//!
//! The versioned JSON registry is the canonical source of capability identity
//! and metadata. This Rust module is a typed projection for runtime consumers;
//! it must not maintain a second capability inventory.

use serde::Deserialize;

const CANONICAL_REGISTRY: &str = include_str!("../../../services/shared/capability_registry.json");

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
    Research,
    Transport,
    Infrastructure,
}

impl CapabilityDomain {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "privacy" => Some(Self::Privacy),
            "security" => Some(Self::Security),
            "data" => Some(Self::Data),
            "provenance" => Some(Self::Provenance),
            "evidence" => Some(Self::Evidence),
            "spatial" => Some(Self::Spatial),
            "knowledge" => Some(Self::Knowledge),
            "intelligence" => Some(Self::Intelligence),
            "research" => Some(Self::Research),
            "transport" => Some(Self::Transport),
            "infrastructure" => Some(Self::Infrastructure),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityStatus {
    Proposed,
    Experimental,
    Validated,
    Implemented,
    Architecture,
    AdapterBoundary,
    Optional,
    Planned,
}

impl CapabilityStatus {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "proposed" => Some(Self::Proposed),
            "experimental" => Some(Self::Experimental),
            "validated" => Some(Self::Validated),
            "implemented" => Some(Self::Implemented),
            "architecture" => Some(Self::Architecture),
            "adapter-boundary" => Some(Self::AdapterBoundary),
            "optional" => Some(Self::Optional),
            "planned" => Some(Self::Planned),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityDescriptor {
    pub id: String,
    pub version: String,
    pub domain: CapabilityDomain,
    pub status: CapabilityStatus,
}

#[derive(Debug, Deserialize)]
struct Registry {
    schema_version: String,
    capabilities: Vec<RegistryCapability>,
}

#[derive(Debug, Deserialize)]
struct RegistryCapability {
    id: String,
    version: String,
    domain: String,
    status: String,
}

fn canonical_registry() -> Registry {
    serde_json::from_str(CANONICAL_REGISTRY).expect("canonical capability registry must be valid")
}

/// Return a typed projection of one capability from the canonical JSON registry.
pub fn capability(id: &str) -> Option<CapabilityDescriptor> {
    canonical_registry()
        .capabilities
        .into_iter()
        .find(|item| item.id == id)
        .and_then(|item| {
            Some(CapabilityDescriptor {
                id: item.id,
                version: item.version,
                domain: CapabilityDomain::parse(&item.domain)?,
                status: CapabilityStatus::parse(&item.status)?,
            })
        })
}

/// Validate that the embedded registry has a supported schema version and that
/// every entry can be projected into the typed runtime representation.
pub fn validate_registry() -> bool {
    let registry = canonical_registry();
    registry.schema_version == "1.0.0"
        && !registry.capabilities.is_empty()
        && registry.capabilities.iter().all(|item| {
            !item.id.is_empty()
                && !item.version.is_empty()
                && CapabilityDomain::parse(&item.domain).is_some()
                && CapabilityStatus::parse(&item.status).is_some()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_registry_is_valid() {
        assert!(validate_registry());
    }

    #[test]
    fn lookup_uses_canonical_registry() {
        let capability = capability("privacy.local_first").expect("privacy capability");
        assert_eq!(capability.version, "0.1.0");
        assert_eq!(capability.domain, CapabilityDomain::Privacy);
        assert_eq!(capability.status, CapabilityStatus::Validated);
    }

    #[test]
    fn canonical_registry_contains_governed_operation_dependencies() {
        for capability_id in ["evidence.research", "intelligence.ai", "protocol.mcp"] {
            assert!(capability(capability_id).is_some(), "missing {capability_id}");
        }
    }

    #[test]
    fn ai_remains_optional() {
        assert_eq!(capability("intelligence.ai").unwrap().status, CapabilityStatus::Optional);
    }
}
