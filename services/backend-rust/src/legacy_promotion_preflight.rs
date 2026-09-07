use std::error::Error;

use sqlx::{PgPool, Postgres, Row, Transaction};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyPromotionPreflight {
    pub total_rows: i64,
    pub null_scope_rows: i64,
    pub partial_scope_rows: i64,
    pub fully_scoped_rows: i64,
    pub scoped_without_applied_event: i64,
    pub applied_event_scope_mismatches: i64,
    pub eligible_unpromoted_rows: i64,
    pub ambiguous_eligible_rows: i64,
    pub preflight_passed: bool,
}

impl LegacyPromotionPreflight {
    pub fn require_pass(&self) -> Result<(), Box<dyn Error>> {
        if self.preflight_passed
            && self.null_scope_rows == 0
            && self.partial_scope_rows == 0
            && self.applied_event_scope_mismatches == 0
        {
            return Ok(());
        }
        Err(format!(
            "legacy promotion preflight failed: null_scope_rows={}, partial_scope_rows={}, applied_event_scope_mismatches={}, eligible_unpromoted_rows={}, ambiguous_eligible_rows={}",
            self.null_scope_rows,
            self.partial_scope_rows,
            self.applied_event_scope_mismatches,
            self.eligible_unpromoted_rows,
            self.ambiguous_eligible_rows,
        )
        .into())
    }
}

pub struct LegacyPromotionPreflightExecutor {
    pool: PgPool,
}

