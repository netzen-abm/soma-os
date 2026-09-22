use super::*;


use super::*;
use tokio::sync::OnceCell;

const MIGRATIONS: &[&str] = &[
    include_str!("../../../database/migrations/0001_initialize_zk_logs.sql"),
    include_str!("../../../database/migrations/0003_protected_data_scope_transition.sql"),
    include_str!("../../../database/migrations/0004_scoped_anonymized_hash.sql"),
    include_str!("../../../database/migrations/0005_legacy_data_classification.sql"),
    include_str!("../../../database/migrations/0006_trusted_db_service_identity.sql"),
    include_str!("../../../database/migrations/0007_protected_data_rls_constraints.sql"),
    include_str!("../../../database/migrations/0008_legacy_data_promotion_events.sql"),
    include_str!("../../../database/migrations/0009_legacy_promotion_executor.sql"),
    include_str!("../../../database/migrations/0012_legacy_promotion_executor_privilege_isolation.sql"),
];

static PREPARED: OnceCell<()> = OnceCell::const_new();

fn valid_request() -> LegacyPromotionRequest {
    LegacyPromotionRequest {
        legacy_log_id: 1,
        classification_id: 1,
        promotion_event_reference: "promotion-1".into(),
        authoritative_provenance: "prov-1".into(),
        initiated_by: "migration-operator-1".into(),
    }
}

async fn pool() -> Option<PgPool> {
    let url = std::env::var("SOMA_TEST_DATABASE_URL").ok().filter(|v| !v.trim().is_empty())?;
    Some(PgPoolOptions::new().max_connections(6).connect(&url).await.expect("SOMA_TEST_DATABASE_URL must be reachable"))
}

async fn prepare(pool: &PgPool) {
    PREPARED.get_or_init(|| async {
        for (index, migration) in MIGRATIONS.iter().enumerate() {
            sqlx::raw_sql(migration).execute(pool).await.expect("promotion migrations must apply cleanly");
            if index == 4 {
                for hash in ["lp-verified", "lp-idempotent", "lp-reject", "lp-rollback"] {
                    sqlx::query("INSERT INTO anonymized_user_vitals (anonymized_user_hash, public_verification_key_hex, verified_vitality_score, salud_schema_version, signature_proof_hex) VALUES ($1, 'pk', 7, '1.0', 'sig')")
                        .bind(hash).execute(pool).await.expect("legacy fixture must be seeded before RLS");
                }
            }
        }
    }).await;
}

async fn log_id(pool: &PgPool, hash: &str) -> i32 {
    sqlx::query("SELECT log_id FROM anonymized_user_vitals WHERE anonymized_user_hash = $1").bind(hash).fetch_one(pool).await.unwrap().get("log_id")
}

async fn classification(pool: &PgPool, log_id: i32, state: &str, tenant: Option<&str>, domain: Option<&str>, provenance: Option<&str>, verification: &str) -> i64 {
    sqlx::query("INSERT INTO anonymized_user_vitals_legacy_classification (legacy_log_id, classification_state, candidate_tenant_id, candidate_data_domain, reason_code, provenance_reference, classified_by, verification_status) VALUES ($1, $2, $3, $4, 'AUTHORITATIVE_SOURCE', $5, 'integration-reviewer', $6) RETURNING classification_id")
        .bind(log_id).bind(state).bind(tenant).bind(domain).bind(provenance).bind(verification).fetch_one(pool).await.unwrap().get("classification_id")
}

async fn assume_control_plane_role(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>) {
    sqlx::query("GRANT somaos_legacy_promotion_executor TO somaos_test").execute(&mut **tx).await.unwrap();
    sqlx::query("SET LOCAL ROLE somaos_legacy_promotion_executor").execute(&mut **tx).await.unwrap();
}

#[test]
fn validates_provider_neutral_request() { assert!(valid_request().validate().is_ok()); }

#[test]
fn rejects_invalid_ids_and_metadata() {
    assert!(LegacyPromotionRequest { legacy_log_id: 0, ..valid_request() }.validate().is_err());
    assert!(LegacyPromotionRequest { classification_id: 0, ..valid_request() }.validate().is_err());
    assert!(LegacyPromotionRequest { promotion_event_reference: "".into(), ..valid_request() }.validate().is_err());
    assert!(LegacyPromotionRequest { initiated_by: "\n".into(), ..valid_request() }.validate().is_err());
}

#[test]
fn request_has_no_caller_supplied_scope() {
    let debug = format!("{:?}", valid_request());
    assert!(!debug.contains("tenant_id"));
    assert!(!debug.contains("data_domain"));
}

