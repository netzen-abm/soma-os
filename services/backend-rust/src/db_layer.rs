use sqlx::{PgPool, Row};
use std::error::Error;

use crate::protected_db_context::{begin_protected_transaction, ProtectedDbContext};

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

    /// Append a protected vital-log record for an explicitly authorized tenant/data domain.
    ///
    /// The DB transaction context is established before SQL execution and is transaction-local.
    /// This method does not evaluate authorization; callers must provide a trusted persistence
    /// context produced after the canonical identity/authorization boundary.
    pub async fn append_anonymous_vital_log(
        &self,
        context: &ProtectedDbContext,
        user_hash: &str,
        pubkey_hex: &str,
        score: i32,
        schema_ver: &str,
        proof_hex: &str,
    ) -> Result<i32, Box<dyn Error>> {
        let mut tx = begin_protected_transaction(&self.pool, context).await?;

        let insert_query = r#"
            INSERT INTO anonymized_user_vitals
            (tenant_id, data_domain, anonymized_user_hash, public_verification_key_hex,
             verified_vitality_score, salud_schema_version, signature_proof_hex)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING log_id;
        "#;

        let row = sqlx::query(insert_query)
            .bind(&context.tenant_id)
            .bind(&context.data_domain)
            .bind(user_hash)
            .bind(pubkey_hex)
            .bind(score)
            .bind(schema_ver)
            .bind(proof_hex)
            .fetch_one(&mut *tx)
            .await?;

        let inserted_id: i32 = row.get("log_id");
        tx.commit().await?;
        Ok(inserted_id)
    }

    /// Fetch vital-log records only inside the explicitly supplied protected scope.
    ///
    /// The explicit scope predicates provide defense in depth even before PostgreSQL RLS is
    /// enabled. Legacy rows with NULL scope are therefore excluded from protected access.
    pub async fn get_logs_by_user_hash(
        &self,
        context: &ProtectedDbContext,
        user_hash: &str,
    ) -> Result<Vec<DbVitalRecord>, Box<dyn Error>> {
        let mut tx = begin_protected_transaction(&self.pool, context).await?;

        let select_query = r#"
            SELECT log_id, anonymized_user_hash, public_verification_key_hex,
                   verified_vitality_score, salud_schema_version, signature_proof_hex
            FROM anonymized_user_vitals
            WHERE anonymized_user_hash = $1
              AND tenant_id = $2
              AND data_domain = $3
            ORDER BY created_at DESC;
        "#;

        let rows = sqlx::query_as::<_, DbVitalRecord>(select_query)
            .bind(user_hash)
            .bind(&context.tenant_id)
            .bind(&context.data_domain)
            .fetch_all(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(rows)
    }
}

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
