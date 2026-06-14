use dioxus::prelude::*;

#[component]
pub fn AffectTheoryQapp() -> Element {
    let mut theorist = use_signal(|| "Spinoza".to_string());
    let mut affect = use_signal(|| "Joy".to_string());
    let mut tradition = use_signal(|| "Deleuzian".to_string());
    let mut site = use_signal(|| "Body".to_string());
    let mut intensity = use_signal(|| 50u32);
    let mut duration = use_signal(|| "Fleeting".to_string());
    let mut notes = use_signal(|| String::new());

    let theorists = ["Spinoza", "Deleuze/Guattari", "Massumi", "Ahmed", "Berlant", "Sedgwick", "Ngai"];
    let affects = ["Joy", "Shame", "Fear", "Disgust", "Anger", "Grief", "Hope", "Anxiety", "Boredom"];
    let traditions = ["Deleuzian", "Psychoanalytic", "Phenomenological", "Feminist", "Neuroscientific"];
    let sites = ["Body", "Social", "Political", "Cultural", "Digital"];
    let durations = ["Fleeting", "Sustained", "Chronic"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #cba6f7; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Affect Theory" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Theorist" }
                select {
                    value: "{theorist}", onchange: move |e| theorist.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in theorists { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Affect" }
                select {
                    value: "{affect}", onchange: move |e| affect.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in affects { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Tradition" }
                select {
                    value: "{tradition}", onchange: move |e| tradition.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in traditions { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Site" }
                select {
                    value: "{site}", onchange: move |e| site.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in sites { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Intensity: {intensity}" }
                input { r#type: "range", min: "0", max: "100", value: "{intensity}",
                    oninput: move |e| intensity.set(e.value().parse().unwrap_or(50)),
                    style: "width: 100%; margin-top: 4px;" }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Duration" }
                select {
                    value: "{duration}", onchange: move |e| duration.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in durations { option { value: "{x}", "{x}" } }
                }
            }
            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea { value: "{notes}", oninput: move |e| notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 60px;" }
            }
            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #cba6f7;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{theorist} | {affect} | {tradition} | {site} | {duration} | intensity: {intensity}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → affect theory engine | discourse sieve | anchor" }
            }
        }
    }
}
