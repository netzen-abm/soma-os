use sqlx::{postgres::Postgres, PgPool, Transaction};
use std::error::Error;

/// Canonical database service identity expected by the protected persistence adapter.
pub const TRUSTED_PERSISTENCE_DB_ROLE: &str = "somaos_persistence";

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
    pub fn new(
        tenant_id: impl Into<String>,
        data_domain: impl Into<String>,
    ) -> Result<Self, Box<dyn Error>> {
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

/// Begin a PostgreSQL transaction and establish protected scope through the
/// dedicated persistence database identity boundary.
///
/// Production deployments must connect with a LOGIN role that is explicitly
/// granted membership in `somaos_persistence`, then this adapter switches into
/// that NOLOGIN role for the transaction. The trusted SQL entry point remains
/// SECURITY INVOKER and therefore observes `current_user = somaos_persistence`.
///
/// `soma_set_protected_scope` is deliberately SECURITY INVOKER and executable only
/// by `somaos_persistence`. Therefore the transaction-local GUCs are isolation state,
/// not an authentication mechanism. An untrusted DB principal cannot invoke the
/// canonical entry point merely by knowing its SQL name.
///
/// This helper remains a persistence boundary, not an authorization evaluator.
pub async fn begin_protected_transaction<'a>(
    pool: &'a PgPool,
    context: &ProtectedDbContext,
) -> Result<Transaction<'a, Postgres>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    if let Err(error) = sqlx::query("SET LOCAL ROLE somaos_persistence").execute(&mut *tx).await {
        let _ = tx.rollback().await;
        return Err(error);
    }

    if let Err(error) = sqlx::query("SELECT public.soma_set_protected_scope($1, $2)")
        .bind(&context.tenant_id)
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
    use super::{ProtectedDbContext, TRUSTED_PERSISTENCE_DB_ROLE};

    #[test]
    fn canonical_persistence_role_is_explicit() {
        assert_eq!(TRUSTED_PERSISTENCE_DB_ROLE, "somaos_persistence");
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
}
