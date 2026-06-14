use dioxus::prelude::*;

#[component]
pub fn SpinozaStudiesQapp() -> Element {
    let mut spinozist_concept = use_signal(|| "Conatus".to_string());
    let mut reading_tradition = use_signal(|| "Radical Democratic".to_string());
    let mut interlocutor = use_signal(|| "Deleuze".to_string());
    let mut text = use_signal(|| "Ethics".to_string());
    let mut interpretive_freedom = use_signal(|| 50u32);
    let mut notes = use_signal(|| String::new());

    let concepts = ["Conatus", "Substance-God-Nature", "Modes and Attributes", "Affect", "Adequate Idea", "Multitude", "Immanence"];
    let traditions = ["Enlightenment", "Radical Democratic", "Marxist-Spinozist", "Deleuzian", "Negri's Multitude", "Feminist Spinozism"];
    let interlocutors = ["Descartes", "Leibniz", "Hegel", "Marx", "Nietzsche", "Deleuze", "Negri"];
    let texts = ["Ethics", "Theological-Political Treatise", "Political Treatise", "Letters"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #cba6f7; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Spinoza Studies" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Spinozist Concept" }
                select {
                    value: "{spinozist_concept}",
                    onchange: move |e| spinozist_concept.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in concepts { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Reading Tradition" }
                select {
                    value: "{reading_tradition}",
                    onchange: move |e| reading_tradition.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in traditions { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Interlocutor" }
                select {
                    value: "{interlocutor}",
                    onchange: move |e| interlocutor.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in interlocutors { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Text" }
                select {
                    value: "{text}",
                    onchange: move |e| text.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in texts { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Interpretive Freedom: {interpretive_freedom}" }
                input {
                    r#type: "range", min: "0", max: "100", value: "{interpretive_freedom}",
                    oninput: move |e| interpretive_freedom.set(e.value().parse().unwrap_or(50)),
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #cba6f7;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{spinozist_concept} | {reading_tradition} | {text} | Freedom: {interpretive_freedom}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → Spinoza studies engine | conatus sieve | immanence anchor" }
            }
        }
    }
}
