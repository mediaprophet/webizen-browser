use dioxus::prelude::*;

#[component]
pub fn AstrophysicsQapp() -> Element {
    let mut phenomenon = use_signal(|| "Stellar Evolution".to_string());
    let mut mass_solar = use_signal(|| 1.0f64);
    let mut luminosity_solar = use_signal(|| 1.0f64);
    let mut temperature_k = use_signal(|| 5778.0f64);
    let mut distance_ly = use_signal(|| 4.24f64);
    let mut model_type = use_signal(|| "ΛCDM".to_string());
    let mut notes = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #cba6f7; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Astrophysics QApp" }

            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Phenomenon" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| phenomenon.set(e.value()),
                        option { "Big Bang Nucleosynthesis" }
                        option { "Stellar Evolution" }
                        option { "Supernovae" }
                        option { "Neutron Stars" }
                        option { "Black Hole Thermodynamics" }
                        option { "Dark Matter" }
                        option { "Dark Energy" }
                        option { "Cosmic Inflation" }
                        option { "Gravitational Lensing" }
                        option { "Accretion Disk Dynamics" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Cosmological Model" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| model_type.set(e.value()),
                        option { "Standard Model" }
                        option { "ΛCDM" }
                        option { "Modified Gravity" }
                        option { "String Cosmology" }
                        option { "Brane World" }
                        option { "Loop Quantum Cosmology" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Mass (Solar Masses)" }
                    input {
                        r#type: "number",
                        value: "{mass_solar}",
                        step: "0.01",
                        min: "0.001",
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        oninput: move |e| mass_solar.set(e.value().parse().unwrap_or(1.0)),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Luminosity (Solar Units)" }
                    input {
                        r#type: "number",
                        value: "{luminosity_solar}",
                        step: "0.1",
                        min: "0.0001",
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        oninput: move |e| luminosity_solar.set(e.value().parse().unwrap_or(1.0)),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Temperature (K)" }
                    input {
                        r#type: "number",
                        value: "{temperature_k}",
                        step: "100",
                        min: "0",
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        oninput: move |e| temperature_k.set(e.value().parse().unwrap_or(5778.0)),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Distance (light-years)" }
                    input {
                        r#type: "number",
                        value: "{distance_ly}",
                        step: "0.01",
                        min: "0",
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        oninput: move |e| distance_ly.set(e.value().parse().unwrap_or(4.24)),
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Research Notes" }
                textarea {
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; min-height: 60px; box-sizing: border-box; resize: vertical;",
                    placeholder: "Enter theoretical framework, equations, observational constraints...",
                    oninput: move |e| notes.set(e.value()),
                    "{notes}"
                }
            }

            div {
                style: "background: #11111b; padding: 16px; border-radius: 8px; border-left: 4px solid #cba6f7; flex: 1;",
                h3 { style: "margin-top: 0; color: #cba6f7; font-size: 0.9rem;", "Output / Analysis" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px; font-size: 0.8rem;",
                    div { style: "color: #a6adc8;", "Phenomenon:" }
                    div { style: "color: #cdd6f4;", "{phenomenon}" }
                    div { style: "color: #a6adc8;", "Model:" }
                    div { style: "color: #cdd6f4;", "{model_type}" }
                    div { style: "color: #a6adc8;", "Mass (M☉):" }
                    div { style: "color: #cdd6f4;", "{mass_solar:.3}" }
                    div { style: "color: #a6adc8;", "Luminosity (L☉):" }
                    div { style: "color: #cdd6f4;", "{luminosity_solar:.4}" }
                    div { style: "color: #a6adc8;", "Temperature:" }
                    div { style: "color: #cdd6f4;", "{temperature_k:.0} K" }
                    div { style: "color: #a6adc8;", "Distance:" }
                    div { style: "color: #cdd6f4;", "{distance_ly:.2} ly" }
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 12px; border-top: 1px solid #313244; padding-top: 8px;",
                    "QualiaDB → numerical ODE solver | Allen Interval Algebra | quantum DFT engine"
                }
            }
        }
    }
}
