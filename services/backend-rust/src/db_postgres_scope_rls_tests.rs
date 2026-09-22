use super::postgres_test_support::{assume_persistence_role, prepare, test_pool};
use sqlx::Row;

#[tokio::test]
async fn concurrent_transactions_cannot_cross_contaminate_scope() {
    let Some(pool) = test_pool().await else {
        return;
    };
    prepare(&pool).await;

    let mut tx_a = pool.begin().await.unwrap();
    let mut tx_b = pool.begin().await.unwrap();
    assume_persistence_role(&mut tx_a).await;
    assume_persistence_role(&mut tx_b).await;
    sqlx::query("SELECT public.soma_set_protected_scope($1, $2)")
        .bind("tenant-a")
        .bind("domain-a")
        .execute(&mut *tx_a)
        .await
        .unwrap();
    sqlx::query("SELECT public.soma_set_protected_scope($1, $2)")
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
    let Some(pool) = test_pool().await else {
        return;
    };
    prepare(&pool).await;

    let hash = "a".repeat(64);
    let insert = "INSERT INTO anonymized_user_vitals (tenant_id, data_domain, anonymized_user_hash, public_verification_key_hex, verified_vitality_score, salud_schema_version, signature_proof_hex) VALUES ($1, $2, $3, 'pk', 1, '1.0', 'sig')";
    sqlx::query(insert).bind("tenant-a").bind("domain-a").bind(&hash).execute(&pool).await.unwrap();
    sqlx::query(insert).bind("tenant-b").bind("domain-b").bind(&hash).execute(&pool).await.unwrap();

    let duplicate = sqlx::query(insert).bind("tenant-a").bind("domain-a").bind(&hash).execute(&pool).await;
    assert!(duplicate.is_err());
}
