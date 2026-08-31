use crate::meta_outbound::MetaOutboundRunner;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, Duration}; // Hooks into your established Meta Cloud API client

#[derive(Clone, Debug)]
pub struct AmbaliTimer {
    pub user_phone: String,
    pub millet_type: String,
    pub start_epoch_secs: u64,
    pub target_duration_hours: u64,
}

pub struct FermentationOrchestrator {
    active_timers: HashMap<String, AmbaliTimer>,
    outbound_runner: MetaOutboundRunner,
}

impl FermentationOrchestrator {
    pub fn new(runner: MetaOutboundRunner) -> Self {
        Self {
            active_timers: HashMap::new(),
            outbound_runner: runner,
        }
    }

    // Spin up an isolated tracking worker for a user's fermenting porridge batch
    pub async fn trigger_fermentation_timer(&mut self, user_phone: &str, millet: &str) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // 8 hours fermentation standard defined by Dr. Khadar Vali's protocols
        let fermentation_hours = 8;

        let timer_profile = AmbaliTimer {
            user_phone: user_phone.to_string(),
            millet_type: millet.to_string(),
            start_epoch_secs: current_time,
            target_duration_hours: fermentation_hours,
        };

        self.active_timers
            .insert(user_phone.to_string(), timer_profile.clone());
        println!(
            "🚀 Fermentation tracker initialized for {}. Millet: {}",
            user_phone, millet
        );

        // Spawn a background thread worker to wait out the maturation period asynchronously
        let runner_clone = self.outbound_runner.clone();
        let phone = user_phone.to_string();
        let millet_name = millet.to_string();

        tokio::spawn(async move {
            // In production code, multiply hours * 3600. Using short delays during system integration tests.
            let delay_duration = Duration::from_secs(fermentation_hours * 3600);
            sleep(delay_duration).await;

            let warning_template = format!(
                "🪐 *SomaOS Health Alert* 🪐\n\nYour unpolished *{} Ambali* has reached its peak 8-hour fermentation window!\n\nDr. Khadar Vali's protocol confirms the probiotic colony is fully mature. The gut-rejuvenating Lactobacillus microflora are active. Your sovereign elixir is ready for consumption.",
                millet_name
            );

            match runner_clone
                .send_whatsapp_text(&phone, &warning_template)
                .await
            {
                Ok(_) => println!(
                    "✅ WhatsApp fermentation alert sent successfully to {}",
                    phone
                ),
                Err(e) => eprintln!(
                    "❌ Outbound WhatsApp timer notification channel error: {:?}",
                    e
                ),
            }
        });
    }
}
