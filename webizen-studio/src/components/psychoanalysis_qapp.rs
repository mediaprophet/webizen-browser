use dioxus::prelude::*;

#[component]
pub fn PsychoanalysisQapp() -> Element {
    let mut school = use_signal(|| "Freudian".to_string());
    let mut concept = use_signal(|| "Unconscious".to_string());
    let mut clinical_modality = use_signal(|| "Individual".to_string());
    let mut analysand_constellation = use_signal(|| "Neurosis".to_string());
    let mut session_frequency = use_signal(|| 3u32);
    let mut analysis_duration_years = use_signal(|| 2u32);
    let mut notes = use_signal(|| String::new());

    let schools = ["Freudian", "Lacanian", "Kleinian", "Object Relations", "Jungian", "Self Psychology", "Relational"];
    let concepts = ["Unconscious", "Transference", "Repression", "Drive", "Fantasy", "Jouissance", "Sublimation", "Narcissism"];
    let modalities = ["Individual", "Group", "Child", "Couple", "Institutional"];
    let constellations = ["Neurosis", "Psychosis", "Perversion", "Borderline"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #89dceb; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Psychoanalysis" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "School" }
                select {
                    value: "{school}",
                    onchange: move |e| school.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in schools { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Concept" }
                select {
                    value: "{concept}",
                    onchange: move |e| concept.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in concepts { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Clinical Modality" }
                select {
                    value: "{clinical_modality}",
                    onchange: move |e| clinical_modality.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in modalities { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Analysand Constellation" }
                select {
                    value: "{analysand_constellation}",
                    onchange: move |e| analysand_constellation.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in constellations { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Session Frequency (per week): {session_frequency}" }
                input {
                    r#type: "range", min: "0", max: "5", value: "{session_frequency}",
                    oninput: move |e| session_frequency.set(e.value().parse().unwrap_or(3)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Analysis Duration (years): {analysis_duration_years}" }
                input {
                    r#type: "range", min: "0", max: "10", value: "{analysis_duration_years}",
                    oninput: move |e| analysis_duration_years.set(e.value().parse().unwrap_or(2)),
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #89dceb;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{school} | {concept} | {clinical_modality} | Freq: {session_frequency}/wk | Duration: {analysis_duration_years}yr" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → psychoanalytic engine | transference sieve | unconscious anchor" }
            }
        }
    }
}
