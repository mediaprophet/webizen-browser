use dioxus::prelude::*;

#[component]
pub fn BiogeochemistryQapp() -> Element {
    let mut cycle = use_signal(|| "Carbon".to_string());
    let mut ecosystem = use_signal(|| "Terrestrial Forest".to_string());
    let mut process = use_signal(|| "Decomposition".to_string());
    let mut pool_size = use_signal(|| 500.0f64);
    let mut flux_rate = use_signal(|| 10.0f64);
    let mut perturbation = use_signal(|| 20u32);
    let mut notes = use_signal(|| String::new());

    let cycles = ["Carbon", "Nitrogen", "Phosphorus", "Sulfur", "Water", "Oxygen"];
    let ecosystems = [
        "Terrestrial Forest", "Grassland", "Wetland", "Marine", "Freshwater", "Arctic", "Agricultural",
    ];
    let processes = [
        "Decomposition", "Nitrification", "Denitrification", "Photosynthesis", "Respiration", "Weathering",
    ];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",

            h2 {
                style: "margin: 0; color: #a6e3a1; border-bottom: 1px solid #313244; padding-bottom: 8px;",
                "Biogeochemistry"
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Biogeochemical Cycle" }
                select {
                    value: "{cycle}",
                    onchange: move |e| cycle.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in cycles { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Ecosystem" }
                select {
                    value: "{ecosystem}",
                    onchange: move |e| ecosystem.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in ecosystems { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Process" }
                select {
                    value: "{process}",
                    onchange: move |e| process.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in processes { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Pool Size (GtC): {pool_size():.1}" }
                input {
                    r#type: "range", min: "0", max: "3000",
                    value: "{pool_size()}",
                    oninput: move |e| pool_size.set(e.value().parse::<f64>().unwrap_or(500.0)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Flux Rate (GtC/yr): {flux_rate():.2}" }
                input {
                    r#type: "range", min: "0", max: "100",
                    value: "{flux_rate()}",
                    oninput: move |e| flux_rate.set(e.value().parse::<f64>().unwrap_or(10.0)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Perturbation Index: {perturbation}" }
                input {
                    r#type: "range", min: "0", max: "100",
                    value: "{perturbation}",
                    oninput: move |e| perturbation.set(e.value().parse().unwrap_or(20)),
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #a6e3a1;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{cycle} | {ecosystem} | {process} | pool {pool_size():.1} GtC | flux {flux_rate():.2} GtC/yr" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → knowledge engine | sieve | anchor" }
            }
        }
    }
}
