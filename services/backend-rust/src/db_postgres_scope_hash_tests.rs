use super::postgres_test_support::{assume_persistence_role, prepare, test_pool};
use sqlx::Row;

#[tokio::test]
async fn legacy_null_scope_is_not_made_accessible_by_scoped_queries() {
    let Some(pool) = test_pool().await else {
        return;
    };
    prepare(&pool).await;

    let count = sqlx::query("SELECT COUNT(*) AS count FROM anonymized_user_vitals WHERE anonymized_user_hash = $1 AND tenant_id = $2 AND data_domain = $3")
        .bind("legacy-null-scope-fixture")
        .bind("tenant-a")
        .bind("domain-a")
        .fetch_one(&pool)
        .await
        .unwrap()
        .get::<i64, _>("count");
    assert_eq!(count, 0);
}

#[tokio::test]
async fn rls_filters_reads_without_requiring_application_scope_predicates() {
    let Some(pool) = test_pool().await else {
        return;
    };
    prepare(&pool).await;

    let hash = "c".repeat(64);
    let insert = "INSERT INTO anonymized_user_vitals (tenant_id, data_domain, anonymized_user_hash, public_verification_key_hex, verified_vitality_score, salud_schema_version, signature_proof_hex) VALUES ($1, $2, $3, 'pk', 1, '1.0', 'sig')";
    sqlx::query(insert).bind("tenant-rls-a").bind("domain-rls-a").bind(&hash).execute(&pool).await.unwrap();
    sqlx::query(insert).bind("tenant-rls-b").bind("domain-rls-b").bind(&hash).execute(&pool).await.unwrap();

    let mut tx = pool.begin().await.unwrap();
    assume_persistence_role(&mut tx).await;
    sqlx::query("SELECT public.soma_set_protected_scope($1, $2)")
        .bind("tenant-rls-a")
        .bind("domain-rls-a")
        .execute(&mut *tx)
        .await
        .unwrap();

    let rows = sqlx::query("SELECT tenant_id, data_domain FROM anonymized_user_vitals WHERE anonymized_user_hash = $1")
        .bind(&hash)
        .fetch_all(&mut *tx)
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get::<String, _>("tenant_id"), "tenant-rls-a");
    assert_eq!(rows[0].get::<String, _>("data_domain"), "domain-rls-a");
    tx.rollback().await.unwrap();
}
