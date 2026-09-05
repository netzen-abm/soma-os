use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use tokio::sync::OnceCell;

const MIGRATIONS: &[&str] = &[
    include_str!("../../../database/migrations/0001_initialize_zk_logs.sql"),
    include_str!("../../../database/migrations/0003_protected_data_scope_transition.sql"),
    include_str!("../../../database/migrations/0004_scoped_anonymized_hash.sql"),
];

static PREPARED: OnceCell<()> = OnceCell::const_new();

async fn test_pool() -> Option<PgPool> {
    let url = std::env::var("SOMA_TEST_DATABASE_URL").ok().filter(|value| !value.trim().is_empty())?;
    Some(
        PgPoolOptions::new()
            .max_connections(4)
            .connect(&url)
            .await
            .expect("SOMA_TEST_DATABASE_URL must point to a reachable PostgreSQL test database"),
    )
}

async fn prepare(pool: &PgPool) {
    PREPARED
        .get_or_init(|| async {
            for migration in MIGRATIONS {
                sqlx::raw_sql(migration)
                    .execute(pool)
                    .await
                    .expect("protected-data test migrations must apply cleanly");
            }
        })
        .await;
}

#[tokio::test]
async fn transaction_local_scope_survives_pool_reuse_without_leakage() {
    let Some(pool) = test_pool().await else { return };
    prepare(&pool).await;

    let mut tx = pool.begin().await.unwrap();
    sqlx::query("SELECT set_config('soma.tenant_id', $1, true), set_config('soma.data_domain', $2, true)")
        .bind("tenant-a")
        .bind("domain-a")
        .execute(&mut *tx)
        .await
        .unwrap();
    let row = sqlx::query("SELECT current_setting('soma.tenant_id'), current_setting('soma.data_domain')")
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    assert_eq!(row.get::<String, _>(0), "tenant-a");
    assert_eq!(row.get::<String, _>(1), "domain-a");
    tx.commit().await.unwrap();

    let row = sqlx::query("SELECT current_setting('soma.tenant_id', true) AS tenant_id, current_setting('soma.data_domain', true) AS data_domain")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(row.get::<Option<String>, _>("tenant_id").is_none());
    assert!(row.get::<Option<String>, _>("data_domain").is_none());
}

#[tokio::test]
async fn concurrent_transactions_cannot_cross_contaminate_scope() {
    let Some(pool) = test_pool().await else { return };
    prepare(&pool).await;

    let mut tx_a = pool.begin().await.unwrap();
    let mut tx_b = pool.begin().await.unwrap();
    sqlx::query("SELECT set_config('soma.tenant_id', $1, true), set_config('soma.data_domain', $2, true)")
        .bind("tenant-a")
        .bind("domain-a")
        .execute(&mut *tx_a)
        .await
        .unwrap();
    sqlx::query("SELECT set_config('soma.tenant_id', $1, true), set_config('soma.data_domain', $2, true)")
        .bind("tenant-b")
        .bind("domain-b")
        .execute(&mut *tx_b)
        .await
        .unwrap();

    let row_a = sqlx::query("SELECT current_setting('soma.tenant_id'), current_setting('soma.data_domain')")
        .fetch_one(&mut *tx_a)
        .await
        .unwrap();
    let row_b = sqlx::query("SELECT current_setting('soma.tenant_id'), current_setting('soma.data_domain')")
        .fetch_one(&mut *tx_b)
        .await
        .unwrap();
    assert_eq!(row_a.get::<String, _>(0), "tenant-a");
    assert_eq!(row_a.get::<String, _>(1), "domain-a");
    assert_eq!(row_b.get::<String, _>(0), "tenant-b");
    assert_eq!(row_b.get::<String, _>(1), "domain-b");

    tx_a.rollback().await.unwrap();
    tx_b.rollback().await.unwrap();
}

#[tokio::test]
async fn scoped_hash_uniqueness_allows_same_hash_across_scopes_and_rejects_same_scope_duplicate() {
    let Some(pool) = test_pool().await else { return };
    prepare(&pool).await;

    let hash = "a".repeat(64);
    let insert = "INSERT INTO anonymized_user_vitals (tenant_id, data_domain, anonymized_user_hash, public_verification_key_hex, verified_vitality_score, salud_schema_version, signature_proof_hex) VALUES ($1, $2, $3, 'pk', 1, '1.0', 'sig')";
    sqlx::query(insert).bind("tenant-a").bind("domain-a").bind(&hash).execute(&pool).await.unwrap();
    sqlx::query(insert).bind("tenant-b").bind("domain-b").bind(&hash).execute(&pool).await.unwrap();

    let duplicate = sqlx::query(insert).bind("tenant-a").bind("domain-a").bind(&hash).execute(&pool).await;
    assert!(duplicate.is_err());
}

#[tokio::test]
async fn legacy_null_scope_is_not_made_accessible_by_scoped_queries() {
    let Some(pool) = test_pool().await else { return };
    prepare(&pool).await;

    let hash = "b".repeat(64);
    sqlx::query("INSERT INTO anonymized_user_vitals (anonymized_user_hash, public_verification_key_hex, verified_vitality_score, salud_schema_version, signature_proof_hex) VALUES ($1, 'pk', 1, '1.0', 'sig')")
        .bind(&hash)
        .execute(&pool)
        .await
        .unwrap();

    let count = sqlx::query("SELECT COUNT(*) AS count FROM anonymized_user_vitals WHERE anonymized_user_hash = $1 AND tenant_id = $2 AND data_domain = $3")
        .bind(&hash)
        .bind("tenant-a")
        .bind("domain-a")
        .fetch_one(&pool)
        .await
        .unwrap()
        .get::<i64, _>("count");
    assert_eq!(count, 0);
}
