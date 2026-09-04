use sqlx::{postgres::Postgres, PgPool, Transaction};
use std::error::Error;

/// Trusted persistence scope established only after the canonical authorization boundary.
///
/// This type carries the tenant/data-domain binding required by the PostgreSQL adapter,
/// but it does not evaluate policy or infer scope from caller metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtectedDbContext {
    pub tenant_id: String,
    pub data_domain: String,
}

impl ProtectedDbContext {
    pub fn new(tenant_id: impl Into<String>, data_domain: impl Into<String>) -> Result<Self, Box<dyn Error>> {
        let tenant_id = tenant_id.into();
        let data_domain = data_domain.into();

        if tenant_id.trim().is_empty() || data_domain.trim().is_empty() {
            return Err("protected database scope must be non-empty".into());
        }

        if tenant_id.chars().any(char::is_control) || data_domain.chars().any(char::is_control) {
            return Err("protected database scope contains control characters".into());
        }

        Ok(Self {
            tenant_id,
            data_domain,
        })
    }
}

/// Begin a PostgreSQL transaction and bind the protected scope for this transaction only.
///
/// `set_config(..., true)` is transaction-local. This is mandatory with pooled connections:
/// scope must never leak from one request/transaction into a later borrower of the connection.
/// This helper is a persistence boundary, not an authorization evaluator.
pub async fn begin_protected_transaction<'a>(
    pool: &'a PgPool,
    context: &ProtectedDbContext,
) -> Result<Transaction<'a, Postgres>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    if let Err(error) =
        sqlx::query("SELECT set_config('soma.tenant_id', $1, true)").bind(&context.tenant_id).execute(&mut *tx).await
    {
        let _ = tx.rollback().await;
        return Err(error);
    }

    if let Err(error) = sqlx::query("SELECT set_config('soma.data_domain', $1, true)")
        .bind(&context.data_domain)
        .execute(&mut *tx)
        .await
    {
        let _ = tx.rollback().await;
        return Err(error);
    }

    Ok(tx)
}

#[cfg(test)]
mod tests {
    use super::{begin_protected_transaction, ProtectedDbContext};
    use sqlx::{postgres::PgPoolOptions, PgPool, Row};

    async fn pool(max_connections: u32) -> Option<PgPool> {
        let url = std::env::var("DATABASE_URL").ok().filter(|value| !value.trim().is_empty())?;
        PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&url)
            .await
            .ok()
    }

    #[test]
    fn accepts_valid_scope() {
        let context = ProtectedDbContext::new("tenant-a", "personal-health").unwrap();
        assert_eq!(context.tenant_id, "tenant-a");
        assert_eq!(context.data_domain, "personal-health");
    }

    #[test]
    fn rejects_empty_scope() {
        assert!(ProtectedDbContext::new("", "personal-health").is_err());
        assert!(ProtectedDbContext::new("tenant-a", "").is_err());
    }

    #[test]
    fn rejects_control_characters() {
        assert!(ProtectedDbContext::new("tenant\n", "personal-health").is_err());
        assert!(ProtectedDbContext::new("tenant-a", "personal\thealth").is_err());
    }

    #[tokio::test]
    async fn transaction_scope_is_bound_and_not_leaked_after_commit() {
        let Some(pool) = pool(1).await else { return };
        let context_a = ProtectedDbContext::new("tenant-a", "domain-a").unwrap();
        let context_b = ProtectedDbContext::new("tenant-b", "domain-b").unwrap();

        let mut tx_a = begin_protected_transaction(&pool, &context_a).await.unwrap();
        let row = sqlx::query("SELECT current_setting('soma.tenant_id'), current_setting('soma.data_domain')")
            .fetch_one(&mut *tx_a)
            .await
            .unwrap();
        assert_eq!(row.get::<String, _>(0), "tenant-a");
        assert_eq!(row.get::<String, _>(1), "domain-a");
        tx_a.commit().await.unwrap();

        let mut tx_b = begin_protected_transaction(&pool, &context_b).await.unwrap();
        let row = sqlx::query("SELECT current_setting('soma.tenant_id'), current_setting('soma.data_domain')")
            .fetch_one(&mut *tx_b)
            .await
            .unwrap();
        assert_eq!(row.get::<String, _>(0), "tenant-b");
        assert_eq!(row.get::<String, _>(1), "domain-b");
        tx_b.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn transaction_scope_does_not_leak_after_rollback() {
        let Some(pool) = pool(1).await else { return };
        let context_a = ProtectedDbContext::new("tenant-a", "domain-a").unwrap();
        let context_b = ProtectedDbContext::new("tenant-b", "domain-b").unwrap();

        let mut tx_a = begin_protected_transaction(&pool, &context_a).await.unwrap();
        let row = sqlx::query("SELECT current_setting('soma.tenant_id')")
            .fetch_one(&mut *tx_a)
            .await
            .unwrap();
        assert_eq!(row.get::<String, _>(0), "tenant-a");
        tx_a.rollback().await.unwrap();

        let mut tx_b = begin_protected_transaction(&pool, &context_b).await.unwrap();
        let row = sqlx::query("SELECT current_setting('soma.tenant_id')")
            .fetch_one(&mut *tx_b)
            .await
            .unwrap();
        assert_eq!(row.get::<String, _>(0), "tenant-b");
        tx_b.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn concurrent_transactions_keep_scopes_isolated() {
        let Some(pool) = pool(2).await else { return };
        let context_a = ProtectedDbContext::new("tenant-a", "domain-a").unwrap();
        let context_b = ProtectedDbContext::new("tenant-b", "domain-b").unwrap();

        let (tx_a, tx_b) = tokio::join!(
            begin_protected_transaction(&pool, &context_a),
            begin_protected_transaction(&pool, &context_b)
        );
        let mut tx_a = tx_a.unwrap();
        let mut tx_b = tx_b.unwrap();

        let (row_a, row_b) = tokio::join!(
            sqlx::query("SELECT current_setting('soma.tenant_id'), current_setting('soma.data_domain')").fetch_one(&mut *tx_a),
            sqlx::query("SELECT current_setting('soma.tenant_id'), current_setting('soma.data_domain')").fetch_one(&mut *tx_b)
        );

        let row_a = row_a.unwrap();
        let row_b = row_b.unwrap();
        assert_eq!(row_a.get::<String, _>(0), "tenant-a");
        assert_eq!(row_a.get::<String, _>(1), "domain-a");
        assert_eq!(row_b.get::<String, _>(0), "tenant-b");
        assert_eq!(row_b.get::<String, _>(1), "domain-b");

        tx_a.rollback().await.unwrap();
        tx_b.rollback().await.unwrap();
    }
}
