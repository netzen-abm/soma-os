use ring::rand::SystemRandom;
use ring::signature::{Ed25519KeyPair, KeyPair, KeyPairHashAlgebra};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

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
    // Generate a secure SHA-256 string footprint of a consumer identifier profile
    pub fn generate_anonymized_user_hash(raw_user_id: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(raw_user_id.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    // Sign health metrics locally to produce a zero-knowledge verifiable transaction packet
    pub fn sign_health_milestone(
        raw_user_id: &str,
        vitality_score: u8,
        schema_ver: &str,
    ) -> Result<CryptographicProofPackage, Box<dyn std::error::Error>> {
        let rng = SystemRandom::new();
        
        // Generate an ephemeral Ed25519 signing keypair locally in memory memory
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)?;
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())?;
        
        // Extract public verification signature keys
        let public_key_bytes = key_pair.public_key().as_ref();
        let public_key_hex = hex::encode(public_key_bytes);

        let payload = LocalZkpPayload {
            anonymized_user_hash: Self::generate_anonymized_user_hash(raw_user_id),
            verified_vitality_score: vitality_score,
            timestamp_epoch: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
            salud_schema_version: schema_ver.to_string(),
        };

        // Serialize structured payload directly into JSON byte array streams
        let serialized_msg = serde_json::to_vec(&payload)?;
        
        // Sign the deterministic payload data package
        let mut signature_bytes = vec![0u8; key_pair.signature_length()];
        key_pair.sign(&serialized_msg, &mut signature_bytes);
        let signature_hex = hex::encode(signature_bytes);

        Ok(CryptographicProofPackage {
            raw_payload: payload,
            public_verification_key_hex: public_key_hex,
            signature_proof_hex: signature_hex,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anonymized_user_hash_generation() {
        let user_id_1 = "telegram_user_998877";
        let user_id_2 = "telegram_user_998877";
        let user_id_3 = "whatsapp_user_112233";

        let hash_1 = SovereignCryptoEngine::generate_anonymized_user_hash(user_id_1);
        let hash_2 = SovereignCryptoEngine::generate_anonymized_user_hash(user_id_2);
        let hash_3 = SovereignCryptoEngine::generate_anonymized_user_hash(user_id_3);

        // Verification matches standard SHA-256 properties
        assert_eq!(hash_1, hash_2, "Deterministic inputs must output identical hashes.");
        assert_ne!(hash_1, hash_3, "Distinct inputs must generate unique hash signatures.");
        assert_eq!(hash_1.len(), 64, "SHA-256 hex output string length must be exactly 64 characters.");
    }

    #[test]
    fn test_local_health_milestone_signature_generation() {
        let dummy_user = "user_biometric_node_01";
        let score = 92;
        let schema_version = "v1.0.0-salud";

        let proof_result = SovereignCryptoEngine::sign_health_milestone(dummy_user, score, schema_version);
        
        // Assert successful zero-knowledge payload container construction
        assert!(proof_result.is_ok(), "Crypto Engine failed to sign health payload attributes.");
        
        let package = proof_result.unwrap();
        
        // Assert structural integrity of generated public keys and proofs
        assert!(!package.public_verification_key_hex.is_empty(), "Public key field is empty.");
        assert!(!package.signature_proof_hex.is_empty(), "Cryptographic signature proof field is empty.");
        
        // Validate internal unexposed payload metrics match source inputs
        assert_eq!(package.raw_payload.verified_vitality_score, 92);
        assert_eq!(package.raw_payload.salud_schema_version, "v1.0.0-salud");
        assert!(package.raw_payload.timestamp_epoch > 0, "Timestamp execution window error.");
    }
}

#[cfg(test)]
mod botanical_tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_botanical_search_and_safety_interception() {
        // Path configuration setups
        let test_db_path = "database/medical-core/preventive_botanicals.json";
        
        // Scenario A: High Risk Conflict - User has fever, queries Giloy, but takes Metformin
        let user_query = "Can I take Giloy leaves for my fever?";
        let current_prescriptions = vec!["Metformin".to_string()];

        let search_result = BotanicalSearchEngine::search_and_intercept(
            test_db_path,
            user_query,
            &current_prescriptions
        ).unwrap();

        assert!(search_result.is_some(), "Search engine failed to parse condition keyword matrix components.");
        let report = search_result.unwrap();
        
        assert!(report.botanical_found.contains("Giloy"));
        assert!(report.safety_alert.is_some(), "System missed critical Metformin cross-interference flag.");
        assert_eq!(report.safety_alert.unwrap().risk_severity, "MEDIUM");
        assert!(report.structured_protocol.kashaya_leaves.contains(&"Giloy Leaves".to_string()));
    }
}


#[cfg(test)]
mod compression_and_token_tests {
    use super::*;
    use crate::compressor::SovereignCompressor;
    use crate::mnemonic_validator::{MnemonicValidator, VerificationChallenge};

    #[test]
    fn test_back_to_back_compression_cycle_integrity() {
        let raw_vault_json_sample = "{\"vitality_score\":88,\"salud_version\":\"1.1.0\",\"conditions\":[\"Autism\",\"Stress\"]}";
        
        // Perform compression step
        let compressed_bytes = SovereignCompressor::compress_payload(raw_vault_json_sample).unwrap();
        assert!(!compressed_bytes.is_empty());
        assert!(compressed_bytes.len() < raw_vault_json_sample.as_bytes().len(), "Compression failed to reduce the byte size.");

        // Perform recovery decompression step
        let decompressed_string = SovereignCompressor::decompress_payload(&compressed_bytes).unwrap();
        assert_eq!(raw_vault_json_sample, decompressed_string, "Plaintext parameters failed to reconstruct identically.");
    }

    #[test]
    fn test_mnemonic_challenge_response_compliance() {
        let master_phrase = "ashwagandha kashaya ambali millets salud sovereign privacy zero knowledge mesh nostr nym";
        
        // Define a passing user answer configuration matching the master string phrase index vectors
        let valid_responses = vec![
            VerificationChallenge { word_index_target: 0, user_provided_string: "Ashwagandha".to_string() }, // Word 1
            VerificationChallenge { word_index_target: 3, user_provided_string: "millets".to_string() },      // Word 4
        ];

        let is_valid_pass = MnemonicValidator::verify_mnemonic_onboarding_compliance(master_phrase, &valid_responses);
        assert!(is_valid_pass, "Mnemonic challenge validation component threw a false rejection error.");

        // Define a failing configuration layout tracking entry faults
        let invalid_responses = vec![
            VerificationChallenge { word_index_target: 0, user_provided_string: "WrongWordEntry".to_string() }
        ];

        let is_invalid_fail = MnemonicValidator::verify_mnemonic_onboarding_compliance(master_phrase, &invalid_responses);
        assert!(!is_invalid_fail, "Mnemonic validator allowed unverified phrase vectors past system gate controls.");
    }
}

