use dioxus::prelude::*;

#[component]
pub fn MaterialCultureStudiesQapp() -> Element {
    let mut object_category = use_signal(|| "Domestic".to_string());
    let mut material = use_signal(|| "Ceramic".to_string());
    let mut production_mode = use_signal(|| "Handcraft".to_string());
    let mut theoretical_lens = use_signal(|| "Agency".to_string());
    let mut preservation_condition = use_signal(|| 70u32);
    let mut notes = use_signal(|| String::new());

    let categories = ["Domestic", "Religious", "Political", "Technological", "Fashion", "Food", "Funerary", "Trade"];
    let materials = ["Ceramic", "Metal", "Textile", "Wood", "Glass", "Stone", "Organic", "Composite"];
    let production_modes = ["Handcraft", "Industrial", "Digital Fabrication", "Found/Modified"];
    let lenses = ["Agency", "Semiotics", "Biography", "Assemblage", "Affordance"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",

            h2 { style: "margin: 0; color: #fab387; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Material Culture Studies" }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Object Category" }
                select {
                    value: "{object_category}",
                    onchange: move |e| object_category.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in categories { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Material" }
                select {
                    value: "{material}",
                    onchange: move |e| material.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in materials { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Production Mode" }
                select {
                    value: "{production_mode}",
                    onchange: move |e| production_mode.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in production_modes { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Lens" }
                select {
                    value: "{theoretical_lens}",
                    onchange: move |e| theoretical_lens.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in lenses { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Preservation Condition: {preservation_condition}%" }
                input {
                    r#type: "range", min: "0", max: "100",
                    value: "{preservation_condition}",
                    oninput: move |e| preservation_condition.set(e.value().parse().unwrap_or(70)),
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #fab387;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{object_category} | {material} | {production_mode} | {theoretical_lens} | Condition: {preservation_condition}%" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → knowledge engine | sieve | anchor" }
            }
        }
    }
}
