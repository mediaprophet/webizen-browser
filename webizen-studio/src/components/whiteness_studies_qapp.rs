use dioxus::prelude::*;

#[component]
pub fn WhitenessStudiesQapp() -> Element {
    let mut theoretical_lens = use_signal(|| "Critical Whiteness Studies".to_string());
    let mut focus = use_signal(|| "Invisibility/Normativity".to_string());
    let mut methodology = use_signal(|| "Discourse Analysis".to_string());
    let mut context = use_signal(|| "US".to_string());
    let mut notes = use_signal(|| String::new());

    let lenses = ["Critical Whiteness Studies", "Critical Race Theory", "Intersectionality", "Labour History", "Sociology"];
    let focuses = ["Invisibility/Normativity", "Privilege", "Historical Construction", "Class-Race Nexus", "Legal Definitions", "Global Whiteness"];
    let methodologies = ["Discourse Analysis", "Autoethnography", "Historical", "Sociological Survey"];
    let contexts = ["US", "UK", "Australia", "Europe", "Global South", "Comparative"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",

            h2 { style: "margin: 0; color: #89b4fa; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Whiteness Studies" }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Lens" }
                select {
                    value: "{theoretical_lens}",
                    onchange: move |e| theoretical_lens.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in lenses { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Focus" }
                select {
                    value: "{focus}",
                    onchange: move |e| focus.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in focuses { option { value: "{x}", "{x}" } }
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
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Context" }
                select {
                    value: "{context}",
                    onchange: move |e| context.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in contexts { option { value: "{x}", "{x}" } }
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea {
                    value: "{notes}",
                    oninput: move |e| notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 80px;"
                }
            }

            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #89b4fa;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{theoretical_lens} | {focus} | {methodology} | Context: {context}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → knowledge engine | sieve | anchor" }
            }
        }
    }
}
