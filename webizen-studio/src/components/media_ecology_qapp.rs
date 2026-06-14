use dioxus::prelude::*;

#[component]
pub fn MediaEcologyQapp() -> Element {
    let mut theorist = use_signal(|| "McLuhan".to_string());
    let mut medium = use_signal(|| "Internet".to_string());
    let mut effects_model = use_signal(|| "Affordance".to_string());
    let mut epoch = use_signal(|| "Digital".to_string());
    let mut media_bias = use_signal(|| "Space".to_string());
    let mut notes = use_signal(|| String::new());

    let theorists = ["McLuhan", "Postman", "Ong", "Innis", "Mumford", "Stiegler", "Kittler"];
    let media = ["Print", "Radio", "Television", "Internet", "Mobile", "VR/AR", "Oral"];
    let effects_models = [
        "Media Determinism", "Social Construction", "Affordance", "Actor-Network", "Mediation",
    ];
    let epochs = ["Oral", "Manuscript", "Print", "Electric", "Digital", "Post-Digital"];
    let media_biases = ["Space", "Time", "Both"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",

            h2 {
                style: "margin: 0; color: #fab387; border-bottom: 1px solid #313244; padding-bottom: 8px;",
                "Media Ecology"
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Media Theorist" }
                select {
                    value: "{theorist}",
                    onchange: move |e| theorist.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in theorists { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Medium" }
                select {
                    value: "{medium}",
                    onchange: move |e| medium.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in media { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Effects Model" }
                select {
                    value: "{effects_model}",
                    onchange: move |e| effects_model.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in effects_models { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Epoch" }
                select {
                    value: "{epoch}",
                    onchange: move |e| epoch.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in epochs { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Media Bias (Innis)" }
                select {
                    value: "{media_bias}",
                    onchange: move |e| media_bias.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in media_biases { option { value: "{x}", "{x}" } }
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
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{theorist} | {medium} | {effects_model} | {epoch} | bias:{media_bias}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → knowledge engine | sieve | anchor" }
            }
        }
    }
}
