// File Path: services/backend-rust/src/localization.rs

use std::collections::HashMap;

pub enum SupportedDialect {
    English,
    Hindi,
    Kannada,
}

pub struct LocalizationEngine {
    // Core map containing translation string matrices
    translation_matrix: HashMap<String, HashMap<String, String>>,
}

impl LocalizationEngine {
    pub fn initialize_localization_profiles() -> Self {
        let mut root_matrix = HashMap::new();

        // 1. English Conversational String Mapping Definitions
        let mut en_map = HashMap::new();
        en_map.insert("welcome_prompt".to_string(), "Welcome to SomaOS. Reply with a single number character to navigate:".to_string());
        en_map.insert("ambali_timer_set".to_string(), "⏳ Ambali fermentation countdown timer initialized successfully for 8 hours.".to_string());
        en_map.insert("medical_disclaimer".to_string(), "Disclaimer: This information is for general educational use only and does not represent direct professional diagnosis.".to_string());
        root_matrix.insert("en".to_string(), en_map);

        // 2. Hindi (हिंदी) Conversational String Mapping Definitions
        let mut hi_map = HashMap::new();
        hi_map.insert("welcome_prompt".to_string(), "SomaOS में आपका स्वागत है। नेविगेट करने के लिए कृपया एक अंक के साथ उत्तर दें:".to_string());
        hi_map.insert("ambali_timer_set".to_string(), "⏳ अम्बाली किण्वन (फ़र्मेंटेशन) टाइमर 8 घंटे के लिए सफलतापूर्वक शुरू हो गया है।".to_string());
        hi_map.insert("medical_disclaimer".to_string(), "अस्वीकरण: यह जानकारी केवल सामान्य शैक्षिक उपयोग के लिए है और चिकित्सा निदान का प्रतिनिधित्व नहीं करती है।".to_string());
        root_matrix.insert("hi".to_string(), hi_map);

        // 3. Kannada (ಕನ್ನಡ) Conversational String Mapping Definitions
        let mut kn_map = HashMap::new();
        kn_map.insert("welcome_prompt".to_string(), "SomaOS ಗೆ ಸುಸ್ವಾಗತ. ನ್ಯಾವಿಗೇಟ್ ಮಾಡಲು ದಯವಿಟ್ಟು ಒಂದು ಸಂಖ್ಯೆಯೊಂದಿಗೆ ಉತ್ತರಿಸಿ:".to_string());
        kn_map.insert("ambali_timer_set".to_string(), "⏳ ಅಂಬಲಿ ಹುದುಗುವಿಕೆ (ಫರ್ಮೆಂಟೇಶನ್) ಟೈಮರ್ 8 ಗಂಟೆಗಳ ಕಾಲ ಯಶಸ್ವಿಯಾಗಿ ಪ್ರಾರಂಭಿಸಲಾಗಿದೆ.".to_string());
        kn_map.insert("medical_disclaimer".to_string(), "ಹಕ್ಕುತ್ಯಾಗ: ಈ ಮಾಹಿತಿಯು ಸಾಮಾನ್ಯ ಶೈಕ್ಷಣಿಕ ಬಳಕೆಗೆ ಮಾತ್ರ ಮತ್ತು ವೈದ್ಯಕೀಯ ರೋಗನಿರ್ಣಯವನ್ನು ಪ್ರತಿನಿಧಿಸುವುದಿಲ್ಲ.".to_string());
        root_matrix.insert("kn".to_string(), kn_map);

        Self { translation_matrix: root_matrix }
    }

    /// Fetches localized string responses using structural lang-code keys
    pub fn retrieve_localized_string(&self, language_code: &str, text_key: &str) -> String {
        if let Some(dialect_bucket) = self.translation_matrix.get(language_code) {
            if let Some(translated_text) = dialect_bucket.get(text_key) {
                return translated_text.clone();
            }
        }
        // Graceful fallback to default English template strings if unmatched
        "SomaOS Engine: Command Processed.".to_string()
    }
}
