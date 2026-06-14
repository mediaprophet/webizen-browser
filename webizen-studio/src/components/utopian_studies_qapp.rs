use dioxus::prelude::*;

#[component]
pub fn UtopianStudiesQapp() -> Element {
    let mut utopian_type = use_signal(|| "Critical Utopia".to_string());
    let mut theoretical_tradition = use_signal(|| "Bloch Hope".to_string());
    let mut domain = use_signal(|| "Political".to_string());
    let mut historical_example = use_signal(|| String::new());
    let mut feasibility_assessment = use_signal(|| 40u32);
    let mut notes = use_signal(|| String::new());

    let utopian_types = ["Literary Utopia", "Practical Commune", "Social Reform Movement", "Political Programme", "Critical Utopia", "Heterotopia", "Anti-Utopia", "Retrotopia"];
    let traditions = ["More", "Bellamy", "Morris", "Bloch Hope", "Jameson Utopia", "Harvey Spaces", "Levitas Utopia as Method", "Munoz Queer Futurity"];
    let domains = ["Political", "Economic", "Ecological", "Technological", "Religious", "Gender", "Spatial"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #a6e3a1; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Utopian Studies" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Utopian Type" }
                    select {
                        value: "{utopian_type}",
                        onchange: move |e| utopian_type.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in utopian_types { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Tradition" }
                    select {
                        value: "{theoretical_tradition}",
                        onchange: move |e| theoretical_tradition.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in traditions { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Domain" }
                    select {
                        value: "{domain}",
                        onchange: move |e| domain.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in domains { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Historical Example" }
                    input {
                        r#type: "text",
                        value: "{historical_example}",
                        oninput: move |e| historical_example.set(e.value()),
                        placeholder: "e.g. Oneida Community, kibbutz, Auroville",
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Feasibility Assessment: {feasibility_assessment}%" }
                input {
                    r#type: "range",
                    min: "0",
                    max: "100",
                    value: "{feasibility_assessment}",
                    oninput: move |e| feasibility_assessment.set(e.value().parse().unwrap_or(40)),
                    style: "width: 100%; margin-top: 4px;"
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #a6e3a1;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{utopian_type} | {theoretical_tradition} | {domain} | Feasibility:{feasibility_assessment}%" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → utopian theory engine | hope principle sieve | spatial anchor" }
            }
        }
    }
}
