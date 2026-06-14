use dioxus::prelude::*;

#[component]
pub fn PoliticalScienceQapp() -> Element {
    let mut subfield = use_signal(|| "Comparative Politics".to_string());
    let mut regime_type = use_signal(|| "Liberal Democracy".to_string());
    let mut theoretical_framework = use_signal(|| "Rational Choice".to_string());
    let mut electoral_system = use_signal(|| "Proportional Representation".to_string());
    let mut polity_score = use_signal(|| 8i32);
    let mut country = use_signal(|| String::new());
    let mut policy_area = use_signal(|| "Fiscal Policy".to_string());
    let mut analysis_notes = use_signal(|| String::new());

    let subfields = [
        "Comparative Politics", "International Relations", "Political Theory",
        "Public Administration", "Public Policy", "Political Economy",
        "Electoral Studies", "Security Studies", "Political Sociology",
    ];
    let regimes = [
        "Liberal Democracy", "Electoral Democracy", "Hybrid Regime",
        "Competitive Authoritarianism", "Hegemonic Authoritarianism",
        "Closed Autocracy", "Theocracy", "Military Junta",
        "Federal Republic", "Constitutional Monarchy",
    ];
    let frameworks = [
        "Rational Choice", "Institutional", "Structural-Functional",
        "Critical Theory", "Constructivism", "Realism (IR)",
        "Liberalism (IR)", "Neo-Gramscian", "Deliberative Democracy",
        "Postcolonial Political Theory",
    ];
    let electoral_systems = [
        "Proportional Representation", "First Past the Post",
        "Two-Round System", "Mixed-Member Proportional",
        "Single Transferable Vote", "Party List", "Borda Count",
        "Instant Runoff", "No Elections (Authoritarian)",
    ];
    let policies = [
        "Fiscal Policy", "Monetary Policy", "Social Policy",
        "Foreign Policy", "Security / Defence Policy",
        "Environmental Policy", "Healthcare Policy", "Education Policy",
        "Migration Policy", "Trade Policy",
    ];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #89b4fa; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Political Science" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Subfield" }
                    select {
                        value: "{subfield}",
                        onchange: move |e| subfield.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in subfields { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Regime Type" }
                    select {
                        value: "{regime_type}",
                        onchange: move |e| regime_type.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in regimes { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Framework" }
                    select {
                        value: "{theoretical_framework}",
                        onchange: move |e| theoretical_framework.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in frameworks { option { value: "{x}", "{x}" } }
                    }
                }
            }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Electoral System" }
                    select {
                        value: "{electoral_system}",
                        onchange: move |e| electoral_system.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in electoral_systems { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Policy Area" }
                    select {
                        value: "{policy_area}",
                        onchange: move |e| policy_area.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in policies { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Country / Polity" }
                    input {
                        type: "text", placeholder: "e.g. France, Nigeria, EU…",
                        value: "{country}",
                        oninput: move |e| country.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Polity Score (−10 autocracy → +10 democracy): {polity_score}" }
                input {
                    type: "range", min: "-10", max: "10",
                    value: "{polity_score}",
                    oninput: move |e| polity_score.set(e.value().parse().unwrap_or(8)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Analysis Notes" }
                textarea {
                    value: "{analysis_notes}",
                    oninput: move |e| analysis_notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 60px;"
                }
            }

            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #89b4fa; display: flex; gap: 16px; flex-wrap: wrap;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{subfield}" }
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{regime_type}" }
                span { style: "font-size: 0.8rem; color: #89b4fa; font-weight: bold;", "Polity: {polity_score:+}" }
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{theoretical_framework}" }
                div { style: "font-size: 0.75rem; color: #585b70; width: 100%;", "QualiaDB → deontic logic | graph theory | epistemic certainty engine" }
            }
        }
    }
}
