use dioxus::prelude::*;

#[component]
pub fn BiologyQapp() -> Element {
    let mut subdiscipline = use_signal(|| "Molecular Biology".to_string());
    let mut organism_domain = use_signal(|| "Eukarya".to_string());
    let mut method = use_signal(|| "Sequencing".to_string());
    let mut gene_or_protein = use_signal(|| String::new());
    let mut pathway = use_signal(|| String::new());
    let mut sample_type = use_signal(|| String::new());
    let mut notes = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #a6e3a1; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Biology QApp" }

            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Subdiscipline" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| subdiscipline.set(e.value()),
                        option { "Cell Biology" }
                        option { "Genetics" }
                        option { "Molecular Biology" }
                        option { "Physiology" }
                        option { "Developmental Biology" }
                        option { "Microbiology" }
                        option { "Immunology" }
                        option { "Ecology" }
                        option { "Evolutionary Biology" }
                        option { "Structural Biology" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Organism Domain" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| organism_domain.set(e.value()),
                        option { "Bacteria" }
                        option { "Archaea" }
                        option { "Eukarya" }
                        option { "Virus" }
                        option { "Viroid" }
                        option { "Prion" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Experimental Method" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| method.set(e.value()),
                        option { "PCR" }
                        option { "Western Blot" }
                        option { "CRISPR" }
                        option { "Microscopy" }
                        option { "Flow Cytometry" }
                        option { "Sequencing" }
                        option { "Electrophysiology" }
                        option { "Mass Spectrometry" }
                        option { "ChIP-seq" }
                        option { "RNA-seq" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Gene or Protein" }
                    input {
                        r#type: "text",
                        value: "{gene_or_protein}",
                        placeholder: "e.g. TP53, BRCA1, Actin...",
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        oninput: move |e| gene_or_protein.set(e.value()),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Biological Pathway" }
                    input {
                        r#type: "text",
                        value: "{pathway}",
                        placeholder: "e.g. MAPK, mTOR, Apoptosis...",
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        oninput: move |e| pathway.set(e.value()),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Sample Type" }
                    input {
                        r#type: "text",
                        value: "{sample_type}",
                        placeholder: "e.g. HEK293, primary neurons, blood...",
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        oninput: move |e| sample_type.set(e.value()),
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Research Notes" }
                textarea {
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; min-height: 60px; box-sizing: border-box; resize: vertical;",
                    placeholder: "Enter hypotheses, protocols, expected outcomes...",
                    oninput: move |e| notes.set(e.value()),
                    "{notes}"
                }
            }

            div {
                style: "background: #11111b; padding: 16px; border-radius: 8px; border-left: 4px solid #a6e3a1; flex: 1;",
                h3 { style: "margin-top: 0; color: #a6e3a1; font-size: 0.9rem;", "Output / Analysis" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px; font-size: 0.8rem;",
                    div { style: "color: #a6adc8;", "Subdiscipline:" }
                    div { style: "color: #cdd6f4;", "{subdiscipline}" }
                    div { style: "color: #a6adc8;", "Domain:" }
                    div { style: "color: #cdd6f4;", "{organism_domain}" }
                    div { style: "color: #a6adc8;", "Method:" }
                    div { style: "color: #cdd6f4;", "{method}" }
                    div { style: "color: #a6adc8;", "Gene/Protein:" }
                    div { style: "color: #cdd6f4;", "{gene_or_protein}" }
                    div { style: "color: #a6adc8;", "Pathway:" }
                    div { style: "color: #cdd6f4;", "{pathway}" }
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 12px; border-top: 1px solid #313244; padding-top: 8px;",
                    "QualiaDB → bioinformatics_lab engine | graph theory pathway analysis | neuro-symbolic sieve"
                }
            }
        }
    }
}
