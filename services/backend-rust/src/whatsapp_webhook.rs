use axum::{
    extract::Query,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct VerificationParams {
    #[serde(rename = "hub.mode")]
    mode: String,
    #[serde(rename = "hub.verify_token")]
    verify_token: String,
    #[serde(rename = "hub.challenge")]
    challenge: String,
}

#[derive(Deserialize, Debug)]
struct WebhookPayload {
    object: String,
    entry: Vec<Entry>,
}

#[derive(Deserialize, Debug)]
struct Entry {
    changes: Vec<Change>,
}

#[derive(Deserialize, Debug)]
struct Change {
    value: Value,
    field: String,
}

#[derive(Deserialize, Debug)]
struct Value {
    messages: Option<Vec<Message>>,
}

#[derive(Deserialize, Debug)]
struct Message {
    from: String,
    #[allow(dead_code)]
    id: String,
    text: Option<TextBody>,
}

#[derive(Deserialize, Debug)]
struct TextBody {
    body: String,
}

async fn verify(Query(params): Query<VerificationParams>) -> (StatusCode, String) {
    match env::var("META_VERIFY_TOKEN") {
        Ok(token) if params.mode == "subscribe" && params.verify_token == token => {
            (StatusCode::OK, params.challenge)
        }
        Ok(_) => (StatusCode::FORBIDDEN, "Token mismatch".to_string()),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Webhook verification is not configured".to_string(),
        ),
    }
}

async fn receive(Json(payload): Json<WebhookPayload>) -> StatusCode {
    if payload.object != "whatsapp_business_account" {
        return StatusCode::BAD_REQUEST;
    }
    for entry in payload.entry {
        for change in entry.changes {
            if change.field != "messages" {
                continue;
            }
            if let Some(messages) = change.value.messages {
                for message in messages {
                    if let Some(text) = message.text {
                        println!(
                            "Received WhatsApp message from {}: {}",
                            message.from, text.body
                        );
                    }
                }
            }
        }
    }
    StatusCode::OK
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/v1/whatsapp", get(verify))
        .route("/api/v1/whatsapp", post(receive))
}
