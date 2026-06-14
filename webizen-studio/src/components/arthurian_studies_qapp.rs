use dioxus::prelude::*;

#[component]
pub fn ArthurianStudiesQapp() -> Element {
    let mut tradition = use_signal(|| "English".to_string());
    let mut text_or_author = use_signal(|| String::new());
    let mut period = use_signal(|| "High Medieval".to_string());
    let mut character_focus = use_signal(|| "Arthur".to_string());
    let mut theoretical_lens = use_signal(|| "Feminist".to_string());
    let mut notes = use_signal(|| String::new());

    let traditions = ["Welsh", "French", "English", "German", "Italian", "American", "Modern Fantasy"];
    let periods = ["Early Medieval", "High Medieval", "Renaissance", "Victorian", "Modern"];
    let characters = ["Arthur", "Guinevere", "Lancelot", "Merlin", "Galahad", "Gawain", "Morgaine"];
    let lenses = ["Feminist", "Postcolonial", "Psychoanalytic", "Nationalist", "Ecocritical"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #cba6f7; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Arthurian Studies" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Tradition" }
                    select {
                        value: "{tradition}",
                        onchange: move |e| tradition.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in traditions { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Period" }
                    select {
                        value: "{period}",
                        onchange: move |e| period.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in periods { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Character Focus" }
                    select {
                        value: "{character_focus}",
                        onchange: move |e| character_focus.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in characters { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Lens" }
                    select {
                        value: "{theoretical_lens}",
                        onchange: move |e| theoretical_lens.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in lenses { option { value: "{x}", "{x}" } }
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Text or Author" }
                input {
                    r#type: "text",
                    value: "{text_or_author}",
                    oninput: move |e| text_or_author.set(e.value()),
                    placeholder: "e.g. Malory Morte d'Arthur, Chrétien de Troyes, Tennyson Idylls",
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #cba6f7;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{tradition} | {period} | {character_focus} | {theoretical_lens}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → medieval legend engine | Arthurian corpus sieve | character anchor" }
            }
        }
    }
}
