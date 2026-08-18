// File Path: apps/client-dioxus/src/dialect_selector.rs

use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActiveRegionalDialect {
    English,
    Malayalam,
    Hindi,
    Kannada,
}

pub fn RegionalDialectControlCockpit(cx: Scope) -> Element {
    // Instantiate a reactive local state hook handling the user's active choice
    let selected_dialect = use_state(cx, || ActiveRegionalDialect::English);

    // Formulate a helper method to map text primitives dynamically based on active selection states
    let display_title = match selected_dialect.get() {
        ActiveRegionalDialect::English => "Select Dashboard Dialect Language",
        ActiveRegionalDialect::Malayalam => "ഡാഷ്‌ബോർഡ് ഭാഷ തിരഞ്ഞെടുക്കുക",
        ActiveRegionalDialect::Hindi => "डैशबोर्ड भाषा का चयन करें",
        ActiveRegionalDialect::Kannada => "ಡ್ಯಾಶ್‌ಬೋರ್ಡ್ ಭಾಷೆಯನ್ನು ಆಯ್ಕೆಮಾಡಿ",
    };

    let selected_badge_subtext = match selected_dialect.get() {
        ActiveRegionalDialect::English => "🔒 Sovereign Edge Data Privacy Enforced",
        ActiveRegionalDialect::Malayalam => "🔒 പരമാധികാര ഡാറ്റാ സ്വകാര്യത ഉറപ്പാക്കിയിരിക്കുന്നു",
        ActiveRegionalDialect::Hindi => "🔒 संप्रभु डेटा गोपनीयता लागू की गई",
        ActiveRegionalDialect::Kannada => "🔒 ಸಾರ್ವಭೌಮ ಡೇಟಾ ಗೌಪ್ಯತೆಯನ್ನು ಜಾರಿಗೊಳಿಸಲಾಗಿದೆ",
    };

    render! {
        div { 
            style: "background: #1C2541; padding: 20px; border-radius: 12px; max-width: 450px; margin: 16px auto; border: 1px solid rgba(255,255,255,0.05); font-family: system-ui, sans-serif;",
            
            // Component Title Label
            h4 { style: "color: #FFFFFF; font-size: 0.95rem; margin: 0 0 4px 0; font-weight: 700;", "{display_title}" }
            p { style: "color: #5BC0BE; font-size: 0.75rem; margin: 0 0 16px 0;", "{selected_badge_subtext}" }

            // Horizontal Button Grid Track Selector Matrix
            div { style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 10px;",
                
                // 🔘 OPTION 1: English
                button {
                    onclick: move |_| selected_dialect.set(ActiveRegionalDialect::English),
                    style: format!(
                        "padding: 12px; border-radius: 8px; border: none; font-weight: 600; font-size: 0.85rem; cursor: pointer; transition: all 0.2s ease; {}",
                        if *selected_dialect.get() == ActiveRegionalDialect::English { "background: #5BC0BE; color: #0B132B;" } else { "background: #111A33; color: #A5A1AA;" }
                    ),
                    "English"
                }

                // 🔘 OPTION 2: Malayalam (മലയാളം)
                button {
                    onclick: move |_| selected_dialect.set(ActiveRegionalDialect::Malayalam),
                    style: format!(
                        "padding: 12px; border-radius: 8px; border: none; font-weight: 600; font-size: 0.85rem; cursor: pointer; transition: all 0.2s ease; {}",
                        if *selected_dialect.get() == ActiveRegionalDialect::Malayalam { "background: #5BC0BE; color: #0B132B;" } else { "background: #111A33; color: #A5A1AA;" }
                    ),
                    "മലയാളം (Malayalam)"
                }

                // 🔘 OPTION 3: Hindi (हिंदी)
                button {
                    onclick: move |_| selected_dialect.set(ActiveRegionalDialect::Hindi),
                    style: format!(
                        "padding: 12px; border-radius: 8px; border: none; font-weight: 600; font-size: 0.85rem; cursor: pointer; transition: all 0.2s ease; {}",
                        if *selected_dialect.get() == ActiveRegionalDialect::Hindi { "background: #5BC0BE; color: #0B132B;" } else { "background: #111A33; color: #A5A1AA;" }
                    ),
                    "हिंदी (Hindi)"
                }

                // 🔘 OPTION 4: Kannada (ಕನ್ನಡ)
                button {
                    onclick: move |_| selected_dialect.set(ActiveRegionalDialect::Kannada),
                    style: format!(
                        "padding: 12px; border-radius: 8px; border: none; font-weight: 600; font-size: 0.85rem; cursor: pointer; transition: all 0.2s ease; {}",
                        if *selected_dialect.get() == ActiveRegionalDialect::Kannada { "background: #5BC0BE; color: #0B132B;" } else { "background: #111A33; color: #A5A1AA;" }
                    ),
                    "ಕನ್ನಡ (Kannada)"
                }
            }
        }
    }
}
