use dioxus::prelude::*;

#[component]
pub fn IntegralStudiesQapp() -> Element {
    let mut integral_framework = use_signal(|| "Wilber's AQAL".to_string());
    let mut quadrant = use_signal(|| "Individual Interior".to_string());
    let mut developmental_level = use_signal(|| "Rational".to_string());
    let mut lines = use_signal(|| "Cognitive".to_string());
    let mut altitude = use_signal(|| 50u32);
    let mut notes = use_signal(|| String::new());

    let integral_frameworks = ["Wilber's AQAL", "Integral Ecology", "Integral Education", "Integral Politics", "Integral Spirituality"];
    let quadrants = ["Individual Interior", "Individual Exterior", "Collective Interior", "Collective Exterior"];
    let developmental_levels = ["Archaic", "Magic", "Mythic", "Rational", "Pluralistic", "Integral", "Super-Integral"];
    let lines_options = ["Cognitive", "Emotional", "Moral", "Spiritual", "Interpersonal"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #a6e3a1; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Integral Studies" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Integral Framework" }
                select {
                    value: "{integral_framework}", onchange: move |e| integral_framework.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in integral_frameworks { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Quadrant" }
                select {
                    value: "{quadrant}", onchange: move |e| quadrant.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in quadrants { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Developmental Level" }
                select {
                    value: "{developmental_level}", onchange: move |e| developmental_level.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in developmental_levels { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Lines" }
                select {
                    value: "{lines}", onchange: move |e| lines.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in lines_options { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Altitude: {altitude}" }
                input { r#type: "range", min: "0", max: "100", value: "{altitude}",
                    oninput: move |e| altitude.set(e.value().parse().unwrap_or(50)),
                    style: "width: 100%; margin-top: 4px;" }
            }
            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea { value: "{notes}", oninput: move |e| notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 60px;" }
            }
            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #a6e3a1;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{integral_framework} | {quadrant} | {developmental_level} | {lines} | altitude: {altitude}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → integral studies engine | discourse sieve | anchor" }
            }
        }
    }
}
