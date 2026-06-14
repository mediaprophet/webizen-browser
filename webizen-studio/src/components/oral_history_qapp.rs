use dioxus::prelude::*;

#[component]
pub fn OralHistoryQapp() -> Element {
    let mut tradition_type = use_signal(|| "Personal Narrative".to_string());
    let mut collection_method = use_signal(|| "Interview".to_string());
    let mut narrator_relationship = use_signal(|| "Eyewitness".to_string());
    let mut recording_format = use_signal(|| "Audio".to_string());
    let mut memory_span_years = use_signal(|| 40u32);
    let mut notes = use_signal(|| String::new());

    let traditions = ["Personal Narrative", "Community Memory", "Folklore", "Epic", "Myth", "Legend", "Testimony"];
    let methods = ["Interview", "Ethnographic", "Archive", "Participatory", "Digital"];
    let relationships = ["Eyewitness", "Descendant", "Community Member", "Outsider"];
    let formats = ["Audio", "Video", "Transcript", "Mixed"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",

            h2 { style: "margin: 0; color: #b4befe; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Oral History" }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Tradition Type" }
                select {
                    value: "{tradition_type}",
                    onchange: move |e| tradition_type.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in traditions { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Collection Method" }
                select {
                    value: "{collection_method}",
                    onchange: move |e| collection_method.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in methods { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Narrator Relationship" }
                select {
                    value: "{narrator_relationship}",
                    onchange: move |e| narrator_relationship.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in relationships { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Recording Format" }
                select {
                    value: "{recording_format}",
                    onchange: move |e| recording_format.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in formats { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Memory Span (years): {memory_span_years}" }
                input {
                    r#type: "range", min: "0", max: "120",
                    value: "{memory_span_years}",
                    oninput: move |e| memory_span_years.set(e.value().parse().unwrap_or(40)),
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #b4befe;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{tradition_type} | {collection_method} | {narrator_relationship} | {recording_format} | Span: {memory_span_years}yr" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → knowledge engine | sieve | anchor" }
            }
        }
    }
}
