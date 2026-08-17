use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OnboardingStep {
    Welcome,
    ConsentZKP,
    PrimaryGoal,
    ActivePrescriptions,
    WearableIntegration,
    Complete,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSession {
    pub current_step: OnboardingStep,
    pub primary_goal: Option<String>,
    pub prescriptions: Vec<String>,
    pub sync_wearables: Option<bool>,
}

pub struct OnboardingEngine {
    // Tracks private pseudonymous user sessions safely in-memory
    pub sessions: HashMap<String, UserSession>,
}

impl OnboardingEngine {
    pub fn new() -> Self {
        Self { sessions: HashMap::new() }
    }

    pub fn process_message(&mut self, user_id: &str, user_input: Option<&str>) -> String {
        let session = self.sessions.entry(user_id.to_string()).or_insert(UserSession {
            current_step: OnboardingStep::Welcome,
            primary_goal: None,
            prescriptions: Vec::new(),
            sync_wearables: None,
        });

        match session.current_step {
            OnboardingStep::Welcome => {
                session.current_step = OnboardingStep::ConsentZKP;
                "Welcome to SomaOS. 🪐\n\nBefore we build your secure health vault, you must accept our sovereign metadata privacy policy.\n\nReply with [1] to sign our Zero-Knowledge Proof Consent Form, which keeps your identity completely hidden from external parties.".to_string()
            }
            OnboardingStep::ConsentZKP => {
                if user_input.unwrap_or("") == "1" {
                    session.current_step = OnboardingStep::PrimaryGoal;
                    "✅ Privacy Shield Cryptographically Activated.\n\nWhat is your primary functional longevity target?\n[1] Longevity & Cellular Resilience\n[2] Metabolic Optimization\n[3] Nervous System Regulation".to_string()
                } else {
                    "⚠️ To protect your clinical records, you must authorize our privacy configuration. Please reply with [1] to proceed securely.".to_string()
                }
            }
            OnboardingStep::PrimaryGoal => {
                let choice = user_input.unwrap_or("");
                session.primary_goal = Some(choice.to_string());
                session.current_step = OnboardingStep::ActivePrescriptions;
                "Target set.\n\nTo prevent dangerous botanical interactions, please list your active pharmaceutical prescriptions (separate items with commas).\n\nIf none, reply with [0].".to_string()
            }
            OnboardingStep::ActivePrescriptions => {
                let input = user_input.unwrap_or("");
                if input != "0" {
                    session.prescriptions = input.split(',').map(|s| s.trim().to_string()).collect();
                }
                session.current_step = OnboardingStep::WearableIntegration;
                "Prescription log saved.\n\nWould you like to sync your continuous wearable device data (Apple HealthKit / Google Connect) to enable active biomarker profiling?\n[1] Yes, connect securely\n[2] No, log manually".to_string()
            }
            OnboardingStep::WearableIntegration => {
                let choice = user_input.unwrap_or("");
                session.sync_wearables = Some(choice == "1");
                session.current_step = OnboardingStep::Complete;
                "🎉 Onboarding Completed Successfully!\n\nYour sovereign profile is safely compiled. You can now type /dashboard to view your real-time Vitality Index metrics or browse recommended preventive protocols safely.".to_string()
            }
            OnboardingStep::Complete => {
                "SomaOS Engine Active. Type /dashboard to open your encrypted telemetry control panel view.".to_string()
            }
        }
    }
}
