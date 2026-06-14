use dioxus::prelude::*;

#[component]
pub fn BotanyQapp() -> Element {
    let mut plant_division = use_signal(|| "Angiosperms".to_string());
    let mut growth_form = use_signal(|| "Tree".to_string());
    let mut habitat = use_signal(|| "Tropical Rainforest".to_string());
    let mut phenological_stage = use_signal(|| "Vegetative".to_string());
    let mut pollination_mechanism = use_signal(|| "Entomophily (insect)".to_string());
    let mut specimen_notes = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #a6e3a1; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Botany QApp" }

            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Plant Division" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| plant_division.set(e.value()),
                        option { "Angiosperms" }
                        option { "Gymnosperms" }
                        option { "Ferns (Pteridophyta)" }
                        option { "Mosses (Bryophyta)" }
                        option { "Algae (Chlorophyta)" }
                        option { "Fungi (Basidiomycota)" }
                        option { "Fungi (Ascomycota)" }
                        option { "Lichens" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Growth Form" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| growth_form.set(e.value()),
                        option { "Tree" }
                        option { "Shrub" }
                        option { "Herb" }
                        option { "Vine" }
                        option { "Epiphyte" }
                        option { "Aquatic" }
                        option { "Succulent" }
                        option { "Geophyte" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Habitat" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| habitat.set(e.value()),
                        option { "Tropical Rainforest" }
                        option { "Temperate Forest" }
                        option { "Desert" }
                        option { "Grassland" }
                        option { "Wetland" }
                        option { "Arctic Tundra" }
                        option { "Alpine" }
                        option { "Mediterranean Shrubland" }
                        option { "Mangrove" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Phenological Stage" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| phenological_stage.set(e.value()),
                        option { "Germination" }
                        option { "Vegetative" }
                        option { "Flowering" }
                        option { "Fruiting" }
                        option { "Senescence" }
                        option { "Dormancy" }
                        option { "Leaf Flush" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Pollination Mechanism" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| pollination_mechanism.set(e.value()),
                        option { "Entomophily (insect)" }
                        option { "Anemophily (wind)" }
                        option { "Ornithophily (bird)" }
                        option { "Chiropterophily (bat)" }
                        option { "Hydrophily (water)" }
                        option { "Self-pollination" }
                        option { "Apomixis" }
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Specimen Notes" }
                textarea {
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; min-height: 70px; box-sizing: border-box; resize: vertical;",
                    placeholder: "Taxonomic identification, GPS coordinates, voucher number, morphological description...",
                    oninput: move |e| specimen_notes.set(e.value()),
                    "{specimen_notes}"
                }
            }

            div {
                style: "background: #11111b; padding: 16px; border-radius: 8px; border-left: 4px solid #a6e3a1; flex: 1;",
                h3 { style: "margin-top: 0; color: #a6e3a1; font-size: 0.9rem;", "Output / Analysis" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px; font-size: 0.8rem;",
                    div { style: "color: #a6adc8;", "Division:" }
                    div { style: "color: #cdd6f4;", "{plant_division}" }
                    div { style: "color: #a6adc8;", "Growth Form:" }
                    div { style: "color: #cdd6f4;", "{growth_form}" }
                    div { style: "color: #a6adc8;", "Habitat:" }
                    div { style: "color: #cdd6f4;", "{habitat}" }
                    div { style: "color: #a6adc8;", "Phenophase:" }
                    div { style: "color: #cdd6f4;", "{phenological_stage}" }
                    div { style: "color: #a6adc8;", "Pollination:" }
                    div { style: "color: #cdd6f4;", "{pollination_mechanism}" }
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 12px; border-top: 1px solid #313244; padding-top: 8px;",
                    "QualiaDB → Allen Interval phenology engine | taxonomy graph | ecological sieve"
                }
            }
        }
    }
}
