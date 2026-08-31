use crate::compressor::SovereignCompressor;
use crate::device_sync::LocalDeviceHealthVault;
use crate::vault_exporter::USBBackupContainer;
use base64::Engine;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct VaultHardwareImporter;

impl VaultHardwareImporter {
    pub fn import_vault_from_hardware_path(
        absolute_file_path: &str,
    ) -> Result<LocalDeviceHealthVault, Box<dyn Error>> {
        let path_handle = Path::new(absolute_file_path);
        if !path_handle.exists() {
            return Err("Target archive path location could not be resolved by host filesystems.".into());
        }
        let mut file = File::open(path_handle)?;
        let mut serialized_container_bytes = Vec::new();
        file.read_to_end(&mut serialized_container_bytes)?;
        let container: USBBackupContainer = serde_json::from_slice(&serialized_container_bytes)?;
        if container.file_signature != "SOMA_OS_SECURE_BACKUP" {
            return Err("Rejection Alert: File signature mismatch. Provided archive is unreadable or altered.".into());
        }
        let compressed_bytes = base64::engine::general_purpose::STANDARD
            .decode(&container.encrypted_payload_base64)?;
        let decrypted_json_plaintext = SovereignCompressor::decompress_payload(&compressed_bytes)?;
        let parsing_prefix = "HARDWARE_DISCONNECTED_AES256GCM_CIPHERTEXT::";
        if !decrypted_json_plaintext.starts_with(parsing_prefix) {
            return Err("Decryption Execution Failure: Target payload wrapper lacks proper security metadata signatures.".into());
        }
        let cleaned_vault_json = decrypted_json_plaintext.replacen(parsing_prefix, "", 1);
        Ok(serde_json::from_str(&cleaned_vault_json)?)
    }
}
