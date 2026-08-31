use ring::rand::SystemRandom;
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalZkpPayload {
    pub anonymized_user_hash: String,
    pub verified_vitality_score: u8,
    pub timestamp_epoch: u64,
    pub salud_schema_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CryptographicProofPackage {
    pub raw_payload: LocalZkpPayload,
    pub public_verification_key_hex: String,
    pub signature_proof_hex: String,
}

pub struct SovereignCryptoEngine;

impl SovereignCryptoEngine {
    pub fn generate_anonymized_user_hash(raw_user_id: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(raw_user_id.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn sign_health_milestone(
        raw_user_id: &str,
        vitality_score: u8,
        schema_ver: &str,
    ) -> Result<CryptographicProofPackage, Box<dyn std::error::Error>> {
        let rng = SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)
            .map_err(|_| "failed to generate Ed25519 key material")?;
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
            .map_err(|_| "failed to parse generated Ed25519 key material")?;

        let payload = LocalZkpPayload {
            anonymized_user_hash: Self::generate_anonymized_user_hash(raw_user_id),
            verified_vitality_score: vitality_score,
            timestamp_epoch: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
            salud_schema_version: schema_ver.to_string(),
        };

        let serialized_msg = serde_json::to_vec(&payload)?;
        let signature = key_pair.sign(&serialized_msg);

        Ok(CryptographicProofPackage {
            raw_payload: payload,
            public_verification_key_hex: hex::encode(key_pair.public_key().as_ref()),
            signature_proof_hex: hex::encode(signature.as_ref()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anonymized_user_hash_generation() {
        let hash_1 = SovereignCryptoEngine::generate_anonymized_user_hash("telegram_user_998877");
        let hash_2 = SovereignCryptoEngine::generate_anonymized_user_hash("telegram_user_998877");
        let hash_3 = SovereignCryptoEngine::generate_anonymized_user_hash("whatsapp_user_112233");
        assert_eq!(hash_1, hash_2);
        assert_ne!(hash_1, hash_3);
        assert_eq!(hash_1.len(), 64);
    }

    #[test]
    fn test_local_health_milestone_signature_generation() {
        let proof_result = SovereignCryptoEngine::sign_health_milestone(
            "user_biometric_node_01", 92, "v1.0.0-salud",
        );
        assert!(proof_result.is_ok());
        let package = proof_result.unwrap();
        assert_eq!(package.public_verification_key_hex.len(), 64);
        assert_eq!(package.signature_proof_hex.len(), 128);
        assert_eq!(package.raw_payload.verified_vitality_score, 92);
        assert_eq!(package.raw_payload.salud_schema_version, "v1.0.0-salud");
    }
}
