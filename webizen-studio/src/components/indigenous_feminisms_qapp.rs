use dioxus::prelude::*;

#[component]
pub fn IndigenousFeminismsQapp() -> Element {
    let mut tradition = use_signal(|| "Maori".to_string());
    let mut key_tension = use_signal(|| "Land-Body Nexus".to_string());
    let mut theorist = use_signal(|| "Moreton-Robinson".to_string());
    let mut praxis = use_signal(|| "Land Defence".to_string());
    let mut notes = use_signal(|| String::new());

    let traditions = ["Mayan", "Andean", "Native North American", "Maori", "Aboriginal Australian", "African Indigenous", "Pacific Islander"];
    let key_tensions = ["Land-Body Nexus", "Sovereignty-Gender", "Colonialism-Patriarchy", "Decolonial-Feminist"];
    let theorists = ["Moreton-Robinson", "Anderson", "Arvin/Tuck/Morrill", "Simpson", "Curley"];
    let praxes = ["Land Defence", "Ceremony", "Legal Action", "Cultural Revitalization", "Coalition"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #f38ba8; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Indigenous Feminisms" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Tradition" }
                select {
                    value: "{tradition}", onchange: move |e| tradition.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in traditions { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Key Tension" }
                select {
                    value: "{key_tension}", onchange: move |e| key_tension.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in key_tensions { option { value: "{x}", "{x}" } }
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
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Praxis" }
                select {
                    value: "{praxis}", onchange: move |e| praxis.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in praxes { option { value: "{x}", "{x}" } }
                }
            }
            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea { value: "{notes}", oninput: move |e| notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 60px;" }
            }
            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #f38ba8;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{tradition} | {key_tension} | {theorist} | {praxis}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → indigenous feminisms engine | discourse sieve | anchor" }
            }
        }
    }
}
