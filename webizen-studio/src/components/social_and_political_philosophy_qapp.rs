use dioxus::prelude::*;

#[component]
pub fn SocialAndPoliticalPhilosophyQapp() -> Element {
    let mut tradition = use_signal(|| "Liberal".to_string());
    let mut value_focus = use_signal(|| "Justice".to_string());
    let mut theory_of_justice = use_signal(|| "Rawlsian Original Position".to_string());
    let mut epistemic_certainty = use_signal(|| 60u32);
    let mut political_proposition = use_signal(|| String::new());
    let mut notes = use_signal(|| String::new());

    let traditions = ["Liberal", "Communitarian", "Republican", "Marxian", "Anarchist", "Libertarian", "Feminist", "Critical Theory", "Postcolonial", "Confucian Political Philosophy"];
    let value_focuses = ["Justice", "Liberty", "Equality", "Solidarity", "Recognition", "Democracy", "Rights", "Common Good", "Power"];
    let theories = ["Rawlsian Original Position", "Nozick Entitlement", "Sen Capabilities", "Communitarian", "Critical", "Luck Egalitarianism"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #f38ba8; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Social & Political Philosophy" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Tradition" }
                    select {
                        value: "{tradition}",
                        onchange: move |e| tradition.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in traditions { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Value Focus" }
                    select {
                        value: "{value_focus}",
                        onchange: move |e| value_focus.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in value_focuses { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Theory of Justice" }
                    select {
                        value: "{theory_of_justice}",
                        onchange: move |e| theory_of_justice.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in theories { option { value: "{x}", "{x}" } }
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Epistemic Certainty: {epistemic_certainty}%" }
                input {
                    r#type: "range",
                    min: "0",
                    max: "100",
                    value: "{epistemic_certainty}",
                    oninput: move |e| epistemic_certainty.set(e.value().parse().unwrap_or(60)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Political Proposition" }
                textarea {
                    value: "{political_proposition}",
                    oninput: move |e| political_proposition.set(e.value()),
                    placeholder: "Enter the political proposition under analysis...",
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #f38ba8;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{tradition} | {value_focus} | {theory_of_justice} | Certainty:{epistemic_certainty}%" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → political philosophy engine | justice theory sieve | normative anchor" }
            }
        }
    }
}
