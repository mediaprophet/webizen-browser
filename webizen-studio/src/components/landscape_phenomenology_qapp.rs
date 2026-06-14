use dioxus::prelude::*;

#[component]
pub fn LandscapePhenomenologyQapp() -> Element {
    let mut phenomenological_tradition = use_signal(|| "Merleau-Ponty".to_string());
    let mut landscape_character = use_signal(|| "Dwelling".to_string());
    let mut spatial_quality = use_signal(|| "Openness".to_string());
    let mut bodily_engagement = use_signal(|| "Visual".to_string());
    let mut place_attachment = use_signal(|| 50u32);
    let mut notes = use_signal(|| String::new());

    let phenomenological_traditions = ["Husserl", "Heidegger", "Merleau-Ponty", "Casey", "Ingold", "Tuan"];
    let landscape_characters = ["Dwelling", "Taskscape", "Affordance", "Genius Loci", "Lifeworld", "Chorography"];
    let spatial_qualities = ["Enclosure", "Openness", "Movement", "Stillness", "Threshold", "Depth"];
    let bodily_engagements = ["Visual", "Tactile", "Auditory", "Olfactory", "Kinaesthetic", "Multi-Sensory"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #fab387; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Landscape Phenomenology" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Phenomenological Tradition" }
                select {
                    value: "{phenomenological_tradition}", onchange: move |e| phenomenological_tradition.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in phenomenological_traditions { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Landscape Character" }
                select {
                    value: "{landscape_character}", onchange: move |e| landscape_character.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in landscape_characters { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Spatial Quality" }
                select {
                    value: "{spatial_quality}", onchange: move |e| spatial_quality.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in spatial_qualities { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Bodily Engagement" }
                select {
                    value: "{bodily_engagement}", onchange: move |e| bodily_engagement.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in bodily_engagements { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Place Attachment: {place_attachment}" }
                input { r#type: "range", min: "0", max: "100", value: "{place_attachment}",
                    oninput: move |e| place_attachment.set(e.value().parse().unwrap_or(50)),
                    style: "width: 100%; margin-top: 4px;" }
            }
            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea { value: "{notes}", oninput: move |e| notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 60px;" }
            }
            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #fab387;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{phenomenological_tradition} | {landscape_character} | {spatial_quality} | {bodily_engagement} | attach: {place_attachment}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → landscape phenomenology engine | discourse sieve | anchor" }
            }
        }
    }
}
