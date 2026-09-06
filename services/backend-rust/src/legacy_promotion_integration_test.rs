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
    let url = std::env::var("SOMA_TEST_DATABASE_URL")
        .ok()
        .filter(|value| !value.trim().is_empty())?;
    Some(
        PgPoolOptions::new()
            .max_connections(6)
            .connect(&url)
            .await
            .expect("SOMA_TEST_DATABASE_URL must point to a reachable PostgreSQL test database"),
    )
}

async fn prepare(pool: &PgPool) {
    PREPARED
        .get_or_init(|| async {
            for (index, migration) in MIGRATIONS.iter().enumerate() {
                sqlx::raw_sql(migration)
                    .execute(pool)
                    .await
                    .expect("legacy-promotion test migrations must apply cleanly");

                // 0007 prevents creation of new NULL-scope rows, so seed the
                // realistic legacy fixture after classification tables exist but
                // before RLS/constraint activation.
                if index == 4 {
                    sqlx::query("INSERT INTO anonymized_user_vitals (anonymized_user_hash, public_verification_key_hex, verified_vitality_score, salud_schema_version, signature_proof_hex) VALUES ('legacy-promotion-fixture', 'pk', 7, '1.0', 'sig')")
                        .execute(pool)
                        .await
                        .expect("legacy promotion fixture must be seedable before RLS");
                }
            }
        })
        .await;
}

async fn assume_persistence_role(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>) {
    sqlx::query("SET LOCAL ROLE somaos_persistence")
        .execute(&mut **tx)
        .await
        .expect("test connection must be able to assume somaos_persistence");
}

async fn insert_classification(
    pool: &PgPool,
    legacy_log_id: i32,
    classification_state: &str,
    tenant_id: Option<&str>,
    data_domain: Option<&str>,
    provenance: Option<&str>,
    verification_status: &str,
    classified_by: &str,
) -> i64 {
    sqlx::query(
        "INSERT INTO anonymized_user_vitals_legacy_classification (legacy_log_id, classification_state, candidate_tenant_id, candidate_data_domain, reason_code, provenance_reference, classified_by, verification_status) VALUES ($1, $2, $3, $4, 'AUTHORITATIVE_SOURCE', $5, $6, $7) RETURNING classification_id",
    )
    .bind(legacy_log_id)
    .bind(classification_state)
    .bind(tenant_id)
    .bind(data_domain)
    .bind(provenance)
    .bind(classified_by)
    .bind(verification_status)
    .fetch_one(pool)
    .await
    .unwrap()
    .get("classification_id")
}

