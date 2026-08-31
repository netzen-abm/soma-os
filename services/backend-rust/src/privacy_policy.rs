//! SOMA privacy policy primitives.
//!
//! This module intentionally contains policy decisions, not storage or transport code.
//! The invariant is: personal and sensitive personal data remain on the user's device
//! unless an explicitly authorized operation requires a minimized encrypted payload.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataDisposition {
    LocalOnly,
    DerivedAndMinimized,
    EncryptedUserAuthorizedTransfer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataClass {
    Public,
    NonPersonal,
    Personal,
    SensitivePersonal,
}

pub struct PrivacyPolicy;

impl PrivacyPolicy {
    /// Default disposition for all personal and sensitive personal data.
    pub const fn default_disposition(class: DataClass) -> DataDisposition {
        match class {
            DataClass::Public | DataClass::NonPersonal => DataDisposition::DerivedAndMinimized,
            DataClass::Personal | DataClass::SensitivePersonal => DataDisposition::LocalOnly,
        }
    }

    /// Central invariant used by capability and transport layers.
    pub const fn transfer_allowed(
        class: DataClass,
        explicit_user_authorization: bool,
        encrypted: bool,
        minimized: bool,
    ) -> bool {
        match class {
            DataClass::Public | DataClass::NonPersonal => minimized,
            DataClass::Personal | DataClass::SensitivePersonal => {
                explicit_user_authorization && encrypted && minimized
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn personal_data_is_local_by_default() {
        assert_eq!(
            PrivacyPolicy::default_disposition(DataClass::Personal),
            DataDisposition::LocalOnly
        );
        assert_eq!(
            PrivacyPolicy::default_disposition(DataClass::SensitivePersonal),
            DataDisposition::LocalOnly
        );
    }

    #[test]
    fn sensitive_transfer_requires_all_guards() {
        assert!(!PrivacyPolicy::transfer_allowed(
            DataClass::SensitivePersonal,
            false,
            true,
            true
        ));
        assert!(!PrivacyPolicy::transfer_allowed(
            DataClass::SensitivePersonal,
            true,
            false,
            true
        ));
        assert!(!PrivacyPolicy::transfer_allowed(
            DataClass::SensitivePersonal,
            true,
            true,
            false
        ));
        assert!(PrivacyPolicy::transfer_allowed(
            DataClass::SensitivePersonal,
            true,
            true,
            true
        ));
    }
}
