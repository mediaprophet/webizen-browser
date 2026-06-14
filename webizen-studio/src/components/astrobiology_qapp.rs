use dioxus::prelude::*;

#[component]
pub fn AstrobiologyQapp() -> Element {
    let mut life_detection_method = use_signal(|| "Spectroscopic Biosignatures".to_string());
    let mut target_body = use_signal(|| "Mars".to_string());
    let mut habitable_zone_type = use_signal(|| "Circumstellar".to_string());
    let mut biomolecule = use_signal(|| "Amino Acid".to_string());
    let mut distance_ly = use_signal(|| 0.0f64);
    let mut temperature_k = use_signal(|| 273.0f64);
    let mut notes = use_signal(|| String::new());

    let methods = [
        "Spectroscopic Biosignatures", "Radio SETI", "Direct Sample",
        "Radar", "Gravitational Microlensing", "Transit Photometry",
    ];
    let targets = [
        "Mars", "Enceladus", "Europa", "Titan",
        "Exoplanet Habitable Zone", "Interstellar",
    ];
    let zone_types = ["Circumstellar", "Subsurface Ocean", "Atmospheric", "Hydrothermal"];
    let biomolecules = ["DNA", "RNA", "Amino Acid", "Lipid", "ATP Analogue", "Unknown"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #89dceb; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Astrobiology" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Life Detection Method" }
                    select {
                        value: "{life_detection_method}",
                        onchange: move |e| life_detection_method.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in methods { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Target Body" }
                    select {
                        value: "{target_body}",
                        onchange: move |e| target_body.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in targets { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Habitable Zone Type" }
                    select {
                        value: "{habitable_zone_type}",
                        onchange: move |e| habitable_zone_type.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in zone_types { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Biomolecule" }
                    select {
                        value: "{biomolecule}",
                        onchange: move |e| biomolecule.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in biomolecules { option { value: "{x}", "{x}" } }
                    }
                }
            }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Distance (light-years)" }
                    input {
                        r#type: "number",
                        value: "{distance_ly}",
                        oninput: move |e| distance_ly.set(e.value().parse().unwrap_or(0.0)),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Temperature (K)" }
                    input {
                        r#type: "number",
                        value: "{temperature_k}",
                        oninput: move |e| temperature_k.set(e.value().parse().unwrap_or(273.0)),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #89dceb;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{target_body} | {life_detection_method} | {biomolecule} | T: {temperature_k:.1}K" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → biosignature engine | habitability sieve | exoplanet graph" }
            }
        }
    }
}
