use dioxus::prelude::*;

#[component]
pub fn LibraryAndInformationScienceQapp() -> Element {
    let mut subfield = use_signal(|| "Metadata".to_string());
    let mut metadata_schema = use_signal(|| "Dublin Core".to_string());
    let mut classification_scheme = use_signal(|| "Dewey".to_string());
    let mut information_type = use_signal(|| "Text".to_string());
    let mut access_model = use_signal(|| "Open Access".to_string());
    let mut notes = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box; overflow-y: auto;",
            h2 { style: "margin: 0; color: #89dceb; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Library & Information Science QApp" }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Subfield" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| subfield.set(e.value()),
                    option { "Cataloguing" }
                    option { selected: true, "Metadata" }
                    option { "Archival Science" }
                    option { "Digital Preservation" }
                    option { "Knowledge Management" }
                    option { "Information Retrieval" }
                    option { "User Experience" }
                    option { "Data Curation" }
                    option { "Community Librarianship" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Metadata Schema" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| metadata_schema.set(e.value()),
                    option { selected: true, "Dublin Core" }
                    option { "MARC21" }
                    option { "MODS" }
                    option { "RDF" }
                    option { "Schema.org" }
                    option { "EAD" }
                    option { "TEI" }
                    option { "PREMIS" }
                    option { "JSON-LD" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Classification Scheme" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| classification_scheme.set(e.value()),
                    option { selected: true, "Dewey" }
                    option { "LCC" }
                    option { "UDC" }
                    option { "BISAC" }
                    option { "Bliss" }
                    option { "Colon Classification" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Information Type" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| information_type.set(e.value()),
                    option { selected: true, "Text" }
                    option { "Image" }
                    option { "Audio" }
                    option { "Video" }
                    option { "Dataset" }
                    option { "3D Object" }
                    option { "Born-Digital" }
                    option { "Physical" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Access Model" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| access_model.set(e.value()),
                    option { selected: true, "Open Access" }
                    option { "Paywalled" }
                    option { "Restricted" }
                    option { "Dark Archive" }
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
                    div { "Subfield: {subfield()}" }
                    div { "Schema: {metadata_schema()}" }
                    div { "Classification: {classification_scheme()}" }
                    div { "Access: {access_model()}" }
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 8px;", "QualiaDB → knowledge graph | RDF engine | SPARQL explorer" }
            }
        }
    }
}
