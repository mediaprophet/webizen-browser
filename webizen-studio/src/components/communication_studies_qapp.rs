use dioxus::prelude::*;

#[component]
pub fn CommunicationStudiesQapp() -> Element {
    let mut subfield = use_signal(|| "Mass Communication".to_string());
    let mut theoretical_framework = use_signal(|| "Framing".to_string());
    let mut medium = use_signal(|| "Social Media".to_string());
    let mut message_type = use_signal(|| String::new());
    let mut audience_type = use_signal(|| String::new());
    let mut research_notes = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box; overflow-y: auto;",
            h2 { style: "margin: 0; color: #cba6f7; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Communication Studies QApp" }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Subfield" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| subfield.set(e.value()),
                    option { "Interpersonal" }
                    option { "Group" }
                    option { "Organisational" }
                    option { selected: true, "Mass Communication" }
                    option { "Political Communication" }
                    option { "Health Communication" }
                    option { "Intercultural" }
                    option { "Digital" }
                    option { "Rhetoric" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Theoretical Framework" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| theoretical_framework.set(e.value()),
                    option { "Agenda-Setting" }
                    option { selected: true, "Framing" }
                    option { "Uses & Gratifications" }
                    option { "Spiral of Silence" }
                    option { "Social Learning" }
                    option { "Cultivation Theory" }
                    option { "Network Theory" }
                    option { "Discourse Analysis" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Medium" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| medium.set(e.value()),
                    option { "Print" }
                    option { "Radio" }
                    option { "Television" }
                    option { selected: true, "Social Media" }
                    option { "Film" }
                    option { "Podcast" }
                    option { "Video Game" }
                    option { "VR" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Message Type" }
                input {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    placeholder: "e.g. persuasive, informational, narrative...",
                    oninput: move |e| message_type.set(e.value()),
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Audience Type" }
                input {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    placeholder: "e.g. general public, niche community...",
                    oninput: move |e| audience_type.set(e.value()),
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Research Notes" }
                textarea {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box; min-height: 60px; resize: vertical;",
                    placeholder: "Additional research notes...",
                    oninput: move |e| research_notes.set(e.value()),
                }
            }

            div {
                style: "background: #11111b; padding: 16px; border-radius: 8px; border-left: 4px solid #cba6f7; flex: 1;",
                h3 { style: "margin-top: 0; color: #cba6f7; font-size: 0.9rem;", "QualiaDB Output" }
                div { style: "font-size: 0.8rem; color: #a6adc8; display: flex; flex-direction: column; gap: 4px;",
                    div { "Subfield: {subfield()}" }
                    div { "Framework: {theoretical_framework()}" }
                    div { "Medium: {medium()}" }
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 8px;", "QualiaDB → discourse sieve | graph network | Allen Interval" }
            }
        }
    }
}
