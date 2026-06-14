use dioxus::prelude::*;

#[component]
pub fn AnimalStudiesHumanAnimalStudiesQapp() -> Element {
    let mut theoretical_lens = use_signal(|| "Posthumanism".to_string());
    let mut relationship_type = use_signal(|| "Companion".to_string());
    let mut species_group = use_signal(|| "Canine".to_string());
    let mut welfare_index = use_signal(|| 60u32);
    let mut agency_recognition = use_signal(|| "Moderate".to_string());
    let mut notes = use_signal(|| String::new());

    let lenses = ["Posthumanism", "Ecocriticism", "Affect Theory", "Ethology", "Critical Animal Studies", "Anthrozoology"];
    let relationships = ["Companion", "Working", "Food", "Wild", "Research", "Symbolic"];
    let species = ["Canine", "Feline", "Bovine", "Primate", "Avian", "Marine", "Insect", "Other"];
    let agencies = ["None", "Minimal", "Moderate", "High", "Full"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",

            h2 { style: "margin: 0; color: #a6e3a1; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Animal Studies / Human-Animal Studies" }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Lens" }
                select {
                    value: "{theoretical_lens}",
                    onchange: move |e| theoretical_lens.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in lenses { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Relationship Type" }
                select {
                    value: "{relationship_type}",
                    onchange: move |e| relationship_type.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in relationships { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Species Group" }
                select {
                    value: "{species_group}",
                    onchange: move |e| species_group.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in species { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Welfare Index: {welfare_index}%" }
                input {
                    r#type: "range", min: "0", max: "100",
                    value: "{welfare_index}",
                    oninput: move |e| welfare_index.set(e.value().parse().unwrap_or(60)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Agency Recognition" }
                select {
                    value: "{agency_recognition}",
                    onchange: move |e| agency_recognition.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in agencies { option { value: "{x}", "{x}" } }
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
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{theoretical_lens} | {relationship_type} | {species_group} | Welfare: {welfare_index}% | Agency: {agency_recognition}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → knowledge engine | sieve | anchor" }
            }
        }
    }
}
