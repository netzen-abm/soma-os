use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VerificationChallenge {
    pub word_index_target: usize, // e.g., index position 3 representing the 4th phrase block word
    pub user_provided_string: String,
}

pub struct MnemonicValidator;

impl MnemonicValidator {
    /// Validates user confirmation inputs against the original derived entropy mnemonic seed string array
    pub fn verify_mnemonic_onboarding_compliance(
        original_generated_phrase: &str,
        user_verification_responses: &[VerificationChallenge],
    ) -> bool {
        // Split the original trusted key string into separate elements
        let original_words: Vec<&str> = original_generated_phrase.split_whitespace().collect();
        
        // Assert basic phrase scale boundaries are preserved
        if original_words.len() != 12 {
            println!("⚠️ Invalid internal mnemonic tracking configuration scale matrix detected.");
            return false;
        }

        // Loop through challenge targets to confirm strict data parity matching matches
        for challenge in user_verification_responses {
            if challenge.word_index_target >= original_words.len() {
                return false; // Out-of-bounds parameter entry defense
            }

            let expected_word = original_words[challenge.word_index_target].trim().to_lowercase();
            let submitted_word = challenge.user_provided_string.trim().to_lowercase();

            if expected_word != submitted_word {
                println!(
                    "❌ Cryptographic identity validation mismatched at word position index marker: {}", 
                    challenge.word_index_target + 1
                );
                return false; // Instant failure loop break
            }
        }

        println!("✅ Compliance phrase verified successfully. Unlocking sovereign health dashboard view state.");
        true
    }
}
