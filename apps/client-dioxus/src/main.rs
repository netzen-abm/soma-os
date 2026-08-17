use dioxus::prelude::*;

fn main() {
    #[cfg(feature = "web")]
    dioxus_web::launch(app);

    #[cfg(feature = "mobile")]
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    // Synchronized real-time health telemetry states
    let health_score = use_state(cx, || 88);
    let hrv_ms = use_state(cx, || 64);
    let nym_status = use_state(cx, || "Connected (Blinded)");

    render! {
        div { 
            style: "font-family: system-ui, sans-serif; padding: 24px; background: #0B132B; color: #FFFFFF; min-height: 100vh; max-width: 600px; margin: 0 auto;",
            
            // Header Bar
            header { 
                style: "display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #1C2541; padding-bottom: 16px; margin-bottom: 24px;",
                div {
                    h1 { style: "font-size: 1.5rem; margin: 0; font-weight: 700; letter-spacing: -0.05em;", "🪐 SomaOS" }
                    p { style: "font-size: 0.75rem; color: #5BC0BE; margin: 4px 0 0 0;", "Sovereign Health Vault" }
                }
                span { 
                    style: "font-size: 0.75rem; background: #1C2541; padding: 6px 12px; border-radius: 20px; color: #6FFFE9; border: 1px solid rgba(111,255,233,0.2);",
                    "• {nym_status}" 
                }
            }

            // Main Score Hero Widget
            div { 
                style: "background: linear-gradient(135deg, #1C2541 0%, #111A33 100%); padding: 32px; border-radius: 20px; text-align: center; border: 1px solid #1C2541; margin-bottom: 20px; box-shadow: 0 10px 30px rgba(0,0,0,0.3);",
                h3 { style: "font-size: 0.85rem; color: #A5A1AA; text-transform: uppercase; letter-spacing: 0.1em; margin: 0;", "Vitality Index" }
                h2 { style: "font-size: 4rem; color: #6FFFE9; margin: 12px 0; font-weight: 800; letter-spacing: -0.02em;", "{health_score}" }
                p { style: "font-size: 0.85rem; color: #5BC0BE; margin: 0;", "Salud Protocol Schema Validated" }
            }

            // Secondary Metric Split View
            div { 
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 16px; margin-bottom: 24px;",
                
                div { 
                    style: "background: #1C2541; padding: 20px; border-radius: 16px; border: 1px solid rgba(255,255,255,0.05);",
                    span { style: "font-size: 0.75rem; color: #A5A1AA; display: block; margin-bottom: 4px;", "HRV Telemetry" }
                    strong { style: "font-size: 1.5rem; color: #FFFFFF;", "{hrv_ms} ms" }
                }
                
                div { 
                    style: "background: #1C2541; padding: 20px; border-radius: 16px; border: 1px solid rgba(255,255,255,0.05);",
                    span { style: "font-size: 0.75rem; color: #A5A1AA; display: block; margin-bottom: 4px;", "ZKP Identity Proof" }
                    strong { style: "font-size: 1.1rem; color: #6FFFE9; display: block; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "0x7f3B...Verified" }
                }
            }

            // Context-Aware Bot Suggestion
            div { 
                style: "background: rgba(91, 192, 190, 0.1); border: 1px solid rgba(91, 192, 190, 0.3); padding: 16px; border-radius: 12px; margin-bottom: 24px;",
                h4 { style: "margin: 0 0 6px 0; color: #5BC0BE; font-size: 0.9rem;", "💡 Predictive Intervention" }
                p { style: "margin: 0; font-size: 0.8rem; color: #E0E0E0; line-height: 1.4;", "HRV stability indicates physical recovery. Botanical suggestion: Ashwagandha formula to support systemic resilience." }
            }
        }
    }
}
