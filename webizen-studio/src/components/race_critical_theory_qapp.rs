use dioxus::prelude::*;

#[component]
pub fn RaceCriticalTheoryQapp() -> Element {
    let mut framework = use_signal(|| "Critical Race Theory".to_string());
    let mut racial_formation_site = use_signal(|| "Law".to_string());
    let mut theorist = use_signal(|| "Crenshaw".to_string());
    let mut structural_depth = use_signal(|| 50u32);
    let mut remedy_orientation = use_signal(|| "Reform".to_string());
    let mut notes = use_signal(|| String::new());

    let frameworks = ["Critical Race Theory", "Structural Racism Analysis", "Interest Convergence", "Counterstorytelling", "Intersectionality", "Racial Formation"];
    let sites = ["Law", "Education", "Housing", "Media", "Police", "Healthcare", "Immigration"];
    let theorists = ["Crenshaw", "Bell", "Delgado", "Matsuda", "Gotanda", "Cho"];
    let remedies = ["Reform", "Abolition", "Transformation"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #f5c2e7; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Race Critical Theory" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Framework" }
                select {
                    value: "{framework}",
                    onchange: move |e| framework.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in frameworks { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Racial Formation Site" }
                select {
                    value: "{racial_formation_site}",
                    onchange: move |e| racial_formation_site.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in sites { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Theorist" }
                select {
                    value: "{theorist}",
                    onchange: move |e| theorist.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in theorists { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Structural Depth: {structural_depth}" }
                input {
                    r#type: "range", min: "0", max: "100", value: "{structural_depth}",
                    oninput: move |e| structural_depth.set(e.value().parse().unwrap_or(50)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Remedy Orientation" }
                select {
                    value: "{remedy_orientation}",
                    onchange: move |e| remedy_orientation.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in remedies { option { value: "{x}", "{x}" } }
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #f5c2e7;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{framework} | {racial_formation_site} | {theorist} | Depth: {structural_depth} | {remedy_orientation}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → critical race theory engine | intersectionality sieve | structure anchor" }
            }
        }
    }
}
