use dioxus::prelude::*;

#[component]
pub fn VitalMaterialismQapp() -> Element {
    let mut theoretical_strand = use_signal(|| "Jane Bennett's Vibrant Matter".to_string());
    let mut matter_type = use_signal(|| "Food".to_string());
    let mut distributed_agency_level = use_signal(|| 50u32);
    let mut assemblage = use_signal(|| "Human-Nonhuman".to_string());
    let mut vitality_index = use_signal(|| 50u32);
    let mut notes = use_signal(|| String::new());

    let strands = ["Jane Bennett's Vibrant Matter", "Spinozist Conatus", "Deleuzian Assemblage", "New Materialism", "Agential Realism", "Thing-Power"];
    let matter_types = ["Food", "Electricity", "Metal", "Microbe", "Chemical", "Soil", "Data", "Body"];
    let assemblages = ["Human-Nonhuman", "Political Ecology", "Networked", "Emergent"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #f9e2af; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Vital Materialism" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Strand" }
                select {
                    value: "{theoretical_strand}",
                    onchange: move |e| theoretical_strand.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in strands { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Matter Type" }
                select {
                    value: "{matter_type}",
                    onchange: move |e| matter_type.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in matter_types { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Distributed Agency Level: {distributed_agency_level}" }
                input {
                    r#type: "range", min: "0", max: "100", value: "{distributed_agency_level}",
                    oninput: move |e| distributed_agency_level.set(e.value().parse().unwrap_or(50)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Assemblage" }
                select {
                    value: "{assemblage}",
                    onchange: move |e| assemblage.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in assemblages { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Vitality Index: {vitality_index}" }
                input {
                    r#type: "range", min: "0", max: "100", value: "{vitality_index}",
                    oninput: move |e| vitality_index.set(e.value().parse().unwrap_or(50)),
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #f9e2af;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{theoretical_strand} | {matter_type} | {assemblage} | Agency: {distributed_agency_level} | Vitality: {vitality_index}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → vital materialism engine | thing-power sieve | vitality anchor" }
            }
        }
    }
}
