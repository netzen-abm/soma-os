use sqlx::{postgres::Postgres, PgPool, Transaction};

/// Canonical database service identity expected by the protected persistence adapter.
pub const TRUSTED_PERSISTENCE_DB_ROLE: &str = "somaos_persistence";

/// Protected persistence scope that has crossed the canonical authorization boundary.
///
/// The fields are intentionally private. External callers cannot construct this type
/// from arbitrary tenant/data-domain strings; the canonical authorization boundary
/// mints it only after an authoritative `ALLOW` decision and request validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedProtectedDbContext {
    principal_ref: String,
    subject_ref: String,
    capability_id: String,
    capability_version: String,
    resource_type: String,
    resource_id: String,
    action: String,
    tenant_id: String,
    data_domain: String,
}

impl AuthorizedProtectedDbContext {
    pub(crate) fn from_authorized_request(
        request: &crate::canonical_authorization::AuthorizationRequest,
    ) -> Result<Self, &'static str> {
        let fields = [
            request.principal_ref.as_str(),
            request.subject_ref.as_str(),
            request.capability_id.as_str(),
            request.resource_type.as_str(),
            request.resource_id.as_str(),
            request.action.as_str(),
            request.tenant_id.as_str(),
            request.data_domain.as_str(),
        ];
        if fields.iter().any(|field| field.trim().is_empty()) {
            return Err("protected authorization request contains an empty field");
        }
        if fields.iter().any(|field| field.chars().any(char::is_control)) {
            return Err("protected authorization request contains control characters");
        }

        Ok(Self {
            principal_ref: request.principal_ref.clone(),
            subject_ref: request.subject_ref.clone(),
            capability_id: request.capability_id.clone(),
            capability_version: request.capability_version.clone(),
            resource_type: request.resource_type.clone(),
            resource_id: request.resource_id.clone(),
            action: request.action.clone(),
            tenant_id: request.tenant_id.clone(),
            data_domain: request.data_domain.clone(),
        })
    }

    pub fn principal_ref(&self) -> &str {
        &self.principal_ref
    }

    pub fn subject_ref(&self) -> &str {
        &self.subject_ref
    }

    pub fn capability_id(&self) -> &str {
        &self.capability_id
    }
    pub fn capability_version(&self) -> &str {
        &self.capability_version
    }

    pub fn resource_type(&self) -> &str {
        &self.resource_type
    }

    pub fn resource_id(&self) -> &str {
        &self.resource_id
    }

    pub fn action(&self) -> &str {
        &self.action
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
    fn context_preserves_authorized_request_scope_read_only() {
        let request = crate::canonical_authorization::AuthorizationRequest {
            principal_ref: "principal-1".into(),
            subject_ref: "person-1".into(),
            capability_id: "health.read".into(),
            capability_version: "1.0.0".into(),
            capability_version: "1.0.0".into(),
            resource_type: "health_record".into(),
            resource_id: "record-1".into(),
            action: "read".into(),
            tenant_id: "tenant-a".into(),
            data_domain: "personal-health".into(),
        };
        let context = AuthorizedProtectedDbContext::from_authorized_request(&request).unwrap();
        assert_eq!(context.principal_ref(), "principal-1");
        assert_eq!(context.subject_ref(), "person-1");
        assert_eq!(context.capability_id(), "health.read");
        assert_eq!(context.resource_type(), "health_record");
        assert_eq!(context.resource_id(), "record-1");
        assert_eq!(context.action(), "read");
        assert_eq!(context.tenant_id(), "tenant-a");
        assert_eq!(context.data_domain(), "personal-health");
    }

    #[test]
    fn rejects_empty_or_control_character_scope() {
        let mut request = crate::canonical_authorization::AuthorizationRequest {
            principal_ref: "principal-1".into(),
            subject_ref: "person-1".into(),
            capability_id: "health.read".into(),
            resource_type: "health_record".into(),
            resource_id: "record-1".into(),
            action: "read".into(),
            tenant_id: "tenant-a".into(),
            data_domain: "personal-health".into(),
        };
        request.action.clear();
        assert!(AuthorizedProtectedDbContext::from_authorized_request(&request).is_err());
        request.action = "read\n".into();
        assert!(AuthorizedProtectedDbContext::from_authorized_request(&request).is_err());
    }
}
