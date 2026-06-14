use dioxus::prelude::*;

#[component]
pub fn LogicQapp() -> Element {
    let mut logic_type = use_signal(|| "First-Order Predicate".to_string());
    let mut proof_system = use_signal(|| "Natural Deduction".to_string());
    let mut formula = use_signal(|| String::new());
    let mut inference_rule = use_signal(|| "Modus Ponens".to_string());
    let mut validity_status = use_signal(|| "Unknown".to_string());
    let mut notes = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #cba6f7; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Logic QApp" }

            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Logic Type" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| logic_type.set(e.value()),
                        option { "Classical Propositional" }
                        option { "First-Order Predicate" }
                        option { "Modal (S4/S5)" }
                        option { "Temporal (LTL/CTL)" }
                        option { "Deontic" }
                        option { "Epistemic" }
                        option { "Intuitionistic" }
                        option { "Paraconsistent" }
                        option { "Fuzzy" }
                        option { "Default / Non-monotonic" }
                        option { "Description Logic" }
                        option { "Higher-Order Logic" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Proof System" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| proof_system.set(e.value()),
                        option { "Natural Deduction" }
                        option { "Sequent Calculus" }
                        option { "Hilbert System" }
                        option { "Resolution" }
                        option { "Tableau" }
                        option { "Type Theory" }
                        option { "DPLL / CDCL" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Inference Rule" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| inference_rule.set(e.value()),
                        option { "Modus Ponens" }
                        option { "Modus Tollens" }
                        option { "Universal Instantiation" }
                        option { "Existential Generalisation" }
                        option { "De Morgan" }
                        option { "Cut" }
                        option { "Conjunction Introduction" }
                        option { "Disjunction Elimination" }
                        option { "Reductio ad Absurdum" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Validity Status" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| validity_status.set(e.value()),
                        option { "Valid" }
                        option { "Invalid" }
                        option { "Unknown" }
                        option { "Undecidable" }
                        option { "Satisfiable" }
                        option { "Unsatisfiable" }
                        option { "Tautology" }
                        option { "Contingent" }
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Formula / Sequent" }
                input {
                    r#type: "text",
                    value: "{formula}",
                    placeholder: "e.g. ∀x(P(x) → Q(x)), □(A → B) → (□A → □B)...",
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box; font-size: 0.9rem;",
                    oninput: move |e| formula.set(e.value()),
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea {
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; min-height: 60px; box-sizing: border-box; resize: vertical;",
                    placeholder: "Proof steps, semantic tableaux, countermodels, decidability arguments...",
                    oninput: move |e| notes.set(e.value()),
                    "{notes}"
                }
            }

            div {
                style: "background: #11111b; padding: 16px; border-radius: 8px; border-left: 4px solid #cba6f7; flex: 1;",
                h3 { style: "margin-top: 0; color: #cba6f7; font-size: 0.9rem;", "Output / Analysis" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px; font-size: 0.8rem;",
                    div { style: "color: #a6adc8;", "Logic Type:" }
                    div { style: "color: #cdd6f4;", "{logic_type}" }
                    div { style: "color: #a6adc8;", "Proof System:" }
                    div { style: "color: #cdd6f4;", "{proof_system}" }
                    div { style: "color: #a6adc8;", "Inference Rule:" }
                    div { style: "color: #cdd6f4;", "{inference_rule}" }
                    div { style: "color: #a6adc8;", "Status:" }
                    div { style: "color: #cdd6f4;", "{validity_status}" }
                }
                if !formula().is_empty() {
                    div {
                        style: "margin-top: 8px; padding: 8px; background: #181825; border-radius: 4px; border: 1px solid #313244;",
                        div { style: "font-size: 0.75rem; color: #a6adc8;", "Formula:" }
                        div { style: "font-size: 0.85rem; color: #f9e2af; margin-top: 4px; word-break: break-all;", "{formula}" }
                    }
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 12px; border-top: 1px solid #313244; padding-top: 8px;",
                    "QualiaDB → epistemic logic engine | deontic logic | resolution theorem prover"
                }
            }
        }
    }
}
