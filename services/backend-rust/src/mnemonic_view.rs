use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct MnemonicProps {
    /// The raw 12-word string generated locally on the consumer's system hardware
    pub recovery_phrase: String,
}

pub fn MnemonicPresentationWidget(cx: Scope<MnemonicProps>) -> Element {
    // Split phrase dynamically into structural arrays to support index grid tracking
    let word_list: Vec<&str> = cx.props.recovery_phrase.split_whitespace().collect();

    // Explicit UI selection confirmation memory states
    let is_saved_offline = use_state(cx, || false);

    render! {
        div {
            style: "background: #0B132B; color: #FFFFFF; font-family: system-ui, sans-serif; padding: 32px; border-radius: 16px; max-width: 500px; margin: 40px auto; border: 1px solid rgba(91, 192, 190, 0.2); box-shadow: 0 12px 40px rgba(0,0,0,0.5);",

            // Header Warning Block Group
            div { style: "text-align: center; margin-bottom: 24px;",
                h2 { style: "color: #D9534F; font-size: 1.4rem; font-weight: 700; margin: 0 0 8px 0;", "🚨 Secure Recovery Seed" }
                p { style: "color: #A5A1AA; font-size: 0.85rem; line-height: 1.4; margin: 0;",
                    "Write these 12 words down in order on physical paper. Keep it completely offline. If you lose this key, your health records cannot be recovered by anyone."
                }
            }

            // Interactive 12-Word Visual Grid View Layout Matrix
            div {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px; background: #111A33; padding: 20px; border-radius: 12px; margin-bottom: 24px; border: 1px solid rgba(255,255,255,0.02);",
                for (index, word) in word_list.iter().enumerate() {
                    div {
                        key: "{index}",
                        style: "background: #1C2541; padding: 10px; border-radius: 8px; text-align: left; display: flex; align-items: center; border: 1px solid rgba(91, 192, 190, 0.05);",
                        span { style: "color: #5BC0BE; font-size: 0.7rem; font-weight: 700; width: 20px; display: inline-block;", "{index + 1}" }
                        strong { style: "color: #6FFFE9; font-size: 0.9rem; font-weight: 600;", "{word}" }
                    }
                }
            }

            // User Consent Verification Form Checkbox Component
            label {
                style: "display: flex; align-items: flex-start; gap: 12px; cursor: pointer; background: rgba(255,255,255,0.01); padding: 12px; border-radius: 8px; margin-bottom: 24px;",
                input {
                    r#type: "checkbox",
                    style: "margin-top: 3px; accent-color: #5BC0BE; height: 16px; width: 16px;",
                    checked: "{is_saved_offline}",
                    onchange: move |evt| is_saved_offline.set(evt.value.parse().unwrap_or(false))
                }
                span { style: "color: #E0E0E0; font-size: 0.8rem; line-height: 1.4;",
                    "I certify that I have copied these 12 words down offline. I understand that SomaOS holds zero central records and cannot reset this pass-phrase."
                }
            }

            // Dynamic Action Initialization Button Target Gate
            button {
                disabled: !*is_saved_offline.get(),
                style: format!(
                    "width: 100%; padding: 16px; border-radius: 8px; border: none; font-weight: 700; font-size: 1rem; cursor: pointer; transition: all 0.2s ease; {}",
                    if *is_saved_offline.get() {
                        "background: #5BC0BE; color: #0B132B;"
                    } else {
                        "background: #1C2541; color: #A5A1AA; cursor: not-allowed;"
                    }
                ),
                "LAUNCH HEALTH COCKPIT 🪐"
            }
        }
    }
}
