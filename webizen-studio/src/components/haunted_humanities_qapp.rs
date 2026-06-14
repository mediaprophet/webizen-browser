use dioxus::prelude::*;

#[component]
pub fn HauntedHumanitiesQapp() -> Element {
    let mut hauntological_concept = use_signal(|| "Derrida's Specter".to_string());
    let mut medium = use_signal(|| "Literature".to_string());
    let mut temporal_mode = use_signal(|| "Nostalgia".to_string());
    let mut affective_register = use_signal(|| "Melancholy".to_string());
    let mut cultural_site = use_signal(|| "Post-Industrial".to_string());
    let mut notes = use_signal(|| String::new());

    let hauntological_concepts = ["Derrida's Specter", "Gordon's Haunting", "Avery Gordon", "Fisher's Hauntology", "Uncanny", "Gothic"];
    let mediums = ["Literature", "Film", "Music", "Architecture", "Digital", "Performance"];
    let temporal_modes = ["Nostalgia", "Trauma Return", "Utopian Residue", "Colonial Ghost"];
    let affective_registers = ["Melancholy", "Dread", "Wonder", "Grief", "Uncanny"];
    let cultural_sites = ["Post-Industrial", "Post-Colonial", "Post-Soviet", "Post-Conflict"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #94e2d5; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Haunted Humanities" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Hauntological Concept" }
                select {
                    value: "{hauntological_concept}", onchange: move |e| hauntological_concept.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in hauntological_concepts { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Medium" }
                select {
                    value: "{medium}", onchange: move |e| medium.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in mediums { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Temporal Mode" }
                select {
                    value: "{temporal_mode}", onchange: move |e| temporal_mode.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in temporal_modes { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Affective Register" }
                select {
                    value: "{affective_register}", onchange: move |e| affective_register.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in affective_registers { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Cultural Site" }
                select {
                    value: "{cultural_site}", onchange: move |e| cultural_site.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in cultural_sites { option { value: "{x}", "{x}" } }
                }
            }
            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea { value: "{notes}", oninput: move |e| notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 60px;" }
            }
            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #94e2d5;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{hauntological_concept} | {medium} | {temporal_mode} | {affective_register} | {cultural_site}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → haunted humanities engine | discourse sieve | anchor" }
            }
        }
    }
}
