use crate::device_sync::LocalDeviceHealthVault;
use std::error::Error;

pub struct VaultHardwareImporter;

impl VaultHardwareImporter {
    pub fn import_vault_from_hardware_path(
        _absolute_file_path: &str,
    ) -> Result<LocalDeviceHealthVault, Box<dyn Error>> {
        Err("Vault import is disabled until authenticated encryption, integrity ".to_string()
            + "verification, and key lifecycle management are implemented "
            + "and tested.".into())
    }
}
