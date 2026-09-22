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
#[path = "legacy_promotion_preflight_tests.rs"]
mod tests;
