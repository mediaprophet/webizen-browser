use dioxus::prelude::*;

#[component]
pub fn PatristicsQapp() -> Element {
    let mut period = use_signal(|| "Post-Nicene".to_string());
    let mut father_or_text = use_signal(|| String::new());
    let mut theological_locus = use_signal(|| "Christology".to_string());
    let mut tradition = use_signal(|| "Greek East".to_string());
    let mut method = use_signal(|| "Historical-Critical".to_string());
    let mut primary_language = use_signal(|| "Greek".to_string());
    let mut notes = use_signal(|| String::new());

    let periods = ["Apostolic Fathers", "Ante-Nicene", "Post-Nicene", "Golden Age", "Late Antique"];
    let loci = ["Trinity", "Christology", "Pneumatology", "Soteriology", "Ecclesiology", "Eschatology", "Scripture", "Anthropology"];
    let traditions = ["Greek East", "Latin West", "Syriac", "Coptic", "Armenian", "Ethiopian"];
    let methods = ["Historical-Critical", "Doctrinal", "Patristic Ressourcement", "Feminist", "Postcolonial Patristics"];
    let languages = ["Greek", "Latin", "Syriac", "Coptic"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #eba0ac; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Patristics" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Period" }
                    select {
                        value: "{period}",
                        onchange: move |e| period.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in periods { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Theological Locus" }
                    select {
                        value: "{theological_locus}",
                        onchange: move |e| theological_locus.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in loci { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Tradition" }
                    select {
                        value: "{tradition}",
                        onchange: move |e| tradition.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in traditions { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Method" }
                    select {
                        value: "{method}",
                        onchange: move |e| method.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in methods { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Primary Language" }
                    select {
                        value: "{primary_language}",
                        onchange: move |e| primary_language.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in languages { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Father or Text" }
                    input {
                        r#type: "text",
                        value: "{father_or_text}",
                        oninput: move |e| father_or_text.set(e.value()),
                        placeholder: "e.g. Augustine Confessions, Origen Contra Celsum",
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #eba0ac;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{period} | {tradition} | {theological_locus} | {primary_language} | {method}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → patristic corpus engine | doctrinal sieve | church father anchor" }
            }
        }
    }
}
