use dioxus::prelude::*;

#[component]
pub fn EvolutionaryBiologyQapp() -> Element {
    let mut mechanism = use_signal(|| "Natural Selection".to_string());
    let mut speciation_mode = use_signal(|| "Allopatric".to_string());
    let mut phylogenetic_method = use_signal(|| "Maximum Likelihood".to_string());
    let mut time_mya = use_signal(|| 10.0f64);
    let mut selection_coefficient_s = use_signal(|| 0.01f64);
    let mut population_size_ne = use_signal(|| 10000u32);
    let mut clade_notes = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #eba0ac; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Evolutionary Biology QApp" }

            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Evolutionary Mechanism" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| mechanism.set(e.value()),
                        option { "Natural Selection" }
                        option { "Genetic Drift" }
                        option { "Gene Flow" }
                        option { "Mutation" }
                        option { "Sexual Selection" }
                        option { "Kin Selection" }
                        option { "Group Selection" }
                        option { "Neutral Evolution" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Speciation Mode" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| speciation_mode.set(e.value()),
                        option { "Allopatric" }
                        option { "Sympatric" }
                        option { "Parapatric" }
                        option { "Peripatric" }
                        option { "Anagenesis" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Phylogenetic Method" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| phylogenetic_method.set(e.value()),
                        option { "Maximum Likelihood" }
                        option { "Bayesian Inference" }
                        option { "Parsimony" }
                        option { "Distance-Based (NJ)" }
                        option { "UPGMA" }
                        option { "Supertree" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Divergence Time (Mya): {time_mya:.1}" }
                    input {
                        r#type: "range",
                        min: "0.001",
                        max: "3500",
                        step: "0.1",
                        value: "{time_mya}",
                        style: "width: 100%; margin-top: 8px;",
                        oninput: move |e| time_mya.set(e.value().parse().unwrap_or(10.0)),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Selection Coefficient s: {selection_coefficient_s:.4}" }
                    input {
                        r#type: "range",
                        min: "0.0",
                        max: "1.0",
                        step: "0.0001",
                        value: "{selection_coefficient_s}",
                        style: "width: 100%; margin-top: 8px;",
                        oninput: move |e| selection_coefficient_s.set(e.value().parse().unwrap_or(0.01)),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Effective Population Size (Ne)" }
                    input {
                        r#type: "number",
                        value: "{population_size_ne}",
                        min: "1",
                        step: "100",
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        oninput: move |e| population_size_ne.set(e.value().parse().unwrap_or(10000)),
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Clade Notes" }
                textarea {
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; min-height: 60px; box-sizing: border-box; resize: vertical;",
                    placeholder: "Synapomorphies, outgroup, fossil calibration points, genomic markers...",
                    oninput: move |e| clade_notes.set(e.value()),
                    "{clade_notes}"
                }
            }

            div {
                style: "background: #11111b; padding: 16px; border-radius: 8px; border-left: 4px solid #eba0ac; flex: 1;",
                h3 { style: "margin-top: 0; color: #eba0ac; font-size: 0.9rem;", "Output / Analysis" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px; font-size: 0.8rem;",
                    div { style: "color: #a6adc8;", "Mechanism:" }
                    div { style: "color: #cdd6f4;", "{mechanism}" }
                    div { style: "color: #a6adc8;", "Speciation:" }
                    div { style: "color: #cdd6f4;", "{speciation_mode}" }
                    div { style: "color: #a6adc8;", "Method:" }
                    div { style: "color: #cdd6f4;", "{phylogenetic_method}" }
                    div { style: "color: #a6adc8;", "Divergence:" }
                    div { style: "color: #cdd6f4;", "{time_mya:.1} Mya" }
                    div { style: "color: #a6adc8;", "s (selection):" }
                    div { style: "color: #cdd6f4;", "{selection_coefficient_s:.4}" }
                    div { style: "color: #a6adc8;", "Ne:" }
                    div { style: "color: #cdd6f4;", "{population_size_ne}" }
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 12px; border-top: 1px solid #313244; padding-top: 8px;",
                    "QualiaDB → graph phylogenetic tree | Bayesian epistemic engine | ODE population genetics"
                }
            }
        }
    }
}
