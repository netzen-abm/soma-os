use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ComplianceValidationReport {
    pub is_permissible_for_delivery: bool,
    pub appended_regulatory_disclaimer: String,
    pub security_compliance_flags: Vec<String>,
}

pub struct SovereignComplianceShield;

impl SovereignComplianceShield {
    pub fn enforce_regulatory_compliance_checks(raw_outbound_message_content: &str) -> ComplianceValidationReport {
        let message_lower = raw_outbound_message_content.to_lowercase();
        let mut compliance_flags = Vec::new();

        Self::check_cure_claims(&message_lower, &mut compliance_flags);
        Self::check_prescriptive_language(&message_lower, &mut compliance_flags);
        Self::check_identifier_exposure(&message_lower, &mut compliance_flags);

        ComplianceValidationReport {
            is_permissible_for_delivery: compliance_flags.is_empty(),
            appended_regulatory_disclaimer: Self::standard_disclaimer(),
            security_compliance_flags: compliance_flags,
        }
    }

    fn check_cure_claims(message: &str, flags: &mut Vec<String>) {
        let has_cure_claim = message.contains("cure cancer") || message.contains("completely eliminate tumors");

        if has_cure_claim {
            flags.push("VIOLATION_INDICATOR::ABSOLUTE_CURE_CLAIM".to_string());
        }
    }

    fn check_prescriptive_language(message: &str, flags: &mut Vec<String>) {
        let has_prescriptive_language =
            message.contains("prescribe") || message.contains("take this medication to cure");

        if has_prescriptive_language {
            flags.push("VIOLATION_INDICATOR::PRESCRIPTIVE_OVERREACH".to_string());
        }
    }

    fn check_identifier_exposure(message: &str, flags: &mut Vec<String>) {
        let has_identifier = message.contains("patient name:") || message.contains("phone number:");

        if has_identifier {
            flags.push("VIOLATION_INDICATOR::PERSONAL_IDENTIFIER_EXPOSURE".to_string());
        }
    }

    fn standard_disclaimer() -> String {
        [
            "SOMA provides informational and management support.",
            "It does not provide a diagnosis or prescription.",
            "Health decisions should be discussed with an appropriate",
            "qualified healthcare professional.",
        ]
        .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_absolute_cure_claims() {
        let report = SovereignComplianceShield::enforce_regulatory_compliance_checks("This will cure cancer.");

        assert!(!report.is_permissible_for_delivery);
    }

    #[test]
    fn blocks_prescriptive_language() {
        let report = SovereignComplianceShield::enforce_regulatory_compliance_checks("Prescribe this medication.");

        assert!(!report.is_permissible_for_delivery);
    }

    #[test]
    fn blocks_identifier_exposure() {
        let report = SovereignComplianceShield::enforce_regulatory_compliance_checks("Patient name: Example");

        assert!(!report.is_permissible_for_delivery);
    }

    #[test]
    fn permits_neutral_management_information() {
        let report =
            SovereignComplianceShield::enforce_regulatory_compliance_checks("This is general management information.");

        assert!(report.is_permissible_for_delivery);
    }
}
