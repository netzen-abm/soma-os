use super::postgres_test_support::{assume_persistence_role, prepare, test_pool};
use sqlx::Row;

#[tokio::test]
async fn rls_denies_missing_or_wrong_scope_and_blocks_cross_scope_mutation() {
    let Some(pool) = test_pool().await else {
        return;
    };
    prepare(&pool).await;

    let hash = "d".repeat(64);
    sqlx::query("INSERT INTO anonymized_user_vitals (tenant_id, data_domain, anonymized_user_hash, public_verification_key_hex, verified_vitality_score, salud_schema_version, signature_proof_hex) VALUES ('tenant-rls-c', 'domain-rls-c', $1, 'pk', 10, '1.0', 'sig')")
        .bind(&hash)
        .execute(&pool)
        .await
        .unwrap();

    let mut no_scope = pool.begin().await.unwrap();
    assume_persistence_role(&mut no_scope).await;
    let no_scope_count =
        sqlx::query("SELECT COUNT(*) AS count FROM anonymized_user_vitals WHERE anonymized_user_hash = $1")
            .bind(&hash)
            .fetch_one(&mut *no_scope)
            .await
            .unwrap()
            .get::<i64, _>("count");
    assert_eq!(no_scope_count, 0);
    no_scope.rollback().await.unwrap();

    let mut wrong_scope = pool.begin().await.unwrap();
    assume_persistence_role(&mut wrong_scope).await;
    sqlx::query("SELECT public.soma_set_protected_scope($1, $2)")
        .bind("tenant-rls-wrong")
        .bind("domain-rls-wrong")
        .execute(&mut *wrong_scope)
        .await
        .unwrap();

    let wrong_update =
        sqlx::query("UPDATE anonymized_user_vitals SET verified_vitality_score = 99 WHERE anonymized_user_hash = $1")
            .bind(&hash)
            .execute(&mut *wrong_scope)
            .await
            .unwrap();
    assert_eq!(wrong_update.rows_affected(), 0);

    let wrong_delete = sqlx::query("DELETE FROM anonymized_user_vitals WHERE anonymized_user_hash = $1")
        .bind(&hash)
        .execute(&mut *wrong_scope)
        .await
        .unwrap();
    assert_eq!(wrong_delete.rows_affected(), 0);
    wrong_scope.rollback().await.unwrap();
}

#[tokio::test]
async fn rls_with_check_rejects_missing_or_mismatched_scope_on_insert() {
    let Some(pool) = test_pool().await else {
        return;
    };
    prepare(&pool).await;

    let mut tx = pool.begin().await.unwrap();
    assume_persistence_role(&mut tx).await;
    sqlx::query("SELECT public.soma_set_protected_scope($1, $2)")
        .bind("tenant-rls-insert")
        .bind("domain-rls-insert")
        .execute(&mut *tx)
        .await
        .unwrap();

    let missing_scope = sqlx::query("INSERT INTO anonymized_user_vitals (anonymized_user_hash, public_verification_key_hex, verified_vitality_score, salud_schema_version, signature_proof_hex) VALUES ($1, 'pk', 1, '1.0', 'sig')")
        .bind("e".repeat(64))
        .execute(&mut *tx)
        .await;
    assert!(missing_scope.is_err());

    let mismatched_scope = sqlx::query("INSERT INTO anonymized_user_vitals (tenant_id, data_domain, anonymized_user_hash, public_verification_key_hex, verified_vitality_score, salud_schema_version, signature_proof_hex) VALUES ('tenant-other', 'domain-other', $1, 'pk', 1, '1.0', 'sig')")
        .bind("f".repeat(64))
        .execute(&mut *tx)
        .await;
    assert!(mismatched_scope.is_err());
    tx.rollback().await.unwrap();
}

#[tokio::test]
async fn protected_scope_check_constraint_rejects_null_scope_for_new_rows() {
    let Some(pool) = test_pool().await else {
        return;
    };
    prepare(&pool).await;

    let result = sqlx::query("INSERT INTO anonymized_user_vitals (anonymized_user_hash, public_verification_key_hex, verified_vitality_score, salud_schema_version, signature_proof_hex) VALUES ($1, 'pk', 1, '1.0', 'sig')")
        .bind("g".repeat(64))
        .execute(&pool)
        .await;
    assert!(result.is_err());
}
