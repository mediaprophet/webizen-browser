use dioxus::prelude::*;

#[component]
pub fn EgyptologyQapp() -> Element {
    let mut period = use_signal(|| "New Kingdom".to_string());
    let mut script = use_signal(|| "Hieroglyphic".to_string());
    let mut source_type = use_signal(|| "Temple Inscription".to_string());
    let mut dynasty = use_signal(|| String::new());
    let mut deity_or_concept = use_signal(|| String::new());
    let mut archaeological_site = use_signal(|| String::new());
    let mut notes = use_signal(|| String::new());

    let periods = ["Predynastic", "Early Dynastic", "Old Kingdom", "Middle Kingdom", "New Kingdom", "Third Intermediate", "Late Period", "Ptolemaic", "Roman Egypt"];
    let scripts = ["Hieroglyphic", "Hieratic", "Demotic", "Coptic", "Greek"];
    let source_types = ["Temple Inscription", "Papyrus", "Ostracon", "Stele", "Tomb Painting", "Artefact"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #f9e2af; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Egyptology" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
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
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Script" }
                    select {
                        value: "{script}",
                        onchange: move |e| script.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in scripts { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Source Type" }
                    select {
                        value: "{source_type}",
                        onchange: move |e| source_type.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in source_types { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Dynasty" }
                    input {
                        r#type: "text",
                        value: "{dynasty}",
                        oninput: move |e| dynasty.set(e.value()),
                        placeholder: "e.g. 18th Dynasty, Ptolemy III",
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Deity or Concept" }
                    input {
                        r#type: "text",
                        value: "{deity_or_concept}",
                        oninput: move |e| deity_or_concept.set(e.value()),
                        placeholder: "e.g. Ra, Ma'at, Osiris",
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Archaeological Site" }
                    input {
                        r#type: "text",
                        value: "{archaeological_site}",
                        oninput: move |e| archaeological_site.set(e.value()),
                        placeholder: "e.g. Karnak, Valley of the Kings",
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #f9e2af;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{period} | {script} | {source_type} | {archaeological_site}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → hieroglyphic corpus | dynastic sieve | site provenance engine" }
            }
        }
    }
}
