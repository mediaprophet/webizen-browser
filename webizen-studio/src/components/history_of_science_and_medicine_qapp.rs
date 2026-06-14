use dioxus::prelude::*;

#[component]
pub fn HistoryOfScienceAndMedicineQapp() -> Element {
    let mut period = use_signal(|| "Scientific Revolution".to_string());
    let mut discipline = use_signal(|| "Medicine".to_string());
    let mut historiographic_approach = use_signal(|| "Social Constructivist".to_string());
    let mut paradigm_shift = use_signal(|| String::new());
    let mut key_figure = use_signal(|| String::new());
    let mut source_type = use_signal(|| "Treatise".to_string());
    let mut notes = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box; overflow-y: auto;",
            h2 { style: "margin: 0; color: #89dceb; border-bottom: 1px solid #313244; padding-bottom: 8px;", "History of Science & Medicine QApp" }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Period" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| period.set(e.value()),
                    option { "Ancient" }
                    option { "Medieval Islamic" }
                    option { selected: true, "Scientific Revolution" }
                    option { "Enlightenment" }
                    option { "19th C." }
                    option { "Early 20th C." }
                    option { "Post-WWII" }
                    option { "Contemporary" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Discipline" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| discipline.set(e.value()),
                    option { "Astronomy" }
                    option { "Chemistry" }
                    option { "Biology" }
                    option { "Physics" }
                    option { selected: true, "Medicine" }
                    option { "Surgery" }
                    option { "Anatomy" }
                    option { "Psychiatry" }
                    option { "Public Health" }
                    option { "Eugenics Studies" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Historiographic Approach" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| historiographic_approach.set(e.value()),
                    option { "Internalist" }
                    option { "Externalist" }
                    option { selected: true, "Social Constructivist" }
                    option { "Feminist" }
                    option { "Postcolonial" }
                    option { "Material Culture" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Paradigm Shift" }
                input {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    placeholder: "e.g. germ theory, heliocentrism...",
                    oninput: move |e| paradigm_shift.set(e.value()),
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Key Figure" }
                input {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    placeholder: "e.g. Pasteur, Vesalius, Newton...",
                    oninput: move |e| key_figure.set(e.value()),
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Source Type" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| source_type.set(e.value()),
                    option { selected: true, "Treatise" }
                    option { "Correspondence" }
                    option { "Lab Notebook" }
                    option { "Patent" }
                    option { "Clinical Record" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Notes" }
                textarea {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box; min-height: 60px; resize: vertical;",
                    placeholder: "Additional notes...",
                    oninput: move |e| notes.set(e.value()),
                }
            }

            div {
                style: "background: #11111b; padding: 16px; border-radius: 8px; border-left: 4px solid #89dceb; flex: 1;",
                h3 { style: "margin-top: 0; color: #89dceb; font-size: 0.9rem;", "QualiaDB Output" }
                div { style: "font-size: 0.8rem; color: #a6adc8; display: flex; flex-direction: column; gap: 4px;",
                    div { "Period: {period()}" }
                    div { "Discipline: {discipline()}" }
                    div { "Approach: {historiographic_approach()}" }
                    div { "Figure: {key_figure()}" }
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 8px;", "QualiaDB → Allen Interval | knowledge graph | provenance sieve" }
            }
        }
    }
}
