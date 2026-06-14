use dioxus::prelude::*;

#[component]
pub fn AppliedLinguisticsQapp() -> Element {
    let mut domain = use_signal(|| "Second Language Acquisition".to_string());
    let mut l1 = use_signal(|| String::new());
    let mut l2 = use_signal(|| String::new());
    let mut acquisition_stage = use_signal(|| "Intermediate Fluency".to_string());
    let mut teaching_method = use_signal(|| "Task-Based".to_string());
    let mut interlanguage_feature = use_signal(|| String::new());
    let mut notes = use_signal(|| String::new());

    let domains = ["Second Language Acquisition", "Language Teaching", "Discourse Analysis", "Language Policy", "Forensic Linguistics", "Language & Technology", "Corpus Linguistics", "Translation Studies"];
    let stages = ["Silent Period", "Early Production", "Speech Emergence", "Intermediate Fluency", "Advanced"];
    let methods = ["Communicative", "Grammar-Translation", "Task-Based", "Content-Based", "Immersion", "Bilingual"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #a6e3a1; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Applied Linguistics" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Domain" }
                    select {
                        value: "{domain}",
                        onchange: move |e| domain.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in domains { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Acquisition Stage" }
                    select {
                        value: "{acquisition_stage}",
                        onchange: move |e| acquisition_stage.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in stages { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Teaching Method" }
                    select {
                        value: "{teaching_method}",
                        onchange: move |e| teaching_method.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in methods { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "L1 (First Language)" }
                    input {
                        r#type: "text",
                        value: "{l1}",
                        oninput: move |e| l1.set(e.value()),
                        placeholder: "e.g. Mandarin, Spanish, Arabic",
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "L2 (Target Language)" }
                    input {
                        r#type: "text",
                        value: "{l2}",
                        oninput: move |e| l2.set(e.value()),
                        placeholder: "e.g. English, French, German",
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Interlanguage Feature" }
                    input {
                        r#type: "text",
                        value: "{interlanguage_feature}",
                        oninput: move |e| interlanguage_feature.set(e.value()),
                        placeholder: "e.g. article omission, verb tense error",
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #a6e3a1;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{domain} | L1:{l1} → L2:{l2} | {acquisition_stage} | {teaching_method}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → SLA engine | interlanguage sieve | corpus anchor" }
            }
        }
    }
}
