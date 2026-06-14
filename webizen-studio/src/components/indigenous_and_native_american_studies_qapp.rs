use dioxus::prelude::*;

#[component]
pub fn IndigenousAndNativeAmericanStudiesQapp() -> Element {
    let mut geographic_region = use_signal(|| "Northeast Woodlands".to_string());
    let mut disciplinary_lens = use_signal(|| "Sovereignty & Legal".to_string());
    let mut community_partnership_mode = use_signal(|| "Collaborative".to_string());
    let mut temporal_focus = use_signal(|| String::new());
    let mut nation_or_people = use_signal(|| String::new());
    let mut notes = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box; overflow-y: auto;",
            h2 { style: "margin: 0; color: #eba0ac; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Indigenous & Native American Studies QApp" }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Geographic Region" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| geographic_region.set(e.value()),
                    option { selected: true, "Northeast Woodlands" }
                    option { "Southeast" }
                    option { "Great Plains" }
                    option { "Southwest" }
                    option { "Pacific Northwest" }
                    option { "Arctic" }
                    option { "Great Basin" }
                    option { "Mesoamerica" }
                    option { "Andean" }
                    option { "Amazonian" }
                    option { "Pacific Islands" }
                    option { "Australia" }
                    option { "Aotearoa NZ" }
                    option { "Other" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Disciplinary Lens" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| disciplinary_lens.set(e.value()),
                    option { selected: true, "Sovereignty & Legal" }
                    option { "Language Revitalisation" }
                    option { "Environmental Knowledge" }
                    option { "Cultural Continuity" }
                    option { "Oral History" }
                    option { "Postcolonial" }
                    option { "Decolonial" }
                    option { "Material Culture" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Community Partnership Mode" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| community_partnership_mode.set(e.value()),
                    option { "Community-Led" }
                    option { selected: true, "Collaborative" }
                    option { "Academic-Led" }
                    option { "Archival Only" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Temporal Focus" }
                input {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    placeholder: "e.g. Pre-contact, Reservation Era, Contemporary...",
                    oninput: move |e| temporal_focus.set(e.value()),
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Nation or People" }
                input {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    placeholder: "e.g. Haudenosaunee, Lakota, Māori...",
                    oninput: move |e| nation_or_people.set(e.value()),
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Notes" }
                textarea {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box; min-height: 70px; resize: vertical;",
                    placeholder: "Research notes...",
                    oninput: move |e| notes.set(e.value()),
                }
            }

            div {
                style: "background: #11111b; padding: 16px; border-radius: 8px; border-left: 4px solid #eba0ac; flex: 1;",
                h3 { style: "margin-top: 0; color: #eba0ac; font-size: 0.9rem;", "QualiaDB Output" }
                div { style: "font-size: 0.8rem; color: #a6adc8; display: flex; flex-direction: column; gap: 4px;",
                    div { "Region: {geographic_region()}" }
                    div { "Lens: {disciplinary_lens()}" }
                    div { "Partnership: {community_partnership_mode()}" }
                    div { "People: {nation_or_people()}" }
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 8px;", "QualiaDB → knowledge graph | provenance | Allen Interval" }
            }
        }
    }
}
