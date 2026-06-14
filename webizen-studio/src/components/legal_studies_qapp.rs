use dioxus::prelude::*;

#[component]
pub fn LegalStudiesQapp() -> Element {
    let mut legal_tradition = use_signal(|| "Common Law".to_string());
    let mut area_of_law = use_signal(|| "Constitutional".to_string());
    let mut legal_theory = use_signal(|| "Legal Positivism".to_string());
    let mut jurisdiction = use_signal(|| String::new());
    let mut case_or_statute = use_signal(|| String::new());
    let mut certainty_of_outcome = use_signal(|| 50u32);
    let mut notes = use_signal(|| String::new());

    let traditions = [
        "Common Law", "Civil Law", "Mixed", "Religious Law", "Customary Law", "Socialist Legal",
    ];
    let areas = [
        "Constitutional", "Criminal", "Contract", "Tort", "Property",
        "International", "Administrative", "Human Rights", "IP", "Family", "Environmental",
    ];
    let theories = [
        "Natural Law", "Legal Positivism", "Legal Realism", "Critical Legal Studies",
        "Feminist Jurisprudence", "Law & Economics", "Postcolonial", "Indigenous Legal Orders",
    ];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #eba0ac; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Legal Studies" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Legal Tradition" }
                    select {
                        value: "{legal_tradition}",
                        onchange: move |e| legal_tradition.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in traditions { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Area of Law" }
                    select {
                        value: "{area_of_law}",
                        onchange: move |e| area_of_law.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in areas { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Legal Theory" }
                    select {
                        value: "{legal_theory}",
                        onchange: move |e| legal_theory.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in theories { option { value: "{x}", "{x}" } }
                    }
                }
            }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Jurisdiction" }
                    input {
                        r#type: "text",
                        value: "{jurisdiction}",
                        oninput: move |e| jurisdiction.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Case / Statute" }
                    input {
                        r#type: "text",
                        value: "{case_or_statute}",
                        oninput: move |e| case_or_statute.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Certainty of Outcome (0–100): {certainty_of_outcome}%" }
                input {
                    r#type: "range",
                    min: "0",
                    max: "100",
                    value: "{certainty_of_outcome}",
                    oninput: move |e| certainty_of_outcome.set(e.value().parse().unwrap_or(50)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea {
                    value: "{notes}",
                    oninput: move |e| notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 70px;"
                }
            }

            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #eba0ac;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{legal_tradition} | {area_of_law} | {legal_theory} | Certainty: {certainty_of_outcome}%" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → jurisprudence engine | precedent sieve | legal theory graph" }
            }
        }
    }
}
