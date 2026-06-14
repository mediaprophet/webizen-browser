use dioxus::prelude::*;

#[component]
pub fn HistoricalLinguisticsQapp() -> Element {
    let mut method = use_signal(|| "Comparative Method".to_string());
    let mut language_family = use_signal(|| "Proto-Indo-European".to_string());
    let mut sound_change_type = use_signal(|| "Grimm's Law".to_string());
    let mut proto_form = use_signal(|| String::new());
    let mut attested_reflexes = use_signal(|| String::new());
    let mut time_depth_bce = use_signal(|| 3000u32);
    let mut notes = use_signal(|| String::new());

    let methods = ["Comparative Method", "Internal Reconstruction", "Glottochronology", "Phylogenetic", "Mass Comparison (Greenberg)"];
    let families = ["Proto-Indo-European", "Proto-Semitic", "Proto-Bantu", "Proto-Austronesian", "Proto-Turkic", "Proto-Uralic"];
    let sound_changes = ["Grimm's Law", "Great Vowel Shift", "Ruki Rule", "Lenition", "Assimilation", "Metathesis"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #89b4fa; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Historical Linguistics" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
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
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Language Family" }
                    select {
                        value: "{language_family}",
                        onchange: move |e| language_family.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in families { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Sound Change Type" }
                    select {
                        value: "{sound_change_type}",
                        onchange: move |e| sound_change_type.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in sound_changes { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Proto-Form" }
                    input {
                        r#type: "text",
                        value: "{proto_form}",
                        oninput: move |e| proto_form.set(e.value()),
                        placeholder: "e.g. *pter-, *gʷen-, *h₂ster-",
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Attested Reflexes" }
                    input {
                        r#type: "text",
                        value: "{attested_reflexes}",
                        oninput: move |e| attested_reflexes.set(e.value()),
                        placeholder: "e.g. Lat. pater, Gk. pater, Skt. pita",
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Time Depth BCE: {time_depth_bce}" }
                input {
                    r#type: "range",
                    min: "0",
                    max: "6000",
                    step: "100",
                    value: "{time_depth_bce}",
                    oninput: move |e| time_depth_bce.set(e.value().parse().unwrap_or(3000)),
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #89b4fa;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{language_family} | {method} | {sound_change_type} | {time_depth_bce} BCE | *{proto_form}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → historical linguistics engine | comparative sieve | sound change anchor" }
            }
        }
    }
}
