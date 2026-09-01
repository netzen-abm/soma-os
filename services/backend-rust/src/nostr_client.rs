//! Nostr adapter boundary.
//!
//! Broadcasting remains intentionally disabled until a standards-compliant
//! Schnorr signing implementation is integrated and independently tested.

use std::error::Error;

pub struct NostrBroadcastEngine;

impl NostrBroadcastEngine {
    /// Fail closed rather than emitting an unsigned or mock-signed event.
    pub async fn broadcast_botanical_update(
        _relay_url: &str,
        _raw_private_key_hex: &str,
        _update_message: &str,
    ) -> Result<(), Box<dyn Error>> {
        Err("Nostr broadcast unavailable: signing adapter is not validated".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn broadcast_fails_closed_without_validated_signing() {
        let result = NostrBroadcastEngine::broadcast_botanical_update(
            "wss://example.invalid",
            "00",
            "test",
        )
        .await;

        assert!(result.is_err());
    }
}
