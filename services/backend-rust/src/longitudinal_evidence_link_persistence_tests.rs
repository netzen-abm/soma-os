use super::*;
use super::domain_fixtures::link;
use crate::health_state_evidence_link::{
    AuthorizedHealthStateEvidenceLinkContext, HealthStateEvidenceLinkBoundary, LinkUncertainty,
};
use crate::health_state_evidence_link_repository::HealthStateEvidenceLinkRepository;

#[test]
fn evidence_link_persistence_preserves_provenance_and_uncertainty() {
    let root = root();
    let mut write_request = request();
    write_request.capability_id = "health.evidence.link".into();
    write_request.resource_type = "health_state_evidence_link".into();
    write_request.resource_id = "person-1-link-1".into();
    let write_context =
        AuthorizedHealthStateEvidenceLinkContext::from_authorized_request(&write_request).unwrap();
    assert_eq!(AllowBoundary.authorize(&write_request), AuthorizationDecision::Allow);

    let evidence_link = link();
    HealthStateEvidenceLinkBoundary::validate(&evidence_link, &write_context).unwrap();
    let repository = HealthStateEvidenceLinkRepository::new(
        &root,
        std::sync::Arc::new(Keys {
            values: {
                let mut values = HashMap::new();
                values.insert("vault-key-v1".to_owned(), [7u8; KEY_LEN]);
                values
            },
            calls: Rc::new(Cell::new(0)),
        }),
        SubjectAuthorizer,
        "vault-key-v1",
    )
    .unwrap();

    repository.put(&evidence_link, &write_context).unwrap();

    let mut read_request = write_request.clone();
    read_request.action = "read".into();
    let read_context =
        AuthorizedHealthStateEvidenceLinkContext::from_authorized_request(&read_request).unwrap();
    let persisted = repository.get("person-1-link-1", &read_context).unwrap();
    assert_eq!(persisted, evidence_link);
    assert_eq!(persisted.provenance.actor_ref.as_deref(), Some("principal-1"));
    assert_eq!(persisted.uncertainty, Some(LinkUncertainty::Reported));

    let mut verify_request = write_request;
    verify_request.action = "verify".into();
    let verify_context =
        AuthorizedHealthStateEvidenceLinkContext::from_authorized_request(&verify_request).unwrap();
    repository.verify("person-1-link-1", &verify_context).unwrap();

    fs::remove_dir_all(root).unwrap();
}
