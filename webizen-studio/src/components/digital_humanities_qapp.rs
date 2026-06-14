use dioxus::prelude::*;

#[component]
pub fn DigitalHumanitiesQapp() -> Element {
    let mut method = use_signal(|| "Text Mining".to_string());
    let mut primary_source_type = use_signal(|| "Literary Text".to_string());
    let mut tool_or_platform = use_signal(|| "Gephi".to_string());
    let mut metadata_standard = use_signal(|| "Dublin Core".to_string());
    let mut digital_preservation_risk = use_signal(|| "Medium".to_string());
    let mut project_scale = use_signal(|| "Small Team".to_string());
    let mut notes = use_signal(|| String::new());

    let methods = [
        "Text Mining", "Network Analysis", "GIS Mapping", "3D Modelling",
        "Digital Archiving", "Corpus Linguistics", "Machine Learning",
        "Crowdsourcing", "Data Visualisation", "Virtual Reality",
    ];
    let source_types = [
        "Literary Text", "Historical Document", "Image", "Audio",
        "Video", "Social Media", "Dataset",
    ];
    let metadata_standards = ["Dublin Core", "TEI", "EAD", "METS", "JSON-LD"];
    let risks = ["Low", "Medium", "High", "Critical"];
    let scales = ["Individual", "Small Team", "Consortium", "International"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #89dceb; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Digital Humanities" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Method" }
                    select {
                        value: "{method}",
                        onchange: move |e| method.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in methods { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Primary Source Type" }
                    select {
                        value: "{primary_source_type}",
                        onchange: move |e| primary_source_type.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in source_types { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Metadata Standard" }
                    select {
                        value: "{metadata_standard}",
                        onchange: move |e| metadata_standard.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in metadata_standards { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Digital Preservation Risk" }
                    select {
                        value: "{digital_preservation_risk}",
                        onchange: move |e| digital_preservation_risk.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in risks { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Project Scale" }
                    select {
                        value: "{project_scale}",
                        onchange: move |e| project_scale.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in scales { option { value: "{x}", "{x}" } }
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Tool or Platform (e.g. Gephi, QGIS, NLTK, Omeka)" }
                input {
                    r#type: "text",
                    value: "{tool_or_platform}",
                    oninput: move |e| tool_or_platform.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #89dceb;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{method} | {metadata_standard} | {project_scale} | Risk: {digital_preservation_risk}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → corpus analysis engine | linked data graph | metadata sieve" }
            }
        }
    }
}
