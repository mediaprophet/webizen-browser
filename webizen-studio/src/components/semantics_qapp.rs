use dioxus::prelude::*;

#[component]
pub fn SemanticsQapp() -> Element {
    let mut semantic_theory = use_signal(|| "Frame Semantics (Fillmore)".to_string());
    let mut semantic_relation = use_signal(|| "Polysemy".to_string());
    let mut truth_conditional = use_signal(|| "True".to_string());
    let mut word_or_phrase = use_signal(|| String::new());
    let mut semantic_frame = use_signal(|| String::new());
    let mut notes = use_signal(|| String::new());

    let theories = ["Formal Semantics (Montague)", "Prototype Theory (Rosch)", "Frame Semantics (Fillmore)", "Conceptual Metaphor (Lakoff)", "Cognitive Semantics", "Distributional Semantics", "Generative Lexicon"];
    let relations = ["Synonymy", "Antonymy", "Hyponymy", "Meronymy", "Polysemy", "Homonymy", "Metaphor", "Metonymy", "Entailment", "Presupposition"];
    let truth_conditionals = ["True", "False", "Indeterminate", "Paradox"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #cba6f7; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Semantics" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Semantic Theory" }
                    select {
                        value: "{semantic_theory}",
                        onchange: move |e| semantic_theory.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in theories { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Semantic Relation" }
                    select {
                        value: "{semantic_relation}",
                        onchange: move |e| semantic_relation.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in relations { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Truth Conditional" }
                    select {
                        value: "{truth_conditional}",
                        onchange: move |e| truth_conditional.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in truth_conditionals { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Semantic Frame" }
                    input {
                        r#type: "text",
                        value: "{semantic_frame}",
                        oninput: move |e| semantic_frame.set(e.value()),
                        placeholder: "e.g. Commercial Transaction, Motion, Cause",
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Word or Phrase Under Analysis" }
                textarea {
                    value: "{word_or_phrase}",
                    oninput: move |e| word_or_phrase.set(e.value()),
                    placeholder: "Enter the lexical item or phrase for semantic analysis...",
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 60px;"
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea {
                    value: "{notes}",
                    oninput: move |e| notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 60px;"
                }
            }

            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #cba6f7;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{semantic_theory} | {semantic_relation} | Truth:{truth_conditional} | Frame:{semantic_frame}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → semantic analysis engine | lexical relation sieve | truth-condition anchor" }
            }
        }
    }
}
