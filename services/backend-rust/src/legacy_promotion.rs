use sqlx::{PgPool, Row};
use std::error::Error;

/// Provider-neutral request for an explicitly verified legacy-data promotion.
///
/// Tenant/data-domain are deliberately absent: protected scope must be derived by
/// the database executor from the authoritative verified classification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyPromotionRequest {
    pub legacy_log_id: i32,
    pub classification_id: i64,
    pub promotion_event_reference: String,
    pub authoritative_provenance: String,
    pub initiated_by: String,
}

impl LegacyPromotionRequest {
    pub fn validate(&self) -> Result<(), Box<dyn Error>> {
        if self.legacy_log_id <= 0 {
            return Err("legacy_log_id must be positive".into());
        }
        if self.classification_id <= 0 {
            return Err("classification_id must be positive".into());
        }

        for (value, field) in [
            (&self.promotion_event_reference, "promotion_event_reference"),
            (&self.authoritative_provenance, "authoritative_provenance"),
            (&self.initiated_by, "initiated_by"),
        ] {
            if value.trim().is_empty() {
                return Err(format!("{field} must be non-empty").into());
            }
            if value.chars().any(char::is_control) {
                return Err(format!("{field} contains control characters").into());
            }
        }

        Ok(())
    }
}

/// Result returned by the atomic PostgreSQL promotion executor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyPromotionResult {
    pub promotion_event_id: i64,
}

/// PostgreSQL adapter for the provider-neutral legacy promotion contract.
///
/// Authorization is expected to have completed before this adapter is invoked.
/// The adapter does not infer scope and does not accept caller-supplied tenant or
/// data-domain values. PostgreSQL derives scope from the verified classification.
pub struct LegacyPromotionExecutor {
    pool: PgPool,
}

impl LegacyPromotionExecutor {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn promote(
        &self,
        request: &LegacyPromotionRequest,
    ) -> Result<LegacyPromotionResult, Box<dyn Error>> {
        request.validate()?;

        let row = sqlx::query(
            "SELECT public.soma_apply_legacy_promotion($1, $2, $3, $4, $5) AS promotion_event_id",
        )
        .bind(request.legacy_log_id)
        .bind(request.classification_id)
        .bind(&request.promotion_event_reference)
        .bind(&request.authoritative_provenance)
        .bind(&request.initiated_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(LegacyPromotionResult {
            promotion_event_id: row.get("promotion_event_id"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::LegacyPromotionRequest;

    fn valid_request() -> LegacyPromotionRequest {
        LegacyPromotionRequest {
            legacy_log_id: 1,
            classification_id: 1,
            promotion_event_reference: "promotion-1".to_string(),
            authoritative_provenance: "prov-1".to_string(),
            initiated_by: "migration-operator-1".to_string(),
        }
    }

    #[test]
    fn validates_provider_neutral_request() {
        assert!(valid_request().validate().is_ok());
    }

    #[test]
    fn rejects_invalid_ids() {
        assert!(LegacyPromotionRequest {
            legacy_log_id: 0,
            ..valid_request()
        }
        .validate()
        .is_err());
        assert!(LegacyPromotionRequest {
            classification_id: 0,
            ..valid_request()
        }
        .validate()
        .is_err());
    }

    #[test]
    fn rejects_empty_metadata() {
        assert!(LegacyPromotionRequest {
            promotion_event_reference: "".to_string(),
            ..valid_request()
        }
        .validate()
        .is_err());
        assert!(LegacyPromotionRequest {
            authoritative_provenance: " ".to_string(),
            ..valid_request()
        }
        .validate()
        .is_err());
        assert!(LegacyPromotionRequest {
            initiated_by: "\n".to_string(),
            ..valid_request()
        }
        .validate()
        .is_err());
    }

    #[test]
    fn request_does_not_carry_caller_supplied_scope() {
        let request = valid_request();
        let debug = format!("{request:?}");
        assert!(!debug.contains("tenant_id"));
        assert!(!debug.contains("data_domain"));
    }
}
