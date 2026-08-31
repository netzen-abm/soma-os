use axum::{extract::Query, http::StatusCode, routing::{get, post}, Json, Router};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct MessengerHandshakeParams {
    #[serde(rename = "hub.mode")]
    mode: String,
    #[serde(rename = "hub.verify_token")]
    verify_token: String,
    #[serde(rename = "hub.challenge")]
    challenge: String,
}

#[derive(Deserialize, Debug)]
pub struct MessengerWebhookPayload {
    pub object: String,
    pub entry: Vec<MessengerEntry>,
}

#[derive(Deserialize, Debug)]
pub struct MessengerEntry {
    #[allow(dead_code)] pub id: String,
    #[allow(dead_code)] pub time: u64,
    pub messaging: Vec<MessengerMessagingEvent>,
}

#[derive(Deserialize, Debug)]
pub struct MessengerMessagingEvent {
    pub sender: MessengerSenderWrapper,
    pub message: Option<MessengerMessageContent>,
}

#[derive(Deserialize, Debug)]
pub struct MessengerSenderWrapper { pub id: String }

#[derive(Deserialize, Debug)]
pub struct MessengerMessageContent {
    #[allow(dead_code)] pub mid: String,
    pub text: Option<String>,
}

async fn verify_messenger_webhook(Query(params): Query<MessengerHandshakeParams>) -> (StatusCode, String) {
    match env::var("META_VERIFY_TOKEN") {
        Ok(token) if params.mode == "subscribe" && params.verify_token == token => (StatusCode::OK, params.challenge),
        Ok(_) => (StatusCode::FORBIDDEN, "Handshake token refused".to_string()),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Webhook verification is not configured".to_string()),
    }
}

async fn inbound_messenger_message_router(Json(payload): Json<MessengerWebhookPayload>) -> StatusCode {
    if payload.object != "page" { return StatusCode::BAD_REQUEST; }
    for entry in payload.entry {
        for event in entry.messaging {
            if let Some(message_content) = event.message {
                if let Some(user_text) = message_content.text {
                    println!("Received Messenger message from {}: {}", event.sender.id, user_text);
                }
            }
        }
    }
    StatusCode::OK
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/v1/messenger", get(verify_messenger_webhook))
        .route("/api/v1/messenger", post(inbound_messenger_message_router))
}
