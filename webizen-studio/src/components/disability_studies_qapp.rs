use dioxus::prelude::*;

#[component]
pub fn DisabilityStudiesQapp() -> Element {
    let mut model = use_signal(|| "Social".to_string());
    let mut disability_category = use_signal(|| "Physical".to_string());
    let mut accessibility_domain = use_signal(|| "Physical".to_string());
    let mut methodology = use_signal(|| "Policy Analysis".to_string());
    let mut policy_context = use_signal(|| "ADA".to_string());
    let mut notes = use_signal(|| String::new());

    let models = [
        "Medical", "Social", "Cultural", "Critical", "Crip Theory",
        "Neurodiversity", "Intersectional", "Poststructuralist",
    ];
    let categories = [
        "Physical", "Sensory", "Cognitive", "Psychiatric",
        "Chronic Illness", "Neurodivergent", "Multiple",
    ];
    let domains = [
        "Physical", "Digital", "Social", "Institutional", "Legal", "Communicative",
    ];
    let methodologies = [
        "Autoethnography", "Survey", "Policy Analysis", "Critical Discourse", "Historical",
    ];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #cba6f7; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Disability Studies" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Model" }
                    select {
                        value: "{model}",
                        onchange: move |e| model.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in models { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Disability Category" }
                    select {
                        value: "{disability_category}",
                        onchange: move |e| disability_category.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in categories { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Accessibility Domain" }
                    select {
                        value: "{accessibility_domain}",
                        onchange: move |e| accessibility_domain.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in domains { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Methodology" }
                    select {
                        value: "{methodology}",
                        onchange: move |e| methodology.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in methodologies { option { value: "{x}", "{x}" } }
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Policy Context (e.g. ADA, CRPD, NDIS)" }
                input {
                    r#type: "text",
                    value: "{policy_context}",
                    oninput: move |e| policy_context.set(e.value()),
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
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{model} Model | {disability_category} | {accessibility_domain} | {policy_context}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → accessibility sieve | intersectional engine | policy graph" }
            }
        }
    }
}
