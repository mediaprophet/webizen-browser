use dioxus::prelude::*;

#[component]
pub fn CriticalRaceAndEthnicStudiesQapp() -> Element {
    let mut theoretical_tradition = use_signal(|| "Critical Race Theory".to_string());
    let mut focus = use_signal(|| "Structural Racism".to_string());
    let mut methodology = use_signal(|| "Counternarrative".to_string());
    let mut inequality_index = use_signal(|| 0.6f64);
    let mut notes = use_signal(|| String::new());

    let traditions = ["Critical Race Theory", "Ethnic Studies", "Intersectionality", "Postcolonialism", "Afrocentrism", "LatCrit", "AsianCrit"];
    let focuses = ["Structural Racism", "Representation", "Identity", "Law and Policy", "Cultural Production", "History"];
    let methodologies = ["Counternarrative", "Intersectional Analysis", "Historical Materialism", "Discourse Analysis"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",

            h2 { style: "margin: 0; color: #fab387; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Critical Race & Ethnic Studies" }

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
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Focus" }
                select {
                    value: "{focus}",
                    onchange: move |e| focus.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in focuses { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Methodology" }
                select {
                    value: "{methodology}",
                    onchange: move |e| methodology.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in methodologies { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Inequality Index: {inequality_index:.2}" }
                input {
                    r#type: "range", min: "0", max: "100",
                    value: "{inequality_index() * 100.0}",
                    oninput: move |e| inequality_index.set(e.value().parse::<f64>().unwrap_or(60.0) / 100.0),
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
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{theoretical_tradition} | {focus} | {methodology} | Inequality: {inequality_index:.2}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → knowledge engine | sieve | anchor" }
            }
        }
    }
}
