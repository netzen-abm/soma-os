use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientSearchQuery {
    pub raw_query: String,
    pub user_prescriptions: Vec<String>,
}

pub fn use_botanical_search(cx: &ScopeState) -> &UseCoroutine<ClientSearchQuery> {
    use_coroutine(cx, |mut rx: UnboundedReceiver<ClientSearchQuery>| {
        to_owned![];
        async move {
            let client = reqwest::Client::new();
            while let Some(query_packet) = rx.next().await {
                println!("Streaming out network search request packet over secure proxy loops...");
                
                let target_endpoint = "http://127.0.0";
                let response = client.post(target_endpoint)
                    .json(&query_packet)
                    .send()
                    .await;

                match response {
                    Ok(res) => {
                        if let Ok(text) = res.text().await {
                            println!("Sovereign Search Data Fetched Successfully: {:?}", text);
                            // Fire update patterns to render state tree widgets dynamically here
                        }
                    }
                    Err(e) => eprintln!("Network routing infrastructure failure footprint: {:?}", e),
                }
            }
        }
    })
}
