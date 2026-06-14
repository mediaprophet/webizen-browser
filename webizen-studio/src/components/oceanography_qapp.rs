use dioxus::prelude::*;

#[component]
pub fn OceanographyQapp() -> Element {
    let mut subdiscipline = use_signal(|| "Physical".to_string());
    let mut ocean_basin = use_signal(|| "Pacific".to_string());
    let mut depth_m = use_signal(|| 200u32);
    let mut temperature_c = use_signal(|| 4.0f64);
    let mut salinity_ppt = use_signal(|| 35.0f64);
    let mut current_system = use_signal(|| "Gulf Stream".to_string());
    let mut dissolved_oxygen_mgl = use_signal(|| 6.5f64);
    let mut sampling_notes = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #89dceb; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Oceanography QApp" }

            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Subdiscipline" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| subdiscipline.set(e.value()),
                        option { "Physical" }
                        option { "Chemical" }
                        option { "Biological" }
                        option { "Geological" }
                        option { "Paleoceanography" }
                        option { "Marine Ecology" }
                        option { "Climate Oceanography" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Ocean Basin" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| ocean_basin.set(e.value()),
                        option { "Pacific" }
                        option { "Atlantic" }
                        option { "Indian" }
                        option { "Arctic" }
                        option { "Southern" }
                        option { "Mediterranean" }
                        option { "Caribbean" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Current System" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| current_system.set(e.value()),
                        option { "Gulf Stream" }
                        option { "AMOC" }
                        option { "Kuroshio" }
                        option { "Antarctic Circumpolar" }
                        option { "El Niño" }
                        option { "La Niña" }
                        option { "Humboldt" }
                        option { "North Atlantic Gyre" }
                        option { "Indian Ocean Dipole" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Depth (m): {depth_m}" }
                    input {
                        r#type: "range",
                        min: "0",
                        max: "11000",
                        step: "10",
                        value: "{depth_m}",
                        style: "width: 100%; margin-top: 8px;",
                        oninput: move |e| depth_m.set(e.value().parse().unwrap_or(200)),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Temperature (°C)" }
                    input {
                        r#type: "number",
                        value: "{temperature_c}",
                        step: "0.1",
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        oninput: move |e| temperature_c.set(e.value().parse().unwrap_or(4.0)),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Salinity (ppt)" }
                    input {
                        r#type: "number",
                        value: "{salinity_ppt}",
                        step: "0.1",
                        min: "0",
                        max: "50",
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        oninput: move |e| salinity_ppt.set(e.value().parse().unwrap_or(35.0)),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Dissolved O₂ (mg/L)" }
                    input {
                        r#type: "number",
                        value: "{dissolved_oxygen_mgl}",
                        step: "0.01",
                        min: "0",
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        oninput: move |e| dissolved_oxygen_mgl.set(e.value().parse().unwrap_or(6.5)),
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Sampling Notes" }
                textarea {
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; min-height: 60px; box-sizing: border-box; resize: vertical;",
                    placeholder: "CTD cast data, cruise details, instrument calibration, sample preservation...",
                    oninput: move |e| sampling_notes.set(e.value()),
                    "{sampling_notes}"
                }
            }

            div {
                style: "background: #11111b; padding: 16px; border-radius: 8px; border-left: 4px solid #89dceb; flex: 1;",
                h3 { style: "margin-top: 0; color: #89dceb; font-size: 0.9rem;", "Output / Analysis" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px; font-size: 0.8rem;",
                    div { style: "color: #a6adc8;", "Subdiscipline:" }
                    div { style: "color: #cdd6f4;", "{subdiscipline}" }
                    div { style: "color: #a6adc8;", "Basin:" }
                    div { style: "color: #cdd6f4;", "{ocean_basin}" }
                    div { style: "color: #a6adc8;", "Current System:" }
                    div { style: "color: #cdd6f4;", "{current_system}" }
                    div { style: "color: #a6adc8;", "Depth:" }
                    div { style: "color: #cdd6f4;", "{depth_m} m" }
                    div { style: "color: #a6adc8;", "T / S:" }
                    div { style: "color: #cdd6f4;", "{temperature_c:.1}°C / {salinity_ppt:.1} ppt" }
                    div { style: "color: #a6adc8;", "DO:" }
                    div { style: "color: #cdd6f4;", "{dissolved_oxygen_mgl:.2} mg/L" }
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 12px; border-top: 1px solid #313244; padding-top: 8px;",
                    "QualiaDB → ODE fluid dynamics solver | Allen Interval temporal | geochemical sieve"
                }
            }
        }
    }
}
