// File Path: apps/client-dioxus/src/multi_modal_view.rs

use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct MultiModalDashboardProps {
    pub active_query_condition: String,
    pub raw_millet_cycle: String,
    pub kashaya_leaves: Vec<String>,
    pub genomic_s3_url: String,
    pub ayush_monograph: String,
    pub raw_diet_rules: String,
    pub emergency_flags: Vec<String>,
}

pub fn MultiModalSovereignCockpit(cx: Scope<MultiModalDashboardProps>) -> Element {
    render! {
        div { style: "background: #0B132B; color: #FFFFFF; font-family: system-ui, sans-serif; padding: 24px; min-height: 100vh;",
            
            // Global Integrated Navigation Top Strip
            header { style: "border-bottom: 1px solid #1C2541; padding-bottom: 16px; margin-bottom: 24px; display: flex; justify-content: space-between; align-items: center;",
                div {
                    h1 { style: "font-size: 1.6rem; font-weight: 800; color: #FFFFFF; margin: 0; letter-spacing: -0.03em;", "🪐 SomaOS Cross-Source Intelligence Console" }
                    p { style: "color: #5BC0BE; font-size: 0.8rem; margin: 4px 0 0 0;", "Active Target Insight Field: {cx.props.active_query_condition}" }
                }
                span { style: "background: rgba(111,255,233,0.1); border: 1px solid #6FFFE9; color: #6FFFE9; font-size: 0.75rem; padding: 6px 12px; border-radius: 20px;", "🔒 Local Encryption Active" }
            }

            // 🛡️ EMERGENCY ESCALATION STATUS HEADER WIDGET (August AI Emulation Feature)
            div { style: "background: rgba(217, 83, 79, 0.1); border: 1px solid #D9534F; padding: 16px; border-radius: 12px; margin-bottom: 24px;",
                h4 { style: "color: #D9534F; margin: 0 0 6px 0; font-size: 0.9rem; font-weight: 700;", "⚠️ Automated Symptom Escalation Guardian" }
                p { style: "margin: 0 0 10px 0; font-size: 0.8rem; color: #E0E0E0; line-height: 1.4;", "If your current baseline symptoms match any of the following physiological escalation markers, seek immediate professional emergency evaluation:" }
                ul { style: "margin: 0; padding-left: 20px; font-size: 0.8rem; color: #FF8884;",
                    for marker in cx.props.emergency_flags.iter() {
                        li { "{marker}" }
                    }
                }
            }

            // Un-Censorable Digital E-Book Library Insights Banner
            div { style: "background: #111A33; border: 1px solid rgba(255,255,255,0.05); padding: 14px; border-radius: 8px; margin-bottom: 24px; font-size: 0.8rem; color: #A5A1AA;",
                strong { style: "color: #5BC0BE;", "📖 Open E-Book Compile Engine: " }
                span { "{cx.props.raw_diet_rules}" }
            }

            // Split Cockpit Core Dashboard Matrix
            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px;",
                
                // LEFT DASHBOARD COLUMN: Traditional Medicine & AYUSH Protocols
                div { style: "background: #1C2541; padding: 24px; border-radius: 16px; border: 1px solid rgba(255,255,255,0.02);",
                    h3 { style: "color: #5BC0BE; font-size: 1.1rem; border-bottom: 1px solid rgba(91,192,190,0.2); padding-bottom: 8px; margin-top: 0;", "🌿 Natural Medicine & Government of India Monograph" }
                    
                    h4 { style: "color: #FFFFFF; font-size: 0.9rem; margin: 16px 0 6px 0;", "Millet Rotation Cycle (Siridhanya Porridge)" }
                    p { style: "font-size: 0.85rem; color: #E0E0E0; background: #0B132B; padding: 12px; border-radius: 8px; margin: 0; line-height: 1.4;", "{cx.props.raw_millet_cycle}" }
                    
                    h4 { style: "color: #FFFFFF; font-size: 0.9rem; margin: 16px 0 6px 0;", "Target Leaf Kashaya Extracts" }
                    ul { style: "margin: 0; padding-left: 20px; font-size: 0.85rem; color: #6FFFE9;",
                        for leaf in cx.props.kashaya_leaves.iter() {
                            li { "{leaf}" }
                        }
                    }

                    h4 { style: "color: #FFFFFF; font-size: 0.9rem; margin: 16px 0 6px 0;", "Ministry of AYUSH Clinical Evaluation Research Summary" }
                    p { style: "font-size: 0.8rem; color: #A5A1AA; line-height: 1.4; margin: 0;", "{cx.props.ayush_monograph}" }
                },

                // RIGHT DASHBOARD COLUMN: Founder-Mode Precision Oncology
                div { style: "background: #1C2541; padding: 24px; border-radius: 16px; border: 1px solid rgba(255,255,255,0.02);",
                    h3 { style: "color: #6FFFE9; font-size: 1.1rem; border-bottom: 1px solid rgba(111,255,233,0.2); padding-bottom: 8px; margin-top: 0;", "🧬 Founder-Mode Precision Oncology Tracks" }
                    
                    h4 { style: "color: #FFFFFF; font-size: 0.9rem; margin: 16px 0 6px 0;", "Public Multi-Omic Dataset Source Repository" }
                    code { style: "font-size: 0.75rem; color: #A5A1AA; display: block; background: #0B132B; padding: 10px; border-radius: 6px; word-break: break-all; border: 1px solid rgba(255,255,255,0.05);", "{cx.props.genomic_s3_url}" }
                    
                    h4 { style: "color: #FFFFFF; font-size: 0.9rem; margin: 16px 0 6px 0;", "Computational Action Pipelines" }
                    ul { style: "margin: 0; padding-left: 20px; font-size: 0.85rem; color: #E0E0E0;",
                        li { "Whole Genome Sequencing Variant Calling (Sarek Pipelines)" }
                        li { "Neoantigen Prediction Modeling (pVACseq Analytics Loops)" }
                        li { "Structural Peptide-MHC Modeling via AlphaFold Multimer" }
                    }

                    h4 { style: "color: #FFFFFF; font-size: 0.9rem; margin: 16px 0 6px 0;", "Parallel Sourcing Targets" }
                    p { style: "font-size: 0.8rem; color: #A5A1AA; line-height: 1.4; margin: 0;", "Deploy personalized tumor vaccines combined with Fibroblast Activation Protein (FAP) targeted radioligand treatments." }
                }
            }
        }
    }
}
