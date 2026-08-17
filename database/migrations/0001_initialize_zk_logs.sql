-- SomaOS Production Migration File
-- File Path: database/migrations/0001_initialize_zk_logs.sql

CREATE TABLE IF NOT EXISTS anonymized_user_vitals (
    -- Unique internal identification seed (Completely unlinked to human names or phones)
    log_id SERIAL PRIMARY KEY,
    
    -- Pseudonymous SHA-256 string footprints verifying the client node identity safely
    anonymized_user_hash CHAR(64) NOT NULL UNIQUE,
    
    -- Hex encoded public key representation used to evaluate cryptographic validation proofs
    public_verification_key_hex VARCHAR(130) NOT NULL,
    
    -- Salud Protocol verified milestone value thresholds (Stored numbers, no identities linked)
    verified_vitality_score INT CHECK (verified_vitality_score >= 0 AND verified_vitality_score <= 100),
    
    -- Current standard string label matching system health schemas
    salud_schema_version VARCHAR(24) NOT NULL,
    
    -- Cryptographic zk signature payload output proof generated locally on consumer device
    signature_proof_hex TEXT NOT NULL,
    
    -- Server insertion tracking timestamp block
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Establish indices over the anonymized hashes to accelerate search queries
CREATE INDEX IF NOT EXISTS idx_user_crypto_hash ON anonymized_user_vitals(anonymized_user_hash);

COMMENT ON TABLE anonymized_user_vitals IS 'Main storage handling pseudonymous health metric proofs with absolute zero identity data tracking metrics.';
