use dioxus::prelude::*;

#[component]
pub fn CuratorialStudiesQapp() -> Element {
    let mut exhibition_type = use_signal(|| "Thematic".to_string());
    let mut curatorial_approach = use_signal(|| "Research-Led".to_string());
    let mut medium = use_signal(|| "Visual Art".to_string());
    let mut object_count = use_signal(|| 40u32);
    let mut venue_type = use_signal(|| "Institution".to_string());
    let mut critical_framework = use_signal(|| "Postmodern".to_string());
    let mut catalog_notes = use_signal(|| String::new());

    let exhibition_types = [
        "Solo Artist", "Group", "Thematic", "Survey", "Retrospective",
        "Site-Specific", "Virtual", "Community-Curated",
    ];
    let approaches = [
        "White Cube", "Relational Aesthetics", "Institutional Critique",
        "Decolonial", "Participatory", "Research-Led", "Archival",
    ];
    let media = [
        "Visual Art", "Performance", "Digital", "Sound",
        "New Media", "Mixed", "Interdisciplinary",
    ];
    let venues = ["Institution", "Gallery", "Public Space", "Online", "Pop-Up", "Museum"];
    let frameworks = [
        "Postmodern", "Feminist", "Postcolonial", "Queer", "Disability", "Ecocritical",
    ];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #cba6f7; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Curatorial Studies" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Exhibition Type" }
                    select {
                        value: "{exhibition_type}",
                        onchange: move |e| exhibition_type.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in exhibition_types { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Curatorial Approach" }
                    select {
                        value: "{curatorial_approach}",
                        onchange: move |e| curatorial_approach.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in approaches { option { value: "{x}", "{x}" } }
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
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Venue Type" }
                    select {
                        value: "{venue_type}",
                        onchange: move |e| venue_type.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in venues { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Critical Framework" }
                    select {
                        value: "{critical_framework}",
                        onchange: move |e| critical_framework.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in frameworks { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Object Count" }
                    input {
                        r#type: "number",
                        value: "{object_count}",
                        oninput: move |e| object_count.set(e.value().parse().unwrap_or(40)),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Catalog Notes" }
                textarea {
                    value: "{catalog_notes}",
                    oninput: move |e| catalog_notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 80px;"
                }
            }

            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #cba6f7;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{exhibition_type} | {curatorial_approach} | {venue_type} | Objects: {object_count}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → exhibition graph | critical theory sieve | collection management engine" }
            }
        }
    }
}
