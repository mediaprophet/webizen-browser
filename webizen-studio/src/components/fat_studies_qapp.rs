use dioxus::prelude::*;

#[component]
pub fn FatStudiesQapp() -> Element {
    let mut theoretical_lens = use_signal(|| "Body Positivity".to_string());
    let mut discourse = use_signal(|| "Medical Pathologisation".to_string());
    let mut site = use_signal(|| "Healthcare".to_string());
    let mut stigma_index = use_signal(|| 65u32);
    let mut notes = use_signal(|| String::new());

    let lenses = ["Body Positivity", "Fat Acceptance", "Disability Studies", "Feminist Theory", "Critical Health Studies", "Anti-Diet"];
    let discourses = ["Medical Pathologisation", "Aesthetic Normativity", "Political Economy", "Representational Politics", "Legal Rights"];
    let sites = ["Media", "Healthcare", "Workplace", "Education", "Fashion", "Public Space"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",

            h2 { style: "margin: 0; color: #f38ba8; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Fat Studies" }

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
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Discourse" }
                select {
                    value: "{discourse}",
                    onchange: move |e| discourse.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in discourses { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Site" }
                select {
                    value: "{site}",
                    onchange: move |e| site.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in sites { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Stigma Index: {stigma_index}%" }
                input {
                    r#type: "range", min: "0", max: "100",
                    value: "{stigma_index}",
                    oninput: move |e| stigma_index.set(e.value().parse().unwrap_or(65)),
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #f38ba8;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{theoretical_lens} | {discourse} | Site: {site} | Stigma: {stigma_index}%" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → knowledge engine | sieve | anchor" }
            }
        }
    }
}
