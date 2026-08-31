use crate::device_sync::LocalDeviceHealthVault;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

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
        vault: &LocalDeviceHealthVault,
        target_usb_directory: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let raw_json_string = serde_json::to_string(vault)?;
        // NOTE: Base64 is transport encoding only. Real AES-GCM must replace this
        // placeholder before sensitive health data is exported in production.
        let embedded_ciphertext = base64::engine::general_purpose::STANDARD.encode(format!(
            "HARDWARE_DISCONNECTED_AES256GCM_CIPHERTEXT::{}",
            raw_json_string
        ));
        let current_timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let backup_container = USBBackupContainer {
            file_signature: "SOMA_OS_SECURE_BACKUP".to_string(),
            export_timestamp: current_timestamp,
            payload_schema_version: "v1.1.0-salud".to_string(),
            encrypted_payload_base64: embedded_ciphertext,
        };
        let filename = format!("soma_health_vault_backup_{}.soma", current_timestamp);
        let absolute_target_path = Path::new(target_usb_directory).join(&filename);
        let mut file_handle = File::create(&absolute_target_path)?;
        file_handle.write_all(&serde_json::to_vec(&backup_container)?)?;
        Ok(absolute_target_path.to_string_lossy().into_owned())
    }
}
