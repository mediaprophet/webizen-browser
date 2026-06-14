use dioxus::prelude::*;

#[component]
pub fn RaceStudiesQapp() -> Element {
    let mut approach = use_signal(|| "Sociological".to_string());
    let mut racial_category = use_signal(|| "Black/African Diaspora".to_string());
    let mut theoretical_tradition = use_signal(|| "Du Bois".to_string());
    let mut racial_inequality_index = use_signal(|| 50u32);
    let mut colorblindness_index = use_signal(|| 30u32);
    let mut notes = use_signal(|| String::new());

    let approaches = ["Sociological", "Historical", "Cultural", "Biological Critique", "Comparative", "Global"];
    let categories = ["Black/African Diaspora", "Asian", "Indigenous", "Latinx", "White", "Mixed/Multiracial", "Other"];
    let traditions = ["Du Bois", "Myrdal", "Park", "Cox", "Omi/Winant", "Bonilla-Silva"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #cba6f7; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Race Studies" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Approach" }
                select {
                    value: "{approach}",
                    onchange: move |e| approach.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in approaches { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Racial Category Studied" }
                select {
                    value: "{racial_category}",
                    onchange: move |e| racial_category.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in categories { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Tradition" }
                select {
                    value: "{theoretical_tradition}",
                    onchange: move |e| theoretical_tradition.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in traditions { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Racial Inequality Index: {racial_inequality_index}" }
                input {
                    r#type: "range", min: "0", max: "100", value: "{racial_inequality_index}",
                    oninput: move |e| racial_inequality_index.set(e.value().parse().unwrap_or(50)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Colorblindness Index: {colorblindness_index}" }
                input {
                    r#type: "range", min: "0", max: "100", value: "{colorblindness_index}",
                    oninput: move |e| colorblindness_index.set(e.value().parse().unwrap_or(30)),
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #cba6f7;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{approach} | {racial_category} | {theoretical_tradition} | Inequality: {racial_inequality_index}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → race studies engine | formation sieve | inequality anchor" }
            }
        }
    }
}
