use sqlx::{postgres::PgPoolOptions, PgPool, Row};
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
];

static PREPARED: OnceCell<()> = OnceCell::const_new();

async fn test_pool() -> Option<PgPool> {
    let url = std::env::var("SOMA_TEST_DATABASE_URL").ok().filter(|v| !v.trim().is_empty())?;
    Some(PgPoolOptions::new().max_connections(6).connect(&url).await.expect("SOMA_TEST_DATABASE_URL must be reachable"))
}

async fn prepare(pool: &PgPool) {
    PREPARED.get_or_init(|| async {
        for (index, migration) in MIGRATIONS.iter().enumerate() {
            sqlx::raw_sql(migration).execute(pool).await.expect("legacy-promotion migrations must apply cleanly");
            if index == 4 {
                for hash in [
                    "legacy-promotion-verified",
                    "legacy-promotion-idempotent",
                    "legacy-promotion-rejection",
                    "legacy-promotion-rollback",
                ] {
                    sqlx::query("INSERT INTO anonymized_user_vitals (anonymized_user_hash, public_verification_key_hex, verified_vitality_score, salud_schema_version, signature_proof_hex) VALUES ($1, 'pk', 7, '1.0', 'sig')")
                        .bind(hash).execute(pool).await.expect("legacy fixture must be seedable before RLS");
                }
            }
        }
    }).await;
}

async fn assume_persistence_role(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>) {
    sqlx::query("SET LOCAL ROLE somaos_persistence").execute(&mut **tx).await.unwrap();
}

async fn log_id(pool: &PgPool, hash: &str) -> i32 {
    sqlx::query("SELECT log_id FROM anonymized_user_vitals WHERE anonymized_user_hash = $1")
        .bind(hash).fetch_one(pool).await.unwrap().get("log_id")
}

async fn insert_classification(pool: &PgPool, legacy_log_id: i32, state: &str, tenant: Option<&str>, domain: Option<&str>, provenance: Option<&str>, verification: &str) -> i64 {
    sqlx::query("INSERT INTO anonymized_user_vitals_legacy_classification (legacy_log_id, classification_state, candidate_tenant_id, candidate_data_domain, reason_code, provenance_reference, classified_by, verification_status) VALUES ($1, $2, $3, $4, 'AUTHORITATIVE_SOURCE', $5, 'integration-reviewer', $6) RETURNING classification_id")
        .bind(legacy_log_id).bind(state).bind(tenant).bind(domain).bind(provenance).bind(verification)
        .fetch_one(pool).await.unwrap().get("classification_id")
}

#[tokio::test]
async fn verified_promotion_derives_scope_and_records_atomic_event() {
    let Some(pool) = test_pool().await else { return; };
    prepare(&pool).await;
    let id = log_id(&pool, "legacy-promotion-verified").await;
    let classification = insert_classification(&pool, id, "SCOPED_VERIFIED", Some("tenant-promotion-a"), Some("domain-promotion-a"), Some("prov-promotion-a"), "VERIFIED").await;

    let mut tx = pool.begin().await.unwrap();
    assume_persistence_role(&mut tx).await;
    let event_id: i64 = sqlx::query_scalar("SELECT public.soma_apply_legacy_promotion($1, $2, $3, $4, $5)")
        .bind(id).bind(classification).bind("promotion-event-a").bind("prov-promotion-a").bind("migration-operator-a")
        .fetch_one(&mut *tx).await.unwrap();
    tx.commit().await.unwrap();

    let row = sqlx::query("SELECT tenant_id, data_domain FROM anonymized_user_vitals WHERE log_id = $1").bind(id).fetch_one(&pool).await.unwrap();
    assert_eq!(row.get::<String, _>("tenant_id"), "tenant-promotion-a");
    assert_eq!(row.get::<String, _>("data_domain"), "domain-promotion-a");
    let event = sqlx::query("SELECT promotion_status, classification_id, provenance_reference FROM anonymized_user_vitals_legacy_promotion_event WHERE promotion_event_id = $1").bind(event_id).fetch_one(&pool).await.unwrap();
    assert_eq!(event.get::<String, _>("promotion_status"), "APPLIED");
    assert_eq!(event.get::<i64, _>("classification_id"), classification);
    assert_eq!(event.get::<String, _>("provenance_reference"), "prov-promotion-a");
}

