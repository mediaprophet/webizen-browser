use dioxus::prelude::*;

#[component]
pub fn PublicPolicyQapp() -> Element {
    let mut policy_stage = use_signal(|| "Policy Formulation".to_string());
    let mut policy_instrument = use_signal(|| "Regulation".to_string());
    let mut theoretical_lens = use_signal(|| "Bounded Rationality".to_string());
    let mut issue_area = use_signal(|| "Health".to_string());
    let mut stakeholder_map = use_signal(|| String::new());
    let mut implementation_gap = use_signal(|| 30u32);
    let mut notes = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box; overflow-y: auto;",
            h2 { style: "margin: 0; color: #89b4fa; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Public Policy QApp" }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Policy Stage" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| policy_stage.set(e.value()),
                    option { "Agenda Setting" }
                    option { selected: true, "Policy Formulation" }
                    option { "Decision Making" }
                    option { "Implementation" }
                    option { "Evaluation" }
                    option { "Termination" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Policy Instrument" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| policy_instrument.set(e.value()),
                    option { selected: true, "Regulation" }
                    option { "Tax" }
                    option { "Subsidy" }
                    option { "Public Provision" }
                    option { "Information" }
                    option { "Co-regulation" }
                    option { "Market Mechanism" }
                    option { "Voluntary Agreement" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Theoretical Lens" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| theoretical_lens.set(e.value()),
                    option { "Rational Choice" }
                    option { selected: true, "Bounded Rationality" }
                    option { "Incrementalism" }
                    option { "Advocacy Coalition" }
                    option { "Multiple Streams" }
                    option { "Punctuated Equilibrium" }
                    option { "Deliberative" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Issue Area" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| issue_area.set(e.value()),
                    option { "Economy" }
                    option { selected: true, "Health" }
                    option { "Education" }
                    option { "Environment" }
                    option { "Housing" }
                    option { "Security" }
                    option { "Social Welfare" }
                    option { "Technology" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Stakeholder Map" }
                textarea {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box; min-height: 70px; resize: vertical;",
                    placeholder: "List key stakeholders and their positions...",
                    oninput: move |e| stakeholder_map.set(e.value()),
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Implementation Gap (0–100): {implementation_gap()}%" }
                input {
                    r#type: "range",
                    min: "0",
                    max: "100",
                    step: "1",
                    value: "{implementation_gap()}",
                    style: "width: 100%; box-sizing: border-box; accent-color: if implementation_gap() > 50 { \"#f38ba8\" } else { \"#a6e3a1\" };",
                    oninput: move |e| implementation_gap.set(e.value().parse().unwrap_or(30)),
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Notes" }
                textarea {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box; min-height: 60px; resize: vertical;",
                    placeholder: "Additional notes...",
                    oninput: move |e| notes.set(e.value()),
                }
            }

            div {
                style: "background: #11111b; padding: 16px; border-radius: 8px; border-left: 4px solid #89b4fa; flex: 1;",
                h3 { style: "margin-top: 0; color: #89b4fa; font-size: 0.9rem;", "QualiaDB Output" }
                div { style: "font-size: 0.8rem; color: #a6adc8; display: flex; flex-direction: column; gap: 4px;",
                    div { "Stage: {policy_stage()}" }
                    div { "Instrument: {policy_instrument()}" }
                    div { "Lens: {theoretical_lens()}" }
                    div { "Issue: {issue_area()}" }
                    div { style: "color: if implementation_gap() > 50 { \"#f38ba8\" } else { \"#a6e3a1\" };", "Implementation Gap: {implementation_gap()}%" }
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 8px;", "QualiaDB → deontic logic | epistemic engine | graph stakeholder network" }
            }
        }
    }
}
