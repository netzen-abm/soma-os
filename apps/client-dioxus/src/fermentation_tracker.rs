// File Path: apps/client-dioxus/src/fermentation_tracker.rs

use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct TrackerProps {
    /// Elapsed minutes since the unpolished millet porridge batch was mixed and covered
    pub elapsed_minutes: u64,
    /// The name of the millet variety currently undergoing fermentation (e.g., "Kodo", "Little")
    pub millet_name: String,
}

pub fn AmbaliFermentationTrackerWidget(cx: Scope<TrackerProps>) -> Element {
    let total_required_minutes: f64 = 480.0; // 8 hours * 60 minutes specified by Dr. Khadar Vali
    let current_minutes = cx.props.elapsed_minutes as f64;
    
    // Calculate accurate completion percentages capped at absolute boundaries
    let percentage = (current_minutes / total_required_minutes * 100.0).min(100.0).max(0.0);
    
    // Evaluate the matching microbiological growth phase based on elapsed runtime durations
    let (phase_title, phase_color, phase_description) = if current_minutes < 120.0 {
        ("Stage 1: Microbial Incubation", "#5BC0BE", "The unpolished millet structure is cooling. Early native lactic acid bacteria strains are starting to mobilize within the grain matrix.")
    } else if current_minutes >= 120.0 && current_minutes < 360.0 {
        ("Stage 2: Active Probiotic Proliferation", "#6FFFE9", "Rapid multiplication phase. Probiotic microflora colonies are actively consuming complex carbohydrates, producing raw gut-rejuvenating enzymes.")
    } else if current_minutes >= 360.0 && current_minutes < 480.0 {
        ("Stage 3: Peak Acidification & Maturation", "#6FFFE9", "The mixture is approaching maximum probiotic density. Flavor is turning slightly sour, indicating a rich generation of active lactic acid.")
    } else {
        ("Stage 4: Probiotic Elixir Matured", "#5BC0BE", "Maturation complete. Probiotic colony counts match Dr. Khadar Vali's therapeutic recommendations. Ready for non-custodial consumption.")
    };

    render! {
        div { 
            style: "background: #1C2541; padding: 24px; border-radius: 16px; border: 1px solid rgba(255,255,255,0.05); max-width: 450px; margin: 20px auto; font-family: system-ui, sans-serif; box-shadow: 0 8px 32px rgba(0,0,0,0.3);",
            
            // Widget Title Header Group
            div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;",
                div {
                    h3 { style: "color: #FFFFFF; font-size: 1.1rem; font-weight: 700; margin: 0;", "⏳ {cx.props.millet_name} Ambali Monitor" }
                    span { style: "color: #A5A1AA; font-size: 0.75rem;", "Dr. Khadar Vali Protocol Sync" }
                }
                span { 
                    style: format!("font-size: 0.75rem; font-weight: 700; color: {}; background: rgba(255,255,255,0.02); padding: 4px 10px; border-radius: 12px; border: 1px solid rgba(255,255,255,0.05);", phase_color),
                    "• LIVE" 
                }
            }

            // Real-time Linear Visual Tracking Bar
            div { style: "background: #111A33; width: 100%; height: 10px; border-radius: 9px; overflow: hidden; margin-bottom: 12px; border: 1px solid rgba(255,255,255,0.02);",
                div { 
                    style: format!(
                        "background: linear-gradient(90deg, #5BC0BE 0%, {} 100%); width: {:.1}%; height: 100%; border-radius: 9px; transition: width 0.5s ease-in-out;", 
                        phase_color, percentage
                    )
                }
            }

            // Data Readout Stat Overlay Metrics Layout
            div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;",
                span { style: "font-size: 0.8rem; color: #A5A1AA;", "Progress: {percentage:.1}%" }
                strong { style: "font-size: 0.95rem; color: #FFFFFF;", "{cx.props.elapsed_minutes} / 480 mins" }
            }

            // Context-Aware Dynamic Stage Description Area Box
            div { style: "background: #111A33; padding: 16px; border-radius: 12px; border-left: 4px solid #5BC0BE;",
                h4 { style: format!("margin: 0 0 6px 0; font-size: 0.85rem; color: {}; font-weight: 700;", phase_color), "{phase_title}" }
                p { style: "margin: 0; font-size: 0.8rem; color: #E0E0E0; line-height: 1.45;", "{phase_description}" }
            }
        }
    }
}
