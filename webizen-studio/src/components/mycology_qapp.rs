use dioxus::prelude::*;

#[component]
pub fn MycologyQapp() -> Element {
    let mut fungal_group = use_signal(|| "Basidiomycota".to_string());
    let mut ecological_role = use_signal(|| "Mycorrhizal".to_string());
    let mut substrate = use_signal(|| "Wood".to_string());
    let mut fruiting_body_type = use_signal(|| "Cap & Stipe".to_string());
    let mut toxicity = use_signal(|| "Edible".to_string());
    let mut spore_dispersal = use_signal(|| "Wind".to_string());
    let mut notes = use_signal(|| String::new());

    let fungal_groups = ["Basidiomycota", "Ascomycota", "Zygomycota", "Chytridiomycota", "Glomeromycota", "Lichen"];
    let ecological_roles = ["Decomposer", "Mycorrhizal", "Pathogenic", "Endophytic", "Saprophytic", "Parasitic"];
    let substrates = ["Wood", "Soil", "Dung", "Living Host", "Rock", "Dead Organic Matter"];
    let fruiting_bodies = ["Cap & Stipe", "Bracket", "Cup", "Truffle", "Coral", "Crust", "Puffball"];
    let toxicities = ["Edible", "Toxic", "Deadly", "Psychoactive", "Medicinal", "Unknown"];
    let dispersals = ["Wind", "Water", "Animal", "Ballistic"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #a6e3a1; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Mycology" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Fungal Group" }
                    select {
                        value: "{fungal_group}",
                        onchange: move |e| fungal_group.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in fungal_groups { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Ecological Role" }
                    select {
                        value: "{ecological_role}",
                        onchange: move |e| ecological_role.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in ecological_roles { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Substrate" }
                    select {
                        value: "{substrate}",
                        onchange: move |e| substrate.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in substrates { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Fruiting Body Type" }
                    select {
                        value: "{fruiting_body_type}",
                        onchange: move |e| fruiting_body_type.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in fruiting_bodies { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Toxicity" }
                    select {
                        value: "{toxicity}",
                        onchange: move |e| toxicity.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in toxicities { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Spore Dispersal" }
                    select {
                        value: "{spore_dispersal}",
                        onchange: move |e| spore_dispersal.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in dispersals { option { value: "{x}", "{x}" } }
                    }
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea {
                    value: "{notes}",
                    oninput: move |e| notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 80px;"
                }
            }

            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #a6e3a1;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{fungal_group} | {ecological_role} | {substrate} | {fruiting_body_type} | {toxicity}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → fungal taxonomy engine | ecological role sieve | spore dispersal anchor" }
            }
        }
    }
}
