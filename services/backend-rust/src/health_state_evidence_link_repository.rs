//! Governed persistence for cross-domain Health State ↔ Evidence Graph links.
//!
//! Domain authorization is performed by CanonicalAuthorizationBoundary before
//! this repository receives its context. The vault then independently authorizes
//! the concrete storage operation (health.vault.*). The two decisions are
//! intentionally distinct: the first governs the domain operation; the second
//! governs access to encrypted storage. Neither decision substitutes for the other.

use std::{path::Path, sync::Arc};

use thiserror::Error;

use crate::{
    canonical_authorization::CanonicalAuthorizationBoundary,
    health_state_evidence_link::{
        AuthorizedHealthStateEvidenceLinkContext, HealthStateEvidenceLink, HealthStateEvidenceLinkBoundary,
        HealthStateEvidenceLinkError,
    },
    local_health_vault::{LocalHealthVaultCrypto, VaultRecordMetadata},
    local_health_vault_storage::{
        AuthorizationContext, LocalFileVaultStore, StorageError, VaultAction, VaultAuthorizer, VaultKeyProvider,
    },
};

const ENTITY_TYPE: &str = "health_state_evidence_link";
const CONTENT_TYPE: &str = "application/json";

impl<K: VaultKeyProvider> VaultKeyProvider for Arc<K> {
    fn key_for(&self, key_ref: &str) -> Result<[u8; crate::local_health_vault_storage::KEY_LEN], StorageError> {
        self.as_ref().key_for(key_ref)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HealthStateEvidenceLinkRepositoryError {
    #[error("authorization denied")]
    AuthorizationDenied,
    #[error("invalid evidence-link context or operation")]
    InvalidContext,
    #[error("invalid evidence link")]
    InvalidLink,
    #[error("evidence-link serialization failed")]
    Serialization,
    #[error("vault cryptographic operation failed")]
    Cryptography,
    #[error("vault storage operation failed")]
    StorageFailure,
}

impl From<StorageError> for HealthStateEvidenceLinkRepositoryError {
    fn from(error: StorageError) -> Self {
        match error {
            StorageError::AuthorizationDenied => Self::AuthorizationDenied,
            StorageError::Serialization => Self::Serialization,
            StorageError::RecordIntegrityFailure | StorageError::IndexIntegrityFailure => Self::Cryptography,
            StorageError::NotFound
            | StorageError::Tombstoned
            | StorageError::InvalidRecord
            | StorageError::KeyResolutionFailed
            | StorageError::Io
            | StorageError::NonceGenerationFailed => Self::StorageFailure,
        }
    }
}

/// Persists only evidence-link records, using the shared encrypted Health Vault.
///
/// The domain context must have been minted by the canonical authorization
/// boundary. The vault performs its own storage authorization before I/O.
pub struct HealthStateEvidenceLinkRepository<K, A> {
    vault: LocalFileVaultStore<Arc<K>, A>,
    key_provider: Arc<K>,
    key_ref: String,
}

impl<K, A> HealthStateEvidenceLinkRepository<K, A>
where
    K: VaultKeyProvider,
    A: VaultAuthorizer,
{
    pub fn new(
        root: impl AsRef<Path>,
        key_provider: Arc<K>,
        authorizer: A,
        key_ref: impl Into<String>,
    ) -> Result<Self, HealthStateEvidenceLinkRepositoryError> {
        let key_ref = key_ref.into();
        if key_ref.trim().is_empty() || key_ref.chars().any(char::is_control) {
            return Err(HealthStateEvidenceLinkRepositoryError::InvalidContext);
        }
        let vault = LocalFileVaultStore::new(root, Arc::clone(&key_provider), authorizer)
            .map_err(HealthStateEvidenceLinkRepositoryError::from)?;
        Ok(Self {
            vault,
            key_provider,
            key_ref,
        })
    }

    fn vault_context(context: &AuthorizedHealthStateEvidenceLinkContext) -> AuthorizationContext {
        AuthorizationContext {
            principal_ref: context.principal_ref().to_owned(),
            subject_ref: context.subject_ref().to_owned(),
            scope: format!("{}:{}", context.tenant_id(), context.data_domain()),
        }
    }

    fn validate_operation(
        context: &AuthorizedHealthStateEvidenceLinkContext,
        resource_id: &str,
        action: &str,
    ) -> Result<(), HealthStateEvidenceLinkRepositoryError> {
        if context.principal_ref().trim().is_empty()
            || context.subject_ref().trim().is_empty()
            || context.tenant_id().trim().is_empty()
            || context.data_domain().trim().is_empty()
            || context.resource_type() != ENTITY_TYPE
            || context.resource_id() != resource_id
            || context.action() != action
        {
            return Err(HealthStateEvidenceLinkRepositoryError::InvalidContext);
        }
        Ok(())
    }

    pub fn put(
        &self,
        link: &HealthStateEvidenceLink,
        context: &AuthorizedHealthStateEvidenceLinkContext,
    ) -> Result<(), HealthStateEvidenceLinkRepositoryError> {
        Self::validate_operation(context, &link.id, "write")?;
        HealthStateEvidenceLinkBoundary::validate(link, context).map_err(|error| match error {
            HealthStateEvidenceLinkError::InvalidAuthorizationContext => {
                HealthStateEvidenceLinkRepositoryError::InvalidContext
            }
            _ => HealthStateEvidenceLinkRepositoryError::InvalidLink,
        })?;

        // A persisted link must have an explicit creation timestamp. Missing
        // provenance is not silently fabricated at the persistence boundary.
        let created_at = link
            .provenance
            .created_at
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .ok_or(HealthStateEvidenceLinkRepositoryError::InvalidLink)?;
        let plaintext = serde_json::to_vec(link).map_err(|_| HealthStateEvidenceLinkRepositoryError::Serialization)?;
        let key = self
            .key_provider
            .key_for(&self.key_ref)
            .map_err(|_| HealthStateEvidenceLinkRepositoryError::Cryptography)?;
        let provenance_ref = format!("evidence-link:{}", link.provenance.method);
        let record = LocalHealthVaultCrypto::encrypt_record(
            &key,
            VaultRecordMetadata {
                record_id: &link.id,
                subject_ref: context.subject_ref(),
                entity_type: ENTITY_TYPE,
                content_type: CONTENT_TYPE,
                key_ref: &self.key_ref,
                provenance_ref: &provenance_ref,
                created_at,
                updated_at: created_at,
            },
            &plaintext,
        )
        .map_err(|_| HealthStateEvidenceLinkRepositoryError::Cryptography)?;

        // The vault's own authorizer must independently allow health.vault.write.
        self.vault.put(record, &Self::vault_context(context)).map_err(HealthStateEvidenceLinkRepositoryError::from)
    }

    pub fn get(
        &self,
        record_id: &str,
        context: &AuthorizedHealthStateEvidenceLinkContext,
    ) -> Result<HealthStateEvidenceLink, HealthStateEvidenceLinkRepositoryError> {
        Self::validate_operation(context, record_id, "read")?;
        let record = self
            .vault
            .get(record_id, &Self::vault_context(context))
            .map_err(HealthStateEvidenceLinkRepositoryError::from)?;
        if record.entity_type != ENTITY_TYPE || record.content_type != CONTENT_TYPE {
            return Err(HealthStateEvidenceLinkRepositoryError::InvalidLink);
        }
        let key = self
            .key_provider
            .key_for(&record.key_ref)
            .map_err(|_| HealthStateEvidenceLinkRepositoryError::Cryptography)?;
        let plaintext = LocalHealthVaultCrypto::decrypt_record(&key, &record)
            .map_err(|_| HealthStateEvidenceLinkRepositoryError::Cryptography)?;
        let link: HealthStateEvidenceLink =
            serde_json::from_slice(&plaintext).map_err(|_| HealthStateEvidenceLinkRepositoryError::Serialization)?;
        if link.id != record_id {
            return Err(HealthStateEvidenceLinkRepositoryError::InvalidLink);
        }
        HealthStateEvidenceLinkBoundary::validate(&link, context)
            .map_err(|_| HealthStateEvidenceLinkRepositoryError::InvalidLink)?;
        Ok(link)
    }

    pub fn tombstone(
        &self,
        record_id: &str,
        context: &AuthorizedHealthStateEvidenceLinkContext,
    ) -> Result<(), HealthStateEvidenceLinkRepositoryError> {
        Self::validate_operation(context, record_id, "tombstone")?;
        self.vault
            .tombstone(record_id, &Self::vault_context(context))
            .map_err(HealthStateEvidenceLinkRepositoryError::from)
    }

    pub fn verify(
        &self,
        record_id: &str,
        context: &AuthorizedHealthStateEvidenceLinkContext,
    ) -> Result<(), HealthStateEvidenceLinkRepositoryError> {
        Self::validate_operation(context, record_id, "verify")?;
        self.vault
            .verify(record_id, &Self::vault_context(context))
            .map_err(HealthStateEvidenceLinkRepositoryError::from)
    }

    /// Documents the intended construction path at the type level: contexts
    /// must be minted after canonical authorization, never from caller strings.
    pub fn authorize_context<B: CanonicalAuthorizationBoundary>(
        boundary: &B,
        request: &crate::canonical_authorization::AuthorizationRequest,
    ) -> Result<AuthorizedHealthStateEvidenceLinkContext, crate::canonical_authorization::AuthorizationDecision> {
        boundary.authorize_health_state_evidence_link_context(request)
    }
}

#[cfg(test)]
#[path = "health_state_evidence_link_repository_tests.rs"]
mod tests;
