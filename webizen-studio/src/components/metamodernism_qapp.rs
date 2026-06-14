use dioxus::prelude::*;

#[component]
pub fn MetamodernismQapp() -> Element {
    let mut oscillation = use_signal(|| "Sincere-Ironic".to_string());
    let mut cultural_form = use_signal(|| "Literature".to_string());
    let mut theorist = use_signal(|| "Vermeulen/van den Akker".to_string());
    let mut generational_context = use_signal(|| "Millennial".to_string());
    let mut oscillation_amplitude = use_signal(|| 50u32);
    let mut depth = use_signal(|| 50u32);
    let mut notes = use_signal(|| String::new());

    let oscillations = ["Sincere-Ironic", "Utopian-Dystopian", "Naive-Knowing", "Optimistic-Pessimistic"];
    let forms = ["Literature", "Film", "Music", "Visual Art", "Internet Culture", "Fashion"];
    let theorists = ["Vermeulen/van den Akker", "Freinacht", "Hanzi", "Timotheus"];
    let generations = ["Gen X", "Millennial", "Gen Z"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #a6e3a1; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Metamodernism" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Metamodern Oscillation" }
                select {
                    value: "{oscillation}",
                    onchange: move |e| oscillation.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in oscillations { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Cultural Form" }
                select {
                    value: "{cultural_form}",
                    onchange: move |e| cultural_form.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in forms { option { value: "{x}", "{x}" } }
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
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Generational Context" }
                select {
                    value: "{generational_context}",
                    onchange: move |e| generational_context.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in generations { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Oscillation Amplitude: {oscillation_amplitude}" }
                input {
                    r#type: "range", min: "0", max: "100", value: "{oscillation_amplitude}",
                    oninput: move |e| oscillation_amplitude.set(e.value().parse().unwrap_or(50)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Depth: {depth}" }
                input {
                    r#type: "range", min: "0", max: "100", value: "{depth}",
                    oninput: move |e| depth.set(e.value().parse().unwrap_or(50)),
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
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{oscillation} | {cultural_form} | Amplitude: {oscillation_amplitude} | Depth: {depth}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → metamodern engine | oscillation sieve | depth anchor" }
            }
        }
    }
}
