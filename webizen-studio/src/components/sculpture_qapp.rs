use dioxus::prelude::*;

#[component]
pub fn SculptureQapp() -> Element {
    let mut method = use_signal(|| "Casting".to_string());
    let mut material = use_signal(|| "Bronze".to_string());
    let mut scale = use_signal(|| "Life-Size".to_string());
    let mut tradition = use_signal(|| "Modernist".to_string());
    let mut site_or_context = use_signal(|| String::new());
    let mut conceptual_basis = use_signal(|| String::new());
    let mut notes = use_signal(|| String::new());

    let methods = ["Carving", "Modelling", "Casting", "Construction", "Installation", "Kinetic", "Land Art", "Site-Specific", "Assemblage", "Digital Fabrication"];
    let materials = ["Stone", "Bronze", "Steel", "Wood", "Ceramics", "Glass", "Textile", "Found Objects", "Ice", "Light", "Sound"];
    let scales = ["Miniature", "Table-Scale", "Life-Size", "Monumental", "Environmental", "Architectural"];
    let traditions = ["Classical", "African", "Asian", "Pre-Columbian", "Modernist", "Minimalist", "Postminimalist", "Contemporary"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #cba6f7; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Sculpture" }

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
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Material" }
                    select {
                        value: "{material}",
                        onchange: move |e| material.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in materials { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Scale" }
                    select {
                        value: "{scale}",
                        onchange: move |e| scale.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in scales { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Tradition" }
                    select {
                        value: "{tradition}",
                        onchange: move |e| tradition.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in traditions { option { value: "{x}", "{x}" } }
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Site or Context" }
                input {
                    r#type: "text",
                    value: "{site_or_context}",
                    oninput: move |e| site_or_context.set(e.value()),
                    placeholder: "e.g. public park, gallery white cube, natural landscape",
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Conceptual Basis" }
                textarea {
                    value: "{conceptual_basis}",
                    oninput: move |e| conceptual_basis.set(e.value()),
                    placeholder: "Describe the conceptual framework...",
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 60px;"
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
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{method} | {material} | {scale} | {tradition}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → sculpture engine | material sieve | site-specificity anchor" }
            }
        }
    }
}
