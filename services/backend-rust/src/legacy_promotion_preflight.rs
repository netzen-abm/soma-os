use std::error::Error;

use sqlx::{PgPool, Row};

/// Read-only verification result used immediately before the final NOT NULL gate.
/// The adapter intentionally does not accept caller-supplied scope or filtering.
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
    /// The final gate is intentionally stricter than a simple zero-NULL check.
    /// Any partial scope or event/scope inconsistency blocks finalization.
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

/// Provider-neutral PostgreSQL adapter for the read-only preflight contract.
/// Authorization for invoking this operation is expected to have completed at
/// the canonical identity/Policy Kernel boundary.
pub struct LegacyPromotionPreflightExecutor {
    pool: PgPool,
}

impl LegacyPromotionPreflightExecutor {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
        }
    }

    pub async fn run(&self) -> Result<LegacyPromotionPreflight, Box<dyn Error>> {
        let row = sqlx::query(
            "SELECT total_rows, null_scope_rows, partial_scope_rows, fully_scoped_rows, scoped_without_applied_event, applied_event_scope_mismatches, eligible_unpromoted_rows, ambiguous_eligible_rows, preflight_passed FROM public.soma_legacy_promotion_preflight()",
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(LegacyPromotionPreflight {
            total_rows: row.get("total_rows"),
            null_scope_rows: row.get("null_scope_rows"),
            partial_scope_rows: row.get("partial_scope_rows"),
            fully_scoped_rows: row.get("fully_scoped_rows"),
            scoped_without_applied_event: row.get("scoped_without_applied_event"),
            applied_event_scope_mismatches: row.get("applied_event_scope_mismatches"),
            eligible_unpromoted_rows: row.get("eligible_unpromoted_rows"),
            ambiguous_eligible_rows: row.get("ambiguous_eligible_rows"),
            preflight_passed: row.get("preflight_passed"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use tokio::sync::OnceCell;

    const MIGRATIONS: &[&str] = &[
        include_str!("../../../database/migrations/0001_initialize_zk_logs.sql"),
        include_str!("../../../database/migrations/0003_protected_data_scope_transition.sql"),
        include_str!("../../../database/migrations/0004_scoped_anonymized_hash.sql"),
        include_str!("../../../database/migrations/0005_legacy_data_classification.sql"),
        include_str!("../../../database/migrations/0006_trusted_db_service_identity.sql"),
        include_str!("../../../database/migrations/0007_protected_data_rls_constraints.sql"),
        include_str!("../../../database/migrations/0008_legacy_data_promotion_events.sql"),
        include_str!("../../../database/migrations/0009_legacy_promotion_executor.sql"),
        include_str!("../../../database/migrations/0010_legacy_promotion_preflight.sql"),
    ];

    static PREPARED: OnceCell<()> = OnceCell::const_new();

    async fn prepare(pool: &PgPool) {
        PREPARED
            .get_or_init(|| async {
                for (index, migration) in MIGRATIONS.iter().enumerate() {
                    sqlx::raw_sql(migration)
                        .execute(pool)
                        .await
                        .expect("preflight migrations must apply cleanly");
                    if index == 3 {
                        for hash in ["lpf-a", "lpf-b", "lpf-c"] {
                            sqlx::query("INSERT INTO anonymized_user_vitals (anonymized_user_hash, public_verification_key_hex, verified_vitality_score, salud_schema_version, signature_proof_hex) VALUES ($1, 'pk', 7, '1.0', 'sig')")
                                .bind(hash)
                                .execute(pool)
                                .await
                                .expect("legacy fixture must be seeded before protected-scope constraints");
                        }
                    }
                }
            })
            .await;
    }

    #[test]
    fn passes_only_for_clean_zero_null_state() {
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
    fn rejects_partial_scope_even_without_null_pair() {
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
    async fn postgres_preflight_detects_null_scope() {
        let Some(url) = std::env::var("SOMA_TEST_DATABASE_URL").ok().filter(|v| !v.trim().is_empty()) else {
            return;
        };
        let pool =
            PgPoolOptions::new().max_connections(4).connect(&url).await.expect("test database must be reachable");
        prepare(&pool).await;

        let executor = LegacyPromotionPreflightExecutor::new(pool);
        let result = executor.run().await.expect("preflight function must be callable");
        assert!(result.null_scope_rows >= 3);
        assert!(!result.preflight_passed);
        assert!(result.require_pass().is_err());
    }
}
