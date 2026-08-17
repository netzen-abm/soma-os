use serde::{Serialize, Deserialize};
use reqwest::{Client, Response};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct WhatsAppTextObject {
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhatsAppOutboundMessage {
    pub messaging_product: String, // Always "whatsapp"
    pub to: String,                // Recipient phone number with country code
    pub r#type: String,            // Always "text"
    pub text: WhatsAppTextObject,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessengerRecipient {
    pub id: String,                // Page Scoped User ID (PSID)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessengerMessageObject {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessengerOutboundMessage {
    pub recipient: MessengerRecipient,
    pub message: MessengerMessageObject,
}

pub struct MetaOutboundRunner {
    client: Client,
    whatsapp_token: String,
    whatsapp_phone_number_id: String,
    messenger_token: String,
}

impl MetaOutboundRunner {
    pub fn new(wa_token: &str, wa_phone_id: &str, fb_token: &str) -> Self {
        Self {
            client: Client::new(),
            whatsapp_token: wa_token.to_string(),
            whatsapp_phone_number_id: wa_phone_id.to_string(),
            messenger_token: fb_token.to_string(),
        }
    }

    // Dispatches a direct text notification to a user's WhatsApp device
    pub async fn send_whatsapp_text(&self, to_phone: &str, message_body: &str) -> Result<Response, Box<dyn Error>> {
        let url = format!(
            "https://facebook.com{}/messages",
            self.whatsapp_phone_number_id
        );

        let payload = WhatsAppOutboundMessage {
            messaging_product: "whatsapp".to_string(),
            to: to_phone.to_string(),
            r#type: "text".to_string(),
            text: WhatsAppTextObject { body: message_body.to_string() },
        };

        let response = self.client.post(&url)
            .bearer_auth(&self.whatsapp_token)
            .json(&payload)
            .send()
            .await?;

        Ok(response)
    }

    // Dispatches a direct text notification back to a Facebook Messenger thread
    pub async fn send_messenger_text(&self, recipient_psid: &str, message_body: &str) -> Result<Response, Box<dyn Error>> {
        let url = format!(
            "https://facebook.comme/messages?access_token={}",
            self.messenger_token
        );

        let payload = MessengerOutboundMessage {
            recipient: MessengerRecipient { id: recipient_psid.to_string() },
            message: MessengerMessageObject { text: message_body.to_string() },
        };

        let response = self.client.post(&url)
            .json(&payload)
            .send()
            .await?;

        Ok(response)
    }
}
