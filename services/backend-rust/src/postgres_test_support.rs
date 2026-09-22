use sqlx::{postgres::PgPoolOptions, PgPool};
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
    include_str!("../../../database/migrations/0010_legacy_promotion_preflight.sql"),
    include_str!("../../../database/migrations/0012_legacy_promotion_executor_privilege_isolation.sql"),
    include_str!("../../../database/migrations/0013_legacy_preflight_privilege_isolation.sql"),
];

static PREPARED: OnceCell<()> = OnceCell::const_new();

pub(crate) async fn test_pool() -> Option<PgPool> {
    let url = std::env::var("SOMA_TEST_DATABASE_URL").ok().filter(|v| !v.trim().is_empty())?;
    Some(PgPoolOptions::new().max_connections(4).connect(&url).await.expect("SOMA_TEST_DATABASE_URL must point to a reachable PostgreSQL test database"))
}

pub(crate) async fn prepare(pool: &PgPool) {
    PREPARED.get_or_init(|| async {
        for (index, migration) in MIGRATIONS.iter().enumerate() {
            sqlx::raw_sql(migration).execute(pool).await.expect("protected-data test migrations must apply cleanly");
            if index == 4 {
                sqlx::query("INSERT INTO anonymized_user_vitals (anonymized_user_hash, public_verification_key_hex, verified_vitality_score, salud_schema_version, signature_proof_hex) VALUES ('legacy-null-scope-fixture', 'pk', 1, '1.0', 'sig')")
                    .execute(pool).await.expect("legacy fixture must be seedable before protected-row constraint");
            }
        }
    }).await;
}

pub(crate) async fn assume_persistence_role(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>) {
    sqlx::query("SET LOCAL ROLE somaos_persistence").execute(&mut **tx).await.expect("test connection must assume persistence role");
}
