use dioxus::prelude::*;

#[component]
pub fn MathematicsQapp() -> Element {
    let mut branch = use_signal(|| "Analysis".to_string());
    let mut proof_method = use_signal(|| "Direct".to_string());
    let mut conjecture_status = use_signal(|| "Open".to_string());
    let mut theorem_statement = use_signal(|| String::new());
    let mut variables_used = use_signal(|| String::new());
    let mut dimension = use_signal(|| 3u32);
    let mut notes = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #f9e2af; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Mathematics QApp" }

            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Branch" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| branch.set(e.value()),
                        option { "Number Theory" }
                        option { "Algebra" }
                        option { "Analysis" }
                        option { "Topology" }
                        option { "Geometry" }
                        option { "Combinatorics" }
                        option { "Probability" }
                        option { "Statistics" }
                        option { "Logic" }
                        option { "Applied Mathematics" }
                        option { "Category Theory" }
                        option { "Differential Equations" }
                        option { "Graph Theory" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Proof Method" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| proof_method.set(e.value()),
                        option { "Direct" }
                        option { "Contradiction" }
                        option { "Induction" }
                        option { "Strong Induction" }
                        option { "Contrapositive" }
                        option { "Existence" }
                        option { "Construction" }
                        option { "Probabilistic" }
                        option { "Exhaustion" }
                        option { "Diagonalisation" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Conjecture Status" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| conjecture_status.set(e.value()),
                        option { "Open" }
                        option { "Proved" }
                        option { "Disproved" }
                        option { "Conditional" }
                        option { "Independent" }
                        option { "Folklore" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Dimension: {dimension}" }
                    input {
                        r#type: "range",
                        min: "1",
                        max: "11",
                        step: "1",
                        value: "{dimension}",
                        style: "width: 100%; margin-top: 8px;",
                        oninput: move |e| dimension.set(e.value().parse().unwrap_or(3)),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Variables Used" }
                    input {
                        r#type: "text",
                        value: "{variables_used}",
                        placeholder: "e.g. x, y, z ∈ ℝ; n ∈ ℕ; G = (V, E)...",
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        oninput: move |e| variables_used.set(e.value()),
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Theorem / Conjecture Statement" }
                textarea {
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; min-height: 80px; box-sizing: border-box; resize: vertical;",
                    placeholder: "State the theorem, lemma, or conjecture formally...",
                    oninput: move |e| theorem_statement.set(e.value()),
                    "{theorem_statement}"
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea {
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; min-height: 50px; box-sizing: border-box; resize: vertical;",
                    placeholder: "Proof sketch, references, open questions, related results...",
                    oninput: move |e| notes.set(e.value()),
                    "{notes}"
                }
            }

            div {
                style: "background: #11111b; padding: 16px; border-radius: 8px; border-left: 4px solid #f9e2af; flex: 1;",
                h3 { style: "margin-top: 0; color: #f9e2af; font-size: 0.9rem;", "Output / Analysis" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px; font-size: 0.8rem;",
                    div { style: "color: #a6adc8;", "Branch:" }
                    div { style: "color: #cdd6f4;", "{branch}" }
                    div { style: "color: #a6adc8;", "Proof Method:" }
                    div { style: "color: #cdd6f4;", "{proof_method}" }
                    div { style: "color: #a6adc8;", "Status:" }
                    div { style: "color: #cdd6f4;", "{conjecture_status}" }
                    div { style: "color: #a6adc8;", "Dimension:" }
                    div { style: "color: #cdd6f4;", "{dimension}" }
                    div { style: "color: #a6adc8;", "Variables:" }
                    div { style: "color: #cdd6f4;", "{variables_used}" }
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 12px; border-top: 1px solid #313244; padding-top: 8px;",
                    "QualiaDB → formal proof engine | graph theory | numerical ODE solver"
                }
            }
        }
    }
}