#[tokio::test]
async fn verified_promotion_derives_scope_and_records_atomic_event() {
    let Some(pool) = test_pool().await else {
        return;
    };
    prepare(&pool).await;

    let log_id: i32 = sqlx::query("SELECT log_id FROM anonymized_user_vitals WHERE anonymized_user_hash = 'legacy-promotion-fixture'")
        .fetch_one(&pool)
        .await
        .unwrap()
        .get("log_id");

    let classification_id = insert_classification(
        &pool,
        log_id,
        "SCOPED_VERIFIED",
        Some("tenant-promotion-a"),
        Some("domain-promotion-a"),
        Some("prov-promotion-a"),
        "VERIFIED",
        "reviewer-a",
    )
    .await;

    let mut tx = pool.begin().await.unwrap();
    assume_persistence_role(&mut tx).await;
    let event_id: i64 = sqlx::query_scalar("SELECT public.soma_apply_legacy_promotion($1, $2, $3, $4, $5)")
        .bind(log_id)
        .bind(classification_id)
        .bind("promotion-event-a")
        .bind("prov-promotion-a")
        .bind("migration-operator-a")
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    let row = sqlx::query("SELECT tenant_id, data_domain FROM anonymized_user_vitals WHERE log_id = $1")
        .bind(log_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(row.get::<String, _>("tenant_id"), "tenant-promotion-a");
    assert_eq!(row.get::<String, _>("data_domain"), "domain-promotion-a");

    let event = sqlx::query("SELECT promotion_event_id, promotion_status, classification_id, provenance_reference, tenant_id, data_domain FROM anonymized_user_vitals_legacy_promotion_event WHERE promotion_event_id = $1")
        .bind(event_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(event.get::<String, _>("promotion_status"), "APPLIED");
    assert_eq!(event.get::<i64, _>("classification_id"), classification_id);
    assert_eq!(event.get::<String, _>("provenance_reference"), "prov-promotion-a");
    assert_eq!(event.get::<String, _>("tenant_id"), "tenant-promotion-a");
    assert_eq!(event.get::<String, _>("data_domain"), "domain-promotion-a");
}

#[tokio::test]
async fn promotion_is_idempotent_for_same_event_and_rejects_conflicting_event() {
    let Some(pool) = test_pool().await else {
        return;
    };
    prepare(&pool).await;

    let log_id: i32 = sqlx::query("SELECT log_id FROM anonymized_user_vitals WHERE anonymized_user_hash = 'legacy-promotion-fixture'")
        .fetch_one(&pool)
        .await
        .unwrap()
        .get("log_id");
    let classification_id = insert_classification(
        &pool,
        log_id,
        "SCOPED_VERIFIED",
        Some("tenant-idempotent"),
        Some("domain-idempotent"),
        Some("prov-idempotent"),
        "VERIFIED",
        "reviewer-idempotent",
    )
    .await;

    let first: i64 = sqlx::query_scalar("SELECT public.soma_apply_legacy_promotion($1, $2, $3, $4, $5)")
        .bind(log_id)
        .bind(classification_id)
        .bind("promotion-idempotent")
        .bind("prov-idempotent")
        .bind("operator-idempotent")
        .fetch_one(&pool)
        .await
        .unwrap();

    let second: i64 = sqlx::query_scalar("SELECT public.soma_apply_legacy_promotion($1, $2, $3, $4, $5)")
        .bind(log_id)
        .bind(classification_id)
        .bind("promotion-idempotent")
        .bind("prov-idempotent")
        .bind("operator-idempotent")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(first, second);

    let conflict = sqlx::query("SELECT public.soma_apply_legacy_promotion($1, $2, $3, $4, $5)")
        .bind(log_id)
        .bind(classification_id)
        .bind("promotion-conflict")
        .bind("prov-idempotent")
        .bind("operator-idempotent")
        .fetch_one(&pool)
        .await;
    assert!(conflict.is_err());
}

#[tokio::test]
async fn unverified_quarantined_and_provenance_mismatch_cannot_promote() {
    let Some(pool) = test_pool().await else {
        return;
    };
    prepare(&pool).await;

    let log_id: i32 = sqlx::query("SELECT log_id FROM anonymized_user_vitals WHERE anonymized_user_hash = 'legacy-promotion-fixture'")
        .fetch_one(&pool)
        .await
        .unwrap()
        .get("log_id");

    let unverified = insert_classification(
        &pool,
        log_id,
        "SCOPED_VERIFIED",
        Some("tenant-reject-a"),
        Some("domain-reject-a"),
        Some("prov-reject-a"),
        "UNVERIFIED",
        "reviewer-reject-a",
    )
    .await;
    let quarantined = insert_classification(
        &pool,
        log_id,
        "QUARANTINED_UNCLASSIFIED",
        None,
        None,
        None,
        "UNVERIFIED",
        "reviewer-reject-b",
    )
    .await;
    let verified = insert_classification(
        &pool,
        log_id,
        "SCOPED_VERIFIED",
        Some("tenant-reject-c"),
        Some("domain-reject-c"),
        Some("prov-reject-c"),
        "VERIFIED",
        "reviewer-reject-c",
    )
    .await;

    for (classification_id, provenance, reference) in [
        (unverified, "prov-reject-a", "promotion-unverified"),
        (quarantined, "prov-any", "promotion-quarantine"),
        (verified, "wrong-provenance", "promotion-mismatch"),
    ] {
        let result = sqlx::query("SELECT public.soma_apply_legacy_promotion($1, $2, $3, $4, $5)")
            .bind(log_id)
            .bind(classification_id)
            .bind(reference)
            .bind(provenance)
            .bind("operator-reject")
            .fetch_one(&pool)
            .await;
        assert!(result.is_err());
    }

    let row = sqlx::query("SELECT tenant_id, data_domain FROM anonymized_user_vitals WHERE log_id = $1")
        .bind(log_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(row.get::<Option<String>, _>("tenant_id").is_none());
    assert!(row.get::<Option<String>, _>("data_domain").is_none());
}

#[tokio::test]
async fn untrusted_role_cannot_invoke_promotion_executor() {
    let Some(pool) = test_pool().await else {
        return;
    };
    prepare(&pool).await;

    let mut connection = pool.acquire().await.unwrap();
    sqlx::query("SET ROLE somaos_untrusted_test")
        .execute(&mut *connection)
        .await
        .unwrap();

    let result = sqlx::query("SELECT public.soma_apply_legacy_promotion($1, $2, $3, $4, $5)")
        .bind(1_i32)
        .bind(1_i64)
        .bind("attacker-event")
        .bind("attacker-provenance")
        .bind("attacker")
        .fetch_one(&mut *connection)
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn rollback_leaves_legacy_row_and_event_unchanged() {
    let Some(pool) = test_pool().await else {
        return;
    };
    prepare(&pool).await;

    let log_id: i32 = sqlx::query("SELECT log_id FROM anonymized_user_vitals WHERE anonymized_user_hash = 'legacy-promotion-fixture'")
        .fetch_one(&pool)
        .await
        .unwrap()
        .get("log_id");
    let classification_id = insert_classification(
        &pool,
        log_id,
        "SCOPED_VERIFIED",
        Some("tenant-rollback"),
        Some("domain-rollback"),
        Some("prov-rollback"),
        "VERIFIED",
        "reviewer-rollback",
    )
    .await;

    let mut tx = pool.begin().await.unwrap();
    assume_persistence_role(&mut tx).await;
    let _event_id: i64 = sqlx::query_scalar("SELECT public.soma_apply_legacy_promotion($1, $2, $3, $4, $5)")
        .bind(log_id)
        .bind(classification_id)
        .bind("promotion-rollback")
        .bind("prov-rollback")
        .bind("operator-rollback")
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    tx.rollback().await.unwrap();

    let row = sqlx::query("SELECT tenant_id, data_domain FROM anonymized_user_vitals WHERE log_id = $1")
        .bind(log_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(row.get::<Option<String>, _>("tenant_id").is_none());
    assert!(row.get::<Option<String>, _>("data_domain").is_none());

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM anonymized_user_vitals_legacy_promotion_event WHERE legacy_log_id = $1 AND promotion_event_reference = 'promotion-rollback'")
        .bind(log_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}
