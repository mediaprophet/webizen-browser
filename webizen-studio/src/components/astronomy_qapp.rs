use dioxus::prelude::*;

#[component]
pub fn AstronomyQapp() -> Element {
    let mut celestial_object_type = use_signal(|| "Star".to_string());
    let mut observation_method = use_signal(|| "Optical".to_string());
    let mut telescope_aperture_m = use_signal(|| 1.0f64);
    let mut redshift_z = use_signal(|| 0.0f64);
    let mut magnitude = use_signal(|| 5.0f64);
    let mut spectral_class = use_signal(|| "G".to_string());
    let mut notes = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #89b4fa; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Astronomy QApp" }

            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Celestial Object Type" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| celestial_object_type.set(e.value()),
                        option { "Star" }
                        option { "Planet" }
                        option { "Galaxy" }
                        option { "Nebula" }
                        option { "Pulsar" }
                        option { "Quasar" }
                        option { "Black Hole" }
                        option { "White Dwarf" }
                        option { "Neutron Star" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Observation Method" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| observation_method.set(e.value()),
                        option { "Optical" }
                        option { "Radio" }
                        option { "X-ray" }
                        option { "Infrared" }
                        option { "UV" }
                        option { "Gravitational Wave" }
                        option { "Gamma Ray" }
                        option { "Neutrino" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Spectral Class" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| spectral_class.set(e.value()),
                        option { "O" }
                        option { "B" }
                        option { "A" }
                        option { "F" }
                        option { "G" }
                        option { "K" }
                        option { "M" }
                        option { "L" }
                        option { "T" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Apparent Magnitude" }
                    input {
                        r#type: "number",
                        value: "{magnitude}",
                        step: "0.1",
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        oninput: move |e| magnitude.set(e.value().parse().unwrap_or(5.0)),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Redshift z" }
                    input {
                        r#type: "number",
                        value: "{redshift_z}",
                        step: "0.001",
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        oninput: move |e| redshift_z.set(e.value().parse().unwrap_or(0.0)),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Telescope Aperture (m): {telescope_aperture_m:.1}" }
                    input {
                        r#type: "range",
                        min: "0.1",
                        max: "39.3",
                        step: "0.1",
                        value: "{telescope_aperture_m}",
                        style: "width: 100%; margin-top: 8px;",
                        oninput: move |e| telescope_aperture_m.set(e.value().parse().unwrap_or(1.0)),
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Observation Notes" }
                textarea {
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; min-height: 60px; box-sizing: border-box; resize: vertical;",
                    placeholder: "Enter observation notes, coordinates, session details...",
                    oninput: move |e| notes.set(e.value()),
                    "{notes}"
                }
            }

            div {
                style: "background: #11111b; padding: 16px; border-radius: 8px; border-left: 4px solid #89b4fa; flex: 1;",
                h3 { style: "margin-top: 0; color: #89b4fa; font-size: 0.9rem;", "Output / Analysis" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px; font-size: 0.8rem;",
                    div { style: "color: #a6adc8;", "Object Type:" }
                    div { style: "color: #cdd6f4;", "{celestial_object_type}" }
                    div { style: "color: #a6adc8;", "Method:" }
                    div { style: "color: #cdd6f4;", "{observation_method}" }
                    div { style: "color: #a6adc8;", "Spectral Class:" }
                    div { style: "color: #cdd6f4;", "{spectral_class}" }
                    div { style: "color: #a6adc8;", "Magnitude:" }
                    div { style: "color: #cdd6f4;", "{magnitude:.2}" }
                    div { style: "color: #a6adc8;", "Redshift z:" }
                    div { style: "color: #cdd6f4;", "{redshift_z:.4}" }
                    div { style: "color: #a6adc8;", "Aperture:" }
                    div { style: "color: #cdd6f4;", "{telescope_aperture_m:.1} m" }
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 12px; border-top: 1px solid #313244; padding-top: 8px;",
                    "QualiaDB → Allen Interval Algebra | spectral analysis engine | numerical ODE solver"
                }
            }
        }
    }
}
