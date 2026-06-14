use dioxus::prelude::*;

#[component]
pub fn DisasterStudiesQapp() -> Element {
    let mut disaster_type = use_signal(|| "Natural".to_string());
    let mut hazard = use_signal(|| "Earthquake".to_string());
    let mut affected_population = use_signal(|| 500000u32);
    let mut damage_estimate = use_signal(|| 10.0f64);
    let mut relief_phase = use_signal(|| "Immediate".to_string());
    let mut notes = use_signal(|| String::new());

    let disaster_types = ["Natural", "Technological", "Complex Emergency", "Pandemic", "Climate"];
    let hazards = ["Earthquake", "Flood", "Hurricane", "Wildfire", "Industrial Accident", "Biological"];
    let phases = ["Immediate", "Recovery", "Reconstruction", "Mitigation"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",

            h2 { style: "margin: 0; color: #f38ba8; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Disaster Studies" }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Disaster Type" }
                select {
                    value: "{disaster_type}",
                    onchange: move |e| disaster_type.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in disaster_types { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Hazard" }
                select {
                    value: "{hazard}",
                    onchange: move |e| hazard.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in hazards { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Affected Population: {affected_population}" }
                input {
                    r#type: "range", min: "0", max: "10000000",
                    value: "{affected_population}",
                    oninput: move |e| affected_population.set(e.value().parse().unwrap_or(500000)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Damage Estimate $B: {damage_estimate:.1}" }
                input {
                    r#type: "range", min: "0", max: "100",
                    value: "{damage_estimate() * 0.2}",
                    oninput: move |e| damage_estimate.set(e.value().parse::<f64>().unwrap_or(2.0) * 5.0),
                    style: "width: 100%; margin-top: 4px;"
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Relief Phase" }
                select {
                    value: "{relief_phase}",
                    onchange: move |e| relief_phase.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in phases { option { value: "{x}", "{x}" } }
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #f38ba8;",
                span {
                    style: if damage_estimate() > 100.0 { "font-size: 0.8rem; color: #f38ba8;" } else { "font-size: 0.8rem; color: #f9e2af;" },
                    "{disaster_type} | {hazard} | Pop: {affected_population} | ${damage_estimate:.1}B | Phase: {relief_phase}"
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → knowledge engine | sieve | anchor" }
            }
        }
    }
}
