use dioxus::prelude::*;

#[component]
pub fn RadicalMediaStudiesQapp() -> Element {
    let mut radical_tradition = use_signal(|| "Marxist Political Economy".to_string());
    let mut ownership_critique = use_signal(|| "Concentration".to_string());
    let mut alternative = use_signal(|| "Cooperative".to_string());
    let mut counter_power_index = use_signal(|| 50u32);
    let mut reach = use_signal(|| 10000u32);
    let mut notes = use_signal(|| String::new());

    let traditions = ["Marxist Political Economy", "Cultural Imperialism", "Alternative Media", "Participatory Media", "Hacker Culture", "Anarchist Media"];
    let critiques = ["Concentration", "Advertising Dependence", "State Control", "Platform Monopoly"];
    let alternatives = ["Cooperative", "Community Radio", "Zine", "Open Source", "Mesh Network"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #a6e3a1; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Radical Media Studies" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Radical Tradition" }
                select {
                    value: "{radical_tradition}",
                    onchange: move |e| radical_tradition.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in traditions { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Ownership Critique" }
                select {
                    value: "{ownership_critique}",
                    onchange: move |e| ownership_critique.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in critiques { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Alternative Form" }
                select {
                    value: "{alternative}",
                    onchange: move |e| alternative.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in alternatives { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Counter-Power Index: {counter_power_index}" }
                input {
                    r#type: "range", min: "0", max: "100", value: "{counter_power_index}",
                    oninput: move |e| counter_power_index.set(e.value().parse().unwrap_or(50)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Reach: {reach}" }
                input {
                    r#type: "range", min: "0", max: "1000000", value: "{reach}",
                    oninput: move |e| reach.set(e.value().parse().unwrap_or(10000)),
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
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{radical_tradition} | {alternative} | Counter-Power: {counter_power_index} | Reach: {reach}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → radical media engine | counter-power sieve | reach anchor" }
            }
        }
    }
}
