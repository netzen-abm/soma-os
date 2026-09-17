
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
