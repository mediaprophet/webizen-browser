use dioxus::prelude::*;

#[component]
pub fn AfricanAmericanStudiesQapp() -> Element {
    let mut era = use_signal(|| "Civil Rights Era (1950s–1968)".to_string());
    let mut theoretical_lens = use_signal(|| "Critical Race Theory".to_string());
    let mut primary_source_type = use_signal(|| "Oral History / Narrative".to_string());
    let mut geographic_focus = use_signal(|| "U.S. South".to_string());
    let mut query = use_signal(|| String::new());
    let mut intersectionality_axes = use_signal(|| "Race + Class".to_string());

    let eras = [
        "Transatlantic Slave Trade (1526–1808)",
        "Antebellum & Slavery (1808–1865)",
        "Reconstruction (1865–1877)",
        "Jim Crow / Great Migration (1877–1940s)",
        "Civil Rights Era (1950s–1968)",
        "Black Power (1966–1975)",
        "Post–Civil Rights (1975–2000)",
        "Contemporary (2000–)",
    ];
    let lenses = [
        "Critical Race Theory",
        "Afrocentrism",
        "Black Feminism / Womanism",
        "Pan-Africanism",
        "Diaspora Studies",
        "Postcolonial Theory",
        "Black Marxism",
        "Intersectionality (Crenshaw)",
    ];
    let source_types = [
        "Oral History / Narrative",
        "Slave Narrative",
        "Legal Document",
        "Newspaper / Periodical",
        "Visual / Photographic",
        "Music / Cultural Artifact",
        "Legislative Record",
        "Literary Text",
    ];
    let geo_foci = [
        "U.S. South", "U.S. North / Great Migration", "Caribbean", "West Africa",
        "East Africa", "Brazil / Afro-Latin America", "UK / Black Britain", "Pan-African Diaspora",
    ];
    let intersections = [
        "Race + Class", "Race + Gender", "Race + Sexuality", "Race + Disability",
        "Race + Religion", "Race + Nationality", "Multidimensional",
    ];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #f38ba8; border-bottom: 1px solid #313244; padding-bottom: 8px;", "African American Studies" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Historical Era" }
                    select {
                        value: "{era}",
                        onchange: move |e| era.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in eras { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Lens" }
                    select {
                        value: "{theoretical_lens}",
                        onchange: move |e| theoretical_lens.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in lenses { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Primary Source Type" }
                    select {
                        value: "{primary_source_type}",
                        onchange: move |e| primary_source_type.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in source_types { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Geographic Focus" }
                    select {
                        value: "{geographic_focus}",
                        onchange: move |e| geographic_focus.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in geo_foci { option { value: "{x}", "{x}" } }
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Intersectionality Axes" }
                select {
                    value: "{intersectionality_axes}",
                    onchange: move |e| intersectionality_axes.set(e.value()),
                    style: "width: 40%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                    for x in intersections { option { value: "{x}", "{x}" } }
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Research Query / Source Excerpt" }
                textarea {
                    value: "{query}",
                    oninput: move |e| query.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 80px;"
                }
            }

            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #f38ba8; display: flex; gap: 16px; flex-wrap: wrap;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "Era: {era}" }
                span { style: "font-size: 0.8rem; color: #a6adc8;", "Lens: {theoretical_lens}" }
                span { style: "font-size: 0.8rem; color: #a6adc8;", "Intersect: {intersectionality_axes}" }
                div { style: "font-size: 0.75rem; color: #585b70; width: 100%;", "QualiaDB → graph theory engine | epistemic sieve | provenance graph" }
            }
        }
    }
}
