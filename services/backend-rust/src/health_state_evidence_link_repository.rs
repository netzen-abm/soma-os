//! Governed Health State ↔ Evidence Graph link persistence through the shared encrypted vault.

use std::{path::Path, sync::Arc};

#[path = "health_state_evidence_link_repository_contract.rs"]
mod contract;
pub use contract::HealthStateEvidenceLinkRepositoryError;

use crate::{
    health_state_evidence_link::{
        AuthorizedHealthStateEvidenceLinkContext, HealthStateEvidenceLink, HealthStateEvidenceLinkBoundary,
        HealthStateEvidenceLinkError,
    },
    local_health_vault::{LocalHealthVaultCrypto, VaultRecordMetadata},
    local_health_vault_storage::{
        AuthorizationContext, LocalFileVaultStore, StorageError, VaultAuthorizer, VaultKeyProvider,
    },
};

const ENTITY_TYPE: &str = "health_state_evidence_link";
const CONTENT_TYPE: &str = "application/json";

impl<K: VaultKeyProvider> VaultKeyProvider for Arc<K> {
    fn key_for(&self, key_ref: &str) -> Result<[u8; crate::local_health_vault_storage::KEY_LEN], StorageError> {
        self.as_ref().key_for(key_ref)
    }
}

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

    pub fn put(
        &self,
        link: &HealthStateEvidenceLink,
        context: &AuthorizedHealthStateEvidenceLinkContext,
    ) -> Result<(), HealthStateEvidenceLinkRepositoryError> {
        if !context.matches_operation(ENTITY_TYPE, &link.id, "write") {
            return Err(HealthStateEvidenceLinkRepositoryError::InvalidContext);
        }
        HealthStateEvidenceLinkBoundary::validate(link, context).map_err(|error| match error {
            HealthStateEvidenceLinkError::InvalidAuthorizationContext => {
                HealthStateEvidenceLinkRepositoryError::InvalidContext
            }
            _ => HealthStateEvidenceLinkRepositoryError::InvalidLink,
        })?;

        let created_at = link
            .provenance
            .created_at
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .ok_or(HealthStateEvidenceLinkRepositoryError::InvalidLink)?;
        let provenance_ref = format!("evidence-link:{}", link.provenance.method);
        self.vault.put_with(&link.id, context.subject_ref(), &self.key_ref, &Self::vault_context(context), |key| {
            let plaintext =
                serde_json::to_vec(link).map_err(|_| HealthStateEvidenceLinkRepositoryError::Serialization)?;
            LocalHealthVaultCrypto::encrypt_record(
                key,
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
            .map_err(|_| HealthStateEvidenceLinkRepositoryError::Cryptography)
        })
        .map_err(HealthStateEvidenceLinkRepositoryError::from)
    }

    pub fn get(
        &self,
        record_id: &str,
        context: &AuthorizedHealthStateEvidenceLinkContext,
    ) -> Result<HealthStateEvidenceLink, HealthStateEvidenceLinkRepositoryError> {
        if !context.matches_operation(ENTITY_TYPE, record_id, "read") {
            return Err(HealthStateEvidenceLinkRepositoryError::InvalidContext);
        }
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
        if !context.matches_operation(ENTITY_TYPE, record_id, "tombstone") {
            return Err(HealthStateEvidenceLinkRepositoryError::InvalidContext);
        }
        self.vault
            .tombstone(record_id, &Self::vault_context(context))
            .map_err(HealthStateEvidenceLinkRepositoryError::from)
    }

    pub fn verify(
        &self,
        record_id: &str,
        context: &AuthorizedHealthStateEvidenceLinkContext,
    ) -> Result<(), HealthStateEvidenceLinkRepositoryError> {
        if !context.matches_operation(ENTITY_TYPE, record_id, "verify") {
            return Err(HealthStateEvidenceLinkRepositoryError::InvalidContext);
        }
        self.vault
            .verify(record_id, &Self::vault_context(context))
            .map_err(HealthStateEvidenceLinkRepositoryError::from)
    }
}

#[cfg(test)]
#[path = "health_state_evidence_link_repository_tests.rs"]
mod tests;
