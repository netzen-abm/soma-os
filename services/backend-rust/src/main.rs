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
mod key_restoration;
mod legacy_promotion_preflight;
mod local_health_vault;
mod local_health_vault_storage;
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

    // Convert the stateless webhook router into a router whose missing state is AppState.
    let app = whatsapp_webhook::routes()
        .merge(messenger_webhook::routes())
        .with_state(())
        .route("/api/health", get(system_health_check))
        .route("/api/v1/webhook/unified", post(process_unified_bot_webhook))
        .with_state(shared_state);

    let bind_address = env::var("SOMA_BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let listener = TcpListener::bind(&bind_address).await?;

    println!("SomaOS backend listening on {bind_address}");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn system_health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "SomaOS Backend Operational")
}

async fn process_unified_bot_webhook(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UnifiedChannelMessage>,
) -> (StatusCode, String) {
    let mut controller = state.menu_controller.lock().await;
    let raw_response = controller.evaluate_channel_input(&payload.sender_id, &payload.text_payload).await;

    let evaluation = SovereignComplianceShield::enforce_regulatory_compliance_checks(&raw_response);

    if !evaluation.is_permissible_for_delivery {
        eprintln!("Outbound message blocked by compliance shield: {:?}", evaluation.security_compliance_flags);
        return (StatusCode::UNPROCESSABLE_ENTITY, "Response blocked by the SOMA-OS compliance policy.".to_string());
    }

    (StatusCode::OK, format!("{}{}", raw_response, evaluation.appended_regulatory_disclaimer))
}
