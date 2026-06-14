use dioxus::prelude::*;

#[component]
pub fn LandscapeStudiesQapp() -> Element {
    let mut landscape_type = use_signal(|| "Urban".to_string());
    let mut theoretical_approach = use_signal(|| "Phenomenological".to_string());
    let mut intervention = use_signal(|| "Conservation".to_string());
    let mut scale = use_signal(|| "Site".to_string());
    let mut biodiversity_index = use_signal(|| 50u32);
    let mut cultural_heritage_value = use_signal(|| 50u32);
    let mut notes = use_signal(|| String::new());

    let landscape_types = [
        "Urban", "Rural", "Wilderness", "Cultural", "Industrial", "Coastal", "Riverine",
    ];
    let theoretical_approaches = [
        "Phenomenological", "Political Ecology", "Aesthetic", "Historical", "Indigenous", "Digital",
    ];
    let interventions = ["Conservation", "Restoration", "Design", "Infrastructure", "Art"];
    let scales = ["Site", "District", "Region", "National", "Global"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",

            h2 {
                style: "margin: 0; color: #94e2d5; border-bottom: 1px solid #313244; padding-bottom: 8px;",
                "Landscape Studies"
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Landscape Type" }
                select {
                    value: "{landscape_type}",
                    onchange: move |e| landscape_type.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in landscape_types { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Approach" }
                select {
                    value: "{theoretical_approach}",
                    onchange: move |e| theoretical_approach.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in theoretical_approaches { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Intervention" }
                select {
                    value: "{intervention}",
                    onchange: move |e| intervention.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in interventions { option { value: "{x}", "{x}" } }
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
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Biodiversity Index: {biodiversity_index}" }
                input {
                    r#type: "range", min: "0", max: "100",
                    value: "{biodiversity_index}",
                    oninput: move |e| biodiversity_index.set(e.value().parse().unwrap_or(50)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Cultural Heritage Value: {cultural_heritage_value}" }
                input {
                    r#type: "range", min: "0", max: "100",
                    value: "{cultural_heritage_value}",
                    oninput: move |e| cultural_heritage_value.set(e.value().parse().unwrap_or(50)),
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #94e2d5;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{landscape_type} | {theoretical_approach} | {intervention} | {scale} | bio {biodiversity_index} | heritage {cultural_heritage_value}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → knowledge engine | sieve | anchor" }
            }
        }
    }
}
