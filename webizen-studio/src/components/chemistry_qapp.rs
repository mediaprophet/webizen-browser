use dioxus::prelude::*;

#[component]
pub fn ChemistryQapp() -> Element {
    let mut subdiscipline = use_signal(|| "Organic".to_string());
    let mut reaction_type = use_signal(|| "Substitution SN2".to_string());
    let mut temperature_k = use_signal(|| 300.0f64);
    let mut pressure_atm = use_signal(|| 1.0f64);
    let mut concentration_m = use_signal(|| 1.0f64);
    let mut ph = use_signal(|| 7.0f64);
    let mut solvent = use_signal(|| "Water".to_string());
    let mut reagent_notes = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #f9e2af; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Chemistry QApp" }

            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Subdiscipline" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| subdiscipline.set(e.value()),
                        option { "Organic" }
                        option { "Inorganic" }
                        option { "Physical" }
                        option { "Analytical" }
                        option { "Biochemistry" }
                        option { "Materials" }
                        option { "Computational" }
                        option { "Green Chemistry" }
                        option { "Polymer" }
                        option { "Nuclear" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Reaction Type" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| reaction_type.set(e.value()),
                        option { "Substitution SN1" }
                        option { "Substitution SN2" }
                        option { "Addition" }
                        option { "Elimination" }
                        option { "Oxidation-Reduction" }
                        option { "Acid-Base" }
                        option { "Coordination" }
                        option { "Pericyclic" }
                        option { "Radical" }
                        option { "Coupling" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Temperature (K): {temperature_k:.0}" }
                    input {
                        r#type: "range",
                        min: "0",
                        max: "2000",
                        step: "1",
                        value: "{temperature_k}",
                        style: "width: 100%; margin-top: 8px;",
                        oninput: move |e| temperature_k.set(e.value().parse().unwrap_or(300.0)),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Pressure (atm): {pressure_atm:.2}" }
                    input {
                        r#type: "range",
                        min: "0.01",
                        max: "100",
                        step: "0.01",
                        value: "{pressure_atm}",
                        style: "width: 100%; margin-top: 8px;",
                        oninput: move |e| pressure_atm.set(e.value().parse().unwrap_or(1.0)),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Concentration (M)" }
                    input {
                        r#type: "number",
                        value: "{concentration_m}",
                        step: "0.001",
                        min: "0",
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        oninput: move |e| concentration_m.set(e.value().parse().unwrap_or(1.0)),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "pH: {ph:.2}" }
                    input {
                        r#type: "range",
                        min: "0",
                        max: "14",
                        step: "0.01",
                        value: "{ph}",
                        style: "width: 100%; margin-top: 8px;",
                        oninput: move |e| ph.set(e.value().parse().unwrap_or(7.0)),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Solvent" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| solvent.set(e.value()),
                        option { "Water" }
                        option { "Ethanol" }
                        option { "DMSO" }
                        option { "DCM" }
                        option { "THF" }
                        option { "Acetonitrile" }
                        option { "Hexane" }
                        option { "DMF" }
                        option { "Acetone" }
                        option { "Toluene" }
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Reagent Notes" }
                textarea {
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; min-height: 60px; box-sizing: border-box; resize: vertical;",
                    placeholder: "Enter reagents, catalysts, expected products, yield targets...",
                    oninput: move |e| reagent_notes.set(e.value()),
                    "{reagent_notes}"
                }
            }

            div {
                style: "background: #11111b; padding: 16px; border-radius: 8px; border-left: 4px solid #f9e2af; flex: 1;",
                h3 { style: "margin-top: 0; color: #f9e2af; font-size: 0.9rem;", "Output / Analysis" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px; font-size: 0.8rem;",
                    div { style: "color: #a6adc8;", "Subdiscipline:" }
                    div { style: "color: #cdd6f4;", "{subdiscipline}" }
                    div { style: "color: #a6adc8;", "Reaction:" }
                    div { style: "color: #cdd6f4;", "{reaction_type}" }
                    div { style: "color: #a6adc8;", "Temperature:" }
                    div { style: "color: #cdd6f4;", "{temperature_k:.0} K" }
                    div { style: "color: #a6adc8;", "Pressure:" }
                    div { style: "color: #cdd6f4;", "{pressure_atm:.2} atm" }
                    div { style: "color: #a6adc8;", "pH:" }
                    div { style: "color: #cdd6f4;", "{ph:.2}" }
                    div { style: "color: #a6adc8;", "Solvent:" }
                    div { style: "color: #cdd6f4;", "{solvent}" }
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 12px; border-top: 1px solid #313244; padding-top: 8px;",
                    "QualiaDB → chemistry_modeler engine | ODE thermodynamic solver | reaction pathway sieve"
                }
            }
        }
    }
}
