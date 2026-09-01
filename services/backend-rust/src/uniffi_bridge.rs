// Auto-generate foreign language interfaces cleanly using uniffi bindings
uniffi::setup_scaffolding!();

use crate::device_sync::LocalDeviceHealthVault;
use crate::vault_exporter::VaultHardwareExporter;

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum MobileBridgeError {
    #[error("Local cryptographic hardware serialization tracking failed: {msg}")]
    ExportFailure { msg: String },
}

#[uniffi::export]
pub fn native_mobile_hardware_export_bridge(
    tracked_millet_history: Vec<String>,
    user_logged_conditions: Vec<String>,
    current_vitality_score: u8,
    target_absolute_system_path: String,
) -> Result<String, MobileBridgeError> {
    // Reconstruct the internal domain data memory state profiles
    let core_vault = LocalDeviceHealthVault {
        tracked_millet_porridge_history: tracked_millet_history,
        user_logged_conditions,
        current_vitality_score,
    };

    // Forward properties down into established filesystem save modules
    match VaultHardwareExporter::export_vault_to_hardware_path(&core_vault, &target_absolute_system_path) {
        Ok(completed_saved_path) => Ok(completed_saved_path),
        Err(error_footprint) => Err(MobileBridgeError::ExportFailure {
            msg: error_footprint.to_string()
        }),
    }
}
