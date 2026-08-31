use crate::device_sync::LocalDeviceHealthVault;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct USBBackupContainer {
    pub file_signature: String,
    pub export_timestamp: u64,
    pub payload_schema_version: String,
    pub encrypted_payload_base64: String,
}

pub struct VaultHardwareExporter;

impl VaultHardwareExporter {
    pub fn export_vault_to_hardware_path(
        _vault: &LocalDeviceHealthVault,
        _target_usb_directory: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Err(
            "Vault export is disabled until authenticated encryption and key "
                .to_string()
                + "lifecycle management are implemented and tested."
                .into(),
        )
    }
}
