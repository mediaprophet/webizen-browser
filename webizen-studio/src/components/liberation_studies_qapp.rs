use dioxus::prelude::*;

#[component]
pub fn LiberationStudiesQapp() -> Element {
    let mut liberation_tradition = use_signal(|| "Freire's Pedagogy".to_string());
    let mut theorist = use_signal(|| "Freire".to_string());
    let mut oppression_structure = use_signal(|| "Class".to_string());
    let mut praxis = use_signal(|| "Conscientisation".to_string());
    let mut liberation_index = use_signal(|| 50u32);
    let mut notes = use_signal(|| String::new());

    let liberation_traditions = ["Liberation Theology", "Freire's Pedagogy", "Black Liberation", "Feminist Liberation", "Third World Liberation", "Disability Liberation", "Animal Liberation"];
    let theorists = ["Freire", "Gutiérrez", "Fanon", "Cone", "Davis", "hooks", "Singer"];
    let oppression_structures = ["Class", "Race", "Gender", "Colonialism", "Ableism", "Species"];
    let praxes = ["Conscientisation", "Coalition", "Sabotage", "Mutual Aid", "Electoral"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #f9e2af; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Liberation Studies" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Liberation Tradition" }
                select {
                    value: "{liberation_tradition}", onchange: move |e| liberation_tradition.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in liberation_traditions { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Theorist" }
                select {
                    value: "{theorist}", onchange: move |e| theorist.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in theorists { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Oppression Structure" }
                select {
                    value: "{oppression_structure}", onchange: move |e| oppression_structure.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in oppression_structures { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Praxis" }
                select {
                    value: "{praxis}", onchange: move |e| praxis.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in praxes { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Liberation Index: {liberation_index}" }
                input { r#type: "range", min: "0", max: "100", value: "{liberation_index}",
                    oninput: move |e| liberation_index.set(e.value().parse().unwrap_or(50)),
                    style: "width: 100%; margin-top: 4px;" }
            }
            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea { value: "{notes}", oninput: move |e| notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 60px;" }
            }
            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #f9e2af;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{liberation_tradition} | {theorist} | {oppression_structure} | {praxis} | index: {liberation_index}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → liberation studies engine | discourse sieve | anchor" }
            }
        }
    }
}
