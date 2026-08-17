use axum::{
    routing::{post, get},
    Json, Router, http::StatusCode, extract::Query,
};
use serde::{Deserialize, Serialize};

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
    pub id: String,
    pub time: u64,
    pub messaging: Vec<MessengerMessagingEvent>,
}

#[derive(Deserialize, Debug)]
pub struct MessengerMessagingEvent {
    pub sender: MessengerSenderWrapper,
    pub message: Option<MessengerMessageContent>,
}

#[derive(Deserialize, Debug)]
pub struct MessengerSenderWrapper {
    pub id: String, // Page-Scoped ID (PSID) string tracking identifier
}

#[derive(Deserialize, Debug)]
pub struct MessengerMessageContent {
    pub mid: String,
    pub text: Option<String>,
}

const MESSENGER_VERIFY_TOKEN: &str = "SOMAOS_CORE_SECURE_TOKEN_STRING_556621";

// Handles the validation handshake routine incoming from Meta platform developer hooks
async fn verify_messenger_webhook(
    Query(params): Query<MessengerHandshakeParams>,
) -> (StatusCode, String) {
    if params.mode == "subscribe" && params.verify_token == MESSENGER_VERIFY_TOKEN {
        println!("Messenger Verification Handshake completed successfully.");
        (StatusCode::OK, params.challenge)
    } else {
        (StatusCode::FORBIDDEN, "Handshake Token Refused".to_string())
    }
}

// Ingests real-time continuous user text input strings from Facebook Messenger threads
async fn inbound_messenger_message_router(
    Json(payload): Json<MessengerWebhookPayload>,
) -> StatusCode {
    if payload.object == "page" {
        for entry in payload.entry {
            for event in entry.messaging {
                if let Some(message_content) = event.message {
                    let sender_psid = event.sender.id;
                    if let Some(user_text) = message_content.text {
                        println!("Inbound Facebook Messenger text from PSID [{}]: {}", sender_psid, user_text);
                        
                        // Connects straight into centralized non-custodial onboarding flow modules
                        // let response_text = onboarding_engine.process_message(&sender_psid, Some(&user_text));
                    }
                }
            }
        }
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    }
}

// Integrates endpoints seamlessly into unified backend app runtime networks
pub fn inject_messenger_routes(router: Router) -> Router {
    router
        .route("/api/v1/messenger", get(verify_messenger_webhook))
        .route("/api/v1/messenger", post(inbound_messenger_message_router))
}
