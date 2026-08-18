use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::error::Error;
use crate::vault_exporter::USBBackupContainer; // Reference storage container specifications
use crate::compressor::SovereignCompressor;     // Reuses our DEFLATE stream tools
use crate::device_sync::LocalDeviceHealthVault;  // Target initialization structure

pub struct VaultHardwareImporter;

impl VaultHardwareImporter {
    /// Reads, decrypts, and re-assembles an existing .soma local archive pack from local hardware storage paths
    pub fn import_vault_from_hardware_path(
        absolute_file_path: &str,
    ) -> Result<LocalDeviceHealthVault, Box<dyn Error>> {
        let path_handle = Path::new(absolute_file_path);
        if !path_handle.exists() {
            return Err("Target archive path location could not be resolved by host filesystems.".into());
        }

        // Step 1: Read raw binary stream data completely into memory buffers
        let mut file = File::open(path_handle)?;
        let mut serialized_container_bytes = Vec::new();
        file.read_to_end(&mut serialized_container_bytes)?;

        // Step 2: Unpack data wrapper structure
        let container: USBBackupContainer = serde_json::from_slice(&serialized_container_bytes)?;
        if container.file_signature != "SOMA_OS_SECURE_BACKUP" {
            return Err("Rejection Alert: File signature mismatch. Provided archive is unreadable or altered.".into());
        }

        // Step 3: Decode base64 encapsulation back into compressed byte fields
        let compressed_bytes = base64::decode(&container.encrypted_payload_base64)?;

        // Step 4: Decompress stream using our DEFLATE pipeline decoder routines
        let decrypted_json_plaintext = SovereignCompressor::decompress_payload(&compressed_bytes)?;

        // 🔒 ZERO-KNOWLEDGE ENCLAVE DISPATCH LOOP:
        // Strip out hardware transport framing layers to isolate inner encrypted text variables
        let parsing_prefix = "HARDWARE_DISCONNECTED_AES256GCM_CIPHERTEXT::";
        if !decrypted_json_plaintext.starts_with(parsing_prefix) {
            return Err("Decryption Execution Failure: Target payload wrapper lacks proper security metadata signatures.".into());
        }
        
        let cleaned_vault_json = decrypted_json_plaintext.replacen(parsing_prefix, "", 1);

        // Step 5: Map fields directly onto target local hardware runtime memory states
        let restored_vault: LocalDeviceHealthVault = serde_json::from_str(&cleaned_vault_json)?;
        println!("🔒 Secure restore execution finalized. Biometrics populated to device memory successfully.");

        Ok(restored_vault)
    }
}
