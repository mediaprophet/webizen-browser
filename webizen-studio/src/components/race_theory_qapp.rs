use dioxus::prelude::*;

#[component]
pub fn RaceTheoryQapp() -> Element {
    let mut theoretical_position = use_signal(|| "Social Construction".to_string());
    let mut debate = use_signal(|| "Essentialism vs Constructivism".to_string());
    let mut methodology = use_signal(|| "Discourse Analysis".to_string());
    let mut race_consciousness = use_signal(|| 50u32);
    let mut notes = use_signal(|| String::new());

    let positions = ["Biological Race Rejection", "Social Construction", "Racial Realism", "Eliminativism", "Critical Race", "Anti-Racism", "Colorism"];
    let debates = ["Essentialism vs Constructivism", "Intersectionality", "Coalition Politics", "Solidarity"];
    let methodologies = ["Discourse Analysis", "Historical Materialism", "Phenomenological", "Statistical"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #f38ba8; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Race Theory" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Position" }
                select {
                    value: "{theoretical_position}",
                    onchange: move |e| theoretical_position.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in positions { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Debate" }
                select {
                    value: "{debate}",
                    onchange: move |e| debate.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in debates { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Methodology" }
                select {
                    value: "{methodology}",
                    onchange: move |e| methodology.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in methodologies { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Race Consciousness: {race_consciousness}" }
                input {
                    r#type: "range", min: "0", max: "100", value: "{race_consciousness}",
                    oninput: move |e| race_consciousness.set(e.value().parse().unwrap_or(50)),
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #f38ba8;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{theoretical_position} | {debate} | {methodology} | Consciousness: {race_consciousness}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → race theory engine | ontology sieve | construction anchor" }
            }
        }
    }
}
