use crate::nostr_client::NostrEvent;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalDeviceHealthVault {
    pub tracked_millet_porridge_history: Vec<String>,
    pub user_logged_conditions: Vec<String>,
    pub current_vitality_score: u8,
}

pub struct NonCustodialSyncEngine;

impl NonCustodialSyncEngine {
    pub fn compile_backup_event(
        _local_vault: &LocalDeviceHealthVault,
        _user_public_key_hex: &str,
    ) -> Result<NostrEvent, Box<dyn std::error::Error>> {
        Err("device sync is disabled until authenticated encryption and signing are implemented".into())
    }
}

#[cfg(test)]
mod sync_tests {
    use super::*;

    #[test]
    fn test_sync_is_disabled_until_security_is_ready() {
        let vault = LocalDeviceHealthVault {
            tracked_millet_porridge_history: Vec::new(),
            user_logged_conditions: Vec::new(),
            current_vitality_score: 0,
        };

        let result = NonCustodialSyncEngine::compile_backup_event(
            &vault,
            "0000000000000000000000000000000000000000000000000000000000000000",
        );

        assert!(result.is_err());
    }
}
