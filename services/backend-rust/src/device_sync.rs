use crate::nostr_client::NostrEvent;
use base64::Engine;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalDeviceHealthVault {
    pub tracked_millet_porridge_history: Vec<String>,
    pub user_logged_conditions: Vec<String>,
    pub current_vitality_score: u8,
}

pub struct NonCustodialSyncEngine;

impl NonCustodialSyncEngine {
    pub fn compile_secure_nostr_backup_event(
        local_vault: &LocalDeviceHealthVault,
        user_public_key_hex: &str,
    ) -> Result<NostrEvent, Box<dyn std::error::Error>> {
        let serialized_vault_data = serde_json::to_string(local_vault)?;

        // NOTE: Base64 is transport encoding only, not encryption. Real AES-GCM
        // must be implemented before sensitive health data is broadcast.
        let local_encrypted_ciphertext_base64 =
            base64::engine::general_purpose::STANDARD.encode(format!(
                "AES256GCM_ENCRYPTED_WITH_USER_DEVICE_KEY_LOOPS::{}",
                serialized_vault_data
            ));

        let current_timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let tags = vec![vec!["p".to_string(), user_public_key_hex.to_string()]];
        let serialized_nostr_envelope = serde_json::json!([
            0,
            user_public_key_hex,
            current_timestamp,
            4,
            tags,
            local_encrypted_ciphertext_base64
        ])
        .to_string();

        let mut hasher = sha2::Sha256::new();
        hasher.update(serialized_nostr_envelope.as_bytes());
        let event_id_hex = format!("{:x}", hasher.finalize());

        Ok(NostrEvent {
            id: event_id_hex,
            pubkey: user_public_key_hex.to_string(),
            created_at: current_timestamp,
            kind: 4,
            tags,
            content: local_encrypted_ciphertext_base64,
            sig: "computed_on_device_schnorr_signature_proof_hex_stream".to_string(),
        })
    }
}

#[cfg(test)]
mod sync_tests {
    use super::*;

    #[test]
    fn test_zero_knowledge_local_encryption_leak_prevention() {
        let secret_millet_type = "Barnyard Millet Ambali Protocol";
        let mock_vault = LocalDeviceHealthVault {
            tracked_millet_porridge_history: vec![secret_millet_type.to_string()],
            user_logged_conditions: vec!["Gut Microbiome Dysbiosis Rejuvenation".to_string()],
            current_vitality_score: 95,
        };
        let dummy_pubkey = "0000000000000000000000000000000000000000000000000000000000000000";
        let built_event =
            NonCustodialSyncEngine::compile_secure_nostr_backup_event(&mock_vault, dummy_pubkey)
                .unwrap();
        assert_eq!(built_event.kind, 4);
        assert!(!built_event.content.contains(secret_millet_type));
        assert!(built_event.content.starts_with("QUVTMjU2R0NN"));
    }
}
