use dioxus::prelude::*;

#[component]
pub fn SoftSkillsTheoryQapp() -> Element {
    let mut skill_domain = use_signal(|| "Emotional Intelligence".to_string());
    let mut theoretical_origin = use_signal(|| "Goleman's EQ".to_string());
    let mut training_modality = use_signal(|| "Workshop".to_string());
    let mut organisational_context = use_signal(|| "Corporate".to_string());
    let mut development_score = use_signal(|| 50u32);
    let mut notes = use_signal(|| String::new());

    let domains = ["Emotional Intelligence", "Communication", "Leadership", "Teamwork", "Adaptability", "Creativity", "Critical Thinking"];
    let origins = ["Goleman's EQ", "Bandura's Self-Efficacy", "Senge's Learning Organisation", "Bourdieu's Habitus", "Positive Psychology"];
    let modalities = ["Workshop", "Coaching", "Peer Learning", "Mentoring", "Simulation"];
    let contexts = ["Corporate", "Education", "Healthcare", "NGO", "Military"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #f5c2e7; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Soft Skills Theory" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Skill Domain" }
                select {
                    value: "{skill_domain}",
                    onchange: move |e| skill_domain.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in domains { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Origin" }
                select {
                    value: "{theoretical_origin}",
                    onchange: move |e| theoretical_origin.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in origins { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Training Modality" }
                select {
                    value: "{training_modality}",
                    onchange: move |e| training_modality.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in modalities { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Organisational Context" }
                select {
                    value: "{organisational_context}",
                    onchange: move |e| organisational_context.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in contexts { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Development Score: {development_score}" }
                input {
                    r#type: "range", min: "0", max: "100", value: "{development_score}",
                    oninput: move |e| development_score.set(e.value().parse().unwrap_or(50)),
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #f5c2e7;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{skill_domain} | {theoretical_origin} | {training_modality} | Score: {development_score}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → soft skills engine | capacity sieve | development anchor" }
            }
        }
    }
}
