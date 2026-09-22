use super::postgres_test_support::{assume_persistence_role, prepare, test_pool};
#[tokio::test]
async fn application_persistence_role_cannot_execute_legacy_control_plane_functions() {
    let Some(pool) = test_pool().await else {
        return;
    };
    prepare(&pool).await;

    let persistence_preflight = sqlx::query_scalar::<_, bool>(
        "SELECT has_function_privilege('somaos_persistence', 'public.soma_legacy_promotion_preflight()', 'EXECUTE')",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let persistence_promotion = sqlx::query_scalar::<_, bool>(
        "SELECT has_function_privilege('somaos_persistence', 'public.soma_apply_legacy_promotion(INTEGER, BIGINT, TEXT, TEXT, TEXT)', 'EXECUTE')",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let migrator_preflight = sqlx::query_scalar::<_, bool>(
        "SELECT has_function_privilege('somaos_migrator', 'public.soma_legacy_promotion_preflight()', 'EXECUTE')",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let executor_promotion = sqlx::query_scalar::<_, bool>(
        "SELECT has_function_privilege('somaos_legacy_promotion_executor', 'public.soma_apply_legacy_promotion(INTEGER, BIGINT, TEXT, TEXT, TEXT)', 'EXECUTE')",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert!(!persistence_preflight);
    assert!(!persistence_promotion);
    assert!(migrator_preflight);
    assert!(executor_promotion);
}

#[tokio::test]
async fn untrusted_db_role_cannot_establish_trusted_context() {
    let Some(pool) = test_pool().await else {
        return;
    };
    prepare(&pool).await;

    let mut connection = pool.acquire().await.unwrap();
    sqlx::query("SET ROLE somaos_untrusted_test")
        .execute(&mut *connection)
        .await
        .expect("test connection must be able to assume the untrusted test role");

    let direct_call = sqlx::query("SELECT public.soma_set_protected_scope($1, $2)")
        .bind("attacker-tenant")
        .bind("attacker-domain")
        .execute(&mut *connection)
        .await;
    assert!(direct_call.is_err(), "untrusted DB role must not invoke the trusted context function");

    sqlx::query("RESET ROLE").execute(&mut *connection).await.unwrap();
}

#[tokio::test]
async fn trusted_context_entry_point_binds_transaction_local_scope() {
    let Some(pool) = test_pool().await else {
        return;
    };
    prepare(&pool).await;

    let mut tx = pool.begin().await.unwrap();
    assume_persistence_role(&mut tx).await;
    sqlx::query("SELECT public.soma_set_protected_scope($1, $2)")
        .bind("tenant-a")
        .bind("domain-a")
        .execute(&mut *tx)
        .await
        .unwrap();

    let row =
        sqlx::query("SELECT current_user, current_setting('soma.tenant_id'), current_setting('soma.data_domain')")
            .fetch_one(&mut *tx)
            .await
            .unwrap();
    assert_eq!(row.get::<String, _>(0), "somaos_persistence");
    assert_eq!(row.get::<String, _>(1), "tenant-a");
    assert_eq!(row.get::<String, _>(2), "domain-a");
    tx.rollback().await.unwrap();
}

#[tokio::test]
async fn transaction_local_scope_survives_pool_reuse_without_leakage() {
    let Some(pool) = test_pool().await else {
        return;
    };
    prepare(&pool).await;

    let mut tx = pool.begin().await.unwrap();
    assume_persistence_role(&mut tx).await;
    sqlx::query("SELECT public.soma_set_protected_scope($1, $2)")
        .bind("tenant-a")
        .bind("domain-a")
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    let row = sqlx::query(
        "SELECT current_setting('soma.tenant_id', true) AS tenant_id, current_setting('soma.data_domain', true) AS data_domain",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(row.get::<Option<String>, _>("tenant_id").is_none());
    assert!(row.get::<Option<String>, _>("data_domain").is_none());
}