impl LegacyPromotionPreflightExecutor {
    /// Constructs the provider-neutral preflight adapter.
    ///
    /// Only the trusted persistence pool is accepted. Scope is intentionally
    /// absent because preflight is a global, read-only verification gate.
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
        }
    }

    /// Runs the canonical database preflight without accepting caller-supplied
    /// scope or metadata that could influence the result.
    pub async fn run(&self) -> Result<LegacyPromotionPreflight, Box<dyn Error>> {
        let row = sqlx::query(
            "SELECT total_rows, null_scope_rows, partial_scope_rows, fully_scoped_rows, scoped_without_applied_event, applied_event_scope_mismatches, eligible_unpromoted_rows, ambiguous_eligible_rows, preflight_passed FROM public.soma_legacy_promotion_preflight()",
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Self::from_row(row))
    }

    /// Runs the same canonical preflight on an existing transaction.
    ///
    /// This is useful for atomic verification of a transaction-local database
    /// state; it does not accept caller-supplied scope or mutation metadata.
    pub async fn run_in_transaction<'a>(
        &self,
        tx: &mut Transaction<'a, Postgres>,
    ) -> Result<LegacyPromotionPreflight, Box<dyn Error>> {
        let row = sqlx::query(
            "SELECT total_rows, null_scope_rows, partial_scope_rows, fully_scoped_rows, scoped_without_applied_event, applied_event_scope_mismatches, eligible_unpromoted_rows, ambiguous_eligible_rows, preflight_passed FROM public.soma_legacy_promotion_preflight()",
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(Self::from_row(row))
    }

    fn from_row(row: sqlx::postgres::PgRow) -> LegacyPromotionPreflight {
        LegacyPromotionPreflight {
            total_rows: row.get("total_rows"),
            null_scope_rows: row.get("null_scope_rows"),
            partial_scope_rows: row.get("partial_scope_rows"),
            fully_scoped_rows: row.get("fully_scoped_rows"),
            scoped_without_applied_event: row.get("scoped_without_applied_event"),
            applied_event_scope_mismatches: row.get("applied_event_scope_mismatches"),
            eligible_unpromoted_rows: row.get("eligible_unpromoted_rows"),
            ambiguous_eligible_rows: row.get("ambiguous_eligible_rows"),
            preflight_passed: row.get("preflight_passed"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    #[test]
    fn accepts_only_clean_zero_null_state() {
        let result = LegacyPromotionPreflight {
            total_rows: 10,
            null_scope_rows: 0,
            partial_scope_rows: 0,
            fully_scoped_rows: 10,
            scoped_without_applied_event: 0,
            applied_event_scope_mismatches: 0,
            eligible_unpromoted_rows: 0,
            ambiguous_eligible_rows: 0,
            preflight_passed: true,
        };
        assert!(result.require_pass().is_ok());
    }

    #[test]
    fn rejects_any_null_scope() {
        let result = LegacyPromotionPreflight {
            total_rows: 10,
            null_scope_rows: 1,
            partial_scope_rows: 0,
            fully_scoped_rows: 9,
            scoped_without_applied_event: 0,
            applied_event_scope_mismatches: 0,
            eligible_unpromoted_rows: 1,
            ambiguous_eligible_rows: 0,
            preflight_passed: false,
        };
        assert!(result.require_pass().is_err());
    }

    #[test]
    fn rejects_partial_scope() {
        let result = LegacyPromotionPreflight {
            total_rows: 10,
            null_scope_rows: 0,
            partial_scope_rows: 1,
            fully_scoped_rows: 9,
            scoped_without_applied_event: 1,
            applied_event_scope_mismatches: 0,
            eligible_unpromoted_rows: 0,
            ambiguous_eligible_rows: 0,
            preflight_passed: false,
        };
        assert!(result.require_pass().is_err());
    }

    #[test]
    fn rejects_event_scope_mismatch() {
        let result = LegacyPromotionPreflight {
            total_rows: 10,
            null_scope_rows: 0,
            partial_scope_rows: 0,
            fully_scoped_rows: 10,
            scoped_without_applied_event: 0,
            applied_event_scope_mismatches: 1,
            eligible_unpromoted_rows: 0,
            ambiguous_eligible_rows: 0,
            preflight_passed: false,
        };
        assert!(result.require_pass().is_err());
    }

    #[test]
    fn request_is_read_only_and_has_no_scope_parameters() {
        let source = include_str!("legacy_promotion_preflight.rs");
        let production_source = source.split("#[cfg(test)]").next().unwrap_or(source);
        assert!(production_source.contains("soma_legacy_promotion_preflight()"));
        assert!(!production_source.contains("tenant_id: String"));
        assert!(!production_source.contains("data_domain: String"));
        assert!(!production_source.contains("UPDATE public.anonymized_user_vitals"));
    }

    #[tokio::test]
    async fn postgres_preflight_detects_null_scope_transactionally() {
        let url = match std::env::var("SOMA_TEST_DATABASE_URL") {
            Ok(value) if !value.trim().is_empty() => value,
            _ => return,
        };
        let pool =
            PgPoolOptions::new().max_connections(4).connect(&url).await.expect("test database must be reachable");
        let mut tx = pool.begin().await.expect("test transaction must begin");

        sqlx::query("ALTER TABLE public.anonymized_user_vitals DROP CONSTRAINT IF EXISTS anonymized_user_vitals_protected_scope_required")
            .execute(&mut *tx)
            .await
            .expect("test must be able to create a transactional legacy NULL-scope fixture");
        sqlx::query(
            "INSERT INTO public.anonymized_user_vitals (anonymized_user_hash, public_verification_key_hex, verified_vitality_score, salud_schema_version, signature_proof_hex) VALUES ('lpf-a', 'pk', 7, '1.0', 'sig')",
        )
        .execute(&mut *tx)
        .await
        .expect("legacy NULL-scope fixture must be seedable inside the test transaction");

        let executor = LegacyPromotionPreflightExecutor::new(pool.clone());
        let result = executor
            .run_in_transaction(&mut tx)
            .await
            .expect("preflight function must be callable inside the fixture transaction");
        assert!(result.null_scope_rows >= 1);
        assert_eq!(result.partial_scope_rows, 0);
        assert!(!result.preflight_passed);
        assert!(result.require_pass().is_err());

        tx.rollback().await.expect("fixture transaction must roll back cleanly");
    }
}
