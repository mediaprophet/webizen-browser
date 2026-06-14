use dioxus::prelude::*;

#[component]
pub fn CeramicsQapp() -> Element {
    let mut technique = use_signal(|| "Wheel Throwing".to_string());
    let mut clay_body = use_signal(|| "Stoneware".to_string());
    let mut firing_temperature_c = use_signal(|| 1200u32);
    let mut glaze_type = use_signal(|| "Reduction".to_string());
    let mut surface_treatment = use_signal(|| "Carved".to_string());
    let mut historical_tradition = use_signal(|| "Japanese".to_string());
    let mut notes = use_signal(|| String::new());

    let techniques = ["Wheel Throwing", "Hand Building", "Slip Casting", "Press Moulding", "Coiling", "Pinching"];
    let clay_bodies = ["Earthenware", "Stoneware", "Porcelain", "Terracotta", "Bone China", "Raku", "Salt-Fire"];
    let glaze_types = ["Oxidation", "Reduction", "Wood Fire", "Soda", "Celadon", "Tenmoku", "Shino", "Unglazed"];
    let surface_treatments = ["Carved", "Stamped", "Sgraffito", "Underglaze", "Majolica", "Crystalline"];
    let traditions = ["Chinese", "Japanese", "Korean", "Islamic", "Indigenous", "Contemporary Studio"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #fab387; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Ceramics" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Technique" }
                    select {
                        value: "{technique}",
                        onchange: move |e| technique.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in techniques { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Clay Body" }
                    select {
                        value: "{clay_body}",
                        onchange: move |e| clay_body.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in clay_bodies { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Glaze Type" }
                    select {
                        value: "{glaze_type}",
                        onchange: move |e| glaze_type.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in glaze_types { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Surface Treatment" }
                    select {
                        value: "{surface_treatment}",
                        onchange: move |e| surface_treatment.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in surface_treatments { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Historical Tradition" }
                    select {
                        value: "{historical_tradition}",
                        onchange: move |e| historical_tradition.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in traditions { option { value: "{x}", "{x}" } }
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Firing Temperature (°C): {firing_temperature_c}" }
                input {
                    r#type: "range",
                    min: "600",
                    max: "1400",
                    value: "{firing_temperature_c}",
                    oninput: move |e| firing_temperature_c.set(e.value().parse().unwrap_or(1200)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea {
                    value: "{notes}",
                    oninput: move |e| notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 80px;"
                }
            }

            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #fab387;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{technique} | {clay_body} | {firing_temperature_c}°C | {glaze_type} | {historical_tradition}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → ceramics engine | glaze chemistry sieve | kiln tradition anchor" }
            }
        }
    }
}
