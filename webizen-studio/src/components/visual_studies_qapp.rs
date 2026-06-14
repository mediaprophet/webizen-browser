use dioxus::prelude::*;

#[component]
pub fn VisualStudiesQapp() -> Element {
    let mut theoretical_approach = use_signal(|| "Visual Culture Studies".to_string());
    let mut image_type = use_signal(|| "Fine Art".to_string());
    let mut gaze = use_signal(|| "Male Gaze".to_string());
    let mut mitchell_concept = use_signal(|| "Picture Theory".to_string());
    let mut visual_complexity = use_signal(|| 50u32);
    let mut cultural_power = use_signal(|| 50u32);
    let mut notes = use_signal(|| String::new());

    let approaches = ["New Art History", "Visual Culture Studies", "Scopic Regime", "Iconology", "Gaze Theory", "Image Science"];
    let image_types = ["Fine Art", "Photography", "Film", "Digital", "Everyday", "Scientific", "Medical"];
    let gazes = ["Male Gaze", "Touristic", "Clinical", "Surveillant", "Postcolonial"];
    let mitchell_concepts = ["Imagetext", "Picture Theory", "What Do Pictures Want?", "Iconology"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #fab387; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Visual Studies" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Approach" }
                select {
                    value: "{theoretical_approach}",
                    onchange: move |e| theoretical_approach.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in approaches { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Image Type" }
                select {
                    value: "{image_type}",
                    onchange: move |e| image_type.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in image_types { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Gaze" }
                select {
                    value: "{gaze}",
                    onchange: move |e| gaze.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in gazes { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "W.J.T. Mitchell Concept" }
                select {
                    value: "{mitchell_concept}",
                    onchange: move |e| mitchell_concept.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in mitchell_concepts { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Visual Complexity: {visual_complexity}" }
                input {
                    r#type: "range", min: "0", max: "100", value: "{visual_complexity}",
                    oninput: move |e| visual_complexity.set(e.value().parse().unwrap_or(50)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Cultural Power: {cultural_power}" }
                input {
                    r#type: "range", min: "0", max: "100", value: "{cultural_power}",
                    oninput: move |e| cultural_power.set(e.value().parse().unwrap_or(50)),
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #fab387;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{theoretical_approach} | {gaze} | {mitchell_concept} | Complexity: {visual_complexity} | Power: {cultural_power}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → visual studies engine | gaze sieve | image anchor" }
            }
        }
    }
}
