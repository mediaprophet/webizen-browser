use dioxus::prelude::*;

#[component]
pub fn MathematicalBiologyQapp() -> Element {
    let mut model_type = use_signal(|| "Lotka-Volterra".to_string());
    let mut biological_system = use_signal(|| "Population".to_string());
    let mut parameter = use_signal(|| "Growth Rate".to_string());
    let mut population_size = use_signal(|| 10000u32);
    let mut growth_rate = use_signal(|| 0.5f64);
    let mut notes = use_signal(|| String::new());

    let model_types = [
        "Lotka-Volterra", "SIR Epidemiological", "Reaction-Diffusion",
        "Game Theory", "Neural Field", "Evolutionary Dynamics", "Network",
    ];
    let biological_systems = ["Population", "Ecosystem", "Cell", "Tissue", "Neural", "Evolutionary"];
    let parameters = [
        "Growth Rate", "Carrying Capacity", "Transmission Rate",
        "Diffusion Coefficient", "Selection Coefficient",
    ];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",

            h2 {
                style: "margin: 0; color: #f5c2e7; border-bottom: 1px solid #313244; padding-bottom: 8px;",
                "Mathematical Biology"
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Model Type" }
                select {
                    value: "{model_type}",
                    onchange: move |e| model_type.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in model_types { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Biological System" }
                select {
                    value: "{biological_system}",
                    onchange: move |e| biological_system.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in biological_systems { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Key Parameter" }
                select {
                    value: "{parameter}",
                    onchange: move |e| parameter.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in parameters { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Population Size: {population_size}" }
                input {
                    r#type: "range", min: "0", max: "1000000",
                    value: "{population_size}",
                    oninput: move |e| population_size.set(e.value().parse().unwrap_or(10000)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Growth Rate: {growth_rate():.3}" }
                input {
                    r#type: "range", min: "0", max: "200",
                    value: "{growth_rate() * 100.0}",
                    oninput: move |e| growth_rate.set(e.value().parse::<f64>().unwrap_or(50.0) / 100.0),
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
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{model_type} | {biological_system} | {parameter} | N={population_size} | r={growth_rate():.3}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → knowledge engine | sieve | anchor" }
            }
        }
    }
}
