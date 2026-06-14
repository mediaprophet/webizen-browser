use dioxus::prelude::*;

#[component]
pub fn GlobalStudiesQapp() -> Element {
    let mut global_process = use_signal(|| "Globalisation".to_string());
    let mut theoretical_lens = use_signal(|| "World-Systems".to_string());
    let mut scale = use_signal(|| "Global".to_string());
    let mut actor_type = use_signal(|| "State".to_string());
    let mut hdi_score = use_signal(|| 0.7f64);
    let mut gini_coefficient = use_signal(|| 0.35f64);
    let mut notes = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box; overflow-y: auto;",
            h2 { style: "margin: 0; color: #94e2d5; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Global Studies QApp" }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Global Process" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| global_process.set(e.value()),
                    option { selected: true, "Globalisation" }
                    option { "Regionalisation" }
                    option { "Deglobalisation" }
                    option { "Climate Change" }
                    option { "Migration" }
                    option { "Pandemics" }
                    option { "Digital Transformation" }
                    option { "Nuclear Proliferation" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Theoretical Lens" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| theoretical_lens.set(e.value()),
                    option { selected: true, "World-Systems" }
                    option { "Global Governance" }
                    option { "Cosmopolitanism" }
                    option { "Critical Theory" }
                    option { "Postcolonial" }
                    option { "Complex Systems" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Scale" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| scale.set(e.value()),
                    option { selected: true, "Global" }
                    option { "Transnational" }
                    option { "Regional" }
                    option { "National" }
                    option { "Local" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Actor Type" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| actor_type.set(e.value()),
                    option { selected: true, "State" }
                    option { "IGO" }
                    option { "NGO" }
                    option { "MNC" }
                    option { "Civil Society" }
                    option { "Social Movement" }
                    option { "Individual" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "HDI Score: {hdi_score():.2}" }
                input {
                    r#type: "range",
                    min: "0.0",
                    max: "1.0",
                    step: "0.01",
                    value: "{hdi_score()}",
                    style: "width: 100%; box-sizing: border-box; accent-color: #94e2d5;",
                    oninput: move |e| hdi_score.set(e.value().parse().unwrap_or(0.7)),
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Gini Coefficient (0=equal, 1=max inequality): {gini_coefficient():.2}" }
                input {
                    r#type: "range",
                    min: "0.0",
                    max: "1.0",
                    step: "0.01",
                    value: "{gini_coefficient()}",
                    style: "width: 100%; box-sizing: border-box; accent-color: #f38ba8;",
                    oninput: move |e| gini_coefficient.set(e.value().parse().unwrap_or(0.35)),
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Notes" }
                textarea {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box; min-height: 60px; resize: vertical;",
                    placeholder: "Additional notes...",
                    oninput: move |e| notes.set(e.value()),
                }
            }

            div {
                style: "background: #11111b; padding: 16px; border-radius: 8px; border-left: 4px solid #94e2d5; flex: 1;",
                h3 { style: "margin-top: 0; color: #94e2d5; font-size: 0.9rem;", "QualiaDB Output" }
                div { style: "font-size: 0.8rem; color: #a6adc8; display: flex; flex-direction: column; gap: 4px;",
                    div { "Process: {global_process()}" }
                    div { "Lens: {theoretical_lens()}" }
                    div { "Scale: {scale()}" }
                    div { style: "color: if hdi_score() > 0.7 { \"#a6e3a1\" } else { \"#f38ba8\" };", "HDI: {hdi_score():.2}" }
                    div { style: "color: if gini_coefficient() < 0.4 { \"#a6e3a1\" } else { \"#f38ba8\" };", "Gini: {gini_coefficient():.2}" }
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 8px;", "QualiaDB → graph theory | Allen Interval | epistemic engine" }
            }
        }
    }
}
