use sqlx::{postgres::Postgres, PgPool, Transaction};

/// Canonical database service identity expected by the protected persistence adapter.
pub const TRUSTED_PERSISTENCE_DB_ROLE: &str = "somaos_persistence";

/// Protected persistence scope that has crossed the canonical authorization boundary.
///
/// The fields are intentionally private. External callers cannot construct this type
/// from arbitrary tenant/data-domain strings; the canonical authorization boundary
/// mints it only after an authoritative `ALLOW` decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedProtectedDbContext {
    tenant_id: String,
    data_domain: String,
}

impl AuthorizedProtectedDbContext {
    pub(crate) fn from_authorized_request(request: &crate::canonical_authorization::AuthorizationRequest) -> Self {
        Self {
            tenant_id: request.tenant_id.clone(),
            data_domain: request.data_domain.clone(),
        }
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn data_domain(&self) -> &str {
        &self.data_domain
    }
}

/// Begin a PostgreSQL transaction and establish protected scope through the
/// dedicated persistence database identity boundary.
///
/// This function accepts only an authorization-bound context. It therefore cannot
/// be used as the authorization decision point and cannot mint its own scope.
/// Production deployments must connect with a LOGIN role that is explicitly
/// granted membership in `somaos_persistence`, then this adapter switches into
/// that NOLOGIN role for the transaction.
pub async fn begin_protected_transaction<'a>(
    pool: &'a PgPool,
    context: &AuthorizedProtectedDbContext,
) -> Result<Transaction<'a, Postgres>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    if let Err(error) = sqlx::query("SET LOCAL ROLE somaos_persistence").execute(&mut *tx).await {
        let _ = tx.rollback().await;
        return Err(error);
    }

    if let Err(error) = sqlx::query("SELECT public.soma_set_protected_scope($1, $2)")
        .bind(context.tenant_id())
        .bind(context.data_domain())
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
    use super::{AuthorizedProtectedDbContext, TRUSTED_PERSISTENCE_DB_ROLE};

    #[test]
    fn canonical_persistence_role_is_explicit() {
        assert_eq!(TRUSTED_PERSISTENCE_DB_ROLE, "somaos_persistence");
    }

    #[test]
    fn context_fields_are_read_only_after_authorization() {
        let request = crate::canonical_authorization::AuthorizationRequest {
            principal_ref: "principal-1".into(),
            capability_id: "health.read".into(),
            resource_type: "health_record".into(),
            resource_id: "record-1".into(),
            action: "read".into(),
            tenant_id: "tenant-a".into(),
            data_domain: "personal-health".into(),
        };
        let context = AuthorizedProtectedDbContext::from_authorized_request(&request);
        assert_eq!(context.tenant_id(), "tenant-a");
        assert_eq!(context.data_domain(), "personal-health");
    }
}
