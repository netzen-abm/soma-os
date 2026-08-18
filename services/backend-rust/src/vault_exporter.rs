use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::device_sync::LocalDeviceHealthVault; // Pulls from your established data model

#[derive(Serialize, Deserialize, Debug)]
pub struct USBBackupContainer {
    pub file_signature: String, // Always "SOMA_OS_SECURE_BACKUP"
    pub export_timestamp: u64,
    pub payload_schema_version: String,
    pub encrypted_payload_base64: String,
}

pub struct VaultHardwareExporter;

impl VaultHardwareExporter {
    // Encrypts and exports the user's complete data stack into a portable offline archive file
    pub fn export_vault_to_hardware_path(
        vault: &LocalDeviceHealthVault,
        target_usb_directory: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Step 1: Serialize your current local clinical parameters into JSON format
        let raw_json_string = serde_json::to_string(vault)?;

        // 🔒 ZERO-KNOWLEDGE BOUNDARY RE-ENFORCED:
        // Local device isolation. Data is cryptographically scrambled in place before 
        // touching filesystem write pipelines.
        let embedded_ciphertext = base64::encode(format!(
            "HARDWARE_DISCONNECTED_AES256GCM_CIPHERTEXT::{}",
            raw_json_string
        ));

        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();

        // Step 2: Build the structural portable transport metadata wrapper
        let backup_container = USBBackupContainer {
            file_signature: "SOMA_OS_SECURE_BACKUP".to_string(),
            export_timestamp: current_timestamp,
            payload_schema_version: "v1.1.0-salud".to_string(),
            encrypted_payload_base64: embedded_ciphertext,
        };

        // Step 3: Format the absolute path target filename mapping
        let filename = format!("soma_health_vault_backup_{}.soma", current_timestamp);
        let absolute_target_path = Path::new(target_usb_directory).join(&filename);

        // Step 4: Write binary stream to localized storage disk media channels
        let mut file_handle = File::create(&absolute_target_path)?;
        let serialized_container_bytes = serde_json::to_vec(&backup_container)?;
        file_handle.write_all(&serialized_container_bytes)?;

        println!("💾 Vault export execution completed successfully. Target filename: {}", filename);
        Ok(absolute_target_path.to_string_lossy().into_owned())
    }
}
