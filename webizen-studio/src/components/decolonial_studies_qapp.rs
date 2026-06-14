use dioxus::prelude::*;

#[component]
pub fn DecolonialStudiesQapp() -> Element {
    let mut theorist = use_signal(|| "Quijano".to_string());
    let mut colonial_matrix = use_signal(|| "Power".to_string());
    let mut modality = use_signal(|| "Epistemic Decolonisation".to_string());
    let mut colonial_legacy = use_signal(|| "Racial Hierarchy".to_string());
    let mut decolonisation_index = use_signal(|| 50u32);
    let mut notes = use_signal(|| String::new());

    let theorists = ["Quijano", "Mignolo", "Lugones", "Maldonado-Torres", "Ndlovu-Gatsheni", "Santos", "Grosfoguel"];
    let colonial_matrices = ["Power", "Being", "Knowledge", "Nature"];
    let modalities = ["Epistemic Decolonisation", "Land Return", "Cultural Revitalisation", "Legal Reform", "Economic Redistribution"];
    let colonial_legacies = ["Racial Hierarchy", "Land Dispossession", "Knowledge Erasure", "Economic Extraction"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #94e2d5; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Decolonial Studies" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Theorist" }
                select {
                    value: "{theorist}", onchange: move |e| theorist.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in theorists { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Colonial Matrix" }
                select {
                    value: "{colonial_matrix}", onchange: move |e| colonial_matrix.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in colonial_matrices { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Modality" }
                select {
                    value: "{modality}", onchange: move |e| modality.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in modalities { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Colonial Legacy" }
                select {
                    value: "{colonial_legacy}", onchange: move |e| colonial_legacy.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in colonial_legacies { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Decolonisation Index: {decolonisation_index}" }
                input { r#type: "range", min: "0", max: "100", value: "{decolonisation_index}",
                    oninput: move |e| decolonisation_index.set(e.value().parse().unwrap_or(50)),
                    style: "width: 100%; margin-top: 4px;" }
            }
            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea { value: "{notes}", oninput: move |e| notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 60px;" }
            }
            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #94e2d5;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{theorist} | {colonial_matrix} | {modality} | index: {decolonisation_index}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → decolonial studies engine | discourse sieve | anchor" }
            }
        }
    }
}
