use dioxus::prelude::*;

#[component]
pub fn CognitiveScienceQapp() -> Element {
    let mut level = use_signal(|| "Computational".to_string());
    let mut paradigm = use_signal(|| "Connectionism".to_string());
    let mut cognitive_process = use_signal(|| "Perception".to_string());
    let mut method = use_signal(|| "Behavioural Experiment".to_string());
    let mut priors_strength = use_signal(|| 0.5f64);
    let mut notes = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box; overflow-y: auto;",
            h2 { style: "margin: 0; color: #f9e2af; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Cognitive Science QApp" }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Level of Analysis" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| level.set(e.value()),
                    option { "Neural" }
                    option { selected: true, "Computational" }
                    option { "Algorithmic" }
                    option { "Representational" }
                    option { "Behavioural" }
                    option { "Social" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Paradigm" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| paradigm.set(e.value()),
                    option { "Symbolicism" }
                    option { selected: true, "Connectionism" }
                    option { "Embodied Cognition" }
                    option { "Predictive Processing" }
                    option { "Bayesian Brain" }
                    option { "Enactivism" }
                    option { "4E Cognition" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Cognitive Process" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| cognitive_process.set(e.value()),
                    option { selected: true, "Perception" }
                    option { "Attention" }
                    option { "Memory" }
                    option { "Language" }
                    option { "Reasoning" }
                    option { "Decision Making" }
                    option { "Motor Control" }
                    option { "Social Cognition" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Method" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| method.set(e.value()),
                    option { "fMRI" }
                    option { "EEG" }
                    option { selected: true, "Behavioural Experiment" }
                    option { "Computational Modelling" }
                    option { "Eye Tracking" }
                    option { "TMS" }
                    option { "Lesion Study" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Priors Strength: {priors_strength():.2}" }
                input {
                    r#type: "range",
                    min: "0.0",
                    max: "1.0",
                    step: "0.01",
                    value: "{priors_strength()}",
                    style: "width: 100%; box-sizing: border-box; accent-color: #f9e2af;",
                    oninput: move |e| priors_strength.set(e.value().parse().unwrap_or(0.5)),
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Notes" }
                textarea {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box; min-height: 60px; resize: vertical;",
                    placeholder: "Research notes...",
                    oninput: move |e| notes.set(e.value()),
                }
            }

            div {
                style: "background: #11111b; padding: 16px; border-radius: 8px; border-left: 4px solid #f9e2af; flex: 1;",
                h3 { style: "margin-top: 0; color: #f9e2af; font-size: 0.9rem;", "QualiaDB Output" }
                div { style: "font-size: 0.8rem; color: #a6adc8; display: flex; flex-direction: column; gap: 4px;",
                    div { "Level: {level()}" }
                    div { "Paradigm: {paradigm()}" }
                    div { "Process: {cognitive_process()}" }
                    div { "Method: {method()}" }
                    div { style: "color: if priors_strength() > 0.7 { \"#a6e3a1\" } else { \"#f38ba8\" };", "Priors: {priors_strength():.2}" }
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 8px;", "QualiaDB → epistemic logic | ODE solver | neuro-symbolic sieve" }
            }
        }
    }
}
