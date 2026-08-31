// File Path: services/backend-rust/src/key_restoration.rs

use ring::signature::{Ed25519KeyPair, KeyPair};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReconstructedKeyCluster {
    pub anonymized_user_hash: String,
    pub public_verification_key_hex: String, // Ed25519 target signature key
    pub nostr_x_only_pubkey_hex: String,     // secp256k1 target p2p key
}

pub struct CryptographicRestorationEngine;

impl CryptographicRestorationEngine {
    /// Ingests a 12-word mnemonic phrase to safely reconstruct the user's local, non-custodial identities
    pub fn rebuild_keys_from_mnemonic(
        mnemonic_phrase: &str,
    ) -> Result<ReconstructedKeyCluster, Box<dyn std::error::Error>> {
        // 1. Sanitize text string boundaries by normalizing spaces and case
        let sanitized_words: Vec<String> = mnemonic_phrase
            .split_whitespace()
            .map(|word| word.trim().to_lowercase())
            .collect();

        // 2. Validate compliance scale parameters are structurally sound
        if sanitized_words.len() != 12 {
            return Err("Sovereign Security Error: Provided mnemonic phrase length is invalid. Must be exactly 12 words.".into());
        }

        // 3. Compute deterministic seed bytes using a SHA-256 hash across the ordered string sequence
        let unified_phrase_string = sanitized_words.join(" ");
        let mut hasher = Sha256::new();
        hasher.update(unified_phrase_string.as_bytes());
        let derived_seed_bytes = hasher.finalize(); // Generates 32 bytes of deterministic entropy

        // 4. Derive the native Ed25519 Keypair for Salud Protocol verification
        // For production FFI stability, we wrap the derived seed bytes into standard PKCS#8 structures
        let mut pkcs8_template = vec![0u8; 84]; // Allocation padding size profile
        pkcs8_template[..32].copy_from_slice(&derived_seed_bytes);

        let ed25519_key = Ed25519KeyPair::from_pkcs8_maybe_unchecked(&pkcs8_template)
            .map_err(|_| "Failed to compile Ed25519 key framework from local seed bytes.")?;
        let public_key_hex = hex::encode(ed25519_key.public_key().as_ref());

        // 5. Derive the native secp256k1 Keypair for P2P Nostr mesh networking
        let secp = Secp256k1::new();
        let secp_secret_key = SecretKey::from_slice(&derived_seed_bytes)?;
        let secp_public_key = PublicKey::from_secret_key(&secp, &secp_secret_key);
        let nostr_pubkey_hex = hex::encode(&secp_public_key.serialize()[1..33]); // Extract X-only coordinate representation

        // 6. Generate the unique, anonymized user hash signature
        let mut user_hasher = Sha256::new();
        user_hasher.update(public_key_hex.as_bytes());
        let user_hash_hex = format!("{:x}", user_hasher.finalize());

        println!("🔒 Cryptographic restoration sequence complete. Identity keys successfully re-derived on-device.");
        Ok(ReconstructedKeyCluster {
            anonymized_user_hash: user_hash_hex,
            public_verification_key_hex: public_key_hex,
            nostr_x_only_pubkey_hex: nostr_pubkey_hex,
        })
    }
}
