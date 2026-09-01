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
pub struct MilletManagementFramework {
    pub target_condition: String,
    pub millet_cycle: String,
    pub administration_form: String,
    pub kashaya_leaves: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PrecisionOncologyTrack {
    pub framework_source: String,
    pub open_data_repository_url: String,
    pub diagnostic_modalities: Vec<String>,
    pub computational_workflows: Vec<String>,
    pub parallel_execution_targets: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BotanicalRecord {
    pub common_name: String,
    pub sanskrit_name: String,
    pub botanical_classification: String,
    pub clinical_pharmacology: String,
    pub drug_interaction_pairings: Vec<DrugInteraction>,
    pub siridhanya_protocols: MilletManagementFramework,
    #[serde(default)]
    pub precision_oncology_tracks: Option<PrecisionOncologyTrack>,
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
    pub structured_management_framework: MilletManagementFramework,
    pub clinical_notes: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClinicalSafetyResponse {
    pub query_match_detected: bool,
    pub clinical_differential_possibilities: Vec<String>,
    pub management_options: Vec<String>,
    pub traditional_millet_framework: Option<MilletManagementFramework>,
    pub open_source_precision_oncology_blueprint: Option<PrecisionOncologyTrack>,
    pub required_safety_label_verification_prompt: String,
}

pub struct BotanicalSearchEngine;

impl BotanicalSearchEngine {
    pub fn search_and_intercept(
        db_path: &str,
        user_raw_query: &str,
        active_user_prescriptions: &[String],
    ) -> Result<Option<InterceptionResult>, Box<dyn std::error::Error>> {
        let db = load_database(db_path)?;
        let query_lowercase = user_raw_query.to_lowercase();

        for botanical in db.botanicals {
            let matches_botanical = query_lowercase.contains(&botanical.common_name.to_lowercase())
                || query_lowercase.contains(&botanical.botanical_classification.to_lowercase());

            let target_condition = &botanical.siridhanya_protocols.target_condition;
            let matches_condition = query_lowercase.contains(&target_condition.to_lowercase())
                || target_condition.split('/').any(|condition| query_lowercase.contains(condition.trim()));

            if matches_botanical || matches_condition {
                let triggered_alert = botanical
                    .drug_interaction_pairings
                    .iter()
                    .find(|interaction| {
                        interaction
                            .example_drugs
                            .iter()
                            .any(|drug| active_user_prescriptions.iter().any(|rx| rx.eq_ignore_ascii_case(drug)))
                    })
                    .cloned();

                return Ok(Some(InterceptionResult {
                    condition_detected: target_condition.clone(),
                    botanical_found: format!("{} ({})", botanical.common_name, botanical.sanskrit_name),
                    safety_alert: triggered_alert,
                    structured_management_framework: botanical.siridhanya_protocols.clone(),
                    clinical_notes: botanical.clinical_pharmacology.clone(),
                }));
            }
        }

        Ok(None)
    }
}

pub struct SomaSearchKernel;

impl SomaSearchKernel {
    pub fn process_sovereign_health_query(
        db_path: &str,
        user_raw_text: &str,
    ) -> Result<ClinicalSafetyResponse, Box<dyn std::error::Error>> {
        let db = load_database(db_path)?;
        let query_lowercase = user_raw_text.to_lowercase();

        for record in db.botanicals {
            let target_condition = record.siridhanya_protocols.target_condition.to_lowercase();
            let contains_keyword = target_condition.split('/').any(|keyword| query_lowercase.contains(keyword.trim()));

            if contains_keyword {
                let differentials = vec![
                    "Potential malignant or benign tissue-growth conditions.".to_string(),
                    "Potential chronic systemic inflammatory or metabolic factors.".to_string(),
                    "Potential immune or microbiome-related factors.".to_string(),
                ];

                let management_options = vec![
                    "Traditional dietary and food-based management framework.".to_string(),
                    "Evidence-based diagnostic and monitoring pathways.".to_string(),
                    "Clinician-guided personalized management options.".to_string(),
                ];

                return Ok(ClinicalSafetyResponse {
                    query_match_detected: true,
                    clinical_differential_possibilities: differentials,
                    management_options,
                    traditional_millet_framework:
                        Some(record.siridhanya_protocols.clone()),
                    open_source_precision_oncology_blueprint:
                        record.precision_oncology_tracks.clone(),
                    required_safety_label_verification_prompt:
                        "Review medication labels and discuss any management change with an appropriate healthcare professional.".to_string(),
                });
            }
        }

        Ok(ClinicalSafetyResponse {
            query_match_detected: false,
            clinical_differential_possibilities: Vec::new(),
            management_options: Vec::new(),
            traditional_millet_framework: None,
            open_source_precision_oncology_blueprint: None,
            required_safety_label_verification_prompt: "No matching condition was identified in the current index."
                .to_string(),
        })
    }
}

fn load_database(db_path: &str) -> Result<BotanicalDatabase, Box<dyn std::error::Error>> {
    let mut file = File::open(Path::new(db_path))?;
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;
    Ok(serde_json::from_str(&json_string)?)
}
