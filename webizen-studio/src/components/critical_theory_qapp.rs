use dioxus::prelude::*;

#[component]
pub fn CriticalTheoryQapp() -> Element {
    let mut school = use_signal(|| "Frankfurt School".to_string());
    let mut thinker = use_signal(|| "Adorno".to_string());
    let mut concept = use_signal(|| "Ideology".to_string());
    let mut critique_target = use_signal(|| "Capital".to_string());
    let mut notes = use_signal(|| String::new());

    let schools = ["Frankfurt School", "Birmingham CCCS", "French Theory", "Feminist", "Postcolonial", "Queer", "Decolonial"];
    let thinkers = ["Horkheimer", "Adorno", "Marcuse", "Habermas", "Bourdieu", "Foucault", "Butler", "Spivak", "Fanon"];
    let concepts = ["Ideology", "Power/Knowledge", "Hegemony", "Alienation", "Reification", "Performativity", "Subaltern"];
    let targets = ["Media", "State", "Capital", "Science", "Culture", "Law"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",

            h2 { style: "margin: 0; color: #89dceb; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Critical Theory" }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "School" }
                select {
                    value: "{school}",
                    onchange: move |e| school.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in schools { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Key Thinker" }
                select {
                    value: "{thinker}",
                    onchange: move |e| thinker.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in thinkers { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Core Concept" }
                select {
                    value: "{concept}",
                    onchange: move |e| concept.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in concepts { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Critique Target" }
                select {
                    value: "{critique_target}",
                    onchange: move |e| critique_target.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in targets { option { value: "{x}", "{x}" } }
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #89dceb;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{school} | {thinker} | Concept: {concept} | Target: {critique_target}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → knowledge engine | sieve | anchor" }
            }
        }
    }
}
