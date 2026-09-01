use sqlx::{PgPool, Row};
use std::error::Error;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DbVitalRecord {
    pub log_id: i32,
    pub anonymized_user_hash: String,
    pub public_verification_key_hex: String,
    pub verified_vitality_score: i32,
    pub salud_schema_version: String,
    pub signature_proof_hex: String,
}

pub struct SomaDatabaseManager {
    pool: PgPool,
}

impl SomaDatabaseManager {
    pub fn new(connection_pool: PgPool) -> Self {
        Self {
            pool: connection_pool,
        }
    }

    // Insert a new anonymized zero-knowledge validation state verification entry into target persistence engine
    pub async fn append_anonymous_vital_log(
        &self,
        user_hash: &str,
        pubkey_hex: &str,
        score: i32,
        schema_ver: &str,
        proof_hex: &str,
    ) -> Result<i32, Box<dyn Error>> {
        let insert_query = r#"
            INSERT INTO anonymized_user_vitals
            (anonymized_user_hash, public_verification_key_hex, verified_vitality_score, salud_schema_version, signature_proof_hex)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING log_id;
        "#;

        let row = sqlx::query(insert_query)
            .bind(user_hash)
            .bind(pubkey_hex)
            .bind(score)
            .bind(schema_ver)
            .bind(proof_hex)
            .fetch_one(&self.pool)
            .await?;

        let inserted_id: i32 = row.get("log_id");
        Ok(inserted_id)
    }

    // Fetch historical metric records for an explicit tracking signature safely without index identification exposure
    pub async fn get_logs_by_user_hash(&self, user_hash: &str) -> Result<Vec<DbVitalRecord>, Box<dyn Error>> {
        let select_query = r#"
            SELECT log_id, anonymized_user_hash, public_verification_key_hex, verified_vitality_score, salud_schema_version, signature_proof_hex
            FROM anonymized_user_vitals
            WHERE anonymized_user_hash = $1
            ORDER BY created_at DESC;
        "#;

        let rows = sqlx::query_as::<_, DbVitalRecord>(select_query) // Requires derivation features enabled inside Cargo configs
            .bind(user_hash)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }
}

// Implement mock manual row mapper manual conversions to accommodate custom trait binding targets
impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for DbVitalRecord {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(DbVitalRecord {
            log_id: row.try_get("log_id")?,
            anonymized_user_hash: row.try_get("anonymized_user_hash")?,
            public_verification_key_hex: row.try_get("public_verification_key_hex")?,
            verified_vitality_score: row.try_get("verified_vitality_score")?,
            salud_schema_version: row.try_get("salud_schema_version")?,
            signature_proof_hex: row.try_get("signature_proof_hex")?,
        })
    }
}
