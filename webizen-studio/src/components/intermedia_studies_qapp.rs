use dioxus::prelude::*;

#[component]
pub fn IntermediaStudiesQapp() -> Element {
    let mut intermedia_concept = use_signal(|| "Fluxus".to_string());
    let mut medium_combination = use_signal(|| "Text-Image".to_string());
    let mut theorist = use_signal(|| "Higgins".to_string());
    let mut platform = use_signal(|| "Gallery".to_string());
    let mut hybridity_index = use_signal(|| 50u32);
    let mut notes = use_signal(|| String::new());

    let intermedia_concepts = ["Fluxus", "Expanded Cinema", "Transmedia", "Cross-Media", "Convergence", "Remediation", "New Media Art"];
    let medium_combinations = ["Text-Image", "Sound-Image", "Performance-Digital", "Body-Technology", "Physical-Virtual"];
    let theorists = ["Higgins", "McLuhan", "Bolter/Grusin", "Jenkins", "Manovich"];
    let platforms = ["Gallery", "Web", "Street", "Theatre", "Game", "Social Media"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #89b4fa; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Intermedia Studies" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Intermedia Concept" }
                select {
                    value: "{intermedia_concept}", onchange: move |e| intermedia_concept.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in intermedia_concepts { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Medium Combination" }
                select {
                    value: "{medium_combination}", onchange: move |e| medium_combination.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in medium_combinations { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Theorist" }
                select {
                    value: "{theorist}", onchange: move |e| theorist.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in theorists { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Platform" }
                select {
                    value: "{platform}", onchange: move |e| platform.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in platforms { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Hybridity Index: {hybridity_index}" }
                input { r#type: "range", min: "0", max: "100", value: "{hybridity_index}",
                    oninput: move |e| hybridity_index.set(e.value().parse().unwrap_or(50)),
                    style: "width: 100%; margin-top: 4px;" }
            }
            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea { value: "{notes}", oninput: move |e| notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 60px;" }
            }
            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #89b4fa;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{intermedia_concept} | {medium_combination} | {theorist} | {platform} | hybridity: {hybridity_index}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → intermedia studies engine | discourse sieve | anchor" }
            }
        }
    }
}
