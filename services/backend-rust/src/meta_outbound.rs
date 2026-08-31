use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::{env, error::Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct WhatsAppTextObject {
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhatsAppOutboundMessage {
    pub messaging_product: String,
    pub to: String,
    pub r#type: String,
    pub text: WhatsAppTextObject,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessengerRecipient {
    pub id: String,
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

#[derive(Clone)]
pub struct MetaOutboundRunner {
    client: Client,
    whatsapp_token: String,
    whatsapp_phone_number_id: String,
    messenger_token: String,
    graph_api_version: String,
}

impl MetaOutboundRunner {
    pub fn new(wa_token: &str, wa_phone_id: &str, fb_token: &str) -> Self {
        Self {
            client: Client::new(),
            whatsapp_token: wa_token.to_string(),
            whatsapp_phone_number_id: wa_phone_id.to_string(),
            messenger_token: fb_token.to_string(),
            graph_api_version: env::var("META_GRAPH_API_VERSION")
                .expect("META_GRAPH_API_VERSION must be configured"),
        }
    }

    pub async fn send_whatsapp_text(
        &self,
        to_phone: &str,
        message_body: &str,
    ) -> Result<Response, Box<dyn Error>> {
        let url = format!(
            "https://graph.facebook.com/{}/{}/messages",
            self.graph_api_version, self.whatsapp_phone_number_id
        );
        let payload = WhatsAppOutboundMessage {
            messaging_product: "whatsapp".to_string(),
            to: to_phone.to_string(),
            r#type: "text".to_string(),
            text: WhatsAppTextObject {
                body: message_body.to_string(),
            },
        };
        Ok(self
            .client
            .post(url)
            .bearer_auth(&self.whatsapp_token)
            .json(&payload)
            .send()
            .await?)
    }

    pub async fn send_messenger_text(
        &self,
        recipient_psid: &str,
        message_body: &str,
    ) -> Result<Response, Box<dyn Error>> {
        let url = format!(
            "https://graph.facebook.com/{}/me/messages",
            self.graph_api_version
        );
        let payload = MessengerOutboundMessage {
            recipient: MessengerRecipient {
                id: recipient_psid.to_string(),
            },
            message: MessengerMessageObject {
                text: message_body.to_string(),
            },
        };
        Ok(self
            .client
            .post(url)
            .bearer_auth(&self.messenger_token)
            .json(&payload)
            .send()
            .await?)
    }
}