#[tokio::test]
async fn verified_promotion_derives_scope_and_records_event() {
    let Some(pool) = pool().await else { return; };
    prepare(&pool).await;
    let id = log_id(&pool, "lp-verified").await;
    let class_id = classification(&pool, id, "SCOPED_VERIFIED", Some("tenant-a"), Some("domain-a"), Some("prov-a"), "VERIFIED").await;
    let executor = LegacyPromotionExecutor::new(pool.clone());
    let result = executor.promote(&LegacyPromotionRequest { legacy_log_id: id, classification_id: class_id, promotion_event_reference: "event-a".into(), authoritative_provenance: "prov-a".into(), initiated_by: "operator-a".into() }).await.unwrap();
    let row = sqlx::query("SELECT tenant_id, data_domain FROM anonymized_user_vitals WHERE log_id = $1").bind(id).fetch_one(&pool).await.unwrap();
    assert_eq!(row.get::<String, _>("tenant_id"), "tenant-a");
    assert_eq!(row.get::<String, _>("data_domain"), "domain-a");
    assert!(result.promotion_event_id > 0);
}

#[tokio::test]
async fn idempotent_same_event_and_conflicting_event_rejected() {
    let Some(pool) = pool().await else { return; };
    prepare(&pool).await;
    let id = log_id(&pool, "lp-idempotent").await;
    let class_id = classification(&pool, id, "SCOPED_VERIFIED", Some("tenant-i"), Some("domain-i"), Some("prov-i"), "VERIFIED").await;
    let request = LegacyPromotionRequest { legacy_log_id: id, classification_id: class_id, promotion_event_reference: "event-i".into(), authoritative_provenance: "prov-i".into(), initiated_by: "operator-i".into() };
    let executor = LegacyPromotionExecutor::new(pool.clone());
    let first = executor.promote(&request).await.unwrap().promotion_event_id;
    let second = executor.promote(&request).await.unwrap().promotion_event_id;
    assert_eq!(first, second);
    let conflict = executor.promote(&LegacyPromotionRequest { promotion_event_reference: "event-conflict".into(), ..request }).await;
    assert!(conflict.is_err());
}

#[tokio::test]
async fn invalid_or_mismatched_evidence_cannot_promote() {
    let Some(pool) = pool().await else { return; };
    prepare(&pool).await;
    let id = log_id(&pool, "lp-reject").await;
    let unverified = classification(&pool, id, "SCOPED_VERIFIED", Some("tenant-u"), Some("domain-u"), Some("prov-u"), "UNVERIFIED").await;
    let quarantined = classification(&pool, id, "QUARANTINED_UNCLASSIFIED", None, None, None, "UNVERIFIED").await;
    let verified = classification(&pool, id, "SCOPED_VERIFIED", Some("tenant-v"), Some("domain-v"), Some("prov-v"), "VERIFIED").await;
    let executor = LegacyPromotionExecutor::new(pool.clone());
    for (class_id, provenance, event) in [(unverified, "prov-u", "event-u"), (quarantined, "prov-q", "event-q"), (verified, "wrong", "event-v")] {
        let result = executor.promote(&LegacyPromotionRequest { legacy_log_id: id, classification_id: class_id, promotion_event_reference: event.into(), authoritative_provenance: provenance.into(), initiated_by: "operator-r".into() }).await;
        assert!(result.is_err());
    }
    let row = sqlx::query("SELECT tenant_id, data_domain FROM anonymized_user_vitals WHERE log_id = $1").bind(id).fetch_one(&pool).await.unwrap();
    assert!(row.get::<Option<String>, _>("tenant_id").is_none());
    assert!(row.get::<Option<String>, _>("data_domain").is_none());
}

#[tokio::test]
async fn rollback_is_atomic_and_untrusted_role_cannot_execute() {
    let Some(pool) = pool().await else { return; };
    prepare(&pool).await;
    let id = log_id(&pool, "lp-rollback").await;
    let class_id = classification(&pool, id, "SCOPED_VERIFIED", Some("tenant-r"), Some("domain-r"), Some("prov-r"), "VERIFIED").await;
    let mut tx = pool.begin().await.unwrap();
    assume_control_plane_role(&mut tx).await;
    let _: i64 = sqlx::query_scalar("SELECT public.soma_apply_legacy_promotion($1, $2, $3, $4, $5)").bind(id).bind(class_id).bind("event-r").bind("prov-r").bind("operator-r").fetch_one(&mut *tx).await.unwrap();
    tx.rollback().await.unwrap();
    let row = sqlx::query("SELECT tenant_id, data_domain FROM anonymized_user_vitals WHERE log_id = $1").bind(id).fetch_one(&pool).await.unwrap();
    assert!(row.get::<Option<String>, _>("tenant_id").is_none());
    assert!(row.get::<Option<String>, _>("data_domain").is_none());

    let mut connection = pool.acquire().await.unwrap();
    sqlx::query("SET ROLE somaos_untrusted_test").execute(&mut *connection).await.unwrap();
    let denied = sqlx::query("SELECT public.soma_apply_legacy_promotion(1, 1, 'attacker', 'attacker', 'attacker')").fetch_one(&mut *connection).await;
    assert!(denied.is_err());
}
