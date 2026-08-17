use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DrugInteraction {
    pub pharmaceutical_class: String,
    pub example_drugs: Vec<String>,
    pub risk_severity: String,
    pub physiological_mechanism: String,
    pub counter_action_protocol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MilletProtocol {
    pub target_condition: String,
    pub millet_cycle: String,
    pub administration_form: String,
    pub kashaya_leaves: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BotanicalRecord {
    pub common_name: String,
    pub sanskrit_name: String,
    pub botanical_classification: String,
    pub clinical_pharmacology: String,
    pub drug_interaction_pairings: Vec<DrugInteraction>,
    pub siridhanya_protocols: MilletProtocol,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BotanicalDatabase {
    pub botanicals: Vec<BotanicalRecord>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InterceptionResult {
    pub condition_detected: String,
    pub botanical_found: String,
    pub safety_alert: Option<DrugInteraction>,
    pub structured_protocol: MilletProtocol,
    pub clinical_notes: String,
}

pub struct BotanicalSearchEngine;

impl BotanicalSearchEngine {
    pub fn search_and_intercept(
        db_path: &str,
        user_raw_query: &str,
        active_user_prescriptions: &[String],
    ) -> Result<Option<InterceptionResult>, Box<dyn std::error::Error>> {
        // Read the knowledge base from disk file parameters
        let mut file = File::open(Path::new(db_path))?;
        let mut json_string = String::new();
        file.read_to_string(&mut json_string)?;
        
        let db: BotanicalDatabase = serde_json::from_str(&json_string)?;
        let query_lowercase = user_raw_query.to_lowercase();

        for botanical in db.botanicals {
            let matches_botanical = query_lowercase.contains(&botanical.common_name.to_lowercase()) 
                || query_lowercase.contains(&botanical.botanical_classification.to_lowercase());
                
            let matches_condition = query_lowercase.contains(&botanical.siridhanya_protocols.target_condition.to_lowercase())
                || botanical.siridhanya_protocols.target_condition.split('/').any(|cond| query_lowercase.contains(cond.trim().to_lowercase().as_str()));

            if matches_botanical || matches_condition {
                // Intercept query and inspect matching user prescriptions for clinical contraindications
                let mut triggered_alert: Option<DrugInteraction> = None;
                for interaction in &botanical.drug_interaction_pairings {
                    for drug in &interaction.example_drugs {
                        if active_user_prescriptions.iter().any(|rx| rx.to_lowercase() == drug.to_lowercase()) {
                            triggered_alert = Some(interaction.clone());
                            break;
                        }
                    }
                }

                return Ok(Some(InterceptionResult {
                    condition_detected: botanical.siridhanya_protocols.target_condition.clone(),
                    botanical_found: format!("{} ({})", botanical.common_name, botanical.sanskrit_name),
                    safety_alert: triggered_alert,
                    structured_protocol: botanical.siridhanya_protocols.clone(),
                    clinical_notes: botanical.clinical_pharmacology.clone(),
                }));
            }
        }
        
        Ok(None)
    }
}
