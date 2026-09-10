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
mod compliance_shield;
mod compressor;
mod crypto;
mod data_parser;
mod db_layer;
mod device_sync;
mod key_restoration;
mod legacy_promotion_preflight;
mod local_health_vault;
mod local_health_vault_storage;
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

    // Convert the stateless webhook router into a router whose missing state is AppState.
    let app = whatsapp_webhook::routes()
        .merge(messenger_webhook::routes())
        .with_state(shared_state.clone())
        .route("/health", get(|| async { StatusCode::OK }))
        .route("/webhook/unified", post(unified_channel_handler));

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn unified_channel_handler(
    State(state): State<Arc<AppState>>,
    Json(message): Json<UnifiedChannelMessage>,
) -> StatusCode {
    let mut controller = state.menu_controller.lock().await;
    controller.handle_message(&message.sender_id, &message.text_payload);
    StatusCode::OK
}

#[allow(dead_code)]
fn _keep_compliance_shield_linked() {
    let _ = SovereignComplianceShield::new();
}