#[tokio::test]
async fn promotion_is_idempotent_and_rejects_conflicting_event() {
    let Some(pool) = test_pool().await else { return; };
    prepare(&pool).await;
    let id = log_id(&pool, "legacy-promotion-idempotent").await;
    let classification = insert_classification(&pool, id, "SCOPED_VERIFIED", Some("tenant-idempotent"), Some("domain-idempotent"), Some("prov-idempotent"), "VERIFIED").await;

    let call = |event: &str| async { sqlx::query_scalar::<_, i64>("SELECT public.soma_apply_legacy_promotion($1, $2, $3, $4, $5)").bind(id).bind(classification).bind(event).bind("prov-idempotent").bind("operator-idempotent").fetch_one(&pool).await };
    let first = call("promotion-idempotent").await.unwrap();
    let second = call("promotion-idempotent").await.unwrap();
    assert_eq!(first, second);
    assert!(call("promotion-conflict").await.is_err());
}

#[tokio::test]
async fn unverified_quarantined_and_provenance_mismatch_cannot_promote() {
    let Some(pool) = test_pool().await else { return; };
    prepare(&pool).await;
    let id = log_id(&pool, "legacy-promotion-rejection").await;
    let unverified = insert_classification(&pool, id, "SCOPED_VERIFIED", Some("tenant-reject-a"), Some("domain-reject-a"), Some("prov-reject-a"), "UNVERIFIED").await;
    let quarantined = insert_classification(&pool, id, "QUARANTINED_UNCLASSIFIED", None, None, None, "UNVERIFIED").await;
    let verified = insert_classification(&pool, id, "SCOPED_VERIFIED", Some("tenant-reject-c"), Some("domain-reject-c"), Some("prov-reject-c"), "VERIFIED").await;

    for (classification, provenance, reference) in [(unverified, "prov-reject-a", "promotion-unverified"), (quarantined, "prov-any", "promotion-quarantine"), (verified, "wrong-provenance", "promotion-mismatch")] {
        let result = sqlx::query("SELECT public.soma_apply_legacy_promotion($1, $2, $3, $4, $5)").bind(id).bind(classification).bind(reference).bind(provenance).bind("operator-reject").fetch_one(&pool).await;
        assert!(result.is_err());
    }

    let row = sqlx::query("SELECT tenant_id, data_domain FROM anonymized_user_vitals WHERE log_id = $1").bind(id).fetch_one(&pool).await.unwrap();
    assert!(row.get::<Option<String>, _>("tenant_id").is_none());
    assert!(row.get::<Option<String>, _>("data_domain").is_none());
}

#[tokio::test]
async fn untrusted_role_cannot_invoke_promotion_executor() {
    let Some(pool) = test_pool().await else { return; };
    prepare(&pool).await;
    let mut connection = pool.acquire().await.unwrap();
    sqlx::query("SET ROLE somaos_untrusted_test").execute(&mut *connection).await.unwrap();
    let result = sqlx::query("SELECT public.soma_apply_legacy_promotion($1, $2, $3, $4, $5)").bind(1_i32).bind(1_i64).bind("attacker-event").bind("attacker-provenance").bind("attacker").fetch_one(&mut *connection).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn rollback_leaves_legacy_row_and_event_unchanged() {
    let Some(pool) = test_pool().await else { return; };
    prepare(&pool).await;
    let id = log_id(&pool, "legacy-promotion-rollback").await;
    let classification = insert_classification(&pool, id, "SCOPED_VERIFIED", Some("tenant-rollback"), Some("domain-rollback"), Some("prov-rollback"), "VERIFIED").await;

    let mut tx = pool.begin().await.unwrap();
    assume_persistence_role(&mut tx).await;
    let _: i64 = sqlx::query_scalar("SELECT public.soma_apply_legacy_promotion($1, $2, $3, $4, $5)").bind(id).bind(classification).bind("promotion-rollback").bind("prov-rollback").bind("operator-rollback").fetch_one(&mut *tx).await.unwrap();
    tx.rollback().await.unwrap();

    let row = sqlx::query("SELECT tenant_id, data_domain FROM anonymized_user_vitals WHERE log_id = $1").bind(id).fetch_one(&pool).await.unwrap();
    assert!(row.get::<Option<String>, _>("tenant_id").is_none());
    assert!(row.get::<Option<String>, _>("data_domain").is_none());
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM anonymized_user_vitals_legacy_promotion_event WHERE legacy_log_id = $1 AND promotion_event_reference = 'promotion-rollback'").bind(id).fetch_one(&pool).await.unwrap();
    assert_eq!(count, 0);
}
