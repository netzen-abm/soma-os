use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use std::{env, sync::Arc};
use tokio::net::TcpListener;

mod ambali_timer;
mod backup_scheduler;
mod bot_menus;
mod canonical_authorization;
mod canonical_vault_authorizer;
mod compliance_shield;
mod compressor;
mod crypto;
mod data_parser;
mod db_layer;
mod device_sync;
mod health_state_evidence_link;
mod health_state_repository;
mod key_restoration;
mod legacy_promotion_preflight;
mod local_health_vault;
mod local_health_vault_storage;
mod local_longitudinal_observation_repository;
mod longitudinal_observation_repository;
mod messenger_webhook;
mod meta_outbound;
mod mnemonic_validator;
mod nostr_client;
mod personal_health_record_repository;
mod privacy_policy;
mod protected_db_context;
mod shared_infrastructure;
mod vault_exporter;
mod vault_importer;
mod whatsapp_webhook;

#[cfg(test)]
mod db_postgres_integration_test;

use ambali_timer::FermentationOrchestrator;
use bot_menus::MultiChannelMenuController;
use compliance_shield::SovereignComplianceShield;
use db_layer::SomaDatabaseManager;
use meta_outbound::MetaOutboundRunner;

struct AppState {
    menu_controller: tokio::sync::Mutex<MultiChannelMenuController>,
    #[allow(dead_code)]
    db_manager: SomaDatabaseManager,
}

#[derive(serde::Deserialize)]
struct UnifiedChannelMessage {
    #[allow(dead_code)]
    channel_source: String,
    sender_id: String,
    text_payload: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("SomaOS Secure Server Runtime starting...");

    let db_url = env::var("DATABASE_URL")?;
    let wa_token = env::var("META_WHATSAPP_TOKEN")?;
    let wa_phone_id = env::var("META_PHONE_ID")?;
    let fb_token = env::var("META_MESSENGER_TOKEN")?;

    let pool = PgPoolOptions::new().max_connections(5).connect(&db_url).await?;
    let db_manager = SomaDatabaseManager::new(pool);

    let outbound_runner = MetaOutboundRunner::new(&wa_token, &wa_phone_id, &fb_token);
    let orchestrator = FermentationOrchestrator::new(outbound_runner);
    let menu_controller = MultiChannelMenuController::new(orchestrator);

    let shared_state = Arc::new(AppState {
        menu_controller: tokio::sync::Mutex::new(menu_controller),
        db_manager,
    });

    let app = axum::Router::new()
        .route("/", get(|| async { "SomaOS is running securely." }))
        .route("/webhook", post(process_unified_bot_webhook))
        .with_state(shared_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("SomaOS listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn process_unified_bot_webhook(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UnifiedChannelMessage>,
) -> (StatusCode, Json<serde_json::Value>) {
    let mut controller = state.menu_controller.lock().await;
    let raw_response = controller
        .evaluate_channel_input(&payload.sender_id, &payload.text_payload)
        .await;
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "success",
            "reply": raw_response
        })),
    )
}

#[allow(dead_code)]
fn compliance_shield() -> SovereignComplianceShield {
    SovereignComplianceShield::default()
}
