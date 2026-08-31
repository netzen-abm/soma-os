use futures_util::SinkExt;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_tungstenite::{
    connect_async,
    tungstenite::protocol::Message,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NostrEvent {
    pub id: String,
    pub pubkey: String,
    pub created_at: u64,
    pub kind: u32,
    pub tags: Vec<Vec<String>>,
    pub content: String,
    pub sig: String,
}

pub struct NostrBroadcastEngine;

impl NostrBroadcastEngine {
    fn calculate_event_id(
        pubkey: &str,
        created_at: u64,
        kind: u32,
        tags: &[Vec<String>],
        content: &str,
    ) -> String {
        let serialized = serde_json::json!([
            0,
            pubkey,
            created_at,
            kind,
            tags,
            content
        ])
        .to_string();

        let mut hasher = Sha256::new();
        hasher.update(serialized.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub async fn broadcast_botanical_update(
        relay_url: &str,
        raw_private_key_hex: &str,
        update_message: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let secp = Secp256k1::new();
        let key_bytes = hex::decode(raw_private_key_hex)?;
        let secret_key = SecretKey::from_slice(&key_bytes)?;
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let pubkey_hex = hex::encode(&public_key.serialize()[1..33]);

        let current_timestamp =
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let tags = vec![vec![
            "t".to_string(),
            "somaos_vitals".to_string(),
        ]];

        let event_id = Self::calculate_event_id(
            &pubkey_hex,
            current_timestamp,
            1,
            &tags,
            update_message,
        );

        // Signing is intentionally disabled until a standards-compliant
        // Nostr Schnorr implementation is integrated and tested.
        let signature = "";

        let event = NostrEvent {
            id: event_id,
            pubkey: pubkey_hex,
            created_at: current_timestamp,
            kind: 1,
            tags,
            content: update_message.to_string(),
            sig: signature.to_string(),
        };

        let envelope = serde_json::json!(["EVENT", event]).to_string();
        let (mut ws_stream, _) = connect_async(relay_url).await?;

        ws_stream.send(Message::Text(envelope)).await?;
        Ok(())
    }
}
