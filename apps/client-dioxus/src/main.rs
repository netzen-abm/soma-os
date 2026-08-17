use dioxus::prelude::*;

fn main() {
    #[cfg(feature = "web")]
    dioxus_web::launch(app);

    #[cfg(feature = "mobile")]
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let health_score = use_state(cx, || 88);

    render! {
        div { class: "soma-container", style: "font-family: sans-serif; padding: 20px; background: #0B132B; color: #FFFFFF; min-height: 100vh;",
            header { class: "soma-header", style: "border-bottom: 1px solid #5BC0BE; padding-bottom: 10px;",
                h1 { "SomaOS Command Center" }
                span { style: "color: #5BC0BE; font-size: 0.8rem;", "🔒 Metadata Blinded via NYM Mixnet" }
            }
            main { style: "margin-top: 30px;",
                div { class: "metric-card", style: "background: #1C2541; padding: 20px; border-radius: 12px; margin-bottom: 15px;",
                    h3 { "Vitality Score" }
                    h2 { style: "color: #6FFFE9; font-size: 2.5rem; margin: 5px 0;", "{health_score}" }
                    p { style: "font-size: 0.9rem; color: #A5A1AA;", "Biomarkers consolidated via Salud Protocol standard." }
                }
            }
        }
    }
}
