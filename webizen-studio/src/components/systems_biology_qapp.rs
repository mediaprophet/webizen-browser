use dioxus::prelude::*;

#[component]
pub fn SystemsBiologyQapp() -> Element {
    let mut network_type = use_signal(|| "Gene Regulatory".to_string());
    let mut systems_approach = use_signal(|| "Top-Down".to_string());
    let mut modelling_framework = use_signal(|| "ODE System".to_string());
    let mut nodes = use_signal(|| 200u32);
    let mut edges = use_signal(|| 1000u32);
    let mut robustness = use_signal(|| 65u32);
    let mut notes = use_signal(|| String::new());

    let network_types = [
        "Gene Regulatory", "Protein-Protein Interaction", "Metabolic",
        "Signalling", "Transcription Factor", "Epigenetic",
    ];
    let systems_approaches = ["Top-Down", "Bottom-Up", "Middle-Out"];
    let modelling_frameworks = [
        "Boolean Network", "ODE System", "Stochastic", "Agent-Based", "Multi-Scale",
    ];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",

            h2 {
                style: "margin: 0; color: #89b4fa; border-bottom: 1px solid #313244; padding-bottom: 8px;",
                "Systems Biology"
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Network Type" }
                select {
                    value: "{network_type}",
                    onchange: move |e| network_type.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in network_types { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Systems Approach" }
                select {
                    value: "{systems_approach}",
                    onchange: move |e| systems_approach.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in systems_approaches { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Modelling Framework" }
                select {
                    value: "{modelling_framework}",
                    onchange: move |e| modelling_framework.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in modelling_frameworks { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Nodes: {nodes}" }
                input {
                    r#type: "range", min: "0", max: "10000",
                    value: "{nodes}",
                    oninput: move |e| nodes.set(e.value().parse().unwrap_or(200)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Edges: {edges}" }
                input {
                    r#type: "range", min: "0", max: "100000",
                    value: "{edges}",
                    oninput: move |e| edges.set(e.value().parse().unwrap_or(1000)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Robustness: {robustness}%" }
                input {
                    r#type: "range", min: "0", max: "100",
                    value: "{robustness}",
                    oninput: move |e| robustness.set(e.value().parse().unwrap_or(65)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea {
                    value: "{notes}",
                    oninput: move |e| notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 60px;"
                }
            }

            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #89b4fa;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{network_type} | {systems_approach} | {modelling_framework} | {nodes} nodes | {edges} edges | rob {robustness}%" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → knowledge engine | sieve | anchor" }
            }
        }
    }
}
